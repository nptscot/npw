use std::collections::HashSet;

use anyhow::Result;
use enum_map::EnumMap;
use geo::LineString;
use geojson::{feature::Id, GeoJson};
use graph::{Graph, PathStep, Position, RoadID};

use crate::join_lines::KeyedLineString;
use crate::route_snapper::make_route_snapper_feature;
use crate::{Dir, InfraType, MapModel, Route, Tier};

impl MapModel {
    // TODO Old case
    pub fn set_route(&mut self, edit_id: Option<usize>, route: Route) -> Result<()> {
        if edit_id.is_none() {
            return self.add_new_route(route);
        }

        let original = if let Some(id) = edit_id {
            match self.routes.remove(&id) {
                Some(route) => Some(route),
                None => bail!("Unknown route {id}"),
            }
        } else {
            None
        };

        // Check for overlaps
        let used_roads = self.used_roads();
        for (r, _) in &route.roads {
            if used_roads.contains(r) {
                // Restore the original
                if let (Some(id), Some(route)) = (edit_id, original) {
                    self.routes.insert(id, route);
                }

                bail!("Another route already crosses the same road {r:?}");
            }
        }

        let id = match edit_id {
            Some(id) => id,
            None => {
                let id = self.id_counter;
                self.id_counter += 1;
                id
            }
        };
        self.routes.insert(id, route);
        self.recalculate_after_edits();
        Ok(())
    }

    fn add_new_route(&mut self, orig_route: Route) -> Result<()> {
        let used_roads = self.used_roads();

        // TODO Refactor
        // Split when:
        // - the auto-recommended infrastructure type changes
        // - the route crosses something existing
        #[derive(PartialEq)]
        enum Case {
            AlreadyExists,
            New(InfraType),
        }
        let case = |(r, _)| {
            if used_roads.contains(&r) {
                Case::AlreadyExists
            } else {
                Case::New(self.best_infra_type(r))
            }
        };

        let mut new_routes = Vec::new();
        for roads in orig_route.roads.chunk_by(|a, b| case(*a) == case(*b)) {
            let infra_type = match case(roads[0]) {
                Case::AlreadyExists => {
                    // TODO Should we modify that route and add to its notes or description? What
                    // if the tier or something else differs?
                    continue;
                }
                Case::New(infra_type) => infra_type,
            };

            let linestring = glue_route(&self.graph, roads);

            new_routes.push(Route {
                // TODO This'll have so many waypoints! Can trim this down. But also think through
                // the UI for the user to force splits -- maybe it doubles as the waypoints
                feature: make_route_snapper_feature(&self.graph, roads, &linestring),
                name: orig_route.name.clone(),
                notes: orig_route.notes.clone(),
                roads: roads.to_vec(),
                infra_type,
                tier: orig_route.tier,
            });
        }

        for route in new_routes {
            let route_id = self.id_counter;
            self.id_counter += 1;
            self.routes.insert(route_id, route);
        }
        self.recalculate_after_edits();

        Ok(())
    }

    pub fn delete_route(&mut self, id: usize) -> Result<()> {
        if self.routes.remove(&id).is_some() {
            self.recalculate_after_edits();
            return Ok(());
        }
        bail!("Unknown route {id}");
    }

    pub fn clear_all_routes(&mut self) {
        self.routes.clear();
        self.id_counter = 0;
        self.recalculate_after_edits();
    }

    pub fn to_routes_gj(&self) -> GeoJson {
        let mut features = Vec::new();
        for (id, route) in &self.routes {
            let mut f = route.feature.clone();
            f.id = Some(Id::Number((*id).into()));
            f.set_property("name", route.name.clone());
            f.set_property("notes", route.notes.clone());
            f.set_property(
                "infra_type",
                serde_json::to_value(&route.infra_type).unwrap(),
            );
            f.set_property("tier", serde_json::to_value(&route.tier).unwrap());
            features.push(f);
        }
        GeoJson::from(features)
    }

    /// Returns the number of edits
    pub fn import_existing_routes(&mut self) -> usize {
        let used_roads = self.used_roads();
        let mut imports = Vec::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            let road_id = RoadID(idx);
            if used_roads.contains(&road_id) {
                continue;
            }
            let Some(infra_type) = crate::existing::classify(&road.osm_tags) else {
                continue;
            };
            if !matches!(
                infra_type,
                InfraType::SegregatedWide | InfraType::OffRoad | InfraType::SegregatedNarrow,
            ) {
                continue;
            }
            imports.push((road_id, infra_type));
        }

        // TODO Can we detect the tier, or should this entire "import" feature go away and be
        // user-driven?
        self.import_roads(imports, Tier::LocalAccess)
    }

    /// Returns the number of edits
    pub fn import_core_network(&mut self) -> usize {
        let used_roads = self.used_roads();
        let mut imports: EnumMap<Tier, Vec<(RoadID, InfraType)>> = EnumMap::default();

        for idx in 0..self.graph.roads.len() {
            let road_id = RoadID(idx);
            if used_roads.contains(&road_id) {
                continue;
            }
            if let Some(tier) = self.core_network[idx] {
                imports[tier].push((road_id, self.best_infra_type(road_id)));
            }
        }

        let mut edits = 0;
        for (tier, roads) in imports {
            edits += self.import_roads(roads, tier)
        }
        edits
    }

    /// Split a route into sections, returning a FeatureCollection
    pub fn autosplit_route(&self, route: Vec<(RoadID, Dir)>) -> Result<String> {
        let used_roads = self.used_roads();

        // Split when:
        // - the auto-recommended infrastructure type changes
        // - the route crosses something existing
        #[derive(PartialEq)]
        enum Case {
            AlreadyExists,
            New(InfraType),
        }
        let case = |(r, _)| {
            if used_roads.contains(&r) {
                Case::AlreadyExists
            } else {
                Case::New(self.best_infra_type(r))
            }
        };

        let mut sections = Vec::new();
        for roads in route.chunk_by(|a, b| case(*a) == case(*b)) {
            let c = case(roads[0]);
            let mut f = self
                .graph
                .mercator
                .to_wgs84_gj(&glue_route(&self.graph, roads));
            match c {
                Case::AlreadyExists => {
                    f.set_property("kind", "overlap");
                }
                Case::New(infra_type) => {
                    f.set_property("kind", "new");
                    f.set_property("infra_type", serde_json::to_value(&infra_type).unwrap());
                }
            }
            sections.push(f);
        }
        Ok(serde_json::to_string(&GeoJson::from(sections))?)
    }

    fn import_roads(&mut self, imports: Vec<(RoadID, InfraType)>, tier: Tier) -> usize {
        // Create individual segments to import
        let mut pieces = Vec::new();
        for (id, infra_type) in imports {
            pieces.push(KeyedLineString {
                linestring: self.graph.roads[id.0].linestring.clone(),
                ids: vec![(id, Dir::Forwards)],
                key: infra_type,
            });
        }

        // Group them in hopefully meaningful chunks
        // TODO Could try more aggressive joining after this, but this one seems to work fine so
        // far. Although oddly it seems to handle more than just degree 2...
        pieces = crate::join_lines::collapse_degree_2(pieces);
        let changes = pieces.len();

        for line in pieces {
            let route = Route {
                feature: make_route_snapper_feature(&self.graph, &line.ids, &line.linestring),
                // Pick the first name
                // TODO Does this short-circuit?
                name: line
                    .ids
                    .iter()
                    .filter_map(|(r, _)| self.graph.roads[r.0].osm_tags.get("name").cloned())
                    .next()
                    .unwrap_or_else(String::new),
                notes: "imported from existing network".to_string(),
                roads: line.ids,
                infra_type: line.key,
                tier,
            };
            let route_id = self.id_counter;
            self.id_counter += 1;
            self.routes.insert(route_id, route);
        }

        self.recalculate_after_edits();
        changes
    }

    fn used_roads(&self) -> HashSet<RoadID> {
        self.routes
            .values()
            .flat_map(|route| route.roads.iter().map(|(r, _)| *r))
            .collect()
    }
}

// TODO Upstream to graph
fn glue_route(graph: &Graph, roads: &[(RoadID, Dir)]) -> LineString {
    graph::Route {
        start: start_pos(roads[0], graph),
        end: end_pos(*roads.last().unwrap(), graph),
        steps: roads
            .into_iter()
            .cloned()
            .map(|(road, dir)| PathStep::Road {
                road,
                forwards: matches!(dir, Dir::Forwards),
            })
            .collect(),
    }
    .linestring(graph)
}

// TODO Upstream to graph
pub fn start_pos((r, dir): (RoadID, Dir), graph: &Graph) -> Position {
    let road = &graph.roads[r.0];
    Position {
        road: r,
        fraction_along: if matches!(dir, Dir::Forwards) {
            0.0
        } else {
            1.0
        },
        intersection: if matches!(dir, Dir::Forwards) {
            road.src_i
        } else {
            road.dst_i
        },
    }
}

pub fn end_pos((road, dir): (RoadID, Dir), graph: &Graph) -> Position {
    let opposite = match dir {
        Dir::Forwards => Dir::Backwards,
        Dir::Backwards => Dir::Forwards,
    };
    start_pos((road, opposite), graph)
}

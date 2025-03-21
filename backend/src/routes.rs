use std::collections::HashSet;

use anyhow::Result;
use geo::{Euclidean, Length};
use geojson::{Feature, GeoJson};
use graph::{Graph, PathStep, Position, RoadID};

use crate::join_lines::KeyedLineString;
use crate::route_snapper::make_route_snapper_feature;
use crate::{
    level_of_service::get_level_of_service, Dir, InfraType, LevelOfService, MapModel, Route, Tier,
};

impl MapModel {
    pub fn set_route(&mut self, edit_id: Option<usize>, orig_route: Route) -> Result<()> {
        // If we're editing an existing route, first delete it
        if let Some(id) = edit_id {
            if self.routes.remove(&id).is_none() {
                bail!("Unknown route {id}");
            }
        }

        let used_roads = self.used_roads();

        // This is deliberately separate from autosplit_route, which splits by more categories
        //
        // Split when:
        // - the auto-recommended infrastructure type changes (unless manually overriden)
        // - the route crosses something existing
        // - the infrastructure type does or does not fit in the available streetspace
        #[derive(PartialEq)]
        enum Case {
            AlreadyExists,
            // bool is fits or not
            New(InfraType, bool),
        }
        let case = |(r, _)| {
            if used_roads.contains(&r) {
                Case::AlreadyExists
            } else {
                if orig_route.override_infra_type {
                    let it = orig_route.infra_type;
                    Case::New(it, self.does_infra_type_fit(r, it))
                } else {
                    let it = self.best_infra_type(r);
                    Case::New(it, self.does_infra_type_fit(r, it))
                }
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
                Case::New(infra_type, _) => infra_type,
            };

            let linestring = glue_route(&self.graph, roads).linestring(&self.graph);

            new_routes.push(Route {
                feature: make_route_snapper_feature(&self.graph, roads, &linestring),
                name: orig_route.name.clone(),
                notes: orig_route.notes.clone(),
                roads: roads.to_vec(),
                infra_type,
                override_infra_type: orig_route.override_infra_type,
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

    pub fn delete_routes(&mut self, ids: Vec<usize>) -> Result<()> {
        for id in ids {
            if !self.routes.remove(&id).is_some() {
                bail!("Unknown route {id}");
            }
        }
        self.recalculate_after_edits();
        return Ok(());
    }

    pub fn clear_all_routes(&mut self) {
        self.routes.clear();
        self.id_counter = 0;
        self.recalculate_after_edits();
    }

    pub fn get_all_routes(&self) -> GeoJson {
        GeoJson::from(
            self.routes
                .iter()
                .map(|(id, r)| r.to_gj(*id))
                .collect::<Vec<_>>(),
        )
    }

    pub fn get_route(&self, id: usize) -> Result<Feature> {
        let Some(route) = self.routes.get(&id) else {
            bail!("No route {id}");
        };
        Ok(route.to_gj(id))
    }

    /// Returns the number of edits
    pub fn import_existing_routes(&mut self, only_some_infra_types: bool) -> usize {
        let used_roads = self.used_roads();
        let mut imports = Vec::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            let road_id = RoadID(idx);
            if used_roads.contains(&road_id) {
                continue;
            }
            let Some(infra_type) =
                crate::existing::classify_existing_osm_infra(self.is_offroad[idx], &road.osm_tags)
            else {
                continue;
            };

            if only_some_infra_types {
                if !matches!(infra_type, InfraType::Segregated | InfraType::OffRoad) {
                    continue;
                }
            } else {
                // We could check if the current LoS is already high, but if it is, it may be
                // because it's an existing separately tagged cycleway that has no modelled traffic
                // volume.
                if get_level_of_service(
                    infra_type,
                    self.speeds[idx],
                    self.traffic_volumes[idx],
                    self.within_settlement[idx],
                ) != LevelOfService::High
                {
                    continue;
                }
                // Always skip footways. The user can trace over these if desired.
                if infra_type == InfraType::SharedFootway {
                    continue;
                }
            }
            let tier = if infra_type == InfraType::Segregated {
                Tier::Primary
            } else {
                Tier::Secondary
            };

            imports.push((road_id, infra_type, tier));
        }

        self.import_roads(imports)
    }

    /// Returns the number of edits
    pub fn import_core_network(&mut self) -> usize {
        let used_roads = self.used_roads();
        let mut imports = Vec::new();

        for idx in 0..self.graph.roads.len() {
            let road_id = RoadID(idx);
            if used_roads.contains(&road_id) {
                continue;
            }
            if let Some(tier) = self.core_network[idx] {
                imports.push((road_id, self.best_infra_type(road_id), tier));
            }
        }

        self.import_roads(imports)
    }

    /// Split a route into sections, returning a FeatureCollection
    pub fn autosplit_route(
        &self,
        editing_route_id: Option<usize>,
        route: Vec<(RoadID, Dir)>,
        override_infra_type: Option<InfraType>,
    ) -> Result<String> {
        let mut used_roads = self.used_roads();
        if let Some(id) = editing_route_id {
            for (r, _) in &self.routes[&id].roads {
                used_roads.remove(r);
            }
        }

        // Split when:
        // - the auto-recommended or manual infrastructure type changes
        // - the route crosses something existing (except the existing route)
        // - the infrastructure type does or does not fit in the available streetspace
        // - the gradient group changes
        // - the level of service changes
        #[derive(PartialEq)]
        enum Case {
            // bool is fits or not, the str is gradient_group
            AlreadyExists(&'static str, LevelOfService),
            New(InfraType, bool, &'static str, LevelOfService),
        }
        let case = |(r, _): (RoadID, _)| {
            let gradient = gradient_group(self.gradients[r.0]);
            if used_roads.contains(&r) {
                Case::AlreadyExists(gradient, self.los[r.0])
            } else if let Some(it) = override_infra_type {
                let los = get_level_of_service(
                    it,
                    self.speeds[r.0],
                    self.traffic_volumes[r.0],
                    self.within_settlement[r.0],
                );
                Case::New(it, self.does_infra_type_fit(r, it), gradient, los)
            } else {
                let it = self.best_infra_type(r);
                let los = get_level_of_service(
                    it,
                    self.speeds[r.0],
                    self.traffic_volumes[r.0],
                    self.within_settlement[r.0],
                );
                Case::New(it, self.does_infra_type_fit(r, it), gradient, los)
            }
        };

        let mut sections = Vec::new();
        for roads in route.chunk_by(|a, b| case(*a) == case(*b)) {
            let c = case(roads[0]);
            let linestring = glue_route(&self.graph, roads).linestring(&self.graph);
            let mut f = self.graph.mercator.to_wgs84_gj(&linestring);
            match c {
                Case::AlreadyExists(gradient, los) => {
                    f.set_property("kind", "overlap");
                    // Don't worry about other routes
                    f.set_property("fits", true);
                    f.set_property("gradient_group", gradient);
                    f.set_property("los", serde_json::to_value(&los).unwrap());
                }
                Case::New(infra_type, fits, gradient, los) => {
                    f.set_property("kind", "new");
                    f.set_property("infra_type", serde_json::to_value(&infra_type).unwrap());
                    f.set_property("fits", fits);
                    f.set_property("gradient_group", gradient);
                    f.set_property("los", serde_json::to_value(&los).unwrap());
                }
            }
            f.set_property("length", linestring.length::<Euclidean>());
            sections.push(f);
        }
        Ok(serde_json::to_string(&GeoJson::from(sections))?)
    }

    pub fn change_tier(&mut self, route_ids: Vec<usize>, tier: Tier) -> Result<()> {
        for id in route_ids {
            if let Some(route) = self.routes.get_mut(&id) {
                route.tier = tier;
            } else {
                bail!("Unknown route {id}");
            }
        }
        self.recalculate_after_edits();
        Ok(())
    }

    pub fn change_infra_type(
        &mut self,
        route_ids: Vec<usize>,
        infra_type: InfraType,
    ) -> Result<()> {
        for id in route_ids {
            if let Some(route) = self.routes.get_mut(&id) {
                if route.infra_type != infra_type {
                    route.infra_type = infra_type;
                    route.override_infra_type = true;
                }
            } else {
                bail!("Unknown route {id}");
            }
        }
        self.recalculate_after_edits();
        Ok(())
    }

    fn import_roads(&mut self, imports: Vec<(RoadID, InfraType, Tier)>) -> usize {
        // Create individual segments to import
        let mut pieces = Vec::new();
        for (id, infra_type, tier) in imports {
            pieces.push(KeyedLineString {
                linestring: self.graph.roads[id.0].linestring.clone(),
                ids: vec![(id, Dir::Forwards)],
                key: (infra_type, tier),
            });
        }

        // Group them in hopefully meaningful chunks
        // TODO Could try more aggressive joining after this, but this one seems to work fine so
        // far. Although oddly it seems to handle more than just degree 2...
        pieces = crate::join_lines::collapse_degree_2(pieces);
        let changes = pieces.len();

        for line in pieces {
            let (infra_type, tier) = line.key;
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
                infra_type,
                override_infra_type: false,
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
pub fn glue_route(graph: &Graph, roads: &[(RoadID, Dir)]) -> graph::Route {
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

impl Route {
    fn to_gj(&self, id: usize) -> Feature {
        let mut f = self.feature.clone();
        f.set_property("id", id);
        f.set_property("name", self.name.clone());
        f.set_property("notes", self.notes.clone());
        f.set_property(
            "infra_type",
            serde_json::to_value(&self.infra_type).unwrap(),
        );
        f.set_property("override_infra_type", self.override_infra_type);
        f.set_property("tier", serde_json::to_value(&self.tier).unwrap());
        f
    }
}

pub fn gradient_group(gradient: f64) -> &'static str {
    let g = gradient.abs();
    if g <= 3.0 {
        "<= 3%"
    } else if g <= 5.0 {
        "3 - 5%"
    } else if g <= 7.0 {
        "5 - 7%"
    } else if g <= 10.0 {
        "7 - 10%"
    } else {
        "> 10%"
    }
}

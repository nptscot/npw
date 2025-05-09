use std::collections::BTreeSet;

use geo::{BoundingRect, Geometry, GeometryCollection, Rect};
use geojson::FeatureCollection;
use graph::{IntersectionID, RoadID};
use petgraph::graphmap::UnGraphMap;

use crate::{utils::into_object_value, MapModel};

impl MapModel {
    // Fast enough to calculate immediately
    pub fn num_connected_components(&self) -> usize {
        let mut graph: UnGraphMap<IntersectionID, RoadID> = UnGraphMap::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            if self.infra_types[idx].is_some() {
                graph.add_edge(road.src_i, road.dst_i, road.id);
            }
        }

        petgraph::algo::kosaraju_scc(&graph).len()
    }

    // TODO Lift into graph from severance_snape and here?
    pub fn get_connected_components(&self) -> FeatureCollection {
        let mut graph: UnGraphMap<IntersectionID, RoadID> = UnGraphMap::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            if self.infra_types[idx].is_some() {
                graph.add_edge(road.src_i, road.dst_i, road.id);
            }
        }

        // (Roads, total length)
        let mut components: Vec<(BTreeSet<RoadID>, usize)> = Vec::new();
        for nodes in petgraph::algo::kosaraju_scc(&graph) {
            let roads = nodes_to_edges(self, nodes);
            let length = roads
                .iter()
                .map(|r| self.graph.roads[r.0].length_meters)
                .sum::<f64>()
                .round() as usize;
            components.push((roads, length));
        }
        components.sort_by_key(|(_, len)| *len);
        components.reverse();

        let mut features = Vec::new();
        let mut component_lengths = Vec::new();
        let mut component_bboxes = Vec::new();
        for (roads, length) in components {
            let component = component_lengths.len();
            component_lengths.push(length);

            let mut collection = Vec::new();
            for r in roads {
                let mut f = self.graph.roads[r.0].to_gj(&self.graph);
                f.set_property("component", component);
                features.push(f);

                // TODO Expensive, make a bbox accumulator
                collection.push(Geometry::LineString(
                    self.graph.roads[r.0].linestring.clone(),
                ));
            }
            let mut bbox: Rect = GeometryCollection(collection)
                .bounding_rect()
                .unwrap()
                .into();
            self.graph.mercator.to_wgs84_in_place(&mut bbox);
            component_bboxes.push(vec![bbox.min().x, bbox.min().y, bbox.max().x, bbox.max().y]);
        }

        FeatureCollection {
            features,
            bbox: None,
            foreign_members: Some(into_object_value(serde_json::json!({
                "component_lengths": component_lengths,
                "component_bboxes": component_bboxes,
            }))),
        }
    }
}

// Note this only works for connected components of nodes!
fn nodes_to_edges(map: &MapModel, nodes: Vec<IntersectionID>) -> BTreeSet<RoadID> {
    let mut edges = BTreeSet::new();
    for i in nodes {
        edges.extend(map.graph.intersections[i.0].roads.clone());
    }
    edges.retain(|r| map.infra_types[r.0].is_some());
    edges
}

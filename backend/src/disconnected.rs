use std::collections::BTreeSet;

use geojson::FeatureCollection;
use graph::{IntersectionID, RoadID};
use petgraph::graphmap::UnGraphMap;

use crate::MapModel;

impl MapModel {
    // TODO Lift into graph from severance_snape and here?
    pub fn get_connected_components(&self) -> FeatureCollection {
        let mut graph: UnGraphMap<IntersectionID, RoadID> = UnGraphMap::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            if self.infra_types[idx].is_some() {
                graph.add_edge(road.src_i, road.dst_i, road.id);
            }
        }

        let mut features = Vec::new();
        let mut component_sizes = Vec::new();
        for nodes in petgraph::algo::kosaraju_scc(&graph) {
            let component = component_sizes.len();
            let roads = nodes_to_edges(self, nodes);
            component_sizes.push(roads.len());

            for r in roads {
                let mut f = self.graph.roads[r.0].to_gj(&self.graph);
                f.set_property("component", component);
                features.push(f);
            }
        }
        component_sizes.sort();
        component_sizes.reverse();

        FeatureCollection {
            features,
            bbox: None,
            foreign_members: Some(
                serde_json::json!({
                    "component_sizes": component_sizes,
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
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

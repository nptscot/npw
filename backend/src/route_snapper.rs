use route_snapper_graph::{Edge, NodeID, RouteSnapperMap};

use graph::Direction;

use crate::MapModel;

impl MapModel {
    pub fn to_route_snapper_graph(&self) -> RouteSnapperMap {
        let profile = self.graph.profile_names["bicycle"];

        let mut nodes = Vec::new();
        for i in &self.graph.intersections {
            nodes.push(self.graph.mercator.to_wgs84(&i.point).into());
        }

        let mut edges = Vec::new();
        for r in &self.graph.roads {
            if r.access[profile.0] == Direction::None {
                continue;
            }
            edges.push(Edge {
                node1: NodeID(r.src_i.0 as u32),
                node2: NodeID(r.dst_i.0 as u32),
                geometry: self.graph.mercator.to_wgs84(&r.linestring),
                name: None,

                // Isn't serialized, doesn't matter
                length_meters: 0.0,
                forward_cost: None,
                backward_cost: None,
            });
        }

        RouteSnapperMap {
            nodes,
            edges,
            override_forward_costs: Vec::new(),
            override_backward_costs: Vec::new(),
        }
    }
}

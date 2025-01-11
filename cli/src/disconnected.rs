use std::collections::BTreeSet;

use anyhow::Result;
use log::info;
use petgraph::graphmap::UnGraphMap;
use utils::osm2graph::{EdgeID, Graph, IntersectionID};

const MIN_ROADS_PER_COMPONENT: usize = 100;

pub fn remove_disconnected_components(graph: &mut Graph) -> Result<()> {
    let mut scc_graph: UnGraphMap<IntersectionID, EdgeID> = UnGraphMap::new();
    for edge in &graph.edges {
        scc_graph.add_edge(edge.src, edge.dst, edge.id);
    }

    let mut components: Vec<BTreeSet<EdgeID>> = Vec::new();
    for nodes in petgraph::algo::kosaraju_scc(&scc_graph) {
        components.push(nodes_to_edges(graph, nodes));
    }
    components.sort_by_key(|scc| scc.len());
    components.reverse();

    let mut remove_edges = BTreeSet::new();
    for scc in components {
        if scc.len() < MIN_ROADS_PER_COMPONENT {
            info!("Removing component of roads with only {}", scc.len());
            remove_edges.extend(scc);
        }
    }

    info!("Removing {} disconnected roads", remove_edges.len());

    Ok(())
}

// Note this only works for connected components of nodes!
fn nodes_to_edges(graph: &Graph, nodes: Vec<IntersectionID>) -> BTreeSet<EdgeID> {
    let mut edges = BTreeSet::new();
    for i in nodes {
        edges.extend(graph.intersections[i.0].edges.clone());
    }
    edges
}

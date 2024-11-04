use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

pub(crate) fn get_shortest_path(graph: Graph<u64, u64>, position_to_node: HashMap<(u64, u64), NodeIndex>, node_to_position: &HashMap<NodeIndex, (u64, u64)>, start: NodeIndex, end: NodeIndex) -> Option<(u64, Vec<NodeIndex>)> {
    // find the shortest path
    let start_pos = position_to_node.iter().find(|(_, v)| **v == start).unwrap().0;
    let end_pos = position_to_node.iter().find(|(_, v)| **v == end).unwrap().0;
    println!("Calculating shortest path from {},{} to {},{}", start_pos.0, start_pos.1, end_pos.0, end_pos.1);
    let path = petgraph::algo::astar(&graph, start, |finish| finish == end, |e| *e.weight(), |_| 0);
    path
}
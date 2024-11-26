use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

pub(crate) fn get_shortest_path(graph: Graph<u64, u64>, position_to_node: HashMap<(u64, u64), NodeIndex>, start: NodeIndex, end: NodeIndex) -> Option<(u64, Vec<NodeIndex>)> {
    if position_to_node.is_empty() {
        return None;
    }
    // find the shortest path
    let start_pos = position_to_node.iter().find(|(_, v)| **v == start).unwrap().0;
    let end_pos = position_to_node.iter().find(|(_, v)| **v == end).unwrap().0;
    println!("Calculating shortest path from {},{} to {},{}", start_pos.0, start_pos.1, end_pos.0, end_pos.1);
    let path = petgraph::algo::astar(&graph, start, |finish| finish == end, |e| *e.weight(), |_| 0);
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shortest_path() {
        let mut graph = Graph::new();
        let a = graph.add_node(0);
        let b = graph.add_node(1);
        let c = graph.add_node(2);
        let d = graph.add_node(3);
        let e = graph.add_node(4);
        let f = graph.add_node(5);
        let g = graph.add_node(6);
        let h = graph.add_node(7);
        let i = graph.add_node(8);
        let j = graph.add_node(9);
        graph.add_edge(a, b, 1);
        graph.add_edge(b, c, 1);
        graph.add_edge(c, d, 1);
        graph.add_edge(d, e, 1);
        graph.add_edge(e, f, 1);
        graph.add_edge(f, g, 1);
        graph.add_edge(g, h, 1);
        graph.add_edge(h, i, 1);
        graph.add_edge(i, j, 1);
        graph.add_edge(a, d, 1);
        graph.add_edge(d, g, 1);
        graph.add_edge(g, j, 1);
        graph.add_edge(a, e, 1);
        graph.add_edge(e, i, 1);
        graph.add_edge(a, f, 1);
        graph.add_edge(f, h, 1);
        graph.add_edge(a, g, 1);
        graph.add_edge(g, i, 1);
        let mut position_to_node: HashMap<(u64,u64),NodeIndex> = HashMap::new();
        position_to_node.insert((0, 0), a);
        position_to_node.insert((1, 0), b);
        position_to_node.insert((2, 0), c);
        position_to_node.insert((3, 0), d);
        position_to_node.insert((4, 0), e);
        position_to_node.insert((0, 1), f);
        position_to_node.insert((1, 1), g);
        position_to_node.insert((2, 1), h);
        position_to_node.insert((3, 1), i);
        position_to_node.insert((4, 1), j);
        let path = get_shortest_path(graph, position_to_node, a, j);
        assert_eq!(path, Some((2, vec![a, g, j])));
    }

    #[test]
    fn test_get_shortest_path_no_nodes() {
        let graph = Graph::new();
        let position_to_node: HashMap<(u64,u64),NodeIndex> = HashMap::new();
        let path = get_shortest_path(graph, position_to_node, NodeIndex::new(0), NodeIndex::new(1));
        assert_eq!(path.is_none(), true);
    }

    #[test]
    fn test_get_shortest_path_no_path() {
        let mut graph = Graph::new();
        let a = graph.add_node(0);
        let b = graph.add_node(1);
        let mut position_to_node: HashMap<(u64,u64),NodeIndex> = HashMap::new();
        position_to_node.insert((0, 0), a);
        position_to_node.insert((1, 0), b);
        let path = get_shortest_path(graph, position_to_node, a, b);
        assert_eq!(path.is_none(), true);
    }
}

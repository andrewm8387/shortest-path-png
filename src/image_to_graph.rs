use std::collections::HashMap;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use crate::image_structs::RgbColor;
use crate::image_structs::MyImage;

pub(crate) fn image_to_graph(img: MyImage) -> (Graph<u32, u32>, HashMap<(u32, u32), NodeIndex>, NodeIndex, NodeIndex) {
    let mut graph = Graph::<u32, u32>::new();
    let width = img.width;
    let height = img.height;
    let mut position_to_node: HashMap<(u32, u32), NodeIndex> = HashMap::new();
    let mut node_to_position: HashMap<NodeIndex, (u32, u32)> = HashMap::new();
    let mut start = None;
    let mut end = None;
    for x in 0..width {
        for y in 0..height {
            match img.get_pixel(x, y).color {
                RgbColor::GREEN => {
                    let id = graph.add_node(1);
                    position_to_node.insert((x, y), id);
                    node_to_position.insert(id, (x,y));
                    start = Some(id);
                }
                RgbColor::RED => {
                    let id = graph.add_node(1);
                    position_to_node.insert((x, y), id);
                    node_to_position.insert(id, (x,y));
                    end = Some(id);
                }
                RgbColor::BLACK => {
                    // do nothing
                }
                RgbColor::UNDEFINED => {
                    let id = graph.add_node(1);
                    position_to_node.insert((x, y), id);
                    node_to_position.insert(id, (x,y));
                }
            }
        }
    }
    for x in 0..width {
        for y in 0..height {
            if !position_to_node.get(&(x, y)).is_none() {
                let node = *position_to_node.get(&(x, y)).unwrap();
                // check if there is a node to the left, right, up, and down
                if x > 0 {
                    match position_to_node.get(&(x - 1, y)) {
                        Some(left) => {
                            graph.add_edge(node, *left, *graph.node_weight(*left).unwrap());
                        }
                        None => {
                            // do nothing
                        }
                    }
                }
                if x < width - 1 {
                    match position_to_node.get(&(x + 1, y)) {
                        Some(right) => {
                            graph.add_edge(node, *right, *graph.node_weight(*right).unwrap());
                        }
                        None => {
                            // do nothing
                        }
                    }
                }
                if y > 0 {
                    match position_to_node.get(&(x, y - 1)) {
                        Some(up) => {
                            graph.add_edge(node, *up, *graph.node_weight(*up).unwrap());
                        }
                        None => {
                            // do nothing
                        }
                    }
                }
                if y < height - 1 {
                    match position_to_node.get(&(x, y + 1)) {
                        Some(down) => {
                            graph.add_edge(node, *down, *graph.node_weight(*down).unwrap());
                        }
                        None => {
                            // do nothing
                        }
                    }
                }
            }
        }
    }
    match start {
        Some(start) => {
            println!("Start node found at {},{}", node_to_position.get(&start).unwrap().0, node_to_position.get(&start).unwrap().1);
        }
        None => {
            println!("No start node found, defaulting start to top left corner (0, 0)");
            let default_start = match position_to_node.get(&(0, 0)) {
                Some(start) => *start,
                None => {
                    panic!("No node at 0,0");
                }
            };
            start = Some(default_start);
        }
    }
    
    match end {
        Some(end) => {
            println!("End node found at {},{}", node_to_position.get(&end).unwrap().0, node_to_position.get(&end).unwrap().1);
        }
        None => {
            println!("No end node found, defaulting end to bottom right corner ({}, {})", width-1, height-1);
            let default_end = match position_to_node.get(&(width - 1, height - 1)) {
                Some(end) => *end,
                None => {
                    panic!("No node at {},{}", width - 1, height - 1);
                }
            };
            end = Some(default_end);
        }
    }

    (graph, position_to_node, start.unwrap(), end.unwrap())
}
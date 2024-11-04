use num_traits::FloatConst;
use std::collections::HashMap;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use crate::image_structs::RgbColor;
use crate::image_structs::MyImage;

pub(crate) fn image_to_graph(img: MyImage) -> (Graph<u64, u64>, HashMap<(u64, u64), NodeIndex>, HashMap<NodeIndex, (u64, u64)>, NodeIndex, NodeIndex) {
    let mut graph = Graph::<u64, u64>::new();
    let width = img.width as u64;
    let height = img.height as u64;
    let mut position_to_node: HashMap<(u64, u64), NodeIndex> = HashMap::new();
    let mut node_to_position: HashMap<NodeIndex, (u64, u64)> = HashMap::new();
    let mut start = None;
    let mut end = None;
    for x in 0..width {
        for y in 0..height {
            match img.get_pixel(x as u32, y as u32).color {
                RgbColor::GREEN => {
                    let id = graph.add_node(100);
                    position_to_node.insert((x, y), id);
                    node_to_position.insert(id, (x,y));
                    start = Some(id);
                }
                RgbColor::RED => {
                    let id = graph.add_node(100);
                    position_to_node.insert((x, y), id);
                    node_to_position.insert(id, (x,y));
                    end = Some(id);
                }
                RgbColor::BLACK | RgbColor::OLIVE | RgbColor::PURPLE => {
                    // do nothing
                }
                RgbColor::UNDEFINED | RgbColor::YELLOW | RgbColor::LIGHT_BROWN | RgbColor::GREY => {
                    let id = graph.add_node(100);
                    position_to_node.insert((x, y), id);
                    node_to_position.insert(id, (x,y));
                }
                RgbColor::WHITE | RgbColor::LIGHT_YELLOW => {
                    let id = graph.add_node(125);
                    position_to_node.insert((x, y), id);
                    node_to_position.insert(id, (x,y));
                }
                RgbColor::LIGHT_GREEN => {
                    let id = graph.add_node(167);
                    position_to_node.insert((x, y), id);
                    node_to_position.insert(id, (x,y));
                }
                RgbColor::MEDIUM_GREEN => {
                    let id = graph.add_node(333);
                    position_to_node.insert((x, y), id);
                    node_to_position.insert(id, (x,y));
                }
                RgbColor::DARK_GREEN => {
                    let id = graph.add_node(1000);
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
                if x > 0 && y > 0 {
                    match position_to_node.get(&(x - 1, y - 1)) {
                        Some(up_left) => {
                            graph.add_edge(node, *up_left, (*graph.node_weight(*up_left).unwrap() as f64 * f64::SQRT_2()) as u64);
                        }
                        None => {
                            // do nothing
                        }
                    }
                }
                if x < width - 1 && y > 0 {
                    match position_to_node.get(&(x + 1, y - 1)) {
                        Some(up_right) => {
                            graph.add_edge(node, *up_right, (*graph.node_weight(*up_right).unwrap() as f64 * f64::SQRT_2()) as u64);
                        }
                        None => {
                            // do nothing
                        }
                    }
                }
                if x > 0 && y < height - 1 {
                    match position_to_node.get(&(x - 1, y + 1)) {
                        Some(down_left) => {
                            graph.add_edge(node, *down_left, (*graph.node_weight(*down_left).unwrap() as f64 * f64::SQRT_2()) as u64);
                        }
                        None => {
                            // do nothing
                        }
                    }
                }
                if x < width - 1 && y < height - 1 {
                    match position_to_node.get(&(x + 1, y + 1)) {
                        Some(down_right) => {
                            graph.add_edge(node, *down_right, (*graph.node_weight(*down_right).unwrap() as f64 * f64::SQRT_2()) as u64);
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
    graph_to_heatmap(&graph, &position_to_node, width, height);
    (graph, position_to_node, node_to_position, start.unwrap(), end.unwrap())
}

pub(crate) fn graph_to_heatmap(graph: &Graph<u64, u64>, position_to_node: &HashMap<(u64, u64), NodeIndex>, width: u64, height: u64) {
    // create a heatmap of the speeds
    let mut img = image::RgbImage::new(width as u32, height as u32);
    for x in 0..width {
        for y in 0..height {
            match position_to_node.get(&(x, y)) {
                Some(node) => {
                    let speed = *graph.node_weight(*node).unwrap();
                    let val  = (25500.0 / speed as f64) as u8;
                    let color = image::Rgb([val, val, val]);
                    img.put_pixel(x as u32, y as u32, color);
                }
                None => {
                    let color = image::Rgb([0, 0, 0]);
                    img.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
    img.save("heatmap.png").expect("fuck")
}
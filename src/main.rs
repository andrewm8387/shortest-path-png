mod get_shortest_path;
mod image_structs;
mod image_to_graph;
mod process_image;
mod parse_input;

fn main() {
    // parse user input
    let args: Vec<String> = std::env::args().collect();
    let file_path = match parse_input::parse(args) {
        None => {
            return
        }
        Some(file) => {
            file
        }
    };

    // process image
    println!("Processing image");
    let img = process_image::get_image(&file_path);
    // convert image to graph
    println!("Converting image to graph representation");
    let (graph, position_to_node, node_to_position, start, end) = image_to_graph::image_to_graph(img);
    // get shortest path
    println!("Calculating shortest path");
    let path = get_shortest_path::get_shortest_path(graph, position_to_node, start, end);
    // print path
    match path {
        Some((distance, path)) => {
            println!("Shortest path found with distance {}", distance);
            // print path to output image
            println!("Exporting image with path");
            process_image::add_path_to_image(path, node_to_position, &file_path);
        }
        None => {
            println!("No path found");
        }
    }
}
mod get_shortest_path;
mod image_structs;
mod image_to_graph;
mod process_image;
fn main() {
    // parse user input
    let args: Vec<String> = std::env::args().collect();
    //validate user input
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }
    let file_path = &args[1];
    if !file_path.ends_with(".png") {
        eprintln!("file type must be png");
        return;
    }
    // process image
    println!("Processing image");
    let img = process_image::get_image(file_path);
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
            process_image::add_path_to_image(path, node_to_position, file_path);
        }
        None => {
            println!("No path found");
        }
    }
}
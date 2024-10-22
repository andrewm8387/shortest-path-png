use std::collections::HashMap;
use image::{GenericImage, GenericImageView, Pixel};
use petgraph::graph::NodeIndex;
use crate::image_structs::{MyImage, MyPixel};
pub(crate) fn get_image(file_path: &str) -> MyImage {
    //get png from file
    let img = image::open(file_path).unwrap();
    let width = img.width();
    let height = img.height();
    let mut pixels = Vec::new();
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let my_pixel = MyPixel::new(pixel[0], pixel[1], pixel[2]);
            pixels.push(my_pixel);
        }
    }
    MyImage {
        pixels,
        width,
        height
    }
}

pub fn add_path_to_image(path: Vec<NodeIndex>, position_to_node: &HashMap<(u32, u32), NodeIndex>, image: &str) -> () {
    let mut img = image::open(image).unwrap();
    for node in path {
        let position = position_to_node.iter().find(|(_, v)| **v == node).unwrap().0;
        let x = position.0;
        let y = position.1;
        img.put_pixel(x, y, image::Rgb([0, 0, 255]).to_rgba());
    }
    // save the image
    img.save("output.png").expect("fuck");
}
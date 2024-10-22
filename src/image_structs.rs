pub(crate) enum RgbColor {
    RED,
    GREEN,
    BLACK,
    UNDEFINED
}

pub(crate) struct MyPixel {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) color: RgbColor
}

impl MyPixel {
    pub(crate) fn new(r: u8, g: u8, b: u8) -> MyPixel {
        let color = match (r, g, b) {
            (255, 0, 0) => RgbColor::RED,
            (0, 255, 0) => RgbColor::GREEN,
            (0, 0, 0) => RgbColor::BLACK,
            _ => RgbColor::UNDEFINED
        };
        MyPixel {
            r,
            g,
            b,
            color
        }
    }
}


pub(crate) struct MyImage {
    pub(crate) pixels: Vec<MyPixel>,
    pub(crate) width: u32,
    pub(crate) height: u32
}

impl MyImage {
    pub(crate) fn get_pixel(&self, x: u32, y: u32) -> &MyPixel {
        &self.pixels[(y + x * self.height) as usize]
    }
}
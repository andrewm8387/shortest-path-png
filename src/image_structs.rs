pub(crate) enum RgbColor {
    RED,
    GREEN,
    DARK_GREEN,
    MEDIUM_GREEN,
    LIGHT_GREEN,
    BLACK,
    YELLOW,
    LIGHT_YELLOW,
    WHITE,
    LIGHT_BROWN,
    GREY,
    OLIVE,
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
            (51, 255, 0) => RgbColor::DARK_GREEN,
            (133, 255, 102) => RgbColor::MEDIUM_GREEN,
            (194, 255, 179) => RgbColor::LIGHT_GREEN,
            (0, 0, 0) => RgbColor::BLACK,
            (89, 89, 89) => RgbColor::BLACK,
            (102, 102, 102) => RgbColor::BLACK,
            (128, 128, 128) => RgbColor::BLACK,
            (255, 186, 54) => RgbColor::YELLOW,
            (255, 255, 255) => RgbColor::WHITE,
            (230, 166, 128) => RgbColor::LIGHT_BROWN,
            (240, 196, 171) => RgbColor::LIGHT_BROWN,
            (204, 204, 204) => RgbColor::GREY,
            (191, 191, 191) => RgbColor::GREY,
            (158, 186, 0) => RgbColor::OLIVE,
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
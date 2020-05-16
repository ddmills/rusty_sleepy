use image::*;
use super::*;

pub struct ColorLUT {
    img: DynamicImage,
    pub table: Vec<HSV>,
}

impl ColorLUT {
    pub fn new() -> ColorLUT {
        let img = image::open("resources/biome-land.png").unwrap();

        ColorLUT {
            img: img,
            table: vec!(HSV::from_f32(0.0, 0.0, 0.0))
        }
    }

    pub fn get_hsv(&self, x: f32, y: f32) -> HSV {
        let (im_w, im_h) = self.img.dimensions();
        let px = (x * ((im_w - 1) as f32)) as u32;
        let py = (y * ((im_h - 1) as f32)) as u32;

        if !self.img.in_bounds(px, py) {
            println!("pixel out of bounds ({},{})", x, y);
            return HSV::from_f32(0.0, 0.0, 0.0);
        }

        let pixel = self.img.get_pixel(px, py);
        let rgb_img = pixel.to_rgb();
        let rgb = RGB::from_u8(rgb_img[0], rgb_img[1], rgb_img[2]);

        rgb.to_hsv()
    }
}

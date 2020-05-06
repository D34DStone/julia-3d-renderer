use na::Vector2 as v2;
use std::path::Path;

pub struct Texture {
    image   : bmp::Image,
    shape   : v2<u32>,
}

impl Texture {
    pub fn new(p: &Path) -> Self {
        let image = bmp::open(p).unwrap();
        let shape = v2::new(image.get_width(), image.get_height());
        Self {
            image: image,
            shape: shape,
        }
    }

    pub fn get_pixel(&self, dot: v2<f32>) -> (u8, u8, u8) {
        let x = ((dot.x + 1.0) * self.shape.x as f32 / 2.).round() as u32;
        let y = ((dot.y + 1.0) * self.shape.y as f32 / 2.).round() as u32;
        let x = std::cmp::min(x, self.shape.x - 1);
        let y = std::cmp::min(y, self.shape.y - 1);
        let pixel = self.image.get_pixel(x, y);
        (pixel.r, pixel.g, pixel.b)
    }
}

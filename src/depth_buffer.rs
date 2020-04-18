use std::ops::IndexMut;

use super::math_utils;

#[derive(Clone)]
struct ColorDepth {
    color: (u8, u8, u8),
    depth: f32,
}

pub struct DepthBuffer {
    width:  u32, 
    height: u32,
    buffer: Vec<ColorDepth>,
}

impl DepthBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        DepthBuffer {
            width: width,
            height: height,
            buffer: vec![ColorDepth {
                color: (0, 0, 0),
                depth: -1.0,
            }; (width * height) as usize],
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn plot_i32(&mut self, coords: (i32, i32, f32), color: (u8, u8, u8)) {
        let (x, y, depth) = coords;
        let index = self.i32_to_index(x, y);
        self.plot(index, depth, color);
    }

    pub fn plot_f32(&mut self, coords: (f32, f32, f32), color: (u8, u8, u8)) {
        let (x, y, depth) = coords;
        let index = self.f32_to_index(x, y);
        self.plot(index, depth, color);
    }

    fn plot(&mut self, index: usize, depth: f32, color: (u8, u8, u8)) -> bool {
        let ColorDepth {
            color: _,
            depth: cur_depth,
        } = self.buffer[index];

        if cur_depth < depth {
            self.buffer[index] = ColorDepth {
                color: color,
                depth: depth,
            };
        }

        (cur_depth < depth)
    }

    pub fn write<T: IndexMut<usize, Output=(u8, u8, u8)>>(&self, dest: &mut T) {
        for ind in 0_usize..(self.width * self.height) as usize {
            dest[ind] = self.buffer[ind].color;
        }
    }

    fn i32_to_index(&self, x: i32, y: i32) -> usize {
        let (width_2u, height_2u) = (
            (self.width  as f32 / 2_f32).round() as u32,
            (self.height as f32 / 2_f32).round() as u32,
        );
        let (xu, yu) = (
            (x + width_2u as i32) as u32,
            (y + height_2u as i32) as u32,
        );
        (xu + self.width * yu) as usize
    }

    fn f32_to_index(&self, x: f32, y: f32) -> usize {
        let (xi, yi) = math_utils::rasterize_dot((x, y), (self.width, self.height));
        self.i32_to_index(xi, yi)
    }
}

mod gcanvas;
mod glfw_canvas;

use gcanvas::{CanvasAPI, EventAPI, Event};
use glfw_canvas::GlfwContext;
use std::mem::swap;
use std::ops::IndexMut;
use std::cmp::{min, max};

extern crate ndarray;
use ndarray::{Array1};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

#[derive(Clone)]
struct ColorDepth {
    color: (u8, u8, u8),
    depth: f32,
}

struct DepthBuffer {
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
        for ind in 0..self.width * self.height {
            let ColorDepth {
                color: color,
                depth: depth,
            } = self.buffer[ind as usize];

            dest[ind as usize] = color;
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

mod math_utils {
    use std::ops::{Sub, Mul};
    use std::cmp::{PartialOrd};

    pub fn rasterize_dot(
        d: (f32, f32),
        canvas_size: (u32, u32)) -> (i32, i32) {
        let (x, y) = d;
        let (width, height) = canvas_size;
        if -1.0 > x || x > 1.0 {
            panic!("X is out of range (must be in [-1, 1])");
        }

        if -1.0 > y || y > 1.0 {
            panic!("Y is out of range (must be in [-1, 1])");
        }

        let (width_2, height_2) = (
            width  as f32 / 2_f32,
            height as f32 / 2_f32,
        );

        (
            (x * width_2).round() as i32,
            (y * height_2).round() as i32,
        )
    }

    pub fn oriented_area<T: Sub<Output=T> + Mul<Output=T> + Copy>(
        a: (T, T),
        b: (T, T),
        c: (T, T)) -> T {
        let (x1, y1) = a;
        let (x2, y2) = b;
        let (x3, y3) = c;
        (x2 - x1) * (y3 - y2) - (x3 - x2) * (y2 - y1)
    }

    pub fn is_dot_inside_triangle<T: 
        Sub<Output=T> 
        + Mul<Output=T> 
        + PartialOrd 
        + From<i32> 
        + Copy>(
        d: (T, T),
        a: (T, T),
        b: (T, T),
        c: (T, T)) -> bool { 
        let (xd, yd) = d; 
        let (xa, ya) = a;
        let (xb, yb) = b;
        let (xc, yc) = c;
        let s1 = self::oriented_area((xa, ya), (xd, yd), (xb, yb)); 
        let s2 = self::oriented_area((xb, yb), (xd, yd), (xc, yc)); 
        let s3 = self::oriented_area((xc, yc), (xd, yd), (xa, ya)); 
        let zero = T::from(0_i32);
        (s1 >= zero && s2 >= zero && s3 >= zero)
        || (s1 <= zero && s2 <= zero && s3 <= zero)
    }
}

trait Rasterizer {
    fn rasterize_line(color: (u8, u8, u8),
                 p1: (f32, f32, f32), 
                 p2: (f32, f32, f32), 
                 buffer: DepthBuffer) -> DepthBuffer;

    fn rasterize_triagnle(color: (u8, u8, u8),
                 a: (f32, f32, f32), 
                 b: (f32, f32, f32), 
                 c: (f32, f32, f32), 
                 buffer: DepthBuffer) -> DepthBuffer;
}

struct NaivelyRas;

impl Rasterizer for NaivelyRas {
    fn rasterize_line(color: (u8, u8, u8),
                 mut p1: (f32, f32, f32), 
                 mut p2: (f32, f32, f32), 
                 mut buffer: DepthBuffer) -> DepthBuffer {
        if (p1.0 > p2.0) { 
            swap(&mut p1, &mut p2); 
        }
        let (x1, y1, z1) = p1;
        let (x2, y2, z2) = p2;
        let (dx, dy, dz) = (
            x2 - x1,
            y2 - y1,
            z2 - z1,
        );

        let DepthBuffer {
            width: w_px,
            height: h_px,
            buffer: _,
        } = buffer;

        let (dx_px, dy_px) = (
            (dx * w_px as f32),
            (dy * h_px as f32),
        );
        let len_px = (dx_px * dx_px + dy_px * dy_px).sqrt().round();
        let mut t_cur = 0_f32;
        let t_end  = (x2 - x1) / dx;
        let t_step = t_end / len_px;

        loop {
            let (x, y, z) = (
                t_cur * dx + x1,
                t_cur * dy + y1,
                t_cur * dz + z1,
            );
            buffer.plot_f32((x, y, z), color);

            t_cur += t_step;
            if t_cur > t_end {
                break;
            }
        }

        buffer
    }

    fn rasterize_triagnle(color: (u8, u8, u8),
                 a: (f32, f32, f32), 
                 b: (f32, f32, f32), 
                 c: (f32, f32, f32), 
                 mut buffer: DepthBuffer) -> DepthBuffer {
         
        let DepthBuffer {
            width: width_px,
            height: height_px, 
            buffer: _,
        } = buffer;

        let (xa, ya, a_depth) = a;
        let (xb, yb, b_depth) = b;
        let (xc, yc, c_depth) = c;
        let (xa_px, ya_px) = math_utils::rasterize_dot((xa, ya), (width_px, height_px));
        let (xb_px, yb_px) = math_utils::rasterize_dot((xb, yb), (width_px, height_px));
        let (xc_px, yc_px) = math_utils::rasterize_dot((xc, yc), (width_px, height_px));
        let (x_px_min, y_px_min) = (
            min(xa_px, min(xb_px, xc_px)),
            min(ya_px, min(yb_px, yc_px)),
        );
        let (x_px_max, y_px_max) = (
            max(xa_px, max(xb_px, xc_px)),
            max(ya_px, max(yb_px, yc_px)),
        );
        for x in x_px_min..=x_px_max {
            for y in y_px_min..=y_px_max {
                if !math_utils::is_dot_inside_triangle(
                    (x, y),
                    (xa_px, ya_px),
                    (xb_px, yb_px),
                    (xc_px, yc_px)) {
                    continue;
                }

                buffer.plot_i32((x, y, 1.0), color);
            }
        }
        buffer
    }
}

type RGB = (u8, u8, u8);
const RED: RGB = (255, 0, 0);
const GREEN: RGB = (0, 255, 0);
const BLUE: RGB = (0, 0, 255);
const WHITE: RGB = (255, 255, 255);
const GREY: RGB = (128, 128, 128);

fn main() {
    let mut ctx = GlfwContext::new(WIDTH, HEIGHT, "George");
    let mut data = vec![(0_u8, 0_u8, 0_u8); (WIDTH*HEIGHT) as usize];

    let mut depth_buffer = DepthBuffer::new(512, 512);
    depth_buffer = NaivelyRas::rasterize_line(
        RED,
        (0.5, 0.5, 0.5), 
        (-0.5, -0.5, -0.5), 
        depth_buffer);

    depth_buffer = NaivelyRas::rasterize_line(
        GREEN,
        (0.5, -0.5, 0.99), 
        (-0.5, 0.5, 0.99), 
        depth_buffer);

    depth_buffer = NaivelyRas::rasterize_triagnle(
        RED, 
        (0.5, 0.5, 1.0),
        (-0.5, 0.5, 1.0),
        (-0.5, -0.5, 1.0),
        depth_buffer);

    depth_buffer.write(&mut data);

    let mut working = true;
    while working {
        for event in ctx.events() {
            match event {
                Event::Close => {
                    working = false;
                }
                _ => {
                    // no-op
                }
            }
        }
        ctx.update(data.as_ptr());
    }
}

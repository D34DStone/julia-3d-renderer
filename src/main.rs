mod gcanvas;
mod glfw_canvas;

use gcanvas::{CanvasAPI, EventAPI, Event};
use glfw_canvas::GlfwContext;
use std::mem::swap;
use std::ops::IndexMut;

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

    pub fn plot(&mut self, coords: (f32, f32, f32), color: (u8, u8, u8)) {
        let (x, y, new_depth) = coords;
        let index = self.to_index(x, y);

        let ColorDepth {
            color: _,
            depth: cur_depth,
        } = self.buffer[index];

        if cur_depth < new_depth {
            self.buffer[index] = ColorDepth {
                color: color,
                depth: new_depth,
            };
        }
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

    fn to_index(&self, x: f32, y: f32) -> usize {
        if -1.0 > x || x > 1.0 {
            panic!("X is out of range (must be in [-1, 1])");
        }

        if -1.0 > y || y > 1.0 {
            panic!("Y is out of range (must be in [-1, 1])");
        }

        let (width_2, height_2) = (
            self.width  as f32 / 2_f32,
            self.height as f32 / 2_f32,
        );

        let (xi, yi) = (
            ((x * width_2).round() as i32 + width_2 as i32) as u32,
            ((y * height_2).round() as i32 + height_2 as i32) as u32,
        );

        (xi + self.width * yi) as usize
    }
}

trait Rasterizer {
    fn rasterize_line(color: (u8, u8, u8),
                 p1: (f32, f32, f32), 
                 p2: (f32, f32, f32), 
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
            buffer.plot((x, y, z), color);

            t_cur += t_step;
            if t_cur > t_end {
                break;
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
        (0.5, -0.5, 1.0), 
        (-0.5, 0.5, 1.0), 
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

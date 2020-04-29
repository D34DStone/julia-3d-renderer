mod gcanvas;
mod glfw_canvas;
mod math_utils;
mod depth_buffer;
mod rasterizer;
mod rasterizer_1;

extern crate nalgebra as na;

use gcanvas::{CanvasAPI, EventAPI, Event};
use glfw_canvas::GlfwContext;
use depth_buffer::DepthBuffer;
use std::time::Instant;

#[allow(dead_code)]
type RGB = (u8, u8, u8);
const RED: RGB = (255, 0, 0);
const GREEN: RGB = (0, 255, 0);
const BLUE: RGB = (0, 0, 255);
const WHITE: RGB = (255, 255, 255);
const GREY: RGB = (128, 128, 128);
const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn main() {
    let mut ctx = GlfwContext::new(WIDTH, HEIGHT, "George");
    let mut data = vec![(0_u8, 0_u8, 0_u8); (WIDTH*HEIGHT) as usize];

    let mut depth_buffer = DepthBuffer::new(512, 512);
    let mut rast = rasterizer_1::Rasterizer_::new(512, 512);

    let time = Instant::now();
    for i in 0..1024 {
        rast.rasterize_triangle_color(
            na::Vector3::<f32>::new(-0.1, 0.0, 1.0),
            na::Vector3::<f32>::new(0.1, 0.1, 0.0),
            na::Vector3::<f32>::new(0.1, -0.1, 0.0),
            GREEN);
    }
    println!("{}ms", time.elapsed().as_millis());
    println!("{}s", time.elapsed().as_secs());

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
        ctx.update(rast.get_color_buffer_ptr());
    }
}

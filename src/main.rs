mod gcanvas;
mod glfw_canvas;
mod math_utils;
mod depth_buffer;
mod rasterizer;

use gcanvas::{CanvasAPI, EventAPI, Event};
use glfw_canvas::GlfwContext;
use depth_buffer::DepthBuffer;

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

    depth_buffer = rasterizer::line(
        GREEN,
        (-0.5, 0.0, 0.7), 
        (0.5, 0.0, 0.7), 
        depth_buffer);
    
    depth_buffer = rasterizer::triagnle(
        RED, 
        (0.5, 0.0, 1.0),
        (-0.5, 0.5, 0.0),
        (-0.5, -0.5, 0.0),
        depth_buffer);

    depth_buffer = rasterizer::triagnle(
        GREEN, 
        (-0.5, 0.0, 1.0),
        (0.5, 0.5, 0.0),
        (0.5, -0.5, 0.0),
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

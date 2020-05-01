mod gcanvas;
mod glfw_canvas;
mod math_utils;
mod rasterizer_1;

extern crate nalgebra as na;

use gcanvas::{CanvasAPI, EventAPI, Event};
use glfw_canvas::GlfwContext;

#[allow(dead_code)]
mod color {
    type RGB = (u8, u8, u8);
    pub const RED: RGB = (255, 0, 0);
    pub const GREEN: RGB = (0, 255, 0);
    pub const BLUE: RGB = (0, 0, 255);
    pub const WHITE: RGB = (255, 255, 255);
    pub const GREY: RGB = (128, 128, 128);
}

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn main() {
    let mut ctx = GlfwContext::new(WIDTH, HEIGHT, "George");
    let mut rast = rasterizer_1::Rasterizer_::new(512, 512);

    /*
    for _i in 0..1024 {
        rast.rasterize_triangle_color(
            na::Vector3::<f32>::new(-0.1, 0.0, 1.0),
            na::Vector3::<f32>::new(0.1, 0.1, 0.0),
            na::Vector3::<f32>::new(0.1, -0.1, 0.0),
            color::GREEN);
    }
    */

    rast.render_segment_(
        rasterizer_1::Vertex {
            coords  : na::Vector3::new(0.0, 0.0, 0.0),
            color   : na::Vector3::new(255_u8, 0_u8, 0_u8),
        },
        rasterizer_1::Vertex {
            coords  : na::Vector3::new(0.5, 0.5, -0.5),
            color   : na::Vector3::new(0_u8, 255_u8, 0_u8),
        });

    let basis = rasterizer_1::VertexBasis3 {
        x: rasterizer_1::Vertex {
            coords  : na::Vector3::new(0.0, 0.0, 0.0),
            color   : na::Vector3::new(255_u8, 0_u8, 0_u8),
        },
        y: rasterizer_1::Vertex {
            coords  : na::Vector3::new(0.5, 0.5, -0.5),
            color   : na::Vector3::new(0_u8, 255_u8, 0_u8),
        },
        z: rasterizer_1::Vertex {
            coords  : na::Vector3::new(0.5, 0.0, -0.5),
            color   : na::Vector3::new(0_u8, 0_u8, 255_u8),
        }
    };

    rast.rasterize_polygon(&basis);

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

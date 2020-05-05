mod gcanvas;
mod glfw_canvas;
mod math_utils;
mod rasterizer_1;
mod rasterizer_2;

extern crate nalgebra as na;

use gcanvas::{CanvasAPI, EventAPI, Event};
use glfw_canvas::GlfwContext;
use na::Vector4 as v4;
use na::Vector3 as v3;
use na::Vector2 as v2;
use na::Matrix4 as mat4;

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

    let mut julia = rasterizer_2::Julia3D::new(512, 512);

    julia.render_polygon(
            rasterizer_2::IVertex {
                coords  : v3::new(-0.9, -0.9, 5.),
                color   : v3::new(255., 0., 0.),
            },
            rasterizer_2::IVertex {
                coords  : v3::new(-1., 0.5, 1.),
                color   : v3::new(0., 0., 255.),
            },
            rasterizer_2::IVertex {
                coords  : v3::new(0.5, 0.5, 1.),
                color   : v3::new(0., 255., 0.),
            });

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
        ctx.update(julia.buff_ptr());
    }
}

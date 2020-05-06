mod gcanvas;
mod glfw_canvas;
mod rasterizer_2;
mod texture;

extern crate nalgebra as na;
extern crate bmp;

use gcanvas::{CanvasAPI, EventAPI, Event};
use glfw_canvas::GlfwContext;
use na::Vector3 as v3;
use na::Vector2 as v2;
use na::Matrix4 as mat4;

fn main() {
    let mut ctx = GlfwContext::new(512, 512, "Julia3D");

    let mut julia = rasterizer_2::Julia3D::new(512, 512);

    let tex = texture::Texture::new(std::path::Path::new("./textures/tex1.bmp"));

    let vertices = [
            rasterizer_2::IVertex {
                coords      : v3::new(-1., -1., 0.),
                color       : v3::new(255., 0., 0.),
                tex_coords  : v2::new(-1., -1.),
            },
            rasterizer_2::IVertex {
                coords      : v3::new(-1., 1., 0.),
                color       : v3::new(0., 0., 255.),
                tex_coords  : v2::new(-1., 1.),
            },
            rasterizer_2::IVertex {
                coords      : v3::new(1., 1., 0.),
                color       : v3::new(0., 255., 0.),
                tex_coords  : v2::new(1., 1.),
            }, 
            rasterizer_2::IVertex {
                coords      : v3::new(1., -1., 0.),
                color       : v3::new(0., 0., 255.),
                tex_coords  : v2::new(1., -1.),
            }];

    let faces = [
        (0, 1, 2),
        (0, 3, 2),
    ];

    let mut t: f32 = 0.;
    let dt = 0.05;
    let rotation_r = 3.;
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

        julia.clear();

        let trans = na::geometry::Translation3::new(rotation_r * t.sin(), rotation_r * t.cos(), 6.).to_homogeneous();
        let rotation = na::geometry::Rotation3::from_axis_angle(&v3::x_axis(), t).to_homogeneous();
        t += dt;
        julia.render(
            &vertices, 
            &faces,
            &tex,
            trans * rotation);

        ctx.update(julia.buff_ptr());
    }
}

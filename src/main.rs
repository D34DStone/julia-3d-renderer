mod gcanvas;
mod glfw_canvas;
mod rasterizer_2;
mod texture;
use std::time::{Duration, Instant};

extern crate nalgebra as na;
extern crate bmp;

use gcanvas::{CanvasAPI, EventAPI, Event};
use glfw_canvas::GlfwContext;
use na::Vector3 as v3;
use na::Vector2 as v2;
use na::Matrix4 as mat4;

fn get_model(t: f32, dx: f32, dy: f32, dz: f32) -> mat4<f32> {
    let rotation_r = 3.;
    let trans = na::geometry::Translation3::new(rotation_r * t.sin() + dx, rotation_r * t.cos() + dy, 4. + dz).to_homogeneous();
    let rotation = na::geometry::Rotation3::from_axis_angle(&v3::x_axis(), t).to_homogeneous();
    trans * rotation
}

fn main() {
    let mut ctx = GlfwContext::new(512, 512, "Julia3D");
    let mut julia = rasterizer_2::Julia3D::new(512, 512);
    let tex = texture::Texture::new(std::path::Path::new("./textures/tex1.bmp"));
    let cube_vertices = [
        // Front vertices
        rasterizer_2::IVertex {
            coords      : v3::new(1., 1., -1.),
            tex_coords  : v2::new(1., 1.),
        }, 
        rasterizer_2::IVertex {
            coords      : v3::new(1., -1., -1.),
            tex_coords  : v2::new(1., -1.),
        },
        rasterizer_2::IVertex {
            coords      : v3::new(-1., -1., -1.),
            tex_coords  : v2::new(-1., -1.),
        },
        rasterizer_2::IVertex {
            coords      : v3::new(-1., 1., -1.),
            tex_coords  : v2::new(-1., 1.),
        },

        // Back vertices
        rasterizer_2::IVertex {
            coords      : v3::new(1., 1., 1.),
            tex_coords  : v2::new(-1., -1.),
        }, 
        rasterizer_2::IVertex {
            coords      : v3::new(1., -1., 1.),
            tex_coords  : v2::new(-1., 1.),
        },
        rasterizer_2::IVertex {
            coords      : v3::new(-1., -1., 1.),
            tex_coords  : v2::new(1., 1.),
        },
        rasterizer_2::IVertex {
            coords      : v3::new(-1., 1., 1.),
            tex_coords  : v2::new(1., -1.),
        }
    ];

    let cube_faces = [
        // front
        (0, 1, 2),
        (0, 2, 3),
    
        // back
        (4, 5, 6),
        (4, 7, 6),

        // left 
        (2, 3, 7),
        (2, 6, 7),

        // right 
        (1, 0, 4),
        (1, 5, 4),

        // top
        (3, 0, 4),
        (3, 7, 4),

        // bot
        (2, 1, 5),
        (2, 6, 5),
    ];

    let mut t: f32 = 0.;
    let dt = 0.005;
    let mut working = true;
    while working {
        let now = Instant::now();
        julia.clear();

        t += dt;
        julia.render(
            &cube_vertices, 
            &cube_faces,
            &tex,
            get_model(t, 0., 0., 5.));

        julia.render(
            &cube_vertices, 
            &cube_faces,
            &tex,
            get_model(4. * t + 5., 3., 0., 5.));

        julia.render(
            &cube_vertices, 
            &cube_faces,
            &tex,
            get_model(6. * t + 5., -3., 0., 2.));

        ctx.update(julia.buff_ptr());

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
        println!("Time per frame: {}ms", now.elapsed().as_millis());
    }
}

mod gcanvas;
mod glfw_canvas;
mod rasterizer_2;
mod texture;
use std::time::{Duration, Instant};
use std::collections::HashSet;

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

mod camera {
    use na::Vector3 as v3;
    use na::Matrix4 as mat4;

    pub struct ResponsiveCamera {
        position    : v3<f32>,
        target      : v3<f32>,
        up          : v3<f32>,
    }

    impl ResponsiveCamera {
        pub fn new(position: v3<f32>, target: v3<f32>) -> Self { 
            let up = v3::new(0., 1., 0.);
            ResponsiveCamera { 
                position, 
                target,
                up,
            } 
        }

        pub fn homogeneous(&self) -> mat4<f32> {
            let direction = na::normalize(&(&self.target - &self.position));
            let cam_right = na::normalize(&direction.cross(&self.up));
            let cam_up    = cam_right.cross(&direction);
            mat4::new(
                cam_right.x,    cam_right.y,    cam_right.z,    0.,
                cam_up.x,       cam_up.y,       cam_up.z,       0.,
                direction.x,    direction.y,    direction.z,    0.,
                0.,             0.,             0.,             1.,
            ) * mat4::new(
                1., 0., 0., -self.position.x,
                0., 1., 0., -self.position.y,
                0., 0., 1., -self.position.z,
                0., 0., 0., 1.,
            )
        }

        pub fn move_to(&mut self, v: v3<f32>) {
            self.position   += v;
            self.target     += v;
        }

        pub fn change_target(&mut self, linear_op: na::geometry::Rotation3<f32>) {
            // println!("D: {:?} {:?} {:?}", self.target, linear_op * self.target - self.target, self.target);
            let direction = self.target - self.position;
            self.target = self.position + linear_op * direction;
            self.target = linear_op * self.target;
        }
    }

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

    let mut camera = camera::ResponsiveCamera::new(
        v3::new(0., 0., -5.),
        v3::new(0., 0., -10.));

    let mut mouse_pos: Option<v2<f32>> = None;
    let mouse_sensativity = 0.01;

    let mut keys_holding = HashSet::<char>::new();
    let key_to_dir = [
        ('a', v3::new(1., 0., 0.)),
        ('d', v3::new(-1.,  0., 0.)),
        ('s', v3::new(0., 0., -1.)),
        ('w', v3::new(0., 0., 1.)),
    ];



    while working {
        julia.clear();

        for (key, v) in key_to_dir.iter() {
            if keys_holding.contains(&key) {
                camera.move_to(*v * 0.1);
            }
        }

        t += dt;
        julia.render(
            &cube_vertices, 
            &cube_faces,
            &tex,
            camera.homogeneous() * get_model(t, 0., 0., 5.));

        julia.render(
            &cube_vertices, 
            &cube_faces,
            &tex,
            camera.homogeneous() * get_model(4. * t + 5., 3., 0., 5.));

        julia.render(
            &cube_vertices, 
            &cube_faces,
            &tex,
            camera.homogeneous() * get_model(6. * t + 5., -3., 0., 2.));

        ctx.update(julia.buff_ptr());

        for event in ctx.events() {
            match event {
                Event::Close => {
                    working = false;
                },
                Event::KeyPressed(key) => {
                    // println!("Pressed {}", key);
                    keys_holding.insert(key);
                },
                Event::KeyReleased(key) => {
                    // println!("unpressed {}", key);
                    keys_holding.remove(&key);
                },
                Event::MouseMoved(x, y) => {
                    let new_mouse_pos = v2::new(x, y);
                    match mouse_pos {
                        None => {
                            mouse_pos = Some(new_mouse_pos);
                        }, 
                        Some(pos) => {
                            let d = new_mouse_pos - pos;
                            let rotation = na::geometry::Rotation3::new(v3::new(
                                    -d.y * mouse_sensativity, 
                                    -d.x * mouse_sensativity, 
                                    0.));
                            camera.change_target(rotation);
                            mouse_pos = Some(new_mouse_pos);
                        },
                    }
                },
                _ => {
                    // no-op
                }
            }
        }
    }
}

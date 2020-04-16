mod gcanvas;
mod glfw_canvas;

use gcanvas::{CanvasAPI, EventAPI, Event};
use glfw_canvas::GlfwContext;

fn main() {
    let mut ctx = GlfwContext::new(512, 512, "George");

    let data = [(255, 0, 0); 512*512];
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

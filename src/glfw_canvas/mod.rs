pub mod event;
pub mod canvas;
mod shader;

use std::sync::mpsc::Receiver;

pub struct GlfwContext {
    window: glfw::Window,
    events: Receiver::<(f64, glfw::WindowEvent)>,
}

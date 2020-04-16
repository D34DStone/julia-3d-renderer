extern crate glfw;

use crate::gcanvas::{EventAPI, Event};
use super::GlfwContext;

/// TODO: Add Handle of other events.
impl EventAPI for GlfwContext {
    fn events(&self) -> Vec<Event> {
        glfw::flush_messages(&self.events).map(|(_, glfw_event)| match glfw_event {
                    glfw::WindowEvent::Close => Event::Close,
                    _ => Event::KeyPressed('a'),
                }).collect()
    }
}

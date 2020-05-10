extern crate glfw;

use crate::gcanvas::{EventAPI, Event};
use super::GlfwContext;

pub fn glfw_key_to_char(key: glfw::Key) -> char {
    match key {
        glfw::Key::W => 'w',
        glfw::Key::A => 'a',
        glfw::Key::S => 's',
        glfw::Key::D => 'd',
        _            => '#',
    }
}

/// TODO: Add Handle of other events.
impl EventAPI for GlfwContext {
    fn events(&self) -> Vec<Event> {
        glfw::flush_messages(&self.events).map(|(_, glfw_event)| match glfw_event {
                    glfw::WindowEvent::Close => {
                        Event::Close
                    },
                    glfw::WindowEvent::Key(key, _scancode, action, _modifiers) => {
                        let key_char = glfw_key_to_char(key);
                        match action {
                            glfw::Action::Press     => Event::KeyPressed(key_char),
                            glfw::Action::Release   => Event::KeyReleased(key_char),
                            _                       => Event::None,
                        }
                    },
                    glfw::WindowEvent::CursorPos(x, y) => {
                        Event::MouseMoved(x as f32, y as f32)
                    },
                    _ => Event::None,
                }).collect()
    }
}

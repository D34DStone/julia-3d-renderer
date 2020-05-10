/// # CanvasAPI provides abstaction under the pixel-buffer. 
pub trait CanvasAPI: Drop {
    fn new(width: u32, height: u32, title: &str) -> Self;

    /// Takes buffer of pixels and display it on the canvas.
    ///
    /// # Requirement 1:
    /// buffer must consists of `width * height * 3` of unsigned bytes. 
    /// It must be linear in memory.
    ///
    /// # Requirement 2:
    /// All additional operations like polling events or redrawing 
    /// of the canvas must be placed there too. In fact, all necessary 
    /// logic(excluding handling of events) must be there.
    fn update(&mut self, buffer: *const (u8, u8, u8));
}

/// # EventAPI provides abstraction under system event queue.
pub trait EventAPI {
    /// Method must copy all current events and flush the queue.
    fn events(&self) -> Vec<Event>;
}

#[allow(dead_code)]
pub enum MouseKey {
    Left,
    Right,
}

#[allow(dead_code)]
pub enum Event {
    Close,
    None,
    KeyPressed(char),
    KeyReleased(char),
    MouseMoved(f32, f32),
    MouseClicked(MouseKey, i32, i32),
}

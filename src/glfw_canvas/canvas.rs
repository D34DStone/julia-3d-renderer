extern crate glfw;
extern crate gl;

use std::sync::mpsc::Receiver;
use std::os::raw::c_void;
use std::ptr;
use std::mem;
use self::glfw::*;
use self::gl::types::*;
use crate::gcanvas::CanvasAPI;
use super::shader::Shader;
use super::GlfwContext;

impl Drop for GlfwContext {
    fn drop(&mut self) {
        self.window.set_should_close(true);
    }
}

impl CanvasAPI for GlfwContext {
    fn new(width: u32, height: u32, title: &str) -> Self {
        let (mut window, events) = init_glfw(width, height, title);
        init_opengl(&mut window);
        GlfwContext {
            window:     window,
            events:     events,
        }
    }

    fn update(&mut self, buffer: *const (u8, u8, u8)) {
        let (width, height) = self.window.get_size();
        unsafe {
            gl::TexSubImage2D(gl::TEXTURE_2D,
                           0,
                           0, 
                           0,
                           width as i32,
                           height as i32,
                           gl::RGB,
                           gl::UNSIGNED_BYTE,
                           buffer as *const c_void); 

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        self.window.swap_buffers();
        self.window.glfw.poll_events();
    }
}

fn init_glfw(width: u32, height: u32, title: &str) -> (glfw::Window,
                                                       Receiver::<(f64, glfw::WindowEvent)>) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Con't initialize GLFW");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    let (mut window, events) = glfw.create_window(
        width, 
        height, 
        title, 
        glfw::WindowMode::Windowed)
        .unwrap();

    window.make_current();
    window.set_all_polling(true);
    (window, events)
}

#[allow(non_snake_case)]
fn init_opengl(window: &mut glfw::Window) {
    let (width, height) = window.get_size();
    let vertices: [f32; 20] = [
        1.0,  1.0, 0.0,    1.0, 1.0,
        1.0, -1.0, 0.0,    1.0, 0.0,
        -1.0, -1.0, 0.0,    0.0, 0.0,
        -1.0,  1.0, 0.0,    0.0, 1.0,
    ];
    let indices = [
        0, 1, 3,
        1, 2, 3,
    ];
    let (mut VBO, mut VAO, mut EBO, mut texture) = (0, 0, 0, 0);
    let data = vec![(0_u8, 0_u8, 0_u8); (width * height) as usize];

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        Shader::new("glfw_canvas/shaders/index.vs", 
                    "glfw_canvas/shaders/index.fs").useProgram();
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);
        gl::BindVertexArray(VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture); 
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGB as i32,
                       width as i32,
                       height as i32,
                       0,
                       gl::RGB,
                       gl::UNSIGNED_BYTE,
                       data.as_ptr() as *const c_void);
    }
}


use gl;
use glfw::Context;

use std::ffi::c_void;

#[macro_use]
mod renderer;

mod vertex_buffer;
pub use vertex_buffer::VertexBuffer;

mod index_buffer;
pub use index_buffer::IndexBuffer;

mod vertex_array;
pub use vertex_array::VertexArray;

mod vertex_buffer_layout;
use vertex_buffer_layout::VertexBufferLayout;

mod shader;
use shader::Shader;

fn get_gl_version() {
    println!(
        "{}",
        unsafe {
            std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
        }
        .to_str()
        .unwrap()
    );
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    //
    // glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    // glfw.window_hint(glfw::WindowHint::OpenGlProfile(
    //     glfw::OpenGlProfileHint::Core,
    // ));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    // Create a windowed mode window and its OpenGL context
    let (mut window, _events) = glfw
        .create_window(
            640,
            480,
            "Hello this is window",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");
    //
    // Make the window's context current
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    // window.set_key_polling(true);
    //
    gl::load_with(|s| window.get_proc_address(s));
    //
    get_gl_version();
    //
    let positions: &[[f32; 2]] =
        &[[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]];
    //
    let indices: &[[u32; 3]] = &[[0, 1, 2], [2, 3, 0]];
    //
    let va = VertexArray::new();
    let vb = VertexBuffer::from(positions);
    //
    let mut layout = VertexBufferLayout::new();
    layout.push::<f32>(2);
    va.add_buffer(&vb, layout);
    //
    va.bind();
    //
    let ib = IndexBuffer::from(indices);
    //
    let mut shader = Shader::new("res/shaders/basic.shader");
    shader.bind();
    //
    va.unbind();
    vb.unbind();
    ib.unbind();
    shader.unbind();
    //
    let timer = std::time::Instant::now();
    // Loop until the user closes the window
    while !window.should_close() {
        gl_call!(gl::ClearColor(0.0, 0.0, 0.0, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT));
        //
        shader.bind();
        shader.set_uniform_4f(
            "u_color\0",
            [
                timer.elapsed().as_secs_f32().sin().powi(2),
                (timer.elapsed().as_secs_f32() + 1.047).sin().powi(2),
                (timer.elapsed().as_secs_f32() + 2.094).sin().powi(2),
                1.0,
            ],
        );
        //
        va.bind();
        ib.bind();
        //
        gl_call!(gl::DrawElements(
            gl::TRIANGLES,
            ib.count,
            gl::UNSIGNED_INT,
            0 as *const c_void
        ));
        //
        // Swap front and back buffers
        window.swap_buffers();
        //
        // Poll for and process events
        glfw.poll_events();
    }
}

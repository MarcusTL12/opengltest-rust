#![allow(incomplete_features)]
#![feature(const_generics)]

use gl;

use imgui_glfw_rs::{
    glfw::{self, Context},
    imgui, ImguiGLFW,
};

#[macro_use]
mod glcall;

mod renderer;
pub use renderer::Renderer;

mod vertex_buffer;
pub use vertex_buffer::VertexBuffer;

mod index_buffer;
pub use index_buffer::IndexBuffer;

mod vertex_array;
pub use vertex_array::VertexArray;

mod vertex_buffer_layout;
pub use vertex_buffer_layout::VertexBufferLayout;

mod shader;
pub use shader::Shader;

mod texture;
pub use texture::Texture;

mod tests;
pub use tests::{test_clear_color::TestClearColor, OGLTest};

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
    //
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(
            1280,
            720,
            "Hello this is window",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");
    //
    // Make the window's context current
    glfw.make_context_current(Some(&window));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    window.set_all_polling(true);
    //
    gl::load_with(|s| window.get_proc_address(s));
    //
    get_gl_version();
    //
    gl_call!(gl::Enable(gl::BLEND));
    gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
    //
    let renderer = Renderer {};
    //
    let mut imgui = imgui::Context::create();
    //
    let mut imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);
    //
    let mut test = TestClearColor::new();
    //
    // Loop until the user closes the window
    while !window.should_close() {
        renderer.clear();
        //
        test.on_update(0.0);
        test.on_render();
        //
        let ui = imgui_glfw.frame(&mut window, &mut imgui);
        //
        test.on_imgui_render(&ui);
        //
        imgui_glfw.draw(ui, &mut window);
        //
        // Swap front and back buffers
        window.swap_buffers();
        //
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            imgui_glfw.handle_event(&mut imgui, &event);
        }
    }
}

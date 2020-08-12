#![allow(incomplete_features)]
#![feature(const_generics)]

use gl;

use imgui_glfw_rs::{
    glfw::{self, Context},
    imgui::{self, im_str},
    ImguiGLFW,
};

use nalgebra_glm as glm;

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
pub use tests::{
    test_clear_color::TestClearColor, test_texture2d::TestTexture2D, OGLTest,
    TestMenu,
};

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
        .create_window(1280, 720, "OpenGL testing", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    //
    // Make the window's context current
    glfw.make_context_current(Some(&window));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
    window.set_all_polling(true);
    //
    gl::load_with(|s| window.get_proc_address(s));
    //
    get_gl_version();
    //
    let renderer = Renderer {};
    //
    let mut imgui = imgui::Context::create();
    //
    let mut imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);
    //
    let mut test_menu = TestMenu::new();
    //
    test_menu.register_test::<TestClearColor>("Clear Color");
    test_menu.register_test::<TestTexture2D>("Texture 2D");
    //
    // Loop until the user closes the window
    while !window.should_close() {
        gl_call!(gl::ClearColor(0.0, 0.0, 0.0, 1.0));
        renderer.clear();
        //
        test_menu.on_update(0.0);
        test_menu.on_render();
        //
        let ui = imgui_glfw.frame(&mut window, &mut imgui);
        //
        test_menu.on_imgui_render(&ui);
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
#![allow(incomplete_features)]
#![feature(const_generics)]

use gl;
use glfw::Context;

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

use nalgebra_glm as glm;

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
    let positions: &[[[f32; 2]; 2]] = &[
        [[-0.5, -0.5], [0.0, 0.0]],
        [[0.5, -0.5], [1.0, 0.0]],
        [[0.5, 0.5], [1.0, 1.0]],
        [[-0.5, 0.5], [0.0, 1.0]],
    ];
    //
    let indices: &[[u32; 3]] = &[[0, 1, 2], [2, 3, 0]];
    //
    gl_call!(gl::Enable(gl::BLEND));
    gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
    //
    let va = VertexArray::new();
    let vb = VertexBuffer::from(positions);
    //
    let mut layout = VertexBufferLayout::new();
    layout.push::<f32>(2);
    layout.push::<f32>(2);
    va.add_buffer(&vb, layout);
    //
    va.bind();
    //
    let ib = IndexBuffer::from(indices);
    //
    let proj = glm::ortho(-2.0, 2.0, -1.5, 1.5, -1.0, 1.0);
    //
    let view = glm::translate(&glm::identity(), &glm::vec3(-0.5, 0.0, 0.0));
    //
    // let model = glm::rotate2d(&glm::identity(), 1.0);
    //
    let mvp = proj * view;
    //
    let mut shader = Shader::new("res/shaders/basic.shader");
    shader.bind();
    shader.set_uniform_mat4f("u_mvp\0", &mvp);
    //
    let tex = Texture::new("res/textures/mandrill.png");
    tex.bind();
    shader.set_uniform_1i("u_texture\0", 0);
    //
    va.unbind();
    vb.unbind();
    ib.unbind();
    shader.unbind();
    //
    let renderer = Renderer {};
    //
    // let mut timer = std::time::Instant::now();
    // let mut fps = 0;
    // Loop until the user closes the window
    while !window.should_close() {
        // gl_call!(gl::ClearColor(0.5, 0.0, 0.7, 1.0));
        renderer.clear();
        //
        // fps += 1;
        // if timer.elapsed().as_secs_f64() >= 1.0 {
        //     println!("fps: {}", fps);
        //     fps = 0;
        //     timer += std::time::Duration::from_secs(1);
        // }
        //
        shader.bind();
        //
        renderer.draw(&va, &ib, &shader);
        //
        // Swap front and back buffers
        window.swap_buffers();
        //
        // Poll for and process events
        glfw.poll_events();
    }
}

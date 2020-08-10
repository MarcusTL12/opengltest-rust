use gl;
use glfw::Context;

use std::ffi::c_void;

use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

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

fn compile_shader(source: &str, shader_type: u32) -> u32 {
    let id = gl_call!(gl::CreateShader(shader_type));
    gl_call!(gl::ShaderSource(
        id,
        1,
        &(source.as_ptr() as *const i8),
        0 as *const i32
    ));
    gl_call!(gl::CompileShader(id));
    //
    let mut result = 0;
    gl_call!(gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut result));
    //
    if result == 0 {
        let mut length = 0;
        gl_call!(gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length));
        let mut message = vec![0; length as usize];
        gl_call!(gl::GetShaderInfoLog(
            id,
            length,
            &mut length,
            message.as_mut_ptr()
        ));
        let message: String =
            message.into_iter().map(|i| i as u8 as char).collect();
        println!(
            "Failed to compile {} shader!, error length: {}\n{}",
            match shader_type {
                gl::VERTEX_SHADER => "vertex",
                gl::FRAGMENT_SHADER => "fragment",
                _ => "unknown",
            },
            length,
            message
        );
    }
    //
    id
}

fn create_shader(vertex_shader: &str, fragment_shader: &str) -> u32 {
    let program = gl_call!(gl::CreateProgram());
    let vs = compile_shader(vertex_shader, gl::VERTEX_SHADER);
    let fs = compile_shader(fragment_shader, gl::FRAGMENT_SHADER);
    //
    gl_call!(gl::AttachShader(program, vs));
    gl_call!(gl::AttachShader(program, fs));
    gl_call!(gl::LinkProgram(program));
    gl_call!(gl::ValidateProgram(program));
    //
    gl_call!(gl::DeleteShader(vs));
    gl_call!(gl::DeleteShader(fs));
    //
    program
}

fn parse_shader(filepath: &str) -> (String, String) {
    let mut vs = Vec::new();
    let mut fs = Vec::new();
    let mut active = &mut vs;
    //
    for line in BufReader::new(File::open(filepath).unwrap())
        .lines()
        .map(|l| l.unwrap())
    {
        match &line[..] {
            "#shader vertex" => active = &mut vs,
            "#shader fragment" => active = &mut fs,
            _ => writeln!(active, "{}", line).unwrap(),
        }
    }
    //
    vs.push(0);
    fs.push(0);
    //
    (
        String::from_utf8(vs).unwrap(),
        String::from_utf8(fs).unwrap(),
    )
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
    let (vertex_shader, fragment_shader) =
        parse_shader("res/shaders/basic.shader");
    //
    let shader = create_shader(&vertex_shader, &fragment_shader);
    gl_call!(gl::UseProgram(shader));
    //
    let location = gl_call!(gl::GetUniformLocation(
        shader,
        "u_color\0".as_ptr() as *const i8
    ));
    assert_ne!(location, -1);
    let timer = std::time::Instant::now();
    // Loop until the user closes the window
    while !window.should_close() {
        gl_call!(gl::ClearColor(0.0, 0.0, 0.0, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT));
        //
        gl_call!(gl::Uniform4f(
            location,
            timer.elapsed().as_secs_f32().sin().powi(2),
            (timer.elapsed().as_secs_f32() + 1.047).sin().powi(2),
            (timer.elapsed().as_secs_f32() + 2.094).sin().powi(2),
            1.0
        ));
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
    //
    gl_call!(gl::DeleteProgram(shader));
}

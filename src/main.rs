use gl;
use glfw::Context;

use std::ffi::c_void;

use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
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

fn compile_shader(source: &str, shader_type: u32) -> u32 {
    unsafe {
        let id = gl::CreateShader(shader_type);
        gl::ShaderSource(
            id,
            1,
            &(source.as_ptr() as *const i8),
            0 as *const i32,
        );
        gl::CompileShader(id);
        //
        let mut result = 0;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut result);
        //
        if result == 0 {
            let mut length = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length);
            let mut message = vec![0; length as usize];
            gl::GetShaderInfoLog(id, length, &mut length, message.as_mut_ptr());
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
}

fn create_shader(vertex_shader: &str, fragment_shader: &str) -> u32 {
    unsafe {
        let program = gl::CreateProgram();
        let vs = compile_shader(vertex_shader, gl::VERTEX_SHADER);
        let fs = compile_shader(fragment_shader, gl::FRAGMENT_SHADER);
        //
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        gl::ValidateProgram(program);
        //
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        //
        program
    }
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
    // window.set_key_polling(true);
    //
    gl::load_with(|s| window.get_proc_address(s));
    //
    get_gl_version();
    //
    let positions: &[f32] = &[-0.5, -0.5, 0.0, 0.5, 0.5, -0.5];
    //
    let mut b: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut b);
        gl::BindBuffer(gl::ARRAY_BUFFER, b);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (positions.len() * std::mem::size_of::<f32>()) as isize,
            positions.as_ptr() as *mut c_void,
            gl::STATIC_DRAW,
        );
        //
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (2 * std::mem::size_of::<f32>()) as i32,
            0 as *mut c_void,
        )
    };
    //
    let (vertex_shader, fragment_shader) =
        parse_shader("res/shaders/basic.shader");
    //
    let shader = create_shader(&vertex_shader, &fragment_shader);
    unsafe {
        gl::UseProgram(shader);
    }
    // Loop until the user closes the window
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            //
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        //
        // Swap front and back buffers
        window.swap_buffers();
        //
        // Poll for and process events
        glfw.poll_events();
    }
    //
    unsafe {
        gl::DeleteProgram(shader);
    }
}

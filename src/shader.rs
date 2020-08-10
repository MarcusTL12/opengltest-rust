use super::glcall;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use nalgebra_glm as glm;

pub struct Shader {
    _filepath: String,
    renderer_id: u32,
    uniform_location_cache: HashMap<String, i32>,
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

impl Shader {
    pub fn new(filepath: &str) -> Self {
        let (vs, fs) = parse_shader(filepath);
        let id = create_shader(&vs, &fs);
        //
        Self {
            _filepath: filepath.to_owned(),
            renderer_id: id,
            uniform_location_cache: HashMap::new(),
        }
    }
    //
    pub fn bind(&self) {
        gl_call!(gl::UseProgram(self.renderer_id));
    }
    //
    pub fn unbind(&self) {
        gl_call!(gl::UseProgram(0));
    }
    //
    fn get_uniform_location(&mut self, name: &str) -> i32 {
        if let Some(&location) = self.uniform_location_cache.get(name) {
            location
        } else {
            let location = gl_call!(gl::GetUniformLocation(
                self.renderer_id,
                name.as_ptr() as *const i8
            ));
            //
            if location == -1 {
                println!("Warning: uniform: '{}', does not exist!", name);
            }
            //
            self.uniform_location_cache
                .insert(name.to_owned(), location);
            location
        }
    }
    //
    pub fn set_uniform_4f(&mut self, name: &str, vals: [f32; 4]) {
        gl_call!(gl::Uniform4f(
            self.get_uniform_location(name),
            vals[0],
            vals[1],
            vals[2],
            vals[3]
        ));
    }
    //
    pub fn set_uniform_1i(&mut self, name: &str, val: i32) {
        gl_call!(gl::Uniform1i(self.get_uniform_location(name), val));
    }
    //
    pub fn set_uniform_mat4f(
        &mut self,
        name: &str,
        val: &glm::Mat4,
    ) {
        gl_call!(gl::UniformMatrix4fv(
            self.get_uniform_location(name),
            1,
            gl::FALSE,
            val.as_slice().as_ptr()
        ));
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.renderer_id));
    }
}

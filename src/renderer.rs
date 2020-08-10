use super::glcall;

use std::ffi::c_void;

use gl;

use super::{IndexBuffer, Shader, VertexArray};
pub struct Renderer {}

impl Renderer {
    pub fn draw(&self, va: &VertexArray, ib: &IndexBuffer, shader: &Shader) {
        shader.bind();
        va.bind();
        ib.bind();
        //
        gl_call!(gl::DrawElements(
            gl::TRIANGLES,
            ib.count,
            gl::UNSIGNED_INT,
            0 as *const c_void
        ));
    }
    //
    pub fn clear(&self) {
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT));
    }
}

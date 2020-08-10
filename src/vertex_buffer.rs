use super::glcall;

use std::ffi::c_void;

pub struct VertexBuffer {
    renderer_id: u32,
}

impl VertexBuffer {
    pub fn bind(&self) {
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.renderer_id));
    }
    pub fn unbind(&self) {
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, 0));
    }
}

impl<T> From<&[T]> for VertexBuffer {
    fn from(data: &[T]) -> Self {
        let mut id = 0;
        //
        gl_call!(gl::GenBuffers(1, &mut id));
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, id));
        gl_call!(gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<T>()) as isize,
            data.as_ptr() as *mut c_void,
            gl::STATIC_DRAW,
        ));
        //
        Self { renderer_id: id }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        gl_call!(gl::DeleteBuffers(1, &self.renderer_id));
    }
}

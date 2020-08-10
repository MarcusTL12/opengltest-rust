use super::glcall;

use std::ffi::c_void;

pub struct IndexBuffer {
    renderer_id: u32,
    pub count: i32,
}

impl IndexBuffer {
    pub fn bind(&self) {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.renderer_id));
    }
    pub fn unbind(&self) {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
    }
}

impl From<&[u32]> for IndexBuffer {
    fn from(data: &[u32]) -> Self {
        let mut id = 0;
        //
        gl_call!(gl::GenBuffers(1, &mut id));
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id));
        gl_call!(gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<u32>()) as isize,
            data.as_ptr() as *mut c_void,
            gl::STATIC_DRAW,
        ));
        //
        Self {
            renderer_id: id,
            count: data.len() as i32,
        }
    }
}

impl From<&[[u32; 3]]> for IndexBuffer {
    fn from(data: &[[u32; 3]]) -> Self {
        unsafe {
            let tmp = data.as_ptr() as *const u32;
            let tmp = std::slice::from_raw_parts(tmp, data.len() * 3);
            Self::from(tmp)
        }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        gl_call!(gl::DeleteBuffers(1, &self.renderer_id));
    }
}

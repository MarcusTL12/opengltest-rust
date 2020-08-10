use super::glcall;

use super::VertexBuffer;
use super::VertexBufferLayout;

use std::ffi::c_void;

pub struct VertexArray {
    renderer_id: u32,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut id = 0;
        gl_call!(gl::GenVertexArrays(1, &mut id));
        Self { renderer_id: id }
    }
    //
    pub fn bind(&self) {
        gl_call!(gl::BindVertexArray(self.renderer_id));
    }
    //
    pub fn unbind(&self) {
        gl_call!(gl::BindVertexArray(0));
    }
    //
    pub fn add_buffer(&self, vb: &VertexBuffer, layout: VertexBufferLayout) {
        // return;
        self.bind();
        vb.bind();
        // let elements = layout.get_elements();
        //
        let mut offset = 0;
        //
        for (i, element) in layout.get_elements().iter().enumerate() {
            let i = i as u32;
            //
            gl_call!(gl::EnableVertexAttribArray(i));
            gl_call!(gl::VertexAttribPointer(
                i,
                element.count,
                element.el_type,
                if element.normalized {
                    gl::TRUE
                } else {
                    gl::FALSE
                },
                layout.get_stride(),
                offset as *const c_void,
            ));
            offset += element.count * element.type_size();
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        gl_call!(gl::DeleteVertexArrays(1, &self.renderer_id));
    }
}

use super::glcall;

use std::ffi::c_void;

use image;

fn load_img(filepath: &str) -> (u32, u32, Vec<u8>) {
    let img = image::open(filepath)
        .expect(&format!("Image: '{}' did not open correctly", filepath))
        .flipv()
        .to_rgba();
    //
    let width = img.width();
    let height = img.height();
    //
    (width, height, img.to_vec())
}

pub struct Texture {
    _filepath: String,
    render_id: u32,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn from_file(filepath: &str) -> Self {
        let mut id = 0;
        gl_call!(gl::GenTextures(1, &mut id));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, id));
        //
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR as i32
        ));
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::LINEAR as i32
        ));
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_EDGE as i32,
        ));
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_EDGE as i32,
        ));
        //
        let (width, height, buffer) = load_img(filepath);
        //
        gl_call!(gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            (&buffer).as_ptr() as *const c_void
        ));
        //
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));
        //
        Self {
            _filepath: filepath.to_owned(),
            render_id: id,
            width: width,
            height: height,
        }
    }
    //
    pub fn bind_slot(&self, slot: u32) {
        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + slot));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.render_id));
    }
    //
    pub fn bind(&self) {
        self.bind_slot(0);
    }
    //
    pub fn unbind(&self) {
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        gl_call!(gl::DeleteTextures(1, &self.render_id))
    }
}

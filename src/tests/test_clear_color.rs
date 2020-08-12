use super::super::glcall;

use super::*;

pub struct TestClearColor {
    color: [f32; 4],
}

impl OGLTest for TestClearColor {
    fn new() -> Self {
        Self {
            color: [77.0 / 255.0, 140.0 / 255.0, 204.0 / 255.0, 1.0],
        }
    }
    //
    fn on_render(&mut self) {
        gl_call!(gl::ClearColor(
            self.color[0],
            self.color[1],
            self.color[2],
            self.color[3]
        ));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT));
    }
    //
    fn on_imgui_render(&mut self, ui: &imgui_glfw_rs::imgui::Ui) {
        ui.color_edit(im_str!("Clear Color"), &mut self.color)
            .build();
    }
}

impl Drop for TestClearColor {
    fn drop(&mut self) {
        println!("Dropping TestClearColor!");
    }
}

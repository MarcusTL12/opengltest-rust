use super::super::glcall;

use super::*;

pub struct TestClearColor {
    color: [f32; 4],
}

impl TestClearColor {
    pub fn new() -> Self {
        Self {
            color: [0.8, 0.3, 0.6, 1.0],
        }
    }
}

impl OGLTest for TestClearColor {
    fn on_update(&mut self, _delta_time: f32) {}
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

pub mod test_clear_color;

use imgui_glfw_rs::{imgui::im_str};

pub trait OGLTest {
    fn on_update(&mut self, delta_time: f32);
    fn on_render(&mut self);
    fn on_imgui_render(&mut self, ui: &imgui_glfw_rs::imgui::Ui);
}

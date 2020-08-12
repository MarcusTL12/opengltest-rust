pub mod test_clear_color;

use imgui_glfw_rs::imgui::{im_str, Window};

pub trait OGLTest {
    fn new() -> Self
    where
        Self: Sized;
    fn on_update(&mut self, _delta_time: f32) {}
    fn on_render(&mut self) {}
    fn on_imgui_render(&mut self, _ui: &imgui_glfw_rs::imgui::Ui) {}
}

pub struct TestMenu {
    current_test: Option<(String, Box<dyn OGLTest>)>,
    tests: Vec<(String, fn() -> Box<dyn OGLTest>)>,
}

impl TestMenu {
    pub fn register_test<T: OGLTest + 'static>(&mut self, name: &str) {
        self.tests.push((name.to_owned(), || Box::new(T::new())))
    }
}

impl OGLTest for TestMenu {
    fn new() -> Self {
        Self {
            current_test: None,
            tests: Vec::new(),
        }
    }
    //
    fn on_update(&mut self, delta_time: f32) {
        if let Some((_, test)) = &mut self.current_test {
            test.on_update(delta_time);
        }
    }
    //
    fn on_render(&mut self) {
        if let Some((_, test)) = &mut self.current_test {
            test.on_render();
        }
    }
    //
    fn on_imgui_render(&mut self, ui: &imgui_glfw_rs::imgui::Ui) {
        if let Some((test_name, test)) = &mut self.current_test {
            let mut close = false;
            Window::new(ui, &im_str!("{}", test_name)).build(|| {
                close = ui.button(&im_str!("<-"), [0.0, 0.0]);
                test.on_imgui_render(ui);
            });
            //
            if close {
                self.current_test = None;
            }
        } else {
            Window::new(ui, &im_str!("Test Menu")).build(|| {
                for (test_name, test_constructor) in &self.tests {
                    if ui.button(&im_str!("{}", test_name), [0.0, 0.0]) {
                        self.current_test =
                            Some((test_name.clone(), test_constructor()));
                    }
                }
            })
        }
    }
}

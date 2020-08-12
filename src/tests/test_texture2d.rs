use super::super::*;

use std::io::Write;

pub struct TestTexture2D {
    translation1: [f32; 3],
    translation2: [f32; 3],
    //
    va: VertexArray,
    _vb: VertexBuffer,
    ib: IndexBuffer,
    //
    shader: Shader,
    //
    proj: glm::Mat4,
    //
    texture1: Texture,
    texture2: Texture,
    //
    renderer: Renderer,
    //
    timer: std::time::Instant,
    fps_counter: u32,
    fps_view: Vec<u8>,
}

impl OGLTest for TestTexture2D {
    fn new() -> Self {
        let positions: &[_] = &[
            [[-0.5f32, -0.5], [0.0, 0.0]],
            [[0.5, -0.5], [1.0, 0.0]],
            [[0.5, 0.5], [1.0, 1.0]],
            [[-0.5, 0.5], [0.0, 1.0]],
        ];
        let indices: &[_] = &[[0, 1, 2], [2, 3, 0]];
        //
        gl_call!(gl::Enable(gl::BLEND));
        gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
        //
        let vb = VertexBuffer::from(positions);
        //
        let mut layout = VertexBufferLayout::new();
        layout.push::<f32>(2);
        layout.push::<f32>(2);
        //
        let va = VertexArray::new();
        va.add_buffer(&vb, layout);
        //
        let ib = IndexBuffer::from(indices);
        //
        let shader = Shader::from_file("res/shaders/basic.shader");
        //
        let proj = glm::ortho(-2.0, 2.0, -1.5, 1.5, -1.0, 1.0);
        //
        let tex1 = Texture::from_file("res/textures/mandrill.png");
        let tex2 = Texture::from_file("res/textures/trans.png");
        //
        Self {
            translation1: [0.0; 3],
            translation2: [0.0; 3],
            va: va,
            _vb: vb,
            ib: ib,
            shader: shader,
            proj: proj,
            texture1: tex1,
            texture2: tex2,
            renderer: Renderer {},
            timer: std::time::Instant::now(),
            fps_counter: 0,
            fps_view: "fps: 0".to_owned().into_bytes(),
        }
    }
    //
    fn on_update(&mut self, _: f32) {
        self.fps_counter += 1;
        if self.timer.elapsed() > std::time::Duration::from_secs(1) {
            self.fps_view.clear();
            write!(self.fps_view, "fps: {}", self.fps_counter).unwrap();
            self.fps_counter = 0;
            self.timer += std::time::Duration::from_secs(1);
        }
    }
    //
    fn on_render(&mut self) {
        {
            let mvp = self.proj
                * glm::translate(
                    &glm::identity(),
                    &glm::vec3(
                        self.translation1[0],
                        self.translation1[1],
                        self.translation1[2],
                    ),
                );
            //
            self.shader.bind();
            self.shader.set_uniform_mat4f("u_mvp\0", &mvp);
            //
            self.texture1.bind();
            self.renderer.draw(&self.va, &self.ib, &self.shader);
        }
        //
        {
            let mvp = self.proj
                * glm::translate(
                    &glm::identity(),
                    &glm::vec3(
                        self.translation2[0],
                        self.translation2[1],
                        self.translation2[2],
                    ),
                );
            //
            self.shader.bind();
            self.shader.set_uniform_mat4f("u_mvp\0", &mvp);
            //
            self.texture2.bind();
            self.renderer.draw(&self.va, &self.ib, &self.shader);
        }
    }
    //
    fn on_imgui_render(&mut self, ui: &imgui_glfw_rs::imgui::Ui) {
        ui.text(&std::str::from_utf8(&self.fps_view).unwrap());
        //
        ui.slider_float3(
            im_str!("Translation A"),
            &mut self.translation1,
            -1.0,
            1.0,
        )
        .build();
        //
        ui.slider_float3(
            im_str!("Translation B"),
            &mut self.translation2,
            -1.0,
            1.0,
        )
        .build();
    }
}

impl Drop for TestTexture2D {
    fn drop(&mut self) {
        println!("Dropping TestTexture2D!");
    }
}

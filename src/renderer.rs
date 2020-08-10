use gl;

// #[macro_export]
macro_rules! gl_call {
    ($x:expr) => {{
        renderer::gl_clear_error();
        let result = unsafe { $x };
        if !(renderer::gl_log_call()) {
            panic!("OpenGL error at\n{}\n", stringify!($x));
        }
        result
    }};
}

pub fn gl_clear_error() {
    unsafe { while gl::GetError() != gl::NO_ERROR {} }
}

pub fn gl_log_call() -> bool {
    unsafe {
        let mut error;
        let mut result = true;
        while {
            error = gl::GetError();
            error != gl::NO_ERROR
        } {
            println!("OpenGL_Error (0x{:x})", error);
            result = false;
        }
        result
    }
}
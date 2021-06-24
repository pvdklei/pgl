#[macro_export]
macro_rules! const_char_ptr {
    ($str:expr) => {{
        std::ffi::CString::new($str)
            .expect(&format!("Could not make const char ptr out of {}", $str))
            .as_ptr() as *const i8
    }};
}

pub mod gl {
    pub fn flush_error() {
        unsafe { while gl::GetError() != gl::NO_ERROR {} }
    }

    pub fn check_error() {
        unsafe {
            loop {
                let error = gl::GetError();
                if error == gl::NO_ERROR {
                    break;
                }
                println!("[GL ERROR CODE] {}", error);
            }
        }
    }

    pub fn finish() {
        unsafe { gl::Finish() }
    }

    pub fn draw(n_indices: usize) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                n_indices as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }

    pub fn draw_offset(n_indices: usize, offset: usize) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                n_indices as i32,
                gl::UNSIGNED_INT,
                offset as _,
            );
        }
    }
    pub fn set_default_options() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn clear() {
        unsafe {
            gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn clear_color(r: f32, g: f32, b: f32) {
        unsafe {
            gl::ClearColor(r, g, b, 1.0);
        }
    }

    pub fn viewport_info() -> (usize, usize, usize, usize) {
        unsafe {
            let p = [0i32; 4];
            gl::GetIntegerv(gl::VIEWPORT, p.as_ptr() as *mut i32);
            (p[0] as usize, p[1] as usize, p[2] as usize, p[3] as usize)
        }
    }

    pub fn viewport(x: usize, y: usize, w: usize, h: usize) {
        unsafe {
            gl::Viewport(x as i32, y as i32, w as i32, h as i32);
        }
    }

    pub fn scissor(x: usize, y: usize, w: usize, h: usize) {
        unsafe {
            gl::Scissor(x as _, y as _, w as _, h as _);
        }
    }
}

pub fn enable(s: &[Option]) {
    s.iter().for_each(|e| e.enable())
}

pub fn disable(s: &[Option]) {
    s.iter().for_each(|e| e.disable())
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Option {
    Depth = gl::DEPTH_TEST,
    Scissor = gl::SCISSOR_TEST,
    Culling = gl::CULL_FACE,
    Blend = gl::BLEND,
    Wireframe,
}

impl Option {
    fn enable(&self) {
        unsafe {
            match self {
                Option::Wireframe => gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE),
                _ => gl::Enable(*self as _),
            }
        }
    }
    fn disable(&self) {
        unsafe {
            match self {
                Option::Wireframe => gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL),
                _ => gl::Disable(*self as _),
            }
        }
    }
}

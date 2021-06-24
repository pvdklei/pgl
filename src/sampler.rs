use gl::types::*;

pub struct Sampler {
    id: GLuint,
}

impl Sampler {
    pub fn new() -> Self {
        unsafe {
            let mut id = 0;
            gl::GenSamplers(1, &mut id);
            Self { id }
        }
    }
    pub fn bind_to(&self, slot: u32) {
        unsafe {
            gl::BindSampler(slot, self.id);
        }
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteSamplers(1, &self.id);
        }
    }
}

#[repr(u32)]
pub enum Param {
    WrapS = gl::TEXTURE_WRAP_S,
    WrapT = gl::TEXTURE_WRAP_T,
    WrapR = gl::TEXTURE_WRAP_R,
    MinFilter = gl::TEXTURE_MIN_FILTER,
    MagFilter = gl::TEXTURE_MAG_FILTER,
}

// TODO: FINSIH

use super::texture::Texture;

pub struct FrameBuffer {
    id: u32,
}

impl FrameBuffer {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut id);
        }
        Self { id }
    }
    pub fn ok(&self) -> Result<(), &'static str> {
        if !unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE } {
            return Err("Framebuffer is not completed, cannot be used yet");
        }
        Ok(())
    }
    pub fn bind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.id) }
    }
    pub fn unbind() {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) }
    }
    pub fn attach_texture(&self, tex: Texture, type_: AttachmentType) {
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, type_.gl(), gl::TEXTURE_2D, tex.id(), 0);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &self.id as *const u32) }
    }
}

pub enum AttachmentType {
    Color(usize),
    Stencil,
    Depth,
}

impl AttachmentType {
    pub fn gl(&self) -> u32 {
        match self {
            Self::Color(n) => gl::COLOR_ATTACHMENT0 + *n as u32,
            Self::Stencil => gl::STENCIL_ATTACHMENT,
            Self::Depth => gl::DEPTH_ATTACHMENT,
        }
    }
}

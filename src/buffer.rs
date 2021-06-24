pub struct Buffer {
    id: u32,
    buffertype: gl::types::GLenum,
    drawtype: gl::types::GLenum,
}

impl Buffer {
    pub fn new(buffertype: BufferType, drawtype: DrawType) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self {
            buffertype: buffertype as u32,
            drawtype: drawtype as u32,
            id,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffertype, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.buffertype, 0);
        }
    }

    /// F.i. for binding a uniform buffer to a certain binding point.
    /// layout (std140, binding=thisbinding) uniform { ... }
    pub fn set_binding(&self, binding: usize) {
        unsafe { gl::BindBufferBase(self.buffertype, binding as u32, self.id) }
    }

    pub fn init(&self, size: usize) {
        unsafe {
            gl::BufferData(
                self.buffertype,
                size as isize,
                std::ptr::null(),
                self.drawtype,
            );
        }
    }

    pub fn buffer<T>(&self, content: &[T]) {
        unsafe {
            gl::BufferData(
                self.buffertype,
                (content.len() * std::mem::size_of::<T>()) as isize,
                content.as_ptr() as *const std::ffi::c_void,
                self.drawtype,
            );
        }
    }

    pub fn subbuffer<T>(&self, content: &[T], offset: usize) {
        unsafe {
            gl::BufferSubData(
                self.buffertype,
                offset as isize,
                (content.len() * std::mem::size_of::<T>()) as isize,
                content.as_ptr() as *const std::ffi::c_void,
            );
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BufferType {
    Index = gl::ELEMENT_ARRAY_BUFFER,
    Vertex = gl::ARRAY_BUFFER,
    Uniform = gl::UNIFORM_BUFFER,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum DrawType {
    Static = gl::STATIC_DRAW,
    Dynamic = gl::DYNAMIC_DRAW,
}

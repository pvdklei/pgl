use gl::types::*;

pub struct Query {
    id: GLuint,
    target: GLenum,
}

impl Query {
    pub fn new(target: Target) -> Self {
        unsafe {
            let mut id = 0;
            gl::GenQueries(1, &mut id);
            Self {
                id,
                target: target as GLenum,
            }
        }
    }

    pub fn begin(&self) {
        unsafe { gl::BeginQuery(self.target, self.id) }
    }
    pub fn end(&self) {
        unsafe { gl::EndQuery(self.target) }
    }
    pub fn result(&self) -> i64 {
        unsafe {
            let mut res = 0;
            gl::GetQueryObjecti64v(self.id, gl::QUERY_RESULT, &mut res);
            res
        }
    }
    pub fn result_available(&self) -> bool {
        unsafe {
            let mut res = 0;
            gl::GetQueryObjecti64v(self.id, gl::QUERY_RESULT_AVAILABLE, &mut res);
            res == 1
        }
    }
}

impl Drop for Query {
    fn drop(&mut self) {
        unsafe { gl::DeleteQueries(1, &self.id) }
    }
}

#[repr(u32)]
pub enum Target {
    TimeElapsed = gl::TIME_ELAPSED,
    TimeStamp = gl::TIMESTAMP,
}

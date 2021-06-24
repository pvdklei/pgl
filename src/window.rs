use glfw::ffi::*;
use std::ptr::null_mut;

pub struct GlfwWindow {
    pub window: *mut GLFWwindow,
}

impl Default for GlfwWindow {
    fn default() -> Self {
        Self::new(800, 800, "MahwFawkinWinow")
    }
}

impl GlfwWindow {
    pub fn new(w: isize, h: isize, des: &str) -> Self {
        unsafe {
            assert!(glfwInit() == 1);
            Self::set_hints(3, 2);
            glfwSetErrorCallback(Some(glfw_error_callback));

            let window = glfwCreateWindow(
                w as i32,
                h as i32,
                const_char_ptr!(des),
                null_mut(),
                null_mut(),
            );
            assert!(!window.is_null());

            let this = Self { window };

            this.make_current();
            gl::load_with(|s| glfwGetProcAddress(const_char_ptr!(s)) as *const _); // load gl function ptrs

            this
        }
    }
    pub fn get_proc_address(s: &'static str) -> *const std::os::raw::c_void {
        unsafe { glfwGetProcAddress(const_char_ptr!(s)) as *const std::os::raw::c_void }
    }
    fn set_hints(major: i32, minor: i32) {
        unsafe {
            glfwWindowHint(OPENGL_PROFILE, OPENGL_CORE_PROFILE);
            glfwWindowHint(CONTEXT_VERSION_MAJOR, major);
            glfwWindowHint(CONTEXT_VERSION_MINOR, minor);
            glfwWindowHint(OPENGL_FORWARD_COMPAT, 1);
        }
    }
    pub fn destroy_glfw() {
        unsafe { glfwTerminate() }
    }
    pub fn should_close(&self) -> bool {
        unsafe { glfwWindowShouldClose(self.window) == 1 }
    }
    pub fn make_current(&self) {
        unsafe { glfwMakeContextCurrent(self.window) }
    }
    pub fn set_window_size(&self, w: isize, h: isize) {
        unsafe {
            glfwSetWindowSize(self.window, w as i32, h as i32);
        }
    }
    pub fn swap_buffers(&self) {
        unsafe { glfwSwapBuffers(self.window) }
    }
    pub fn poll_events(&self) {
        unsafe { glfwPollEvents() }
    }
    pub fn is_key_pressed(&self, key: Key) -> bool {
        unsafe { glfwGetKey(self.window, key as i32) == PRESS }
    }

    pub fn is_mouse_presses(&self) -> bool {
        unsafe { glfwGetMouseButton(self.window, MOUSE_BUTTON_LEFT) == PRESS }
    }

    pub fn cursor_pos(&self) -> (f32, f32) {
        let mut x = 0.0f64;
        let mut y = 0.0f64;
        unsafe {
            glfwGetCursorPos(self.window, &mut x as *mut f64, &mut y as *mut f64);
        }
        (x as f32, y as f32)
    }
    pub fn window_size(&self) -> (u32, u32) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            glfwGetWindowSize(self.window, &mut w as *mut i32, &mut h as *mut i32);
        }
        (w as u32, h as u32)
    }
    pub fn framebuffer_size(&self) -> (u32, u32) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            glfwGetFramebufferSize(self.window, &mut w, &mut h);
        }
        (w as u32, h as u32)
    }
    pub fn hidpi_factor(&self) -> f32 {
        self.framebuffer_size().0 as f32 / self.window_size().0 as f32
    }
    pub fn time() -> f64 {
        unsafe { glfwGetTime() }
    }
    pub fn aspect(&self) -> f32 {
        let (w, h) = self.window_size();
        w as f32 / h as f32
    }
}

impl Drop for GlfwWindow {
    fn drop(&mut self) {
        unsafe { glfwDestroyWindow(self.window) }
    }
}

#[repr(i32)]
pub enum Key {
    Up = KEY_UP,
    Down = KEY_DOWN,
    Left = KEY_LEFT,
    Right = KEY_RIGHT,
    Escape = KEY_ESCAPE,
    Space = KEY_SPACE,
    A = KEY_A,
    W = KEY_W,
    D = KEY_D,
    S = KEY_S,
    N = KEY_N,
    M = KEY_M,
}

extern "C" fn glfw_error_callback(error: i32, desc: *const i8) {
    unsafe {
        let desc = std::ffi::CStr::from_ptr(desc);
        println!("[GLFW ERROR] {}: {:?}", error, desc);
        panic!()
    }
}

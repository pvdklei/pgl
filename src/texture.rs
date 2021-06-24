use gl;
use std::os::raw::c_void;

pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn from_path(path: &str, options: Options) -> Self {
        let img = image::open(path)
            .expect(&format!("Could not open image from path '{:?}'", path))
            .flipv()
            .into_rgba8();
        let dims = img.dimensions();
        Self::from_data(&img.into_raw(), options, dims)
    }

    pub fn from_data<T>(img: &[T], options: Options, (width, height): (u32, u32)) -> Self {
        unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const c_void,
            );
            if options.mipmap {
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
            options.set();
            gl::BindTexture(gl::TEXTURE_2D, 0);
            Self { id }
        }
    }

    pub fn empty(options: Options, (width, height): (u32, u32)) -> Self {
        unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            if options.mipmap {
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
            options.set();
            Self::unbind();
            Self { id }
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    // Note: do all the binding after each other before draw
    //       f.i. dont create new textures between bind_to and draw
    pub fn bind_to(&self, slot: u32) -> Result<(), &'static str> {
        if slot >= 16 {
            return Err("The textureslot cannot be higher than 16");
        }
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            self.bind();
            gl::ActiveTexture(gl::TEXTURE0);
        }
        Ok(())
    }

    // for acces by framebuffer
    pub(crate) fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id as *const u32);
        }
    }
}

pub struct Options {
    pub wrapping: Wrapping,
    pub filtering: Filtering,
    pub mipmap: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            filtering: Filtering::Linear,
            wrapping: Wrapping::Repeat,
            mipmap: true,
        }
    }
}

// TODO: DROP FUNCTION

impl Options {
    fn set(&self) {
        use Filtering::*;
        use Wrapping::*;

        unsafe {
            let wrap = match self.wrapping {
                Repeat => gl::REPEAT,
                MirrorRepeat => gl::MIRRORED_REPEAT,
                ClampEdge => gl::CLAMP_TO_EDGE,
                Constant(r, g, b) => {
                    let c = &[r, g, b, 1.0] as *const f32;
                    gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, c);
                    gl::CLAMP_TO_BORDER
                }
            } as i32;
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap);
            match self.filtering {
                Linear => {
                    gl::TexParameteri(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MIN_FILTER,
                        if self.mipmap {
                            gl::LINEAR_MIPMAP_LINEAR as i32
                        } else {
                            gl::LINEAR as i32
                        },
                    );
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                }
                Nearest => {
                    gl::TexParameteri(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MIN_FILTER,
                        if self.mipmap {
                            gl::NEAREST_MIPMAP_LINEAR as i32
                        } else {
                            gl::NEAREST as i32
                        },
                    );
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                }
            };
        }
    }
}

pub enum Wrapping {
    Repeat,
    MirrorRepeat,
    ClampEdge,
    Constant(f32, f32, f32),
}

pub enum Filtering {
    Linear,
    Nearest,
}

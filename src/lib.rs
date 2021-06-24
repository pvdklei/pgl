#[macro_use]
pub mod utils;
pub mod buffer;
pub mod framebuffer;
pub mod query;
pub mod sampler;
pub mod settings;
pub mod shader;
pub mod texture;
pub mod vao;
pub mod window;

pub use nalgebra_glm as glm;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum DType {
    Float = gl::FLOAT,
    Int = gl::INT,
    UInt = gl::UNSIGNED_INT,
    Byte = gl::BYTE,
}

impl DType {
    pub fn size(&self) -> usize {
        match self {
            Self::Float => 4,
            Self::Int => 4,
            Self::UInt => 4,
            Self::Byte => 1,
        }
    }
}

// Do not add arrays to this, you cannot have more then 
// 4 elements in a vertex attribute. 
#[derive(Copy, Clone, Debug)]
pub enum GlslDType {
    Vec2,
    Vec3,
    Vec4,
    Float,
    Mat4,
    Mat3,
}

impl GlslDType {
    fn dtype(&self) -> DType {
        match self {
            Self::Float
            | Self::Vec2
            | Self::Vec3
            | Self::Vec4
            | Self::Mat3
            | Self::Mat4 => DType::Float,
        }
    }
    fn n_elements(&self) -> usize {
        match self {
            Self::Float => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 => 4,
            Self::Mat3 => 9,
            Self::Mat4 => 16,
        }
    }
}

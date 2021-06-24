use gl::types::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::path::Path;

pub struct ShaderOptions {
    pub vs_defines: Vec<String>, 
    pub fs_defines: Vec<String>,
}

impl Default for ShaderOptions {
    fn default() -> Self {
        Self {
            vs_defines: Vec::new(),
            fs_defines: Vec::new(),
        }
    }
}

pub struct ShaderProgram {
    id: GLuint,
    loc_cache: HashMap<String, GLint>,
}

impl ShaderProgram {
    pub fn from_frag_and_vert_src(fs: &str, vs: &str) -> Result<Self, String> {
        let fs = Shader::from_source(fs, gl::FRAGMENT_SHADER).unwrap();

        let vs = Shader::from_source(vs, gl::VERTEX_SHADER).unwrap();
        Self::from_frag_and_vert_structs(fs, vs)
    }

    pub fn from_frag_and_vert_path(
        fs: impl AsRef<Path>,
        vs: impl AsRef<Path>,
    ) -> Result<Self, String> {
        let fs = Shader::from_path(fs, gl::FRAGMENT_SHADER)?;

        let vs = Shader::from_path(vs, gl::VERTEX_SHADER)?;
        Self::from_frag_and_vert_structs(fs, vs)
    }

    /// usable with the ''#type fragment/vertex' syntax
    /// and you can add options like defines 
    pub fn from_path(src_path: impl AsRef<Path>, options: ShaderOptions) -> Result<Self, String> {
        let path_base = src_path
            .as_ref()
            .ancestors()
            .skip(1)
            .next()
            .unwrap()
            .to_owned();
        let path_base = path_base.to_str().unwrap();

        let src = std::fs::read_to_string(src_path).unwrap();
        let mut split = src.split("#type").into_iter();
        let vertex_src = split
            .find(|&x| x.trim_start().starts_with("vertex"))
            .unwrap()
            .split_once("\n")
            .unwrap()
            .1;
        let vertex_src = Shader::unroll_includes(vertex_src, path_base);
        let vertex_src = Self::add_defines(&vertex_src, &options.vs_defines);

        let fragment_src = split
            .find(|&x| x.trim_start().starts_with("fragment"))
            .unwrap()
            .split_once("\n")
            .unwrap()
            .1;
        let fragment_src = Shader::unroll_includes(fragment_src, path_base);
        let fragment_src = Self::add_defines(&fragment_src, &options.fs_defines);

        Self::from_frag_and_vert_src(&fragment_src, &vertex_src)
    }

    fn from_frag_and_vert_structs(fs: Shader, vs: Shader) -> Result<Self, String> {
        unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, fs.id);
            gl::AttachShader(id, vs.id);
            gl::LinkProgram(id);
            // ERROR HANDLING
            let mut it_worked = gl::FALSE as gl::types::GLint;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut it_worked);
            if it_worked == 0 {
                let mut len = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                let buf = String::from_utf8_unchecked(buf);

                let error = String::from(&format!("[ERROR] Problem with shader linking: {:?}", buf));
                return Err(error);
            }
            // END ERROR HANDLING
            gl::DetachShader(id, fs.id);
            gl::DetachShader(id, vs.id);

            let loc_cache = HashMap::new();

            Ok(Self { id, loc_cache })
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_location(&mut self, name: &str) -> i32 {
        match self.loc_cache.get(name) {
            Some(loc) => *loc,
            None => {
                let loc: GLint =
                    unsafe { gl::GetUniformLocation(self.id, const_char_ptr!(name) as *const i8) };
                if loc == -1 {
                    println!("uniform name does not exist or starts with reserved prefix");
                } else {
                    self.loc_cache.insert(name.to_string(), loc);
                }
                loc
            }
        }
    }

    pub fn bind_uniform_block(&self, name: &str, binding: usize) {
        unsafe {
            let index = gl::GetUniformBlockIndex(self.id, const_char_ptr!(name));
            gl::UniformBlockBinding(self.id, index, binding as u32);
        }
    }

    pub fn set_uint(&mut self, name: &str, uint: u32) {
        unsafe {
            let loc = self.get_location(name);
            gl::Uniform1ui(loc, uint);
        }
    }

    pub fn set_int(&mut self, name: &str, int: i32) {
        unsafe {
            let loc = self.get_location(name);
            gl::Uniform1i(loc, int);
        }
    }

    pub fn set_float(&mut self, name: &str, float: GLfloat) {
        unsafe {
            let loc = self.get_location(name);
            gl::Uniform1f(loc, float);
        }
    }

    pub fn set_vec3fs<T>(&mut self, name: &str, values: &[T]) {
        unsafe {
            let loc = self.get_location(name);
            gl::Uniform3fv(loc, values.len() as i32, values.as_ptr() as *const f32)
        }
    }

    pub fn set_vec4fs<T>(&mut self, name: &str, values: &[T]) {
        unsafe {
            let loc = self.get_location(name);
            gl::Uniform4fv(loc, values.len() as i32, values.as_ptr() as *const f32)
        }
    }

    pub fn set_mat4fs<T>(&mut self, name: &str, mats: &[T]) {
        unsafe {
            let loc = self.get_location(name);
            gl::UniformMatrix4fv(
                loc,
                mats.len() as i32,
                gl::FALSE,
                mats.as_ptr() as *const f32,
            );
        }
    }

    pub fn set_mat3fs<T>(&mut self, name: &str, mats: &[T]) {
        unsafe {
            let loc = self.get_location(name);
            gl::UniformMatrix3fv(
                loc,
                mats.len() as i32,
                gl::FALSE,
                mats.as_ptr() as *const f32,
            );
        }
    }

    fn add_defines(src: &str, defines: &[String]) -> String {
        let mut defines = defines.iter().map(|d| format!("\n#define {}", d)).collect::<Vec<_>>().join("\n");
        src.split("\n").map(|l| {
            if l.starts_with("#version") {
                let mut l = l.to_string();
                l.push_str(&defines);
                l
            } else {
                l.to_string()
            }
        }).collect::<Vec<String>>().join("\n")
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn from_source(src: &str, shader_type: GLuint) -> Result<Self, String> {
        unsafe {
            let src = CString::new(src).unwrap();
            let src = src.as_c_str();

            let id = gl::CreateShader(shader_type);
            gl::ShaderSource(id, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
            // ERROR HANDLING
            let mut it_worked: GLint = gl::FALSE as GLint;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut it_worked);
            if it_worked != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                let buf = String::from_utf8_unchecked(buf);
                return Err(format!("Shader could not be made, LOG: {:?}", buf));
            }
            // END ERROR HANDLING

            Ok(Self { id })
        }
    }

    pub fn from_path(path: impl AsRef<Path>, shader_type: GLuint) -> Result<Self, String> {
        // get path base needed for #include in right folder
        let path_split = path
            .as_ref()
            .to_str()
            .unwrap()
            .split('/')
            .collect::<Vec<_>>();
        let path_base = path_split[0..path_split.len() - 1]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("/");

        let mut src = std::fs::read_to_string(&path)
            .expect(format!("Could not find shader source at {:?}", path.as_ref()).as_str());

        src = Self::unroll_includes(&src, &path_base);
        let res = Self::from_source(src.as_str(), shader_type);
        if let Err(e) = res {
            return Err(format!("shader from path '{:?}': {}", path.as_ref(), e));
        } else {
            res
        }
    }

    /// Makes the #include path/to/file.glsl possible
    pub fn unroll_includes(src: &str, path_base: &str) -> String {
        src.split("\n")
            .map(|line| {
                if line.starts_with("#include") {
                    let path_end = line
                        .split(' ')
                        .nth(1)
                        .expect(&format!("Line: '{}' has an include syntax error", line));
                    let mut path = path_base.to_owned();
                    path.push('/');
                    path.push_str(path_end);
                    let path = std::path::Path::new(&path);
                    let src_ = std::fs::read_to_string(path).expect(&format!(
                        "Could not include file from '{}'",
                        path.to_str().unwrap()
                    ));
                    src_
                } else {
                    line.into()
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

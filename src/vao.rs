use crate::{
    buffer::{Buffer, BufferType, DrawType},
    DType, GlslDType,
};
use std::collections::HashMap;

/// VertexArray is an object that stores the index buffer
/// and attribute pointers. That belong to a single drawcall.
/// This way, you do not have to bind the index buffer &
/// vertex buffers (these are stored in the attribute pointers)
/// and set the attribute layouts. Everytime you make a drawcall.
///
/// Logic:
///     You bind the VertexArray and untill you unbind it, it
///     stores all attribute pointers set, and index buffers
///     bound. So make sure to not unbind index buffers before
///     unbinding the VertexArray.

pub struct VertexArray {
    vao: u32,
    vbos: HashMap<String, Buffer>,
    pub ibo: Buffer,
    drawtype: DrawType,
}

impl VertexArray {
    pub fn new(drawtype: DrawType) -> Self {
        let vbos = HashMap::new();
        let mut vao: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        };

        let ibo = Buffer::new(BufferType::Index, drawtype.clone());
        ibo.bind(); // the bound vao saves ibo whenever bind is called

        Self {
            vao,
            vbos,
            ibo,
            drawtype,
        }
    }
    pub fn new_static() -> Self {
        Self::new(DrawType::Static)
    }

    pub fn new_dynamic() -> Self {
        Self::new(DrawType::Dynamic)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn new_vertex_buffer_filled<T>(&mut self, name: &str, content: &[T])
    where
        T: HasVertexAttributes,
    {
        let vbo = Buffer::new(BufferType::Vertex, self.drawtype);
        vbo.bind();
        vbo.buffer(content);
        T::set_layouts();
        self.vbos.insert(name.into(), vbo);
    }

    pub fn new_vertex_buffer_empty<T>(&mut self, name: &str, n_vertices: usize)
    where
        T: HasVertexAttributes,
    {
        let vbo = Buffer::new(BufferType::Vertex, self.drawtype);
        vbo.bind();
        vbo.init(n_vertices * std::mem::size_of::<T>());
        T::set_layouts();
        self.vbos.insert(name.into(), vbo);
    }

    pub fn drop_vertex_buffer(&mut self, name: &str) {
        self.vbos.remove(name);
    }

    pub fn buffer_indices<T>(&self, indices: &[T]) {
        self.ibo.bind();
        self.ibo.buffer(indices);
    }

    pub fn subbuffer<V, I>(
        &mut self,
        vbo_name: &str,
        vertices: &[V],
        indices: &[I],
        offset: usize,
    ) {
        self.ibo.bind();
        self.ibo.subbuffer(indices, offset);
        let vbo = self
            .vbos
            .get(vbo_name)
            .expect("[ERROR] Do not know this vbo name");
        vbo.bind();
        vbo.subbuffer(vertices, offset);
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao as *const u32);
        }
    }
}

pub trait HasVertexAttributes {
    fn layouts() -> Vec<AttributeLayout> {
        let attrs = Self::attributes();
        let attr_infos = attrs
            .iter()
            .map(|a| (a.n_elements(), a.dtype()))
            .collect::<Vec<_>>();
        let sizes = attr_infos
            .iter()
            .map(|(n_el, ty)| ty.size() * n_el)
            .collect::<Vec<_>>();
        let stride = sizes.iter().sum();
        let offsets = sizes.iter().scan(0, |sum, size| {
            let res = Some(*sum);
            *sum += size;
            res
        });
        let layouts = offsets
            .enumerate()
            .map(|(i, offs)| {
                let (n_elements, type_) = attr_infos[i];
                AttributeLayout {
                    stride,
                    location: i,
                    n_elements,
                    byte_offset: offs,
                    type_,
                }
            })
            .collect::<Vec<_>>();
        return layouts;
    }
    fn attributes() -> Vec<GlslDType> {
        unimplemented!("You must either override 'layouts()' or implement 'attributes()'")
    }
    fn set_layouts() {
        let layouts = Self::layouts();
        for layout in layouts.iter() {
            layout.set();
        }
    }
}

#[derive(Debug)]
pub struct AttributeLayout {
    pub stride: usize,
    pub location: usize,
    pub n_elements: usize,
    pub byte_offset: usize,
    pub type_: DType,
}

impl AttributeLayout {
    pub fn set(&self) {
        let &Self {
            location,
            n_elements,
            stride,
            byte_offset,
            type_,
        } = self;
        unsafe {
            gl::EnableVertexAttribArray(location as u32);
            gl::VertexAttribPointer(
                location as u32,
                n_elements as i32,
                type_ as u32,
                gl::FALSE,
                stride as i32,
                byte_offset as *const gl::types::GLvoid,
            );
        }
    }
}

impl HasVertexAttributes for [f32; 3] {
    fn attributes() -> Vec<GlslDType> {
        vec![GlslDType::Vec3]
    }
}

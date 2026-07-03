// FILE: graphic3d_buffer.rs
// occt: Graphic3d_Buffer, Graphic3d_IndexBuffer, Graphic3d_BoundBuffer

/// Data type tag for buffer element fields.
/// occt: Graphic3d_TypeOfData
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Graphic3dTypeOfData {
    Float,
    Double,
    Uint8,
    Int32,
    Vec2,
    Vec3,
    Vec4,
    Vec2d,
    Vec3d,
    Vec4d,
}

impl Graphic3dTypeOfData {
    pub fn bytes(&self) -> usize {
        match self {
            Self::Float | Self::Int32 => 4,
            Self::Double => 8,
            Self::Uint8 => 1,
            Self::Vec2 => 8,
            Self::Vec3 => 12,
            Self::Vec4 => 16,
            Self::Vec2d => 16,
            Self::Vec3d => 24,
            Self::Vec4d => 32,
        }
    }
}

/// Attribute definition for a vertex buffer layout.
/// occt: Graphic3d_Attribute
#[derive(Clone, Debug)]
pub struct Graphic3dAttribute {
    pub name: String,
    pub data_type: Graphic3dTypeOfData,
    pub offset: usize,
}

impl Graphic3dAttribute {
    pub fn new(name: &str, data_type: Graphic3dTypeOfData, offset: usize) -> Self {
        Self { name: name.to_string(), data_type, offset }
    }
}

/// Generic vertex buffer (interleaved layout).
/// occt: Graphic3d_Buffer
#[derive(Clone, Debug)]
pub struct Graphic3dBuffer {
    pub nb_elements: usize,
    pub stride: usize,
    pub data: Vec<u8>,
    pub attributes: Vec<Graphic3dAttribute>,
}

impl Default for Graphic3dBuffer {
    fn default() -> Self {
        Self { nb_elements: 0, stride: 0, data: Vec::new(), attributes: Vec::new() }
    }
}

impl Graphic3dBuffer {
    pub fn new() -> Self { Self::default() }

    /// Allocate buffer for `nb` elements with given stride (bytes per element).
    pub fn init(&mut self, nb: usize, stride: usize) {
        self.nb_elements = nb;
        self.stride = stride;
        self.data = vec![0u8; nb * stride];
    }

    pub fn add_attribute(&mut self, attr: Graphic3dAttribute) {
        self.attributes.push(attr);
    }

    pub fn is_empty(&self) -> bool { self.nb_elements == 0 }
    pub fn size_bytes(&self) -> usize { self.data.len() }
    pub fn nb_attributes(&self) -> usize { self.attributes.len() }

    /// Write f32 value at element `i`, attribute offset.
    pub fn set_f32(&mut self, i: usize, offset: usize, v: f32) {
        let pos = i * self.stride + offset;
        if pos + 4 <= self.data.len() {
            let bytes = v.to_le_bytes();
            self.data[pos..pos+4].copy_from_slice(&bytes);
        }
    }

    pub fn get_f32(&self, i: usize, offset: usize) -> Option<f32> {
        let pos = i * self.stride + offset;
        if pos + 4 <= self.data.len() {
            let bytes: [u8; 4] = self.data[pos..pos+4].try_into().ok()?;
            Some(f32::from_le_bytes(bytes))
        } else { None }
    }
}

/// Triangle index buffer.
/// occt: Graphic3d_IndexBuffer
#[derive(Clone, Debug, Default)]
pub struct Graphic3dIndexBuffer {
    pub indices: Vec<u32>,
    pub nb_elements: usize,
}

impl Graphic3dIndexBuffer {
    pub fn new() -> Self { Self::default() }

    pub fn init_triangle_strip(&mut self, nb_vertices: usize) {
        if nb_vertices < 3 { return; }
        self.indices.clear();
        for i in 0..nb_vertices - 2 {
            if i % 2 == 0 {
                self.indices.extend_from_slice(&[i as u32, (i+1) as u32, (i+2) as u32]);
            } else {
                self.indices.extend_from_slice(&[i as u32, (i+2) as u32, (i+1) as u32]);
            }
        }
        self.nb_elements = self.indices.len();
    }

    pub fn init_quads(&mut self, nb_quads: usize) {
        self.indices.clear();
        for i in 0..nb_quads {
            let base = (i * 4) as u32;
            self.indices.extend_from_slice(&[base, base+1, base+2, base, base+2, base+3]);
        }
        self.nb_elements = self.indices.len();
    }

    pub fn is_empty(&self) -> bool { self.indices.is_empty() }
    pub fn nb_elements(&self) -> usize { self.nb_elements }
    pub fn nb_triangles(&self) -> usize { self.nb_elements / 3 }
    pub fn value(&self, i: usize) -> Option<u32> { self.indices.get(i).copied() }
}

/// Primitive bounds buffer (for indexed primitive sets).
/// occt: Graphic3d_BoundBuffer
#[derive(Clone, Debug, Default)]
pub struct Graphic3dBoundBuffer {
    pub bounds: Vec<i32>,    // number of elements per primitive
    pub colors: Vec<[f32; 4]>,
    pub has_colors: bool,
}

impl Graphic3dBoundBuffer {
    pub fn new() -> Self { Self::default() }

    pub fn init(&mut self, nb_bounds: usize, has_colors: bool) {
        self.bounds = vec![0; nb_bounds];
        self.has_colors = has_colors;
        if has_colors { self.colors = vec![[1.0, 1.0, 1.0, 1.0]; nb_bounds]; }
    }

    pub fn set_bound(&mut self, i: usize, nb: i32) {
        if i < self.bounds.len() { self.bounds[i] = nb; }
    }

    pub fn set_color(&mut self, i: usize, c: [f32; 4]) {
        if self.has_colors && i < self.colors.len() { self.colors[i] = c; }
    }

    pub fn nb_bounds(&self) -> usize { self.bounds.len() }
    pub fn bound(&self, i: usize) -> Option<i32> { self.bounds.get(i).copied() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_of_data_bytes() {
        assert_eq!(Graphic3dTypeOfData::Float.bytes(), 4);
        assert_eq!(Graphic3dTypeOfData::Vec3.bytes(), 12);
        assert_eq!(Graphic3dTypeOfData::Vec4d.bytes(), 32);
    }

    #[test]
    fn buffer_init_and_access() {
        let mut b = Graphic3dBuffer::new();
        b.init(4, 12);  // 4 vec3 vertices
        b.set_f32(0, 0, 1.0);
        b.set_f32(0, 4, 2.0);
        assert!((b.get_f32(0, 0).unwrap() - 1.0).abs() < 1e-6);
        assert!((b.get_f32(0, 4).unwrap() - 2.0).abs() < 1e-6);
        assert_eq!(b.size_bytes(), 48);
    }

    #[test]
    fn index_buffer_quads() {
        let mut ib = Graphic3dIndexBuffer::new();
        ib.init_quads(2);
        assert_eq!(ib.nb_elements(), 12);
        assert_eq!(ib.nb_triangles(), 4);
        // First quad: 0,1,2 and 0,2,3
        assert_eq!(ib.value(0), Some(0));
        assert_eq!(ib.value(2), Some(2));
    }

    #[test]
    fn index_buffer_strip() {
        let mut ib = Graphic3dIndexBuffer::new();
        ib.init_triangle_strip(5);
        assert_eq!(ib.nb_triangles(), 3);
    }

    #[test]
    fn bound_buffer_set_get() {
        let mut bb = Graphic3dBoundBuffer::new();
        bb.init(3, true);
        bb.set_bound(0, 4);
        bb.set_color(0, [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(bb.bound(0), Some(4));
        assert_eq!(bb.nb_bounds(), 3);
    }
}

// FILE: select3_d_sensitive_primitive_array.rs
// occt: Select3D_SensitivePrimitiveArray

#[derive(Clone, Debug, Default)]
pub struct SensitivePrimitiveArray {
    vertices: Vec<[f32; 3]>,
    indices: Vec<u32>,
    primitive_type: u32,
}

impl SensitivePrimitiveArray {
    pub fn new() -> Self { Self::default() }
    pub fn vertices(&self) -> &[[f32; 3]] { &self.vertices }
    pub fn indices(&self) -> &[u32] { &self.indices }
    pub fn primitive_type(&self) -> u32 { self.primitive_type }
    pub fn add_vertex(&mut self, v: [f32; 3]) { self.vertices.push(v); }
    pub fn add_index(&mut self, i: u32) { self.indices.push(i); }
    pub fn set_primitive_type(&mut self, t: u32) { self.primitive_type = t; }
    pub fn clear(&mut self) { self.vertices.clear(); self.indices.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = SensitivePrimitiveArray::new();
        assert!(a.vertices().is_empty());
    }
}

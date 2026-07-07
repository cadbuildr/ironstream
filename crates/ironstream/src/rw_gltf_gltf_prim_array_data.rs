// FILE: rw_gltf_gltf_prim_array_data.rs
// occt: RWGltf_GltfPrimArrayData

//! Primitive array data for glTF.

#[derive(Debug, Clone)]
pub struct PrimArrayData {
    data: Vec<f32>,
    stride: usize,
}

impl PrimArrayData {
    pub fn new() -> Self {
        Self { data: Vec::new(), stride: 0 }
    }

    pub fn append(&mut self, value: f32) {
        self.data.push(value);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn set_stride(&mut self, stride: usize) {
        self.stride = stride;
    }

    pub fn get_data(&self) -> &[f32] {
        &self.data
    }
}

impl Default for PrimArrayData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut data = PrimArrayData::new();
        data.append(1.0);
        assert_eq!(data.len(), 1);
    }
}

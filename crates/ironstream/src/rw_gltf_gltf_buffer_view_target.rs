// FILE: rw_gltf_gltf_buffer_view_target.rs
// occt: RWGltf_GltfBufferViewTarget

//! Buffer view target enumeration for glTF.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferViewTarget {
    ArrayBuffer = 34962,
    ElementArrayBuffer = 34963,
}

impl BufferViewTarget {
    pub fn value(&self) -> u32 {
        *self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(BufferViewTarget::ArrayBuffer.value(), 34962);
    }
}

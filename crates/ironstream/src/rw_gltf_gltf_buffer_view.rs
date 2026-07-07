// FILE: rw_gltf_gltf_buffer_view.rs
// occt: RWGltf_GltfBufferView

//! Buffer view descriptor for glTF.

#[derive(Debug, Clone)]
pub struct BufferView {
    buffer_id: u32,
    byte_offset: usize,
    byte_length: usize,
    byte_stride: usize,
}

impl BufferView {
    pub fn new(buffer_id: u32, offset: usize, length: usize) -> Self {
        Self {
            buffer_id,
            byte_offset: offset,
            byte_length: length,
            byte_stride: 0,
        }
    }

    pub fn buffer_id(&self) -> u32 {
        self.buffer_id
    }

    pub fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    pub fn byte_length(&self) -> usize {
        self.byte_length
    }

    pub fn byte_stride(&self) -> usize {
        self.byte_stride
    }

    pub fn set_byte_stride(&mut self, stride: usize) {
        self.byte_stride = stride;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let view = BufferView::new(0, 0, 100);
        assert_eq!(view.buffer_id(), 0);
        assert_eq!(view.byte_length(), 100);
    }
}

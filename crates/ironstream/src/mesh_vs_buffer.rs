// FILE: mesh_vs_buffer.rs
// occt: MeshVS_Buffer

//! Buffer for mesh data storage.

#[derive(Clone, Debug)]
pub struct Buffer {
    pub buffer_id: u32,
    pub data: Vec<u8>,
    pub element_size: usize,
}

impl Buffer {
    pub fn new(buffer_id: u32, element_size: usize) -> Self {
        Buffer {
            buffer_id,
            data: Vec::new(),
            element_size,
        }
    }

    pub fn with_capacity(buffer_id: u32, element_size: usize, capacity: usize) -> Self {
        Buffer {
            buffer_id,
            data: Vec::with_capacity(capacity * element_size),
            element_size,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() / self.element_size
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_creation() {
        let buf = Buffer::new(1, 4);
        assert_eq!(buf.buffer_id, 1);
        assert_eq!(buf.element_size, 4);
        assert!(buf.is_empty());
    }

    #[test]
    fn buffer_capacity() {
        let buf = Buffer::with_capacity(1, 4, 100);
        assert_eq!(buf.element_size, 4);
        assert!(buf.is_empty());
    }

    #[test]
    fn buffer_operations() {
        let mut buf = Buffer::new(1, 4);
        let data = [1u8, 2, 3, 4];
        buf.push_bytes(&data);
        assert_eq!(buf.len(), 1);
        assert!(!buf.is_empty());
        buf.clear();
        assert!(buf.is_empty());
    }
}

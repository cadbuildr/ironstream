// FILE: n_collection_buffer.rs
// occt: NCollection_Buffer

/// Buffer for data storage.
pub struct NCollectionBuffer {
    data: Vec<u8>,
}

impl NCollectionBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn clear(&mut self) {
        for byte in &mut self.data {
            *byte = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buf = NCollectionBuffer::new(10);
        assert_eq!(buf.size(), 10);
    }

    #[test]
    fn test_clear() {
        let mut buf = NCollectionBuffer::new(5);
        buf.data_mut()[0] = 42;
        buf.clear();
        assert_eq!(buf.data()[0], 0);
    }
}

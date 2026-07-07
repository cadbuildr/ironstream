// FILE: rw_gltf_gltf_o_stream_writer.rs
// occt: RWGltf_GltfOStreamWriter

//! Output stream writer for glTF.

#[derive(Debug)]
pub struct OStreamWriter {
    buffer: Vec<u8>,
}

impl OStreamWriter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn write_bytes(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn write_string(&mut self, s: &str) {
        self.buffer.extend_from_slice(s.as_bytes());
    }

    pub fn get_data(&self) -> &[u8] {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for OStreamWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut writer = OStreamWriter::new();
        writer.write_string("test");
        assert_eq!(writer.len(), 4);
    }
}

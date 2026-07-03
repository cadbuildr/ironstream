// FILE: bin_tools_o_stream.rs
// occt: BinTools_OStream

use std::io::Write;

/// Output stream for writing binary shape data.
pub struct BinToolsOStream {
    data: Vec<u8>,
}

impl BinToolsOStream {
    /// Create a new output stream.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Write bytes to the stream.
    pub fn write(&mut self, buffer: &[u8]) {
        self.data.extend_from_slice(buffer);
    }

    /// Write a single byte.
    pub fn write_byte(&mut self, byte: u8) {
        self.data.push(byte);
    }

    /// Write a u32 in little-endian.
    pub fn write_u32(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        self.data.extend_from_slice(&bytes);
    }

    /// Get the data written so far.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get mutable data.
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    /// Get the size of data written.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Clear the stream.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Consume the stream and return the data.
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }
}

impl Default for BinToolsOStream {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stream = BinToolsOStream::new();
        assert_eq!(stream.size(), 0);
    }

    #[test]
    fn test_write_byte() {
        let mut stream = BinToolsOStream::new();
        stream.write_byte(1);
        stream.write_byte(2);

        assert_eq!(stream.size(), 2);
        assert_eq!(stream.data(), &[1, 2]);
    }

    #[test]
    fn test_write_u32() {
        let mut stream = BinToolsOStream::new();
        stream.write_u32(256);

        assert_eq!(stream.size(), 4);
        assert_eq!(stream.data(), &[0, 1, 0, 0]);
    }

    #[test]
    fn test_clear() {
        let mut stream = BinToolsOStream::new();
        stream.write_byte(1);
        stream.clear();

        assert_eq!(stream.size(), 0);
    }

    #[test]
    fn test_into_vec() {
        let mut stream = BinToolsOStream::new();
        stream.write_byte(1);
        stream.write_byte(2);

        let vec = stream.into_vec();
        assert_eq!(vec, vec![1, 2]);
    }
}

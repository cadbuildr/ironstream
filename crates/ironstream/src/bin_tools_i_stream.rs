// FILE: bin_tools_i_stream.rs
// occt: BinTools_IStream

use std::io::Read;

/// Input stream for reading binary shape data.
pub struct BinToolsIStream {
    data: Vec<u8>,
    position: usize,
}

impl BinToolsIStream {
    /// Create a new input stream from data.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    /// Read bytes into a buffer.
    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        let remaining = self.data.len() - self.position;
        let to_read = buffer.len().min(remaining);

        if to_read > 0 {
            buffer[..to_read].copy_from_slice(&self.data[self.position..self.position + to_read]);
            self.position += to_read;
        }

        to_read
    }

    /// Read a single byte.
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.position < self.data.len() {
            let byte = self.data[self.position];
            self.position += 1;
            Some(byte)
        } else {
            None
        }
    }

    /// Read a u32 in little-endian.
    pub fn read_u32(&mut self) -> Option<u32> {
        if self.position + 4 <= self.data.len() {
            let bytes = &self.data[self.position..self.position + 4];
            self.position += 4;
            Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        } else {
            None
        }
    }

    /// Get current position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get remaining bytes.
    pub fn remaining(&self) -> usize {
        self.data.len() - self.position
    }

    /// Check if end of stream.
    pub fn is_eof(&self) -> bool {
        self.position >= self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stream = BinToolsIStream::new(vec![1, 2, 3, 4]);
        assert_eq!(stream.position(), 0);
        assert_eq!(stream.remaining(), 4);
        assert!(!stream.is_eof());
    }

    #[test]
    fn test_read_byte() {
        let mut stream = BinToolsIStream::new(vec![1, 2, 3]);
        assert_eq!(stream.read_byte(), Some(1));
        assert_eq!(stream.read_byte(), Some(2));
        assert_eq!(stream.position(), 2);
    }

    #[test]
    fn test_read_u32() {
        let data = vec![1, 0, 0, 0, 5, 6, 7];
        let mut stream = BinToolsIStream::new(data);
        assert_eq!(stream.read_u32(), Some(1));
        assert_eq!(stream.position(), 4);
    }

    #[test]
    fn test_eof() {
        let mut stream = BinToolsIStream::new(vec![1]);
        stream.read_byte();
        assert!(stream.is_eof());
    }
}

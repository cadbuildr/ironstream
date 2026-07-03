// FILE: standard_array_stream_buffer.rs
// occt: Standard_ArrayStreamBuffer

/// Stream buffer using a fixed-size array as underlying storage.
/// Implements a std::streambuf-like interface for in-memory stream operations.
pub struct StandardArrayStreamBuffer {
    buffer: Vec<u8>,
    pos: usize,
}

impl StandardArrayStreamBuffer {
    /// Constructs an empty array stream buffer.
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            pos: 0,
        }
    }

    /// Constructs from existing data.
    pub fn from_slice(data: &[u8]) -> Self {
        Self {
            buffer: data.to_vec(),
            pos: 0,
        }
    }

    /// Returns the size of the buffer.
    pub fn size(&self) -> usize {
        self.buffer.len()
    }

    /// Returns the current position in the buffer.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Sets the current position in the buffer.
    pub fn set_pos(&mut self, p: usize) {
        if p <= self.buffer.len() {
            self.pos = p;
        }
    }

    /// Reads a byte at the current position and advances.
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.pos < self.buffer.len() {
            let byte = self.buffer[self.pos];
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }

    /// Writes a byte at the current position and advances.
    pub fn write_byte(&mut self, byte: u8) {
        if self.pos < self.buffer.len() {
            self.buffer[self.pos] = byte;
            self.pos += 1;
        } else {
            self.buffer.push(byte);
            self.pos += 1;
        }
    }

    /// Reads a slice of bytes.
    pub fn read_slice(&mut self, len: usize) -> Vec<u8> {
        let end = std::cmp::min(self.pos + len, self.buffer.len());
        let slice = self.buffer[self.pos..end].to_vec();
        self.pos = end;
        slice
    }

    /// Writes a slice of bytes.
    pub fn write_slice(&mut self, data: &[u8]) {
        for &byte in data {
            self.write_byte(byte);
        }
    }

    /// Returns a reference to the entire buffer.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Returns a mutable reference to the entire buffer.
    pub fn buffer_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }

    /// Clears the buffer and resets position.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.pos = 0;
    }

    /// Returns whether the buffer is at end.
    pub fn is_eof(&self) -> bool {
        self.pos >= self.buffer.len()
    }
}

impl Default for StandardArrayStreamBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let buf = StandardArrayStreamBuffer::new();
        assert_eq!(buf.size(), 0);
        assert_eq!(buf.pos(), 0);
    }

    #[test]
    fn test_from_slice() {
        let data = b"hello";
        let buf = StandardArrayStreamBuffer::from_slice(data);
        assert_eq!(buf.size(), 5);
        assert_eq!(buf.buffer(), b"hello");
    }

    #[test]
    fn test_read_byte() {
        let data = b"hi";
        let mut buf = StandardArrayStreamBuffer::from_slice(data);
        assert_eq!(buf.read_byte(), Some(b'h'));
        assert_eq!(buf.pos(), 1);
        assert_eq!(buf.read_byte(), Some(b'i'));
        assert_eq!(buf.pos(), 2);
        assert_eq!(buf.read_byte(), None);
    }

    #[test]
    fn test_write_byte() {
        let mut buf = StandardArrayStreamBuffer::new();
        buf.write_byte(b'a');
        buf.write_byte(b'b');
        assert_eq!(buf.buffer(), b"ab");
        assert_eq!(buf.pos(), 2);
    }

    #[test]
    fn test_read_slice() {
        let data = b"hello world";
        let mut buf = StandardArrayStreamBuffer::from_slice(data);
        let slice = buf.read_slice(5);
        assert_eq!(slice, b"hello");
        assert_eq!(buf.pos(), 5);
    }

    #[test]
    fn test_write_slice() {
        let mut buf = StandardArrayStreamBuffer::new();
        buf.write_slice(b"test");
        assert_eq!(buf.buffer(), b"test");
    }

    #[test]
    fn test_clear() {
        let mut buf = StandardArrayStreamBuffer::from_slice(b"data");
        assert!(!buf.is_eof());
        buf.clear();
        assert_eq!(buf.size(), 0);
        assert!(buf.is_eof());
    }
}

// FILE: fsd_b_stream.rs
// occt: FSD_BStream

/// Binary stream for file storage.
#[derive(Debug, Clone)]
pub struct FsdBStream {
    buffer: Vec<u8>,
    position: usize,
}

impl FsdBStream {
    /// Constructs an empty binary stream.
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
        }
    }

    /// Constructs from existing data.
    pub fn from_vec(data: Vec<u8>) -> Self {
        Self {
            buffer: data,
            position: 0,
        }
    }

    /// Returns the size of the stream.
    pub fn size(&self) -> usize {
        self.buffer.len()
    }

    /// Returns the current position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Sets the current position.
    pub fn set_position(&mut self, pos: usize) {
        if pos <= self.buffer.len() {
            self.position = pos;
        }
    }

    /// Reads a byte.
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.position < self.buffer.len() {
            let byte = self.buffer[self.position];
            self.position += 1;
            Some(byte)
        } else {
            None
        }
    }

    /// Writes a byte.
    pub fn write_byte(&mut self, byte: u8) {
        if self.position < self.buffer.len() {
            self.buffer[self.position] = byte;
        } else {
            self.buffer.push(byte);
        }
        self.position += 1;
    }

    /// Reads bytes.
    pub fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let end = std::cmp::min(self.position + len, self.buffer.len());
        let slice = self.buffer[self.position..end].to_vec();
        self.position = end;
        slice
    }

    /// Writes bytes.
    pub fn write_bytes(&mut self, data: &[u8]) {
        for &byte in data {
            self.write_byte(byte);
        }
    }

    /// Returns the buffer.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Clears the stream.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.position = 0;
    }
}

impl Default for FsdBStream {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stream = FsdBStream::new();
        assert_eq!(stream.size(), 0);
        assert_eq!(stream.position(), 0);
    }

    #[test]
    fn test_write_read() {
        let mut stream = FsdBStream::new();
        stream.write_byte(42);
        stream.write_byte(100);

        stream.set_position(0);
        assert_eq!(stream.read_byte(), Some(42));
        assert_eq!(stream.read_byte(), Some(100));
    }

    #[test]
    fn test_write_bytes() {
        let mut stream = FsdBStream::new();
        stream.write_bytes(&[1, 2, 3, 4]);
        assert_eq!(stream.size(), 4);

        stream.set_position(0);
        let data = stream.read_bytes(2);
        assert_eq!(data, vec![1, 2]);
    }
}

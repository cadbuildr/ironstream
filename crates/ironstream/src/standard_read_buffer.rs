// FILE: standard_read_buffer.rs
// occt: Standard_ReadBuffer

//! Auxiliary tool for buffered reading from input stream within chunks of constant size.

use std::io::Read;

/// Buffered reader for processing chunks of fixed size from input stream.
pub struct StandardReadBuffer {
    buffer: [u8; 4096],
    buffer_ptr: usize,
    buffer_end: usize,
    data_len: i64,
    data_read: i64,
    chunk_len: usize,
    nb_chunks: usize,
    buffer_len: usize,
}

impl StandardReadBuffer {
    /// Constructor with initialization.
    ///
    /// # Arguments
    ///
    /// * `data_len` - the full length of input data to read from stream
    /// * `chunk_len` - the length of single chunk to read
    /// * `is_partial_payload` - if false, data_len will be aligned to multiple of chunk_len;
    ///   if true, last chunk will be read exactly until data_len
    pub fn new(data_len: i64, chunk_len: usize, is_partial_payload: bool) -> Self {
        let mut reader = StandardReadBuffer {
            buffer: [0u8; 4096],
            buffer_ptr: 4096,
            buffer_end: 4096,
            data_len: 0,
            data_read: 0,
            chunk_len,
            nb_chunks: 0,
            buffer_len: 0,
        };
        reader.init(data_len, chunk_len, is_partial_payload);
        reader
    }

    /// Initialize the buffer.
    pub fn init(&mut self, data_len: i64, chunk_len: usize, is_partial_payload: bool) {
        self.data_read = 0;
        self.chunk_len = chunk_len;

        if is_partial_payload {
            self.data_len = data_len;
        } else {
            self.data_len = data_len - (data_len % chunk_len as i64);
        }

        self.nb_chunks = self.buffer.len() / chunk_len;
        self.buffer_len = chunk_len * self.nb_chunks;
        self.buffer_end = self.buffer.len();
        self.buffer_ptr = self.buffer.len();

        for i in 0..self.buffer.len() {
            self.buffer[i] = 0;
        }

        if chunk_len > self.buffer.len() {
            panic!("Chunk size is greater than preallocated buffer");
        }
    }

    /// Return true if amount of read bytes equals requested length of entire data.
    pub fn is_done(&self) -> bool {
        self.data_read == self.data_len
    }

    /// Read next chunk from stream.
    /// Returns pointer to the chunk or None on error/end of reading buffer.
    pub fn read_data_chunk<R: Read>(&mut self, stream: &mut R) -> Option<&[u8]> {
        self.buffer_ptr += self.chunk_len;

        if self.buffer_ptr < self.buffer_end {
            return Some(&self.buffer[self.buffer_ptr..self.buffer_ptr + self.chunk_len]);
        }

        let data_left = self.data_len - self.data_read;
        if data_left <= 0 {
            return None;
        }

        let data_to_read = if self.buffer_len as i64 > data_left {
            data_left as usize
        } else {
            self.buffer_len
        };

        match stream.read(&mut self.buffer[..data_to_read]) {
            Ok(n) if n == data_to_read => {
                self.buffer_ptr = 0;
                self.buffer_end = data_to_read;
                self.data_read += data_to_read as i64;
                Some(&self.buffer[0..self.chunk_len.min(data_to_read)])
            }
            _ => None,
        }
    }

    /// Get the data length.
    pub fn data_len(&self) -> i64 {
        self.data_len
    }

    /// Get the amount of data read.
    pub fn data_read(&self) -> i64 {
        self.data_read
    }

    /// Get the chunk length.
    pub fn chunk_len(&self) -> usize {
        self.chunk_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_buffer_creation() {
        let reader = StandardReadBuffer::new(1024, 256, false);
        assert_eq!(reader.chunk_len(), 256);
        assert!(!reader.is_done());
    }

    #[test]
    fn test_read_buffer_partial_payload() {
        let reader = StandardReadBuffer::new(1000, 256, true);
        assert_eq!(reader.data_len(), 1000);
    }

    #[test]
    fn test_read_buffer_aligned_payload() {
        let reader = StandardReadBuffer::new(1000, 256, false);
        assert_eq!(reader.data_len(), 768); // Aligned to 256-byte chunks
    }

    #[test]
    fn test_read_buffer_zero_len() {
        let reader = StandardReadBuffer::new(0, 256, false);
        assert!(reader.is_done());
    }

    #[test]
    fn test_read_data_chunk() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut cursor = Cursor::new(data);
        let mut reader = StandardReadBuffer::new(8, 4, true);

        let chunk = reader.read_data_chunk(&mut cursor);
        assert!(chunk.is_some());
    }

    #[test]
    fn test_read_buffer_tracking() {
        let reader = StandardReadBuffer::new(1024, 256, false);
        assert_eq!(reader.data_read(), 0);
        assert_eq!(reader.data_len(), 1024);
    }

    #[test]
    #[should_panic]
    fn test_read_buffer_chunk_too_large() {
        let _reader = StandardReadBuffer::new(1024, 10000, false);
    }
}

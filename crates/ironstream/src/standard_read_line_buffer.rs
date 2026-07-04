// FILE: standard_read_line_buffer.rs
// occt: Standard_ReadLineBuffer

use std::io::{BufRead, BufReader};

/// Line reading buffer.
pub struct ReadLineBuffer {
    buffer: String,
}

impl ReadLineBuffer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn read_line<R: BufRead>(&mut self, reader: &mut R) -> std::io::Result<usize> {
        self.buffer.clear();
        reader.read_line(&mut self.buffer)
    }

    pub fn buffer(&self) -> &str {
        &self.buffer
    }
}

impl Default for ReadLineBuffer {
    fn default() -> Self {
        Self::new()
    }
}

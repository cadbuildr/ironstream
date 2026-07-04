// FILE: osd_stream_buffer.rs
// occt: OSD_StreamBuffer

use std::io::{BufReader, BufWriter, Write};

/// Stream buffer wrapper.
pub struct StreamBuffer {
    buffer_size: usize,
}

impl StreamBuffer {
    pub fn new(buffer_size: usize) -> Self {
        Self { buffer_size }
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

impl Default for StreamBuffer {
    fn default() -> Self {
        Self::new(8192)
    }
}

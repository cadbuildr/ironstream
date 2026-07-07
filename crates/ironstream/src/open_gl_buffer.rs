// FILE: open_gl_buffer.rs
// occt: OpenGl_Buffer

pub struct OpenGlBuffer {
    buffer_id: u32,
    data: Vec<u8>,
}

impl OpenGlBuffer {
    pub fn new(buffer_id: u32) -> Self {
        OpenGlBuffer {
            buffer_id,
            data: Vec::new(),
        }
    }

    pub fn buffer_id(&self) -> u32 {
        self.buffer_id
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buf = OpenGlBuffer::new(1);
        assert_eq!(buf.buffer_id(), 1);
        assert_eq!(buf.size(), 0);
    }
}

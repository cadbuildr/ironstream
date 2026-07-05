// FILE: open_gl_index_buffer.rs
// occt: OpenGl_IndexBuffer

/// OpenGL index buffer for indexed primitive rendering.
#[derive(Debug, Clone)]
pub struct OpenGlIndexBuffer {
    count: u32,
    gl_handle: u32,
}

impl OpenGlIndexBuffer {
    /// Creates a new index buffer.
    pub fn new() -> Self {
        OpenGlIndexBuffer { count: 0, gl_handle: 0 }
    }

    /// Gets the index count.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Sets the index count.
    pub fn set_count(&mut self, count: u32) {
        self.count = count;
    }

    /// Gets the OpenGL handle.
    pub fn gl_handle(&self) -> u32 {
        self.gl_handle
    }

    /// Binds the buffer.
    pub fn bind(&self) -> bool {
        self.gl_handle > 0
    }
}

impl Default for OpenGlIndexBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_buffer_creation() {
        let buffer = OpenGlIndexBuffer::new();
        assert_eq!(buffer.count(), 0);
    }

    #[test]
    fn test_index_buffer_count() {
        let mut buffer = OpenGlIndexBuffer::new();
        buffer.set_count(42);
        assert_eq!(buffer.count(), 42);
    }
}

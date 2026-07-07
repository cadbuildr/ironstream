// FILE: open_gl_uniform_buffer.rs
// occt: OpenGl_UniformBuffer

//! Uniform buffer object.
//! Inherits from OpenGl_Buffer and specializes for uniform buffer binding.

const GL_UNIFORM_BUFFER: u32 = 0x8A11;

/// Simplified OpenGL Buffer base class
#[derive(Debug)]
pub struct GlBuffer {
    buffer_id: u32,
    target: u32,
    size: usize,
}

impl GlBuffer {
    pub fn new() -> Self {
        Self {
            buffer_id: 0,
            target: GL_UNIFORM_BUFFER,
            size: 0,
        }
    }

    pub fn get_target(&self) -> u32 {
        self.target
    }

    pub fn buffer_id(&self) -> u32 {
        self.buffer_id
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}

impl Default for GlBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenGL Uniform Buffer Object
#[derive(Debug)]
pub struct OpenGlUniformBuffer {
    buffer: GlBuffer,
}

impl OpenGlUniformBuffer {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            buffer: GlBuffer {
                buffer_id: 0,
                target: GL_UNIFORM_BUFFER,
                size: 0,
            },
        }
    }

    /// Return buffer object target (GL_UNIFORM_BUFFER)
    pub fn get_target(&self) -> u32 {
        GL_UNIFORM_BUFFER
    }

    /// Get buffer ID
    pub fn buffer_id(&self) -> u32 {
        self.buffer.buffer_id
    }

    /// Get buffer size
    pub fn size(&self) -> usize {
        self.buffer.size
    }

    /// Bind buffer to base index
    pub fn bind_buffer_base(&self, index: u32) {
        // In real implementation: glBindBufferBase(GL_UNIFORM_BUFFER, index, buffer_id)
    }

    /// Bind buffer range to base index
    pub fn bind_buffer_range(&self, index: u32, offset: usize, size: usize) {
        // In real implementation: glBindBufferRange(GL_UNIFORM_BUFFER, index, buffer_id, offset, size)
    }

    /// Unbind buffer from base index
    pub fn unbind_buffer_base(&self, index: u32) {
        // In real implementation: glBindBufferBase(GL_UNIFORM_BUFFER, index, 0)
    }

    /// Create buffer with initial data
    pub fn create(&mut self, data: &[u8]) -> bool {
        self.buffer.buffer_id = 1; // Simplified: would be actual GL id
        self.buffer.size = data.len();
        true
    }

    /// Update buffer data
    pub fn update(&mut self, data: &[u8]) -> bool {
        if data.len() > self.buffer.size {
            return false;
        }
        true
    }

    /// Release buffer
    pub fn release(&mut self) {
        self.buffer.buffer_id = 0;
        self.buffer.size = 0;
    }

    /// Check if buffer is valid
    pub fn is_valid(&self) -> bool {
        self.buffer.buffer_id != 0
    }
}

impl Default for OpenGlUniformBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ubo = OpenGlUniformBuffer::new();
        assert_eq!(ubo.get_target(), GL_UNIFORM_BUFFER);
        assert!(!ubo.is_valid());
    }

    #[test]
    fn test_get_target() {
        let ubo = OpenGlUniformBuffer::new();
        assert_eq!(ubo.get_target(), 0x8A11);
    }

    #[test]
    fn test_buffer_create() {
        let mut ubo = OpenGlUniformBuffer::new();
        let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];

        assert!(ubo.create(&data));
        assert!(ubo.is_valid());
        assert_eq!(ubo.size(), 8);
        assert!(ubo.buffer_id() != 0);
    }

    #[test]
    fn test_buffer_update_valid() {
        let mut ubo = OpenGlUniformBuffer::new();
        let data = vec![1u8; 16];

        ubo.create(&data);
        let new_data = vec![2u8; 8];

        assert!(ubo.update(&new_data));
    }

    #[test]
    fn test_buffer_update_overflow() {
        let mut ubo = OpenGlUniformBuffer::new();
        let data = vec![1u8; 8];

        ubo.create(&data);
        let new_data = vec![2u8; 16]; // Larger than buffer

        assert!(!ubo.update(&new_data));
    }

    #[test]
    fn test_buffer_release() {
        let mut ubo = OpenGlUniformBuffer::new();
        let data = vec![1u8; 8];

        ubo.create(&data);
        assert!(ubo.is_valid());

        ubo.release();
        assert!(!ubo.is_valid());
        assert_eq!(ubo.size(), 0);
    }

    #[test]
    fn test_bind_operations() {
        let mut ubo = OpenGlUniformBuffer::new();
        let data = vec![1u8; 16];
        ubo.create(&data);

        // These shouldn't panic
        ubo.bind_buffer_base(0);
        ubo.bind_buffer_range(1, 0, 8);
        ubo.unbind_buffer_base(0);
    }

    #[test]
    fn test_default() {
        let ubo = OpenGlUniformBuffer::default();
        assert_eq!(ubo.get_target(), GL_UNIFORM_BUFFER);
        assert!(!ubo.is_valid());
    }
}

// FILE: open_gl_vertex_buffer.rs
// occt: OpenGl_VertexBuffer

//! Vertex Buffer Object - general storage object for vertex attributes
//! (position, normal, color). Use OpenGl_IndexBuffer for indices.

const GL_ARRAY_BUFFER: u32 = 0x8892;

/// Attribute mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeMode {
    VertexArray = 0,
    NormalArray = 1,
    ColorArray = 2,
    IndexArray = 3,
    TextureCoordArray = 4,
}

/// Simplified OpenGL Buffer base class
#[derive(Debug)]
pub struct GlBuffer {
    buffer_id: u32,
    target: u32,
    size: usize,
    components_nb: u32,
    data_type: u32,
    offset: usize,
}

impl GlBuffer {
    pub fn new() -> Self {
        Self {
            buffer_id: 0,
            target: GL_ARRAY_BUFFER,
            size: 0,
            components_nb: 0,
            data_type: 0x1406, // GL_FLOAT
            offset: 0,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.buffer_id != 0
    }

    pub fn get_target(&self) -> u32 {
        GL_ARRAY_BUFFER
    }

    pub fn components_nb(&self) -> u32 {
        self.components_nb
    }

    pub fn data_type(&self) -> u32 {
        self.data_type
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl Default for GlBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenGL Vertex Buffer Object
#[derive(Debug)]
pub struct OpenGlVertexBuffer {
    buffer: GlBuffer,
}

impl OpenGlVertexBuffer {
    /// Create uninitialized VBO
    pub fn new() -> Self {
        Self {
            buffer: GlBuffer::new(),
        }
    }

    /// Return buffer target GL_ARRAY_BUFFER
    pub fn get_target(&self) -> u32 {
        GL_ARRAY_BUFFER
    }

    /// Bind this VBO to active GLSL program
    pub fn bind_vertex_attrib(&self, attrib_loc: u32) {
        // In real implementation: glBindAttribLocation or glVertexAttribPointer
    }

    /// Unbind any VBO from active GLSL program
    pub fn unbind_vertex_attrib(&self, attrib_loc: u32) {
        // In real implementation: glDisableVertexAttribArray
    }

    /// Bind VBO and enable specified attribute
    pub fn bind_attribute(&self, mode: AttributeMode) {
        if self.buffer.is_valid() {
            // In real implementation: bind and enable attribute
        }
    }

    /// Unbind VBO and disable specified attribute
    pub fn unbind_attribute(&self, mode: AttributeMode) {
        if self.buffer.is_valid() {
            // In real implementation: unbind and disable attribute
        }
    }

    /// Setup array pointer for GLSL program or FFP
    pub fn bind_array_attribute(
        mode: AttributeMode,
        nb_comp: u32,
        data_type: u32,
        stride: i32,
        offset: usize,
    ) {
        // In real implementation: glVertexAttribPointer or glArrayElement
    }

    /// Disable GLSL array pointer
    pub fn unbind_array_attribute(mode: AttributeMode) {
        // In real implementation: glDisableVertexAttribArray
    }

    /// Create buffer with data
    pub fn create(&mut self, data: &[u8], nb_comp: u32) -> bool {
        self.buffer.buffer_id = 1;
        self.buffer.size = data.len();
        self.buffer.components_nb = nb_comp;
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
        self.buffer.is_valid()
    }

    /// Get buffer size
    pub fn size(&self) -> usize {
        self.buffer.size
    }

    /// Get number of components
    pub fn components_nb(&self) -> u32 {
        self.buffer.components_nb
    }

    /// Bind buffer to context
    pub fn bind(&self) {
        // In real implementation: glBindBuffer(GL_ARRAY_BUFFER, buffer_id)
    }

    /// Unbind buffer
    pub fn unbind(&self) {
        // In real implementation: glBindBuffer(GL_ARRAY_BUFFER, 0)
    }
}

impl Default for OpenGlVertexBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let vbo = OpenGlVertexBuffer::new();
        assert_eq!(vbo.get_target(), GL_ARRAY_BUFFER);
        assert!(!vbo.is_valid());
    }

    #[test]
    fn test_buffer_create() {
        let mut vbo = OpenGlVertexBuffer::new();
        let data = vec![0.0f32; 12].iter().map(|&x| x as u8).collect::<Vec<_>>();

        assert!(vbo.create(&data, 3));
        assert!(vbo.is_valid());
        assert_eq!(vbo.size(), data.len());
        assert_eq!(vbo.components_nb(), 3);
    }

    #[test]
    fn test_buffer_update() {
        let mut vbo = OpenGlVertexBuffer::new();
        let data = vec![0u8; 16];

        vbo.create(&data, 4);
        let new_data = vec![1u8; 8];

        assert!(vbo.update(&new_data));
    }

    #[test]
    fn test_buffer_update_overflow() {
        let mut vbo = OpenGlVertexBuffer::new();
        let data = vec![0u8; 8];

        vbo.create(&data, 2);
        let new_data = vec![1u8; 16];

        assert!(!vbo.update(&new_data));
    }

    #[test]
    fn test_buffer_release() {
        let mut vbo = OpenGlVertexBuffer::new();
        let data = vec![0u8; 8];

        vbo.create(&data, 2);
        assert!(vbo.is_valid());

        vbo.release();
        assert!(!vbo.is_valid());
    }

    #[test]
    fn test_bind_unbind_attribute() {
        let mut vbo = OpenGlVertexBuffer::new();
        let data = vec![0u8; 12];
        vbo.create(&data, 3);

        vbo.bind_attribute(AttributeMode::VertexArray);
        vbo.unbind_attribute(AttributeMode::VertexArray);
        // These shouldn't panic
    }

    #[test]
    fn test_attribute_modes() {
        assert_eq!(AttributeMode::VertexArray as i32, 0);
        assert_eq!(AttributeMode::NormalArray as i32, 1);
        assert_eq!(AttributeMode::ColorArray as i32, 2);
    }

    #[test]
    fn test_default() {
        let vbo = OpenGlVertexBuffer::default();
        assert!(!vbo.is_valid());
        assert_eq!(vbo.get_target(), GL_ARRAY_BUFFER);
    }
}

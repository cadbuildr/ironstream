// FILE: open_gl_vertex_buffer_compat.rs
// occt: OpenGl_VertexBufferCompat

//! Deprecated typedef aliases for compatibility.
//! OpenGl_VertexBufferCompat is a type alias for OpenGl_BufferCompatT<OpenGl_VertexBuffer>.
//! OpenGl_IndexBufferCompat is a type alias for OpenGl_BufferCompatT<OpenGl_IndexBuffer>.

/// Simplified OpenGL Buffer
#[derive(Debug)]
pub struct GlBuffer {
    buffer_id: u32,
    target: u32,
}

impl GlBuffer {
    pub fn new() -> Self {
        Self {
            buffer_id: 0,
            target: 0,
        }
    }
}

impl Default for GlBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Buffer compatibility wrapper template
#[derive(Debug)]
pub struct BufferCompat<T> {
    buffer: T,
}

impl<T> BufferCompat<T> {
    pub fn new(buffer: T) -> Self {
        Self { buffer }
    }

    pub fn inner(&self) -> &T {
        &self.buffer
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.buffer
    }

    pub fn into_inner(self) -> T {
        self.buffer
    }
}

/// Vertex buffer for compatibility mode
#[derive(Debug)]
pub struct VertexBuffer {
    buffer_id: u32,
    size: usize,
}

impl VertexBuffer {
    pub fn new() -> Self {
        Self {
            buffer_id: 0,
            size: 0,
        }
    }

    pub fn buffer_id(&self) -> u32 {
        self.buffer_id
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_valid(&self) -> bool {
        self.buffer_id != 0
    }

    pub fn create(&mut self) {
        self.buffer_id = 1;
    }

    pub fn release(&mut self) {
        self.buffer_id = 0;
        self.size = 0;
    }
}

impl Default for VertexBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Index buffer for compatibility mode
#[derive(Debug)]
pub struct IndexBuffer {
    buffer_id: u32,
    size: usize,
}

impl IndexBuffer {
    pub fn new() -> Self {
        Self {
            buffer_id: 0,
            size: 0,
        }
    }

    pub fn buffer_id(&self) -> u32 {
        self.buffer_id
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_valid(&self) -> bool {
        self.buffer_id != 0
    }

    pub fn create(&mut self) {
        self.buffer_id = 1;
    }

    pub fn release(&mut self) {
        self.buffer_id = 0;
        self.size = 0;
    }
}

impl Default for IndexBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for vertex buffer compatibility wrapper
pub type VertexBufferCompat = BufferCompat<VertexBuffer>;

/// Type alias for index buffer compatibility wrapper
pub type IndexBufferCompat = BufferCompat<IndexBuffer>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_compat_wrapper() {
        let vb = VertexBuffer::new();
        let wrapped = BufferCompat::new(vb);

        assert!(!wrapped.inner().is_valid());
    }

    #[test]
    fn test_vertex_buffer_compat() {
        let mut vb = VertexBuffer::new();
        vb.create();

        let compat = VertexBufferCompat::new(vb);
        assert!(compat.inner().is_valid());
    }

    #[test]
    fn test_index_buffer_compat() {
        let mut ib = IndexBuffer::new();
        ib.create();

        let compat = IndexBufferCompat::new(ib);
        assert!(compat.inner().is_valid());
    }

    #[test]
    fn test_buffer_compat_mutable_access() {
        let vb = VertexBuffer::new();
        let mut compat = BufferCompat::new(vb);

        compat.inner_mut().create();
        assert!(compat.inner().is_valid());
    }

    #[test]
    fn test_buffer_compat_into_inner() {
        let vb = VertexBuffer::new();
        let compat = BufferCompat::new(vb);

        let recovered = compat.into_inner();
        assert!(!recovered.is_valid());
    }

    #[test]
    fn test_vertex_buffer_create_release() {
        let mut vb = VertexBuffer::new();
        assert!(!vb.is_valid());

        vb.create();
        assert!(vb.is_valid());
        assert!(vb.buffer_id() != 0);

        vb.release();
        assert!(!vb.is_valid());
        assert_eq!(vb.buffer_id(), 0);
    }

    #[test]
    fn test_index_buffer_create_release() {
        let mut ib = IndexBuffer::new();
        assert!(!ib.is_valid());

        ib.create();
        assert!(ib.is_valid());

        ib.release();
        assert!(!ib.is_valid());
    }

    #[test]
    fn test_default_buffers() {
        let vb = VertexBuffer::default();
        assert!(!vb.is_valid());

        let ib = IndexBuffer::default();
        assert!(!ib.is_valid());
    }
}

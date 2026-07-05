// FILE: open_gl_vertex_buffer_editor.rs
// occt: OpenGl_VertexBufferEditor

//! Auxiliary class to iteratively modify data of existing VBO.
//! Provides iteration interface with delayed CPU->GPU memory transfer.
//! Temporary buffer on CPU side can be initialized with lesser capacity than VBO
//! to allow re-usage of shared buffer with fixed size between VBOs.

/// Generic temporary buffer
#[derive(Debug)]
struct TmpBuffer {
    data: Vec<Vec3f>,
}

impl TmpBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            data: vec![Vec3f::new(0.0, 0.0, 0.0); capacity],
        }
    }

    fn value(&self, index: usize) -> Option<&Vec3f> {
        self.data.get(index)
    }

    fn change_value(&mut self, index: usize) -> Option<&mut Vec3f> {
        self.data.get_mut(index)
    }

    fn upper(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

/// 3D float vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length() -> usize {
        3
    }
}

impl Default for Vec3f {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// Simplified OpenGL Buffer
#[derive(Debug)]
pub struct GlBuffer {
    buffer_id: u32,
    components_nb: u32,
    valid: bool,
}

impl GlBuffer {
    pub fn new() -> Self {
        Self {
            buffer_id: 0,
            components_nb: 0,
            valid: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn components_nb(&self) -> u32 {
        self.components_nb
    }

    pub fn sub_data(&self, elem_from: usize, elems_nb: usize, data: &[Vec3f]) -> bool {
        // In real implementation: glBufferSubData
        true
    }
}

impl Default for GlBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Simplified OpenGL Context
#[derive(Debug, Clone)]
pub struct GlContext;

/// Vertex Buffer Editor for iterative VBO modification
#[derive(Debug)]
pub struct VertexBufferEditor {
    gl_ctx: Option<GlContext>,
    vbo: Option<GlBuffer>,
    elem_from: usize,
    elems_nb: usize,
    tmp_buffer: TmpBuffer,
}

impl VertexBufferEditor {
    /// Creates empty editor with default temp buffer
    pub fn new() -> Self {
        Self::with_capacity(2048)
    }

    /// Creates empty editor with specified temp buffer capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let cap = if capacity > 0 { capacity } else { 2048 };
        Self {
            gl_ctx: None,
            vbo: None,
            elem_from: 0,
            elems_nb: 0,
            tmp_buffer: TmpBuffer::new(cap),
        }
    }

    /// Initialize editor for specified buffer object
    pub fn init(&mut self, ctx: &GlContext, vbo: &GlBuffer) -> bool {
        if !vbo.is_valid() || vbo.components_nb() != Vec3f::length() as u32 {
            return false;
        }

        self.gl_ctx = Some(ctx.clone());
        self.vbo = Some(GlBuffer::new()); // Simplified: would be actual vbo
        self.elem_from = 0;
        self.elems_nb = 0;

        true
    }

    /// Modify current element in VBO
    pub fn value(&mut self) -> Option<&mut Vec3f> {
        if self.elems_nb <= self.tmp_buffer.upper() {
            self.tmp_buffer.change_value(self.elems_nb)
        } else {
            None
        }
    }

    /// Move to the next position in VBO
    pub fn next(&mut self) -> bool {
        self.elems_nb += 1;

        if self.elems_nb > self.tmp_buffer.upper() {
            self.flush()
        } else {
            true
        }
    }

    /// Push current data from local buffer to VBO
    pub fn flush(&mut self) -> bool {
        if self.elems_nb <= 0 {
            return true;
        }

        if let Some(vbo) = &self.vbo {
            let data_slice = &self.tmp_buffer.data[0..self.elems_nb];
            if !vbo.sub_data(self.elem_from, self.elems_nb, data_slice) {
                return false;
            }
        }

        self.elem_from += self.elems_nb;
        self.elems_nb = 0;

        true
    }

    /// Return assigned VBO
    pub fn get_vbo(&self) -> Option<&GlBuffer> {
        self.vbo.as_ref()
    }

    /// Return current element count
    pub fn element_count(&self) -> usize {
        self.elems_nb
    }

    /// Return element offset
    pub fn element_from(&self) -> usize {
        self.elem_from
    }

    /// Return temporary buffer capacity
    pub fn tmp_buffer_capacity(&self) -> usize {
        self.tmp_buffer.len()
    }
}

impl Default for VertexBufferEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let editor = VertexBufferEditor::new();
        assert_eq!(editor.element_count(), 0);
        assert_eq!(editor.element_from(), 0);
        assert_eq!(editor.tmp_buffer_capacity(), 2048);
    }

    #[test]
    fn test_create_with_capacity() {
        let editor = VertexBufferEditor::with_capacity(512);
        assert_eq!(editor.tmp_buffer_capacity(), 512);
    }

    #[test]
    fn test_init_invalid_buffer() {
        let mut editor = VertexBufferEditor::new();
        let ctx = GlContext;
        let invalid_vbo = GlBuffer::new();

        assert!(!editor.init(&ctx, &invalid_vbo));
    }

    #[test]
    fn test_vec3f() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.length(), 3);
    }

    #[test]
    fn test_value_modification() {
        let mut editor = VertexBufferEditor::new();

        if let Some(v) = editor.value() {
            v.x = 1.0;
            v.y = 2.0;
            v.z = 3.0;
        }

        assert_eq!(editor.element_count(), 0);
    }

    #[test]
    fn test_next_within_buffer() {
        let mut editor = VertexBufferEditor::with_capacity(10);

        for _ in 0..5 {
            if let Some(v) = editor.value() {
                v.x = 1.0;
            }
            assert!(editor.next());
        }

        assert_eq!(editor.element_count(), 5);
    }

    #[test]
    fn test_flush() {
        let mut editor = VertexBufferEditor::new();

        if let Some(v) = editor.value() {
            v.x = 1.0;
        }
        editor.elems_nb = 1;

        assert!(editor.flush());
        assert_eq!(editor.element_count(), 0);
        assert_eq!(editor.element_from(), 1);
    }

    #[test]
    fn test_temp_buffer_structure() {
        let buf = TmpBuffer::new(100);
        assert_eq!(buf.len(), 100);
        assert_eq!(buf.upper(), 99);
    }

    #[test]
    fn test_default() {
        let editor = VertexBufferEditor::default();
        assert_eq!(editor.element_count(), 0);
    }
}

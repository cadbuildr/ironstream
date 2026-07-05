// FILE: open_gl_primitive_array.rs
// occt: OpenGl_PrimitiveArray

/// Array of primitives for efficient batch rendering.
#[derive(Debug, Clone)]
pub struct OpenGlPrimitiveArray {
    vertex_count: u32,
}

impl OpenGlPrimitiveArray {
    pub fn new() -> Self {
        OpenGlPrimitiveArray { vertex_count: 0 }
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn set_vertex_count(&mut self, count: u32) {
        self.vertex_count = count;
    }
}

impl Default for OpenGlPrimitiveArray {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_array() {
        let mut arr = OpenGlPrimitiveArray::new();
        arr.set_vertex_count(100);
        assert_eq!(arr.vertex_count(), 100);
    }
}

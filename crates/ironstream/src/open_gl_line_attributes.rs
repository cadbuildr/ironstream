// FILE: open_gl_line_attributes.rs
// occt: OpenGl_LineAttributes

/// OpenGL line rendering attributes (width, style, etc).
#[derive(Debug, Clone)]
pub struct OpenGlLineAttributes {
    width: f32,
}

impl OpenGlLineAttributes {
    /// Creates line attributes.
    pub fn new() -> Self {
        OpenGlLineAttributes { width: 1.0 }
    }

    /// Sets line width.
    pub fn set_width(&mut self, w: f32) {
        self.width = w;
    }

    /// Gets line width.
    pub fn width(&self) -> f32 {
        self.width
    }
}

impl Default for OpenGlLineAttributes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_attributes() {
        let mut attr = OpenGlLineAttributes::new();
        attr.set_width(2.0);
        assert_eq!(attr.width(), 2.0);
    }
}

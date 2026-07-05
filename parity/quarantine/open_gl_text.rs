// FILE: open_gl_text.rs
// occt: OpenGl_Text

/// Rendered text in 3D space.
#[derive_DEBUG, Clone)]
pub struct OpenGlText {
    is_valid: bool,
}

impl OpenGlText {
    pub fn new() -> Self {
        OpenGlText { is_valid: true }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl Default for OpenGlText {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let t = OpenGlText::new();
        assert!(t.is_valid());
    }
}

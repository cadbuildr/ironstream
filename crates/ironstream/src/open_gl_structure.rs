// FILE: open_gl_structure.rs
// occt: OpenGl_Structure

/// OpenGL structure for scene hierarchies.
#[derive(Debug, Clone)]
pub struct OpenGlStructure {
    is_visible: bool,
}

impl OpenGlStructure {
    pub fn new() -> Self {
        OpenGlStructure { is_visible: true }
    }

    pub fn set_visible(&mut self, v: bool) {
        self.is_visible = v;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}

impl Default for OpenGlStructure {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let mut s = OpenGlStructure::new();
        assert!(s.is_visible());
        s.set_visible(false);
        assert!(!s.is_visible());
    }
}

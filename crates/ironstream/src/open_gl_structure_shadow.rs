// FILE: open_gl_structure_shadow.rs
// occt: OpenGl_StructureShadow

/// Shadow structure.
#[derive(Debug, Clone)]
pub struct OpenGlStructureShadow;

impl OpenGlStructureShadow {
    pub fn new() -> Self {
        OpenGlStructureShadow
    }
}

impl Default for OpenGlStructureShadow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure_shadow() {
        let _shadow = OpenGlStructureShadow::new();
    }
}

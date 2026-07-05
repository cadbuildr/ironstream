// FILE: open_gl_text_builder.rs
// occt: OpenGl_TextBuilder

/// Builds text geometry for rendering.
#[derive(Debug, Clone)]
pub struct OpenGlTextBuilder;

impl OpenGlTextBuilder {
    pub fn new() -> Self {
        OpenGlTextBuilder
    }
}

impl Default for OpenGlTextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_builder() {
        let _builder = OpenGlTextBuilder::new();
    }
}

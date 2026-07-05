// FILE: open_gl_set_of_shader_programs.rs
// occt: OpenGl_SetOfShaderPrograms

/// Collection of shader programs.
#[derive(Debug, Clone)]
pub struct OpenGlSetOfShaderPrograms;

impl OpenGlSetOfShaderPrograms {
    pub fn new() -> Self {
        OpenGlSetOfShaderPrograms
    }
}

impl Default for OpenGlSetOfShaderPrograms {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_programs() {
        let _set = OpenGlSetOfShaderPrograms::new();
    }
}

// FILE: open_gl_shader_states.rs
// occt: OpenGl_ShaderStates

/// Tracks shader state.
#[derive(Debug, Clone)]
pub struct OpenGlShaderStates {
    is_bound: bool,
}

impl OpenGlShaderStates {
    pub fn new() -> Self {
        OpenGlShaderStates { is_bound: false }
    }

    pub fn bind(&mut self) {
        self.is_bound = true;
    }

    pub fn is_bound(&self) -> bool {
        self.is_bound
    }
}

impl Default for OpenGlShaderStates {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_states() {
        let mut state = OpenGlShaderStates::new();
        assert!(!state.is_bound());
        state.bind();
        assert!(state.is_bound());
    }
}

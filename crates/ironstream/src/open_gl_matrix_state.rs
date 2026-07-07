// FILE: open_gl_matrix_state.rs
// occt: OpenGl_MatrixState

/// Tracks OpenGL matrix state.
#[derive(Debug, Clone)]
pub struct OpenGlMatrixState {
    is_identity: bool,
}

impl OpenGlMatrixState {
    pub fn new() -> Self {
        OpenGlMatrixState { is_identity: true }
    }

    pub fn is_identity(&self) -> bool {
        self.is_identity
    }

    pub fn set_identity(&mut self) {
        self.is_identity = true;
    }
}

impl Default for OpenGlMatrixState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_state() {
        let state = OpenGlMatrixState::new();
        assert!(state.is_identity());
    }
}

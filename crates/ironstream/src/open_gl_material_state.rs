// FILE: open_gl_material_state.rs
// occt: OpenGl_MaterialState

/// Tracks OpenGL material state changes.
#[derive(Debug, Clone)]
pub struct OpenGlMaterialState {
    is_set: bool,
}

impl OpenGlMaterialState {
    pub fn new() -> Self {
        OpenGlMaterialState { is_set: false }
    }

    pub fn set(&mut self) {
        self.is_set = true;
    }

    pub fn is_set(&self) -> bool {
        self.is_set
    }
}

impl Default for OpenGlMaterialState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_state() {
        let mut state = OpenGlMaterialState::new();
        assert!(!state.is_set());
        state.set();
        assert!(state.is_set());
    }
}

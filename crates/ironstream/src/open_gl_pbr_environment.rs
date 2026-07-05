// FILE: open_gl_pbr_environment.rs
// occt: OpenGl_PBREnvironment

/// PBR (Physically-Based Rendering) environment map.
#[derive(Debug, Clone)]
pub struct OpenGlPbrEnvironment {
    is_loaded: bool,
}

impl OpenGlPbrEnvironment {
    pub fn new() -> Self {
        OpenGlPbrEnvironment { is_loaded: false }
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    pub fn load(&mut self) {
        self.is_loaded = true;
    }
}

impl Default for OpenGlPbrEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbr_environment() {
        let mut env = OpenGlPbrEnvironment::new();
        assert!(!env.is_loaded());
        env.load();
        assert!(env.is_loaded());
    }
}

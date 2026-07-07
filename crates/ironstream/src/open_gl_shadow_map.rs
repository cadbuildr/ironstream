// FILE: open_gl_shadow_map.rs
// occt: OpenGl_ShadowMap

/// Shadow map for shadow rendering.
#[derive(Debug, Clone)]
pub struct OpenGlShadowMap {
    is_valid: bool,
}

impl OpenGlShadowMap {
    pub fn new() -> Self {
        OpenGlShadowMap { is_valid: false }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn validate(&mut self) {
        self.is_valid = true;
    }
}

impl Default for OpenGlShadowMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_map() {
        let mut map = OpenGlShadowMap::new();
        assert!(!map.is_valid());
        map.validate();
        assert!(map.is_valid());
    }
}

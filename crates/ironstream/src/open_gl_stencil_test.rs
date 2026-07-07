// FILE: open_gl_stencil_test.rs
// occt: OpenGl_StencilTest

/// Stencil test configuration.
#[derive(Debug, Clone)]
pub struct OpenGlStencilTest {
    enabled: bool,
}

impl OpenGlStencilTest {
    pub fn new() -> Self {
        OpenGlStencilTest { enabled: false }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for OpenGlStencilTest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stencil_test() {
        let mut test = OpenGlStencilTest::new();
        assert!(!test.is_enabled());
        test.enable();
        assert!(test.is_enabled());
    }
}

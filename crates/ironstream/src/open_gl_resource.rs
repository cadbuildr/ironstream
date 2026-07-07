// FILE: open_gl_resource.rs
// occt: OpenGl_Resource

/// Base OpenGL resource class.
#[derive(Debug, Clone)]
pub struct OpenGlResource {
    is_valid: bool,
}

impl OpenGlResource {
    pub fn new() -> Self {
        OpenGlResource { is_valid: true }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl Default for OpenGlResource {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource() {
        let res = OpenGlResource::new();
        assert!(res.is_valid());
    }
}

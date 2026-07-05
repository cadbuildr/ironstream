// FILE: open_gl_named_resource.rs
// occt: OpenGl_NamedResource

/// Named OpenGL resource with reference counting.
#[derive(Debug, Clone)]
pub struct OpenGlNamedResource {
    name: String,
}

impl OpenGlNamedResource {
    pub fn new(name: String) -> Self {
        OpenGlNamedResource { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_resource() {
        let res = OpenGlNamedResource::new("test".into());
        assert_eq!(res.name(), "test");
    }
}

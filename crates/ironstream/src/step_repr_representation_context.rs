// FILE: step_repr_representation_context.rs
// occt: StepRepr_RepresentationContext

/// Represents a STEP representation context with identifier and type information.
pub struct RepresentationContext {
    context_identifier: Option<String>,
    context_type: Option<String>,
}

impl RepresentationContext {
    /// Create a new RepresentationContext
    pub fn new() -> Self {
        RepresentationContext {
            context_identifier: None,
            context_type: None,
        }
    }

    /// Initialize representation context with identifier and type
    pub fn init(&mut self, identifier: String, context_type: String) {
        self.context_identifier = Some(identifier);
        self.context_type = Some(context_type);
    }

    /// Set the context identifier
    pub fn set_context_identifier(&mut self, identifier: String) {
        self.context_identifier = Some(identifier);
    }

    /// Get the context identifier
    pub fn context_identifier(&self) -> Option<&str> {
        self.context_identifier.as_deref()
    }

    /// Set the context type
    pub fn set_context_type(&mut self, context_type: String) {
        self.context_type = Some(context_type);
    }

    /// Get the context type
    pub fn context_type(&self) -> Option<&str> {
        self.context_type.as_deref()
    }
}

impl Default for RepresentationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let ctx = RepresentationContext::new();
        assert_eq!(ctx.context_identifier(), None);
        assert_eq!(ctx.context_type(), None);
    }

    #[test]
    fn test_init() {
        let mut ctx = RepresentationContext::new();
        ctx.init("ID123".to_string(), "3D".to_string());
        assert_eq!(ctx.context_identifier(), Some("ID123"));
        assert_eq!(ctx.context_type(), Some("3D"));
    }

    #[test]
    fn test_set_and_get_identifier() {
        let mut ctx = RepresentationContext::new();
        ctx.set_context_identifier("TestID".to_string());
        assert_eq!(ctx.context_identifier(), Some("TestID"));
    }

    #[test]
    fn test_set_and_get_type() {
        let mut ctx = RepresentationContext::new();
        ctx.set_context_type("2D".to_string());
        assert_eq!(ctx.context_type(), Some("2D"));
    }
}

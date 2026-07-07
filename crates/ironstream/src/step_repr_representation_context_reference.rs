// FILE: step_repr_representation_context_reference.rs
// occt: StepRepr_RepresentationContextReference

/// Represents a reference to a STEP representation context.
pub struct RepresentationContextReference {
    context_identifier: Option<String>,
}

impl RepresentationContextReference {
    /// Create a new RepresentationContextReference
    pub fn new() -> Self {
        RepresentationContextReference {
            context_identifier: None,
        }
    }

    /// Initialize representation context reference with identifier
    pub fn init(&mut self, identifier: String) {
        self.context_identifier = Some(identifier);
    }

    /// Get the context identifier
    pub fn context_identifier(&self) -> Option<&str> {
        self.context_identifier.as_deref()
    }

    /// Set the context identifier
    pub fn set_context_identifier(&mut self, identifier: String) {
        self.context_identifier = Some(identifier);
    }
}

impl Default for RepresentationContextReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let ref_ctx = RepresentationContextReference::new();
        assert_eq!(ref_ctx.context_identifier(), None);
    }

    #[test]
    fn test_init() {
        let mut ref_ctx = RepresentationContextReference::new();
        ref_ctx.init("CTX_001".to_string());
        assert_eq!(ref_ctx.context_identifier(), Some("CTX_001"));
    }

    #[test]
    fn test_set_and_get_identifier() {
        let mut ref_ctx = RepresentationContextReference::new();
        ref_ctx.set_context_identifier("REF_ID".to_string());
        assert_eq!(ref_ctx.context_identifier(), Some("REF_ID"));
    }
}

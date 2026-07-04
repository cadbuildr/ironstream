// FILE: step_repr_representation_reference.rs
// occt: StepRepr_RepresentationReference

/// Placeholder for RepresentationContextReference
#[derive(Clone, Debug, PartialEq)]
pub struct RepresentationContextReference {
    identifier: String,
}

/// Represents a reference to a STEP representation with context information.
pub struct RepresentationReference {
    id: Option<String>,
    context_of_items: Option<RepresentationContextReference>,
}

impl RepresentationReference {
    /// Create a new RepresentationReference
    pub fn new() -> Self {
        RepresentationReference {
            id: None,
            context_of_items: None,
        }
    }

    /// Initialize representation reference with id and context
    pub fn init(&mut self, id: String, context_of_items: RepresentationContextReference) {
        self.id = Some(id);
        self.context_of_items = Some(context_of_items);
    }

    /// Get the id
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Set the id
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Get the context of items
    pub fn context_of_items(&self) -> Option<&RepresentationContextReference> {
        self.context_of_items.as_ref()
    }

    /// Set the context of items
    pub fn set_context_of_items(&mut self, context: RepresentationContextReference) {
        self.context_of_items = Some(context);
    }
}

impl Default for RepresentationReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let ref_rep = RepresentationReference::new();
        assert_eq!(ref_rep.id(), None);
        assert_eq!(ref_rep.context_of_items(), None);
    }

    #[test]
    fn test_init() {
        let mut ref_rep = RepresentationReference::new();
        let ctx = RepresentationContextReference {
            identifier: "CTX_001".to_string(),
        };
        ref_rep.init("REP_001".to_string(), ctx.clone());
        assert_eq!(ref_rep.id(), Some("REP_001"));
        assert_eq!(ref_rep.context_of_items(), Some(&ctx));
    }

    #[test]
    fn test_set_and_get_id() {
        let mut ref_rep = RepresentationReference::new();
        ref_rep.set_id("ID_123".to_string());
        assert_eq!(ref_rep.id(), Some("ID_123"));
    }

    #[test]
    fn test_set_and_get_context() {
        let mut ref_rep = RepresentationReference::new();
        let ctx = RepresentationContextReference {
            identifier: "context".to_string(),
        };
        ref_rep.set_context_of_items(ctx.clone());
        assert_eq!(ref_rep.context_of_items(), Some(&ctx));
    }
}

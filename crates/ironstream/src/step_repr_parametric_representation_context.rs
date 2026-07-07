// FILE: step_repr_parametric_representation_context.rs
// occt: StepRepr_ParametricRepresentationContext

/// StepRepr_ParametricRepresentationContext: Parametric representation context
/// Inherits from StepRepr_RepresentationContext
#[derive(Clone, Debug)]
pub struct StepReprParametricRepresentationContext {
    context_id: String,
    context_type: String,
}

impl StepReprParametricRepresentationContext {
    /// Returns a ParametricRepresentationContext
    pub fn new() -> Self {
        StepReprParametricRepresentationContext {
            context_id: String::new(),
            context_type: String::new(),
        }
    }

    /// Get context id
    pub fn context_id(&self) -> &str {
        &self.context_id
    }

    /// Set context id
    pub fn set_context_id(&mut self, context_id: String) {
        self.context_id = context_id;
    }

    /// Get context type
    pub fn context_type(&self) -> &str {
        &self.context_type
    }

    /// Set context type
    pub fn set_context_type(&mut self, context_type: String) {
        self.context_type = context_type;
    }
}

impl Default for StepReprParametricRepresentationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let prc = StepReprParametricRepresentationContext::new();
        assert_eq!(prc.context_id(), "");
        assert_eq!(prc.context_type(), "");
    }

    #[test]
    fn test_set_context_id() {
        let mut prc = StepReprParametricRepresentationContext::new();
        prc.set_context_id("ctx1".to_string());
        assert_eq!(prc.context_id(), "ctx1");
    }

    #[test]
    fn test_set_context_type() {
        let mut prc = StepReprParametricRepresentationContext::new();
        prc.set_context_type("parametric".to_string());
        assert_eq!(prc.context_type(), "parametric");
    }
}

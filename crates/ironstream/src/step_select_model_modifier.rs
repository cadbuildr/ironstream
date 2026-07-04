// FILE: step_select_model_modifier.rs
// occt: StepSelect_ModelModifier

/// Trait for model modifiers
pub trait ModelModifier {
    /// Perform modifications
    fn performing(&self);

    /// Perform with protocol
    fn perform_protocol(&self) {
        self.performing();
    }
}

/// Default implementation of ModelModifier
pub struct DefaultModelModifier {
    may_change_graph: bool,
}

impl DefaultModelModifier {
    /// Create a new DefaultModelModifier
    pub fn new(may_change_graph: bool) -> Self {
        DefaultModelModifier { may_change_graph }
    }

    /// Get the may_change_graph flag
    pub fn may_change_graph(&self) -> bool {
        self.may_change_graph
    }

    /// Set the may_change_graph flag
    pub fn set_may_change_graph(&mut self, may_change: bool) {
        self.may_change_graph = may_change;
    }
}

impl ModelModifier for DefaultModelModifier {
    fn performing(&self) {
        // Default implementation does nothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_with_flag() {
        let modifier = DefaultModelModifier::new(true);
        assert!(modifier.may_change_graph());
    }

    #[test]
    fn test_set_may_change_graph() {
        let mut modifier = DefaultModelModifier::new(false);
        modifier.set_may_change_graph(true);
        assert!(modifier.may_change_graph());
    }

    #[test]
    fn test_trait_implementation() {
        let modifier = DefaultModelModifier::new(true);
        let _: &dyn ModelModifier = &modifier;
        modifier.performing(); // Just verify it can be called
        modifier.perform_protocol(); // And this one
    }
}

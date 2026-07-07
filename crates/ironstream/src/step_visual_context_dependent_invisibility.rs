// FILE: step_visual_context_dependent_invisibility.rs
// occt: StepVisual_ContextDependentInvisibility

/// A context-dependent invisibility definition in STEP representation.
///
/// This specifies visibility context for invisible items.
pub struct ContextDependentInvisibility {
    context: i32,
}

impl ContextDependentInvisibility {
    /// Creates a new context-dependent invisibility.
    pub fn new() -> Self {
        ContextDependentInvisibility { context: 0 }
    }

    /// Sets the presentation context.
    pub fn set_presentation_context(&mut self, context: i32) {
        self.context = context;
    }

    /// Returns the presentation context.
    pub fn presentation_context(&self) -> i32 {
        self.context
    }
}

impl Default for ContextDependentInvisibility {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_dependent_invisibility_new() {
        let inv = ContextDependentInvisibility::new();
        assert_eq!(inv.presentation_context(), 0);
    }

    #[test]
    fn test_set_presentation_context() {
        let mut inv = ContextDependentInvisibility::new();
        inv.set_presentation_context(42);
        assert_eq!(inv.presentation_context(), 42);
    }
}

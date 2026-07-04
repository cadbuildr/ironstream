// FILE: step_visual_context_dependent_over_riding_styled_item.rs
// occt: StepVisual_ContextDependentOverRidingStyledItem

/// A context-dependent overriding styled item in STEP representation.
///
/// This defines style overrides for specific contexts.
pub struct ContextDependentOverRidingStyledItem {
    name: String,
    style_context: i32,
}

impl ContextDependentOverRidingStyledItem {
    /// Creates a new context-dependent overriding styled item.
    pub fn new(name: String) -> Self {
        ContextDependentOverRidingStyledItem {
            name,
            style_context: 0,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the style context.
    pub fn set_style_context(&mut self, context: i32) {
        self.style_context = context;
    }

    /// Returns the style context.
    pub fn style_context(&self) -> i32 {
        self.style_context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_dependent_over_riding_styled_item_new() {
        let item = ContextDependentOverRidingStyledItem::new("ItemName".to_string());
        assert_eq!(item.name(), "ItemName");
        assert_eq!(item.style_context(), 0);
    }

    #[test]
    fn test_set_style_context() {
        let mut item = ContextDependentOverRidingStyledItem::new("Item".to_string());
        item.set_style_context(123);
        assert_eq!(item.style_context(), 123);
    }
}

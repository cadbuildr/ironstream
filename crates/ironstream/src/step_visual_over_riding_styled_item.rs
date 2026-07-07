// FILE: step_visual_over_riding_styled_item.rs
// occt: StepVisual_OverRidingStyledItem

/// An overriding styled item in STEP representation.
///
/// This overrides the style of another styled item.
pub struct OverRidingStyledItem {
    name: String,
    override_style: String,
}

impl OverRidingStyledItem {
    /// Creates a new overriding styled item.
    pub fn new(name: String) -> Self {
        OverRidingStyledItem {
            name,
            override_style: String::new(),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the override style.
    pub fn set_override_style(&mut self, style: String) {
        self.override_style = style;
    }

    /// Returns the override style.
    pub fn override_style(&self) -> &str {
        &self.override_style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_over_riding_styled_item_new() {
        let item = OverRidingStyledItem::new("StyledItem".to_string());
        assert_eq!(item.name(), "StyledItem");
        assert_eq!(item.override_style(), "");
    }

    #[test]
    fn test_set_override_style() {
        let mut item = OverRidingStyledItem::new("Item".to_string());
        item.set_override_style("BoldStyle".to_string());
        assert_eq!(item.override_style(), "BoldStyle");
    }
}

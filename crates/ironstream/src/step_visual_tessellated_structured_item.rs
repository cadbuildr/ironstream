// FILE: step_visual_tessellated_structured_item.rs
// occt: StepVisual_TessellatedStructuredItem

/// Represents a STEP TessellatedStructuredItem entity.
/// A base class for tessellated items with structure.
pub struct TessellatedStructuredItem {
    name: String,
}

impl TessellatedStructuredItem {
    /// Creates a new tessellated structured item.
    pub fn new() -> Self {
        TessellatedStructuredItem {
            name: String::new(),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for TessellatedStructuredItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tsi = TessellatedStructuredItem::new();
        assert_eq!(tsi.name(), "");
    }

    #[test]
    fn test_set_name() {
        let mut tsi = TessellatedStructuredItem::new();
        tsi.set_name("TestItem".to_string());
        assert_eq!(tsi.name(), "TestItem");
    }
}

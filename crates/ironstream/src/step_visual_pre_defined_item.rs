// FILE: step_visual_pre_defined_item.rs
// occt: StepVisual_PreDefinedItem

/// A pre-defined item in STEP representation.
///
/// This is a base for standard predefined items.
pub struct PreDefinedItem {
    name: String,
    item_type: String,
}

impl PreDefinedItem {
    /// Creates a new pre-defined item.
    pub fn new(name: String) -> Self {
        PreDefinedItem {
            name,
            item_type: String::new(),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the item type.
    pub fn set_item_type(&mut self, item_type: String) {
        self.item_type = item_type;
    }

    /// Returns the item type.
    pub fn item_type(&self) -> &str {
        &self.item_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_defined_item_new() {
        let item = PreDefinedItem::new("Item".to_string());
        assert_eq!(item.name(), "Item");
        assert_eq!(item.item_type(), "");
    }

    #[test]
    fn test_set_item_type() {
        let mut item = PreDefinedItem::new("PredefinedItem".to_string());
        item.set_item_type("Marker".to_string());
        assert_eq!(item.item_type(), "Marker");
    }
}

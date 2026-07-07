// FILE: step_repr_representation_item.rs
// occt: StepRepr_RepresentationItem

/// Represents a STEP representation item, a basic element in a representation.
pub struct RepresentationItem {
    name: Option<String>,
}

impl RepresentationItem {
    /// Create a new RepresentationItem
    pub fn new() -> Self {
        RepresentationItem { name: None }
    }

    /// Initialize representation item with name
    pub fn init(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl Default for RepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = RepresentationItem::new();
        assert_eq!(item.name(), None);
    }

    #[test]
    fn test_init() {
        let mut item = RepresentationItem::new();
        item.init("ItemName".to_string());
        assert_eq!(item.name(), Some("ItemName"));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut item = RepresentationItem::new();
        item.set_name("TestItem".to_string());
        assert_eq!(item.name(), Some("TestItem"));
    }
}

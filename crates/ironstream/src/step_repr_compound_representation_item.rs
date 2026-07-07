// FILE: step_repr_compound_representation_item.rs
// occt: StepRepr_CompoundRepresentationItem

/// StepRepr_CompoundRepresentationItem:
/// Added for Dimensional Tolerances.
/// A RepresentationItem that contains an array of other RepresentationItems.
#[derive(Clone, Debug)]
pub struct StepReprCompoundRepresentationItem {
    name: String,
    item_elements: Vec<String>, // Simplified: storing names of representation items
}

impl StepReprCompoundRepresentationItem {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprCompoundRepresentationItem {
            name: String::new(),
            item_elements: Vec::new(),
        }
    }

    /// Initialize with name and items
    pub fn init(&mut self, name: String, items: Vec<String>) {
        self.name = name;
        self.item_elements = items;
    }

    /// Get item elements
    pub fn item_element(&self) -> &[String] {
        &self.item_elements
    }

    /// Get number of item elements
    pub fn nb_item_element(&self) -> usize {
        self.item_elements.len()
    }

    /// Set item elements
    pub fn set_item_element(&mut self, items: Vec<String>) {
        self.item_elements = items;
    }

    /// Get item element value by index (1-based)
    pub fn item_element_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.item_elements.len() {
            Some(&self.item_elements[num - 1])
        } else {
            None
        }
    }

    /// Set item element value by index (1-based)
    pub fn set_item_element_value(&mut self, num: usize, element: String) -> bool {
        if num > 0 && num <= self.item_elements.len() {
            self.item_elements[num - 1] = element;
            true
        } else {
            false
        }
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for StepReprCompoundRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = StepReprCompoundRepresentationItem::new();
        assert_eq!(item.name(), "");
        assert_eq!(item.nb_item_element(), 0);
    }

    #[test]
    fn test_init() {
        let mut item = StepReprCompoundRepresentationItem::new();
        let elements = vec!["elem1".to_string(), "elem2".to_string()];
        item.init("test".to_string(), elements);
        assert_eq!(item.name(), "test");
        assert_eq!(item.nb_item_element(), 2);
    }

    #[test]
    fn test_set_item_element() {
        let mut item = StepReprCompoundRepresentationItem::new();
        let elements = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        item.set_item_element(elements);
        assert_eq!(item.nb_item_element(), 3);
    }

    #[test]
    fn test_item_element_value() {
        let mut item = StepReprCompoundRepresentationItem::new();
        let elements = vec!["first".to_string(), "second".to_string()];
        item.set_item_element(elements);
        assert_eq!(item.item_element_value(1), Some(&"first".to_string()));
        assert_eq!(item.item_element_value(2), Some(&"second".to_string()));
        assert_eq!(item.item_element_value(3), None);
    }

    #[test]
    fn test_set_item_element_value() {
        let mut item = StepReprCompoundRepresentationItem::new();
        let elements = vec!["a".to_string(), "b".to_string()];
        item.set_item_element(elements);
        assert!(item.set_item_element_value(1, "new_a".to_string()));
        assert_eq!(item.item_element_value(1), Some(&"new_a".to_string()));
        assert!(!item.set_item_element_value(5, "x".to_string()));
    }
}

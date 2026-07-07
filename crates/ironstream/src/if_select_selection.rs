// FILE: if_select_selection.rs
// occt: IFSelect_Selection

/// Abstract base for defining a set of Interface entities.
/// Entities are selected based on criteria that can be replayed across variants.
/// Input can be a model, other selections, or other outputs.
#[derive(Clone, Debug)]
pub struct IFSelectSelection {
    label_text: String,
}

impl IFSelectSelection {
    /// Creates a Selection with a label
    pub fn new(label: String) -> Self {
        Self { label_text: label }
    }

    /// Returns the label defining the selection criterium
    pub fn label(&self) -> &str {
        &self.label_text
    }

    /// Returns true if RootResult guarantees uniqueness for each entity.
    /// Default is false. Can be overridden in subclasses.
    pub fn has_unique_result(&self) -> bool {
        false
    }

    /// Sets the label
    pub fn set_label(&mut self, label: String) {
        self.label_text = label;
    }
}

impl Default for IFSelectSelection {
    fn default() -> Self {
        Self {
            label_text: "Selection".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sel = IFSelectSelection::new("Test Selection".to_string());
        assert_eq!(sel.label(), "Test Selection");
    }

    #[test]
    fn test_has_unique_result() {
        let sel = IFSelectSelection::new("Test".to_string());
        assert!(!sel.has_unique_result());
    }

    #[test]
    fn test_set_label() {
        let mut sel = IFSelectSelection::new("Original".to_string());
        assert_eq!(sel.label(), "Original");
        sel.set_label("Modified".to_string());
        assert_eq!(sel.label(), "Modified");
    }

    #[test]
    fn test_default() {
        let sel = IFSelectSelection::default();
        assert_eq!(sel.label(), "Selection");
        assert!(!sel.has_unique_result());
    }
}

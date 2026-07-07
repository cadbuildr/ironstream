// FILE: step_ap214_applied_document_reference.rs
// occt: StepAP214_AppliedDocumentReference

#[derive(Clone, Debug)]
pub struct DocumentReferenceItem {
    // Placeholder
}

/// Representation of STEP AP214 AppliedDocumentReference entity.
/// Extends DocumentReference with a list of items to which the document reference applies.
#[derive(Clone, Debug)]
pub struct AppliedDocumentReference {
    items: Vec<DocumentReferenceItem>,
}

impl AppliedDocumentReference {
    /// Creates a new AppliedDocumentReference.
    pub fn new() -> Self {
        AppliedDocumentReference {
            items: Vec::new(),
        }
    }

    /// Initializes with items.
    pub fn init(&mut self, items: Vec<DocumentReferenceItem>) {
        self.items = items;
    }

    /// Sets the items.
    pub fn set_items(&mut self, items: Vec<DocumentReferenceItem>) {
        self.items = items;
    }

    /// Returns the items collection.
    pub fn items(&self) -> &[DocumentReferenceItem] {
        &self.items
    }

    /// Returns the item at the given index (1-based).
    pub fn items_value(&self, num: usize) -> Option<&DocumentReferenceItem> {
        if num > 0 && num <= self.items.len() {
            Some(&self.items[num - 1])
        } else {
            None
        }
    }

    /// Returns the number of items.
    pub fn nb_items(&self) -> usize {
        self.items.len()
    }
}

impl Default for AppliedDocumentReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_applied_document_reference() {
        let reference = AppliedDocumentReference::new();
        assert_eq!(reference.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut reference = AppliedDocumentReference::new();
        let items = vec![DocumentReferenceItem {}];
        reference.set_items(items);
        assert_eq!(reference.nb_items(), 1);
    }

    #[test]
    fn test_items_value() {
        let mut reference = AppliedDocumentReference::new();
        let items = vec![DocumentReferenceItem {}, DocumentReferenceItem {}];
        reference.set_items(items);
        assert!(reference.items_value(1).is_some());
        assert!(reference.items_value(2).is_some());
        assert!(reference.items_value(3).is_none());
    }
}

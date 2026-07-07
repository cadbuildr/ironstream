// FILE: shape_persistent_h_sequence.rs
// occt: ShapePersistent_HSequence

/// Dynamic sequence of persistent objects
pub struct HSequence {
    items: Vec<Option<String>>,
}

impl HSequence {
    /// Create a new empty sequence
    pub fn new() -> Self {
        HSequence {
            items: Vec::new(),
        }
    }

    /// Append an element
    pub fn append(&mut self, item: Option<String>) {
        self.items.push(item);
    }

    /// Get the number of elements
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if sequence is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get element at index
    pub fn get(&self, index: usize) -> Option<&Option<String>> {
        self.items.get(index)
    }

    /// Clear the sequence
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for HSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let seq = HSequence::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = HSequence::new();
        seq.append(Some("item1".to_string()));
        seq.append(Some("item2".to_string()));

        assert_eq!(seq.len(), 2);
        assert_eq!(seq.get(0), Some(&Some("item1".to_string())));
    }

    #[test]
    fn test_clear() {
        let mut seq = HSequence::new();
        seq.append(Some("item".to_string()));
        seq.clear();
        assert!(seq.is_empty());
    }
}

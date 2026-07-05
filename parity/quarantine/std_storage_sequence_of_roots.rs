// FILE: std_storage_sequence_of_roots.rs
// occt: StdStorage_SequenceOfRoots, StdStorage_SequenceIteratorOfSequenceOfRoots

/// Deprecated typedef for backward compatibility.
/// A sequence of storage roots using 1-based indexing (OCCT semantics).
/// Corresponds to NCollection_Sequence<opencascade::handle<StdStorage_Root>>
pub struct StdStorageSequenceOfRoots {
    // 1-based indexing: store data starting at index 1, leave index 0 empty
    data: Vec<Option<String>>,
}

impl StdStorageSequenceOfRoots {
    /// Create a new empty sequence.
    pub fn new() -> Self {
        StdStorageSequenceOfRoots {
            data: vec![None],
        }
    }

    /// Append an element to the sequence.
    pub fn append(&mut self, value: String) {
        self.data.push(Some(value));
    }

    /// Prepend an element to the sequence.
    pub fn prepend(&mut self, value: String) {
        self.data.insert(1, Some(value));
    }

    /// Get an element by 1-based index.
    pub fn value(&self, index: usize) -> Option<&String> {
        if index > 0 && index < self.data.len() {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    /// Get the length.
    pub fn len(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.data.clear();
        self.data.push(None);
    }

    /// Remove element at 1-based index.
    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index > 0 && index < self.data.len() {
            self.data.remove(index).flatten()
        } else {
            None
        }
    }

    /// Get lower bound (always 1).
    pub fn lower(&self) -> usize {
        1
    }

    /// Get upper bound.
    pub fn upper(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }
}

impl Default for StdStorageSequenceOfRoots {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sequence_operations() {
        let mut seq = StdStorageSequenceOfRoots::new();
        assert!(seq.is_empty());

        seq.append("root1".to_string());
        assert_eq!(seq.len(), 1);
        assert_eq!(seq.value(1), Some(&"root1".to_string()));

        seq.append("root2".to_string());
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.value(2), Some(&"root2".to_string()));
    }

    #[test]
    fn test_1based_bounds() {
        let mut seq = StdStorageSequenceOfRoots::new();
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 0);

        seq.append("item".to_string());
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 1);
    }

    #[test]
    fn test_prepend() {
        let mut seq = StdStorageSequenceOfRoots::new();
        seq.append("second".to_string());
        seq.prepend("first".to_string());

        assert_eq!(seq.value(1), Some(&"first".to_string()));
        assert_eq!(seq.value(2), Some(&"second".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut seq = StdStorageSequenceOfRoots::new();
        seq.append("a".to_string());
        seq.append("b".to_string());
        seq.append("c".to_string());

        let removed = seq.remove(2);
        assert_eq!(removed, Some("b".to_string()));
        assert_eq!(seq.len(), 2);
    }
}

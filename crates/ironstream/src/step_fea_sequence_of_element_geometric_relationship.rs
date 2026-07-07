// FILE: step_fea_sequence_of_element_geometric_relationship.rs
// occt: StepFEA_SequenceOfElementGeometricRelationship

use std::vec::Vec;

/// Deprecated typedef alias for NCollection_Sequence<StepFEA_ElementGeometricRelationship>.
/// Provides 1-based indexing semantics compatible with OCCT's Sequence container.
pub struct StepFEASequenceOfElementGeometricRelationship {
    data: Vec<Option<String>>, // Using String as placeholder for StepFEA_ElementGeometricRelationship handle
    lower: usize,
}

impl StepFEASequenceOfElementGeometricRelationship {
    /// Create an empty sequence.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            lower: 1,
        }
    }

    /// Create a sequence with reserved capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            lower: 1,
        }
    }

    /// Get the lower bound index (always 1 for OCCT Sequence).
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Get the upper bound index (length, 1-based).
    pub fn upper(&self) -> usize {
        self.lower + self.data.len() - 1
    }

    /// Get the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Append an element to the end of the sequence.
    pub fn append(&mut self, value: Option<String>) {
        self.data.push(value);
    }

    /// Get an element by 1-based index. Returns None if index is out of bounds.
    pub fn value(&self, index: usize) -> Option<&Option<String>> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let zero_based_index = index - self.lower;
        self.data.get(zero_based_index)
    }

    /// Set an element by 1-based index. Returns false if index is out of bounds.
    pub fn set_value(&mut self, index: usize, value: Option<String>) -> bool {
        if index < self.lower || index > self.upper() {
            return false;
        }
        let zero_based_index = index - self.lower;
        if let Some(elem) = self.data.get_mut(zero_based_index) {
            *elem = value;
            true
        } else {
            false
        }
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Remove an element at a 1-based index. Returns the removed element or None.
    pub fn remove(&mut self, index: usize) -> Option<Option<String>> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let zero_based_index = index - self.lower;
        if zero_based_index < self.data.len() {
            Some(self.data.remove(zero_based_index))
        } else {
            None
        }
    }

    /// Prepend an element to the sequence.
    pub fn prepend(&mut self, value: Option<String>) {
        self.data.insert(0, value);
    }
}

impl Default for StepFEASequenceOfElementGeometricRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence_is_empty() {
        let seq = StepFEASequenceOfElementGeometricRelationship::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 0);
    }

    #[test]
    fn test_append_and_length() {
        let mut seq = StepFEASequenceOfElementGeometricRelationship::new();
        seq.append(Some("rel1".to_string()));
        seq.append(Some("rel2".to_string()));

        assert_eq!(seq.len(), 2);
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 2);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_one_based_indexing() {
        let mut seq = StepFEASequenceOfElementGeometricRelationship::new();
        seq.append(Some("r1".to_string()));
        seq.append(Some("r2".to_string()));
        seq.append(Some("r3".to_string()));

        assert_eq!(seq.value(1), Some(&Some("r1".to_string())));
        assert_eq!(seq.value(2), Some(&Some("r2".to_string())));
        assert_eq!(seq.value(3), Some(&Some("r3".to_string())));

        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(4), None);
    }

    #[test]
    fn test_set_value() {
        let mut seq = StepFEASequenceOfElementGeometricRelationship::new();
        seq.append(Some("old".to_string()));

        let success = seq.set_value(1, Some("new".to_string()));
        assert!(success);
        assert_eq!(seq.value(1), Some(&Some("new".to_string())));

        let out_of_bounds = seq.set_value(99, Some("ignored".to_string()));
        assert!(!out_of_bounds);
    }

    #[test]
    fn test_remove() {
        let mut seq = StepFEASequenceOfElementGeometricRelationship::new();
        seq.append(Some("a".to_string()));
        seq.append(Some("b".to_string()));
        seq.append(Some("c".to_string()));

        let removed = seq.remove(2);
        assert_eq!(removed, Some(Some("b".to_string())));
        assert_eq!(seq.len(), 2);

        assert_eq!(seq.value(1), Some(&Some("a".to_string())));
        assert_eq!(seq.value(2), Some(&Some("c".to_string())));
    }

    #[test]
    fn test_prepend() {
        let mut seq = StepFEASequenceOfElementGeometricRelationship::new();
        seq.append(Some("second".to_string()));
        seq.prepend(Some("first".to_string()));

        assert_eq!(seq.len(), 2);
        assert_eq!(seq.value(1), Some(&Some("first".to_string())));
        assert_eq!(seq.value(2), Some(&Some("second".to_string())));
    }

    #[test]
    fn test_clear() {
        let mut seq = StepFEASequenceOfElementGeometricRelationship::new();
        seq.append(Some("item".to_string()));

        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }
}

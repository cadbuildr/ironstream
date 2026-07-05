// FILE: t_col_std_sequence_of_h_extended_string.rs
// occt: TColStd_SequenceOfHExtendedString

use std::collections::VecDeque;

/// Deprecated typedef alias for a sequence of shared Handle<TCollection_HExtendedString>.
///
/// This is a 1-based sequence container, mirroring NCollection_Sequence semantics.
/// In Rust, we use a VecDeque with 1-based indexing to maintain OCCT compatibility.
#[derive(Debug, Clone)]
pub struct TColStdSequenceOfHExtendedString {
    data: VecDeque<String>, // Simplified: storing String instead of Handle<TCollection_HExtendedString>
}

impl TColStdSequenceOfHExtendedString {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        TColStdSequenceOfHExtendedString {
            data: VecDeque::new(),
        }
    }

    /// Returns the length of the sequence.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Appends an element to the end of the sequence (1-based indexing).
    pub fn append(&mut self, value: String) {
        self.data.push_back(value);
    }

    /// Returns the element at 1-based index (1..=len()).
    /// Panics if index is out of bounds.
    pub fn get(&self, idx: usize) -> &String {
        if idx < 1 || idx > self.data.len() {
            panic!("Index out of range: {}", idx);
        }
        &self.data[idx - 1]
    }

    /// Returns the element at 1-based index (mutable).
    pub fn get_mut(&mut self, idx: usize) -> &mut String {
        if idx < 1 || idx > self.data.len() {
            panic!("Index out of range: {}", idx);
        }
        &mut self.data[idx - 1]
    }

    /// Returns the first element (Lower bound = 1).
    pub fn first(&self) -> Option<&String> {
        self.data.front()
    }

    /// Returns the last element (Upper bound = len()).
    pub fn last(&self) -> Option<&String> {
        self.data.back()
    }

    /// Clears the sequence.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns the lower bound (always 1 for OCCT compatibility).
    pub fn lower(&self) -> usize {
        1
    }

    /// Returns the upper bound (len() or 0 if empty).
    pub fn upper(&self) -> usize {
        self.data.len()
    }
}

impl Default for TColStdSequenceOfHExtendedString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_append_and_len() {
        let mut seq = TColStdSequenceOfHExtendedString::new();
        assert_eq!(seq.len(), 0);

        seq.append("hello".to_string());
        seq.append("world".to_string());
        assert_eq!(seq.len(), 2);
    }

    #[test]
    fn test_sequence_get_with_one_based_indexing() {
        let mut seq = TColStdSequenceOfHExtendedString::new();
        seq.append("first".to_string());
        seq.append("second".to_string());

        // 1-based indexing
        assert_eq!(seq.get(1), "first");
        assert_eq!(seq.get(2), "second");
    }

    #[test]
    fn test_sequence_bounds() {
        let mut seq = TColStdSequenceOfHExtendedString::new();
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 0); // empty

        seq.append("item".to_string());
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 1);
    }

    #[test]
    fn test_sequence_first_last() {
        let mut seq = TColStdSequenceOfHExtendedString::new();
        assert!(seq.first().is_none());
        assert!(seq.last().is_none());

        seq.append("only".to_string());
        assert_eq!(seq.first(), Some(&"only".to_string()));
        assert_eq!(seq.last(), Some(&"only".to_string()));

        seq.append("second".to_string());
        assert_eq!(seq.first(), Some(&"only".to_string()));
        assert_eq!(seq.last(), Some(&"second".to_string()));
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = TColStdSequenceOfHExtendedString::new();
        seq.append("a".to_string());
        seq.append("b".to_string());
        assert_eq!(seq.len(), 2);

        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_sequence_get_out_of_bounds() {
        let seq = TColStdSequenceOfHExtendedString::new();
        let _ = seq.get(1); // Should panic on empty
    }
}

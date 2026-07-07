// FILE: t_col_std_sequence_of_transient.rs
// occt: TColStd_SequenceOfTransient

use std::collections::VecDeque;

/// Deprecated typedef alias for a sequence of shared Handle<Standard_Transient>.
///
/// This is a 1-based sequence container, mirroring NCollection_Sequence semantics.
/// In Rust, we use a VecDeque with 1-based indexing to maintain OCCT compatibility.
#[derive(Debug, Clone)]
pub struct TColStdSequenceOfTransient {
    data: VecDeque<usize>, // Simplified: storing usize IDs representing Handle<Standard_Transient>
}

impl TColStdSequenceOfTransient {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        TColStdSequenceOfTransient {
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
    pub fn append(&mut self, handle_id: usize) {
        self.data.push_back(handle_id);
    }

    /// Returns the element at 1-based index (1..=len()).
    /// Panics if index is out of bounds.
    pub fn get(&self, idx: usize) -> usize {
        if idx < 1 || idx > self.data.len() {
            panic!("Index out of range: {}", idx);
        }
        self.data[idx - 1]
    }

    /// Returns the first element (Lower bound = 1).
    pub fn first(&self) -> Option<usize> {
        self.data.front().copied()
    }

    /// Returns the last element (Upper bound = len()).
    pub fn last(&self) -> Option<usize> {
        self.data.back().copied()
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

    /// Removes and returns the element at 1-based index.
    pub fn remove(&mut self, idx: usize) -> usize {
        if idx < 1 || idx > self.data.len() {
            panic!("Index out of range: {}", idx);
        }
        self.data.remove(idx - 1).expect("Element should exist")
    }
}

impl Default for TColStdSequenceOfTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_append_and_len() {
        let mut seq = TColStdSequenceOfTransient::new();
        assert_eq!(seq.len(), 0);

        seq.append(100);
        seq.append(200);
        assert_eq!(seq.len(), 2);
    }

    #[test]
    fn test_sequence_get_with_one_based_indexing() {
        let mut seq = TColStdSequenceOfTransient::new();
        seq.append(42);
        seq.append(84);

        // 1-based indexing
        assert_eq!(seq.get(1), 42);
        assert_eq!(seq.get(2), 84);
    }

    #[test]
    fn test_sequence_bounds() {
        let mut seq = TColStdSequenceOfTransient::new();
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 0); // empty

        seq.append(999);
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 1);
    }

    #[test]
    fn test_sequence_first_last() {
        let mut seq = TColStdSequenceOfTransient::new();
        assert!(seq.first().is_none());
        assert!(seq.last().is_none());

        seq.append(10);
        assert_eq!(seq.first(), Some(10));
        assert_eq!(seq.last(), Some(10));

        seq.append(20);
        assert_eq!(seq.first(), Some(10));
        assert_eq!(seq.last(), Some(20));
    }

    #[test]
    fn test_sequence_remove() {
        let mut seq = TColStdSequenceOfTransient::new();
        seq.append(1);
        seq.append(2);
        seq.append(3);

        let removed = seq.remove(2);
        assert_eq!(removed, 2);
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.get(1), 1);
        assert_eq!(seq.get(2), 3);
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = TColStdSequenceOfTransient::new();
        seq.append(10);
        seq.append(20);
        assert_eq!(seq.len(), 2);

        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_sequence_get_out_of_bounds() {
        let seq = TColStdSequenceOfTransient::new();
        let _ = seq.get(1);
    }
}

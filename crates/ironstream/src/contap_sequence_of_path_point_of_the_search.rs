// FILE: contap_sequence_of_path_point_of_the_search.rs
// occt: Contap_SequenceOfPathPointOfTheSearch

//! Deprecated typedef alias for backward compatibility.
//! This was `NCollection_Sequence<Contap_ThePathPointOfTheSearch>` in OCCT.
//!
//! In Rust, we model this as a newtype over `Vec` for sequence semantics.
//! A sequence is an ordered collection with insertion/removal at any position.

use std::ops::{Deref, DerefMut};

/// Contap_ThePathPointOfTheSearch placeholder for test purposes.
/// In real usage, this would be a complete type from contap module.
#[derive(Clone, Debug, PartialEq)]
pub struct ContapThePathPointOfTheSearch {
    value: i32,
}

impl ContapThePathPointOfTheSearch {
    pub fn new(value: i32) -> Self {
        ContapThePathPointOfTheSearch { value }
    }
}

/// A sequence (ordered collection) of Contap_ThePathPointOfTheSearch items.
/// Models `NCollection_Sequence<Contap_ThePathPointOfTheSearch>` from OCCT.
#[derive(Clone, Debug)]
pub struct ContapSequenceOfPathPointOfTheSearch {
    items: Vec<ContapThePathPointOfTheSearch>,
}

impl ContapSequenceOfPathPointOfTheSearch {
    /// Create an empty sequence.
    pub fn new() -> Self {
        ContapSequenceOfPathPointOfTheSearch {
            items: Vec::new(),
        }
    }

    /// Append an item to the end of the sequence.
    pub fn append(&mut self, item: ContapThePathPointOfTheSearch) {
        self.items.push(item);
    }

    /// Remove and return the item at the given index (1-based for OCCT compatibility).
    /// Returns None if the index is out of bounds.
    pub fn remove(&mut self, index: usize) -> Option<ContapThePathPointOfTheSearch> {
        if index > 0 && index <= self.items.len() {
            Some(self.items.remove(index - 1))
        } else {
            None
        }
    }

    /// Get the number of items in the sequence.
    pub fn length(&self) -> usize {
        self.items.len()
    }

    /// Get a reference to an item at the given 1-based index.
    pub fn value(&self, index: usize) -> Option<&ContapThePathPointOfTheSearch> {
        if index > 0 && index <= self.items.len() {
            Some(&self.items[index - 1])
        } else {
            None
        }
    }

    /// Get a mutable reference to an item at the given 1-based index.
    pub fn value_mut(&mut self, index: usize) -> Option<&mut ContapThePathPointOfTheSearch> {
        if index > 0 && index <= self.items.len() {
            Some(&mut self.items[index - 1])
        } else {
            None
        }
    }

    /// Clear the sequence, removing all items.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Insert an item at the given 1-based position.
    pub fn insert_at(&mut self, index: usize, item: ContapThePathPointOfTheSearch) {
        if index > 0 && index <= self.items.len() + 1 {
            self.items.insert(index - 1, item);
        }
    }
}

impl Default for ContapSequenceOfPathPointOfTheSearch {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ContapSequenceOfPathPointOfTheSearch {
    type Target = Vec<ContapThePathPointOfTheSearch>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for ContapSequenceOfPathPointOfTheSearch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = ContapSequenceOfPathPointOfTheSearch::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_append_and_length() {
        let mut seq = ContapSequenceOfPathPointOfTheSearch::new();
        seq.append(ContapThePathPointOfTheSearch::new(10));
        seq.append(ContapThePathPointOfTheSearch::new(20));
        seq.append(ContapThePathPointOfTheSearch::new(30));
        assert_eq!(seq.length(), 3);
    }

    #[test]
    fn test_sequence_value_1_based_indexing() {
        let mut seq = ContapSequenceOfPathPointOfTheSearch::new();
        let item1 = ContapThePathPointOfTheSearch::new(100);
        let item2 = ContapThePathPointOfTheSearch::new(200);
        seq.append(item1.clone());
        seq.append(item2.clone());

        assert_eq!(seq.value(1), Some(&item1));
        assert_eq!(seq.value(2), Some(&item2));
        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(3), None);
    }

    #[test]
    fn test_sequence_remove() {
        let mut seq = ContapSequenceOfPathPointOfTheSearch::new();
        seq.append(ContapThePathPointOfTheSearch::new(10));
        seq.append(ContapThePathPointOfTheSearch::new(20));
        seq.append(ContapThePathPointOfTheSearch::new(30));

        let removed = seq.remove(2);
        assert_eq!(removed, Some(ContapThePathPointOfTheSearch::new(20)));
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(2), Some(&ContapThePathPointOfTheSearch::new(30)));
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = ContapSequenceOfPathPointOfTheSearch::new();
        seq.append(ContapThePathPointOfTheSearch::new(1));
        seq.append(ContapThePathPointOfTheSearch::new(2));
        assert_eq!(seq.length(), 2);

        seq.clear();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_insert_at() {
        let mut seq = ContapSequenceOfPathPointOfTheSearch::new();
        seq.append(ContapThePathPointOfTheSearch::new(10));
        seq.append(ContapThePathPointOfTheSearch::new(30));

        seq.insert_at(2, ContapThePathPointOfTheSearch::new(20));
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.value(2), Some(&ContapThePathPointOfTheSearch::new(20)));
    }

    #[test]
    fn test_sequence_deref() {
        let mut seq = ContapSequenceOfPathPointOfTheSearch::new();
        seq.append(ContapThePathPointOfTheSearch::new(5));
        seq.append(ContapThePathPointOfTheSearch::new(15));

        // Deref allows Vec-like iteration
        assert_eq!(seq.iter().count(), 2);
    }

    #[test]
    fn test_sequence_value_mut() {
        let mut seq = ContapSequenceOfPathPointOfTheSearch::new();
        seq.append(ContapThePathPointOfTheSearch::new(10));

        if let Some(item) = seq.value_mut(1) {
            item.value = 99;
        }

        assert_eq!(seq.value(1), Some(&ContapThePathPointOfTheSearch::new(99)));
    }
}

// FILE: contap_sequence_of_segment_of_the_search.rs
// occt: Contap_SequenceOfSegmentOfTheSearch

//! Deprecated typedef alias for backward compatibility.
//! This was `NCollection_Sequence<Contap_TheSegmentOfTheSearch>` in OCCT.
//!
//! In Rust, we model this as a newtype over `Vec` for sequence semantics.

use std::ops::{Deref, DerefMut};

/// Contap_TheSegmentOfTheSearch placeholder for test purposes.
#[derive(Clone, Debug, PartialEq)]
pub struct ContapTheSegmentOfTheSearch {
    id: usize,
}

impl ContapTheSegmentOfTheSearch {
    pub fn new(id: usize) -> Self {
        ContapTheSegmentOfTheSearch { id }
    }
}

/// A sequence (ordered collection) of Contap_TheSegmentOfTheSearch items.
/// Models `NCollection_Sequence<Contap_TheSegmentOfTheSearch>` from OCCT.
#[derive(Clone, Debug)]
pub struct ContapSequenceOfSegmentOfTheSearch {
    items: Vec<ContapTheSegmentOfTheSearch>,
}

impl ContapSequenceOfSegmentOfTheSearch {
    /// Create an empty sequence.
    pub fn new() -> Self {
        ContapSequenceOfSegmentOfTheSearch {
            items: Vec::new(),
        }
    }

    /// Append an item to the end of the sequence.
    pub fn append(&mut self, item: ContapTheSegmentOfTheSearch) {
        self.items.push(item);
    }

    /// Remove and return the item at the given index (1-based for OCCT compatibility).
    pub fn remove(&mut self, index: usize) -> Option<ContapTheSegmentOfTheSearch> {
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
    pub fn value(&self, index: usize) -> Option<&ContapTheSegmentOfTheSearch> {
        if index > 0 && index <= self.items.len() {
            Some(&self.items[index - 1])
        } else {
            None
        }
    }

    /// Get a mutable reference to an item at the given 1-based index.
    pub fn value_mut(&mut self, index: usize) -> Option<&mut ContapTheSegmentOfTheSearch> {
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
    pub fn insert_at(&mut self, index: usize, item: ContapTheSegmentOfTheSearch) {
        if index > 0 && index <= self.items.len() + 1 {
            self.items.insert(index - 1, item);
        }
    }
}

impl Default for ContapSequenceOfSegmentOfTheSearch {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ContapSequenceOfSegmentOfTheSearch {
    type Target = Vec<ContapTheSegmentOfTheSearch>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for ContapSequenceOfSegmentOfTheSearch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = ContapSequenceOfSegmentOfTheSearch::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_append_and_length() {
        let mut seq = ContapSequenceOfSegmentOfTheSearch::new();
        seq.append(ContapTheSegmentOfTheSearch::new(1));
        seq.append(ContapTheSegmentOfTheSearch::new(2));
        seq.append(ContapTheSegmentOfTheSearch::new(3));
        assert_eq!(seq.length(), 3);
    }

    #[test]
    fn test_sequence_value_1_based_indexing() {
        let mut seq = ContapSequenceOfSegmentOfTheSearch::new();
        let seg1 = ContapTheSegmentOfTheSearch::new(100);
        let seg2 = ContapTheSegmentOfTheSearch::new(200);
        seq.append(seg1.clone());
        seq.append(seg2.clone());

        assert_eq!(seq.value(1), Some(&seg1));
        assert_eq!(seq.value(2), Some(&seg2));
        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(3), None);
    }

    #[test]
    fn test_sequence_remove() {
        let mut seq = ContapSequenceOfSegmentOfTheSearch::new();
        seq.append(ContapTheSegmentOfTheSearch::new(10));
        seq.append(ContapTheSegmentOfTheSearch::new(20));
        seq.append(ContapTheSegmentOfTheSearch::new(30));

        let removed = seq.remove(2);
        assert_eq!(removed, Some(ContapTheSegmentOfTheSearch::new(20)));
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(2), Some(&ContapTheSegmentOfTheSearch::new(30)));
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = ContapSequenceOfSegmentOfTheSearch::new();
        seq.append(ContapTheSegmentOfTheSearch::new(5));
        seq.append(ContapTheSegmentOfTheSearch::new(7));
        assert_eq!(seq.length(), 2);

        seq.clear();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_insert_at() {
        let mut seq = ContapSequenceOfSegmentOfTheSearch::new();
        seq.append(ContapTheSegmentOfTheSearch::new(1));
        seq.append(ContapTheSegmentOfTheSearch::new(3));

        seq.insert_at(2, ContapTheSegmentOfTheSearch::new(2));
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.value(2), Some(&ContapTheSegmentOfTheSearch::new(2)));
    }
}

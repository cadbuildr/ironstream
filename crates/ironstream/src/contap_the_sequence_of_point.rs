// FILE: contap_the_sequence_of_point.rs
// occt: Contap_TheSequenceOfPoint

//! Deprecated typedef alias for backward compatibility.
//! This was `NCollection_Sequence<Contap_Point>` in OCCT.
//!
//! In Rust, we model this as a newtype over `Vec` for sequence semantics.

use std::ops::{Deref, DerefMut};

/// Contap_Point placeholder for test purposes.
#[derive(Clone, Debug, PartialEq)]
pub struct ContapPoint {
    x: f64,
    y: f64,
    z: f64,
}

impl ContapPoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        ContapPoint { x, y, z }
    }
}

/// A sequence (ordered collection) of Contap_Point items.
/// Models `NCollection_Sequence<Contap_Point>` from OCCT.
#[derive(Clone, Debug)]
pub struct ContapTheSequenceOfPoint {
    items: Vec<ContapPoint>,
}

impl ContapTheSequenceOfPoint {
    /// Create an empty sequence.
    pub fn new() -> Self {
        ContapTheSequenceOfPoint {
            items: Vec::new(),
        }
    }

    /// Append an item to the end of the sequence.
    pub fn append(&mut self, item: ContapPoint) {
        self.items.push(item);
    }

    /// Remove and return the item at the given index (1-based for OCCT compatibility).
    pub fn remove(&mut self, index: usize) -> Option<ContapPoint> {
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
    pub fn value(&self, index: usize) -> Option<&ContapPoint> {
        if index > 0 && index <= self.items.len() {
            Some(&self.items[index - 1])
        } else {
            None
        }
    }

    /// Get a mutable reference to an item at the given 1-based index.
    pub fn value_mut(&mut self, index: usize) -> Option<&mut ContapPoint> {
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
    pub fn insert_at(&mut self, index: usize, item: ContapPoint) {
        if index > 0 && index <= self.items.len() + 1 {
            self.items.insert(index - 1, item);
        }
    }
}

impl Default for ContapTheSequenceOfPoint {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ContapTheSequenceOfPoint {
    type Target = Vec<ContapPoint>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for ContapTheSequenceOfPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = ContapTheSequenceOfPoint::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_append_and_length() {
        let mut seq = ContapTheSequenceOfPoint::new();
        seq.append(ContapPoint::new(0.0, 0.0, 0.0));
        seq.append(ContapPoint::new(1.0, 1.0, 1.0));
        seq.append(ContapPoint::new(2.0, 2.0, 2.0));
        assert_eq!(seq.length(), 3);
    }

    #[test]
    fn test_sequence_value_1_based_indexing() {
        let mut seq = ContapTheSequenceOfPoint::new();
        let pt1 = ContapPoint::new(10.0, 20.0, 30.0);
        let pt2 = ContapPoint::new(40.0, 50.0, 60.0);
        seq.append(pt1.clone());
        seq.append(pt2.clone());

        assert_eq!(seq.value(1), Some(&pt1));
        assert_eq!(seq.value(2), Some(&pt2));
        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(3), None);
    }

    #[test]
    fn test_sequence_remove() {
        let mut seq = ContapTheSequenceOfPoint::new();
        seq.append(ContapPoint::new(1.0, 1.0, 1.0));
        seq.append(ContapPoint::new(2.0, 2.0, 2.0));
        seq.append(ContapPoint::new(3.0, 3.0, 3.0));

        let removed = seq.remove(2);
        assert_eq!(removed, Some(ContapPoint::new(2.0, 2.0, 2.0)));
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(2), Some(&ContapPoint::new(3.0, 3.0, 3.0)));
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = ContapTheSequenceOfPoint::new();
        seq.append(ContapPoint::new(0.1, 0.2, 0.3));
        seq.append(ContapPoint::new(0.4, 0.5, 0.6));
        assert_eq!(seq.length(), 2);

        seq.clear();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_insert_at() {
        let mut seq = ContapTheSequenceOfPoint::new();
        seq.append(ContapPoint::new(1.0, 0.0, 0.0));
        seq.append(ContapPoint::new(3.0, 0.0, 0.0));

        seq.insert_at(2, ContapPoint::new(2.0, 0.0, 0.0));
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.value(2), Some(&ContapPoint::new(2.0, 0.0, 0.0)));
    }
}

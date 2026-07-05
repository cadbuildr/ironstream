// FILE: contap_the_sequence_of_line.rs
// occt: Contap_TheSequenceOfLine

//! Deprecated typedef alias for backward compatibility.
//! This was `NCollection_Sequence<Contap_Line>` in OCCT.
//!
//! In Rust, we model this as a newtype over `Vec` for sequence semantics.

use std::ops::{Deref, DerefMut};

/// Contap_Line placeholder for test purposes.
#[derive(Clone, Debug, PartialEq)]
pub struct ContapLine {
    index: u32,
}

impl ContapLine {
    pub fn new(index: u32) -> Self {
        ContapLine { index }
    }
}

/// A sequence (ordered collection) of Contap_Line items.
/// Models `NCollection_Sequence<Contap_Line>` from OCCT.
#[derive(Clone, Debug)]
pub struct ContapTheSequenceOfLine {
    items: Vec<ContapLine>,
}

impl ContapTheSequenceOfLine {
    /// Create an empty sequence.
    pub fn new() -> Self {
        ContapTheSequenceOfLine {
            items: Vec::new(),
        }
    }

    /// Append an item to the end of the sequence.
    pub fn append(&mut self, item: ContapLine) {
        self.items.push(item);
    }

    /// Remove and return the item at the given index (1-based for OCCT compatibility).
    pub fn remove(&mut self, index: usize) -> Option<ContapLine> {
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
    pub fn value(&self, index: usize) -> Option<&ContapLine> {
        if index > 0 && index <= self.items.len() {
            Some(&self.items[index - 1])
        } else {
            None
        }
    }

    /// Get a mutable reference to an item at the given 1-based index.
    pub fn value_mut(&mut self, index: usize) -> Option<&mut ContapLine> {
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
    pub fn insert_at(&mut self, index: usize, item: ContapLine) {
        if index > 0 && index <= self.items.len() + 1 {
            self.items.insert(index - 1, item);
        }
    }
}

impl Default for ContapTheSequenceOfLine {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ContapTheSequenceOfLine {
    type Target = Vec<ContapLine>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for ContapTheSequenceOfLine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = ContapTheSequenceOfLine::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_append_and_length() {
        let mut seq = ContapTheSequenceOfLine::new();
        seq.append(ContapLine::new(1));
        seq.append(ContapLine::new(2));
        seq.append(ContapLine::new(3));
        assert_eq!(seq.length(), 3);
    }

    #[test]
    fn test_sequence_value_1_based_indexing() {
        let mut seq = ContapTheSequenceOfLine::new();
        let line1 = ContapLine::new(10);
        let line2 = ContapLine::new(20);
        seq.append(line1.clone());
        seq.append(line2.clone());

        assert_eq!(seq.value(1), Some(&line1));
        assert_eq!(seq.value(2), Some(&line2));
        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(3), None);
    }

    #[test]
    fn test_sequence_remove() {
        let mut seq = ContapTheSequenceOfLine::new();
        seq.append(ContapLine::new(100));
        seq.append(ContapLine::new(200));
        seq.append(ContapLine::new(300));

        let removed = seq.remove(2);
        assert_eq!(removed, Some(ContapLine::new(200)));
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(2), Some(&ContapLine::new(300)));
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = ContapTheSequenceOfLine::new();
        seq.append(ContapLine::new(50));
        seq.append(ContapLine::new(75));
        assert_eq!(seq.length(), 2);

        seq.clear();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_insert_at() {
        let mut seq = ContapTheSequenceOfLine::new();
        seq.append(ContapLine::new(1));
        seq.append(ContapLine::new(3));

        seq.insert_at(2, ContapLine::new(2));
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.value(2), Some(&ContapLine::new(2)));
    }
}

// FILE: int_surf_sequence_of_couple.rs
// occt: IntSurf_SequenceOfCouple

use std::vec::Vec;

/// Deprecated alias for a sequence of couples in surface intersection.
#[derive(Clone, Debug)]
pub struct IntSurf_SequenceOfCouple {
    couples: Vec<(u32, u32)>,
}

impl IntSurf_SequenceOfCouple {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntSurf_SequenceOfCouple {
            couples: Vec::new(),
        }
    }

    /// Add a couple to the sequence.
    pub fn append(&mut self, a: u32, b: u32) {
        self.couples.push((a, b));
    }

    /// Get the number of couples.
    pub fn length(&self) -> usize {
        self.couples.len()
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.couples.is_empty()
    }

    /// Get a couple by index.
    pub fn couple(&self, index: usize) -> Option<(u32, u32)> {
        self.couples.get(index).copied()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.couples.clear();
    }
}

impl Default for IntSurf_SequenceOfCouple {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntSurf_SequenceOfCouple::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = IntSurf_SequenceOfCouple::new();
        seq.append(1, 2);
        assert_eq!(seq.length(), 1);
        assert_eq!(seq.couple(0), Some((1, 2)));
    }

    #[test]
    fn test_clear() {
        let mut seq = IntSurf_SequenceOfCouple::new();
        seq.append(1, 2);
        seq.clear();
        assert!(seq.is_empty());
    }
}

// FILE: bopds_vector_of_vector_of_pair.rs
// occt: BOPDS_VectorOfVectorOfPair

//! Deprecated typedef for backward compatibility.
//! BOPDS_VectorOfVectorOfPair is a 2D vector of pairs, typically used in
//! Boolean operation data structures. This is a container of BOPDS_VectorOfPair.

use std::vec::Vec;

/// Represents a deprecated 2D vector of pairs in BOPDS (Boolean Operation Data Structure).
/// This is a simple wrapper around Vec<Vec<(u32, u32)>> for backward compatibility.
pub struct BopdsVectorOfVectorOfPair {
    data: Vec<Vec<(u32, u32)>>,
}

impl BopdsVectorOfVectorOfPair {
    /// Creates a new empty 2D vector of pairs.
    pub fn new() -> Self {
        BopdsVectorOfVectorOfPair {
            data: Vec::new(),
        }
    }

    /// Returns the number of outer vectors.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the container is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a reference to the inner vector at the given index.
    pub fn get(&self, index: usize) -> Option<&Vec<(u32, u32)>> {
        self.data.get(index)
    }

    /// Returns a mutable reference to the inner vector at the given index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Vec<(u32, u32)>> {
        self.data.get_mut(index)
    }

    /// Adds a new inner vector.
    pub fn push(&mut self, inner: Vec<(u32, u32)>) {
        self.data.push(inner);
    }

    /// Clears all data.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for BopdsVectorOfVectorOfPair {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = BopdsVectorOfVectorOfPair::new();
        assert!(v.is_empty());
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_push_and_len() {
        let mut v = BopdsVectorOfVectorOfPair::new();
        v.push(vec![(1, 2), (3, 4)]);
        assert_eq!(v.len(), 1);
        assert!(!v.is_empty());
    }

    #[test]
    fn test_get() {
        let mut v = BopdsVectorOfVectorOfPair::new();
        v.push(vec![(1, 2), (3, 4)]);
        let inner = v.get(0).unwrap();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0], (1, 2));
    }

    #[test]
    fn test_clear() {
        let mut v = BopdsVectorOfVectorOfPair::new();
        v.push(vec![(1, 2)]);
        v.clear();
        assert!(v.is_empty());
    }
}

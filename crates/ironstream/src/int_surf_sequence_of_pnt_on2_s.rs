// FILE: int_surf_sequence_of_pnt_on2_s.rs
// occt: IntSurf_SequenceOfPntOn2S

use std::vec::Vec;

/// Deprecated alias for a sequence of points on two surfaces.
#[derive(Clone, Debug)]
pub struct IntSurf_SequenceOfPntOn2S {
    points: Vec<([f64; 3], [f64; 3])>,
}

impl IntSurf_SequenceOfPntOn2S {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntSurf_SequenceOfPntOn2S {
            points: Vec::new(),
        }
    }

    /// Add a point pair to the sequence.
    pub fn append(&mut self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) {
        self.points.push(([x1, y1, z1], [x2, y2, z2]));
    }

    /// Get the number of point pairs.
    pub fn length(&self) -> usize {
        self.points.len()
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get a point pair by index.
    pub fn point(&self, index: usize) -> Option<([f64; 3], [f64; 3])> {
        self.points.get(index).copied()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.points.clear();
    }
}

impl Default for IntSurf_SequenceOfPntOn2S {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntSurf_SequenceOfPntOn2S::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = IntSurf_SequenceOfPntOn2S::new();
        seq.append(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        assert_eq!(seq.length(), 1);
    }

    #[test]
    fn test_clear() {
        let mut seq = IntSurf_SequenceOfPntOn2S::new();
        seq.append(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        seq.clear();
        assert!(seq.is_empty());
    }
}

// FILE: int_surf_sequence_of_interior_point.rs
// occt: IntSurf_SequenceOfInteriorPoint

use std::vec::Vec;

/// Deprecated alias for a sequence of interior points in surface intersection.
#[derive(Clone, Debug)]
pub struct IntSurf_SequenceOfInteriorPoint {
    points: Vec<[f64; 3]>,
}

impl IntSurf_SequenceOfInteriorPoint {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntSurf_SequenceOfInteriorPoint {
            points: Vec::new(),
        }
    }

    /// Add a point to the sequence.
    pub fn append(&mut self, x: f64, y: f64, z: f64) {
        self.points.push([x, y, z]);
    }

    /// Get the number of points.
    pub fn length(&self) -> usize {
        self.points.len()
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get a point by index.
    pub fn point(&self, index: usize) -> Option<[f64; 3]> {
        self.points.get(index).copied()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.points.clear();
    }
}

impl Default for IntSurf_SequenceOfInteriorPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntSurf_SequenceOfInteriorPoint::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = IntSurf_SequenceOfInteriorPoint::new();
        seq.append(1.5, 2.5, 3.5);
        assert_eq!(seq.length(), 1);
        assert_eq!(seq.point(0), Some([1.5, 2.5, 3.5]));
    }

    #[test]
    fn test_clear() {
        let mut seq = IntSurf_SequenceOfInteriorPoint::new();
        seq.append(1.0, 2.0, 3.0);
        seq.clear();
        assert!(seq.is_empty());
    }
}

// FILE: int_res2d_sequence_of_intersection_point.rs
// occt: IntRes2d_SequenceOfIntersectionPoint

use std::vec::Vec;

/// Deprecated alias for a sequence of 2D intersection points.
#[derive(Clone, Debug)]
pub struct IntRes2d_SequenceOfIntersectionPoint {
    points: Vec<[f64; 2]>,
}

impl IntRes2d_SequenceOfIntersectionPoint {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntRes2d_SequenceOfIntersectionPoint {
            points: Vec::new(),
        }
    }

    /// Add a point to the sequence.
    pub fn append(&mut self, x: f64, y: f64) {
        self.points.push([x, y]);
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
    pub fn point(&self, index: usize) -> Option<[f64; 2]> {
        self.points.get(index).copied()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.points.clear();
    }
}

impl Default for IntRes2d_SequenceOfIntersectionPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntRes2d_SequenceOfIntersectionPoint::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = IntRes2d_SequenceOfIntersectionPoint::new();
        seq.append(1.0, 2.0);
        seq.append(3.0, 4.0);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.point(0), Some([1.0, 2.0]));
    }

    #[test]
    fn test_clear() {
        let mut seq = IntRes2d_SequenceOfIntersectionPoint::new();
        seq.append(1.0, 2.0);
        seq.clear();
        assert!(seq.is_empty());
    }
}

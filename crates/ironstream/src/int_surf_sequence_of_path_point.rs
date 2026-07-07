// FILE: int_surf_sequence_of_path_point.rs
// occt: IntSurf_SequenceOfPathPoint

use std::vec::Vec;

/// Deprecated alias for a sequence of path points in surface intersection.
#[derive(Clone, Debug)]
pub struct IntSurf_SequenceOfPathPoint {
    points: Vec<u32>,
}

impl IntSurf_SequenceOfPathPoint {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntSurf_SequenceOfPathPoint {
            points: Vec::new(),
        }
    }

    /// Add a point to the sequence.
    pub fn append(&mut self, point_id: u32) {
        self.points.push(point_id);
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
    pub fn point(&self, index: usize) -> Option<u32> {
        self.points.get(index).copied()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.points.clear();
    }
}

impl Default for IntSurf_SequenceOfPathPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntSurf_SequenceOfPathPoint::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = IntSurf_SequenceOfPathPoint::new();
        seq.append(1);
        assert_eq!(seq.length(), 1);
        assert_eq!(seq.point(0), Some(1));
    }

    #[test]
    fn test_clear() {
        let mut seq = IntSurf_SequenceOfPathPoint::new();
        seq.append(1);
        seq.clear();
        assert!(seq.is_empty());
    }
}

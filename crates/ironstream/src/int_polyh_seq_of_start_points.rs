// FILE: int_polyh_seq_of_start_points.rs
// occt: IntPolyh_SeqOfStartPoints

use std::vec::Vec;

/// Deprecated alias for a sequence of start points in polyhedral intersection.
#[derive(Clone, Debug)]
pub struct IntPolyh_SeqOfStartPoints {
    points: Vec<[f64; 3]>,
}

impl IntPolyh_SeqOfStartPoints {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntPolyh_SeqOfStartPoints {
            points: Vec::new(),
        }
    }

    /// Add a start point to the sequence.
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

impl Default for IntPolyh_SeqOfStartPoints {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntPolyh_SeqOfStartPoints::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = IntPolyh_SeqOfStartPoints::new();
        seq.append(0.0, 0.0, 0.0);
        seq.append(1.0, 1.0, 1.0);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.point(0), Some([0.0, 0.0, 0.0]));
    }

    #[test]
    fn test_clear() {
        let mut seq = IntPolyh_SeqOfStartPoints::new();
        seq.append(1.0, 1.0, 1.0);
        seq.clear();
        assert!(seq.is_empty());
    }
}

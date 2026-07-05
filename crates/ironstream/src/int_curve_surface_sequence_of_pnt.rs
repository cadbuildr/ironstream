// FILE: int_curve_surface_sequence_of_pnt.rs
// occt: IntCurveSurface_SequenceOfPnt

use std::vec::Vec;

/// Deprecated alias for a sequence of points from curve-surface intersection.
#[derive(Clone, Debug)]
pub struct IntCurveSurface_SequenceOfPnt {
    points: Vec<[f64; 3]>,
}

impl IntCurveSurface_SequenceOfPnt {
    /// Create a new sequence of points.
    pub fn new() -> Self {
        IntCurveSurface_SequenceOfPnt {
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

impl Default for IntCurveSurface_SequenceOfPnt {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntCurveSurface_SequenceOfPnt::new();
        assert!(seq.is_empty());
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_append() {
        let mut seq = IntCurveSurface_SequenceOfPnt::new();
        seq.append(1.0, 2.0, 3.0);
        seq.append(4.0, 5.0, 6.0);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.point(0), Some([1.0, 2.0, 3.0]));
        assert_eq!(seq.point(1), Some([4.0, 5.0, 6.0]));
    }

    #[test]
    fn test_clear() {
        let mut seq = IntCurveSurface_SequenceOfPnt::new();
        seq.append(1.0, 2.0, 3.0);
        seq.clear();
        assert!(seq.is_empty());
    }
}

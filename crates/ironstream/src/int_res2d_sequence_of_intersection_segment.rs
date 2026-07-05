// FILE: int_res2d_sequence_of_intersection_segment.rs
// occt: IntRes2d_SequenceOfIntersectionSegment

use std::vec::Vec;

/// Deprecated alias for a sequence of 2D intersection segments.
#[derive(Clone, Debug)]
pub struct IntRes2d_SequenceOfIntersectionSegment {
    segments: Vec<([f64; 2], [f64; 2])>,
}

impl IntRes2d_SequenceOfIntersectionSegment {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntRes2d_SequenceOfIntersectionSegment {
            segments: Vec::new(),
        }
    }

    /// Add a segment to the sequence.
    pub fn append(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.segments.push(([x1, y1], [x2, y2]));
    }

    /// Get the number of segments.
    pub fn length(&self) -> usize {
        self.segments.len()
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Get a segment by index.
    pub fn segment(&self, index: usize) -> Option<([f64; 2], [f64; 2])> {
        self.segments.get(index).copied()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.segments.clear();
    }
}

impl Default for IntRes2d_SequenceOfIntersectionSegment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntRes2d_SequenceOfIntersectionSegment::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = IntRes2d_SequenceOfIntersectionSegment::new();
        seq.append(0.0, 0.0, 1.0, 1.0);
        seq.append(2.0, 2.0, 3.0, 3.0);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.segment(0), Some(([0.0, 0.0], [1.0, 1.0])));
    }

    #[test]
    fn test_clear() {
        let mut seq = IntRes2d_SequenceOfIntersectionSegment::new();
        seq.append(0.0, 0.0, 1.0, 1.0);
        seq.clear();
        assert!(seq.is_empty());
    }
}

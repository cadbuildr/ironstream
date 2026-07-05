// FILE: int_curve_surface_sequence_of_seg.rs
// occt: IntCurveSurface_SequenceOfSeg

use std::vec::Vec;

/// Deprecated alias for a sequence of segments from curve-surface intersection.
#[derive(Clone, Debug)]
pub struct IntCurveSurface_SequenceOfSeg {
    segments: Vec<(i32, i32)>,
}

impl IntCurveSurface_SequenceOfSeg {
    /// Create a new sequence of segments.
    pub fn new() -> Self {
        IntCurveSurface_SequenceOfSeg {
            segments: Vec::new(),
        }
    }

    /// Add a segment to the sequence.
    pub fn append(&mut self, start: i32, end: i32) {
        self.segments.push((start, end));
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
    pub fn segment(&self, index: usize) -> Option<(i32, i32)> {
        self.segments.get(index).copied()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.segments.clear();
    }
}

impl Default for IntCurveSurface_SequenceOfSeg {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntCurveSurface_SequenceOfSeg::new();
        assert!(seq.is_empty());
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_append() {
        let mut seq = IntCurveSurface_SequenceOfSeg::new();
        seq.append(1, 2);
        seq.append(3, 4);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.segment(0), Some((1, 2)));
        assert_eq!(seq.segment(1), Some((3, 4)));
    }

    #[test]
    fn test_clear() {
        let mut seq = IntCurveSurface_SequenceOfSeg::new();
        seq.append(1, 2);
        seq.clear();
        assert!(seq.is_empty());
    }
}

// FILE: int_patch_sequence_of_segment_of_the_s_on_bounds.rs
// occt: IntPatch_SequenceOfSegmentOfTheSOnBounds

use std::vec::Vec;

/// Deprecated alias for a sequence of segments on surface boundaries.
#[derive(Clone, Debug)]
pub struct IntPatch_SequenceOfSegmentOfTheSOnBounds {
    segments: Vec<(u32, u32)>,
}

impl IntPatch_SequenceOfSegmentOfTheSOnBounds {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntPatch_SequenceOfSegmentOfTheSOnBounds {
            segments: Vec::new(),
        }
    }

    /// Add a segment to the sequence.
    pub fn append(&mut self, start: u32, end: u32) {
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
    pub fn segment(&self, index: usize) -> Option<(u32, u32)> {
        self.segments.get(index).copied()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.segments.clear();
    }
}

impl Default for IntPatch_SequenceOfSegmentOfTheSOnBounds {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntPatch_SequenceOfSegmentOfTheSOnBounds::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = IntPatch_SequenceOfSegmentOfTheSOnBounds::new();
        seq.append(1, 2);
        seq.append(3, 4);
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn test_clear() {
        let mut seq = IntPatch_SequenceOfSegmentOfTheSOnBounds::new();
        seq.append(1, 2);
        seq.clear();
        assert!(seq.is_empty());
    }
}

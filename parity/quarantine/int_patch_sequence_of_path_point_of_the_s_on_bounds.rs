// FILE: int_patch_sequence_of_path_point_of_the_s_on_bounds.rs
// occt: IntPatch_SequenceOfPathPointOfTheSOnBounds

use std::vec::Vec;

/// Deprecated alias for a sequence of path points on surface boundaries.
#[derive(Clone, Debug)]
pub struct IntPatch_SequenceOfPathPointOfTheSOnBounds {
    points: Vec<u32>,
}

impl IntPatch_SequenceOfPathPointOfTheSOnBounds {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntPatch_SequenceOfPathPointOfTheSOnBounds {
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

impl Default for IntPatch_SequenceOfPathPointOfTheSOnBounds {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntPatch_SequenceOfPathPointOfTheSOnBounds::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = IntPatch_SequenceOfPathPointOfTheSOnBounds::new();
        seq.append(1);
        seq.append(2);
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn test_clear() {
        let mut seq = IntPatch_SequenceOfPathPointOfTheSOnBounds::new();
        seq.append(1);
        seq.clear();
        assert!(seq.is_empty());
    }
}

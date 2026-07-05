// FILE: int_patch_sequence_of_iw_line_of_the_i_walking.rs
// occt: IntPatch_SequenceOfIWLineOfTheIWalking

use std::vec::Vec;

/// Deprecated alias for a sequence of intersection walking lines.
#[derive(Clone, Debug)]
pub struct IntPatch_SequenceOfIWLineOfTheIWalking {
    lines: Vec<u32>,
}

impl IntPatch_SequenceOfIWLineOfTheIWalking {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntPatch_SequenceOfIWLineOfTheIWalking {
            lines: Vec::new(),
        }
    }

    /// Add a line to the sequence.
    pub fn append(&mut self, line_id: u32) {
        self.lines.push(line_id);
    }

    /// Get the number of lines.
    pub fn length(&self) -> usize {
        self.lines.len()
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Get a line by index.
    pub fn line(&self, index: usize) -> Option<u32> {
        self.lines.get(index).copied()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.lines.clear();
    }
}

impl Default for IntPatch_SequenceOfIWLineOfTheIWalking {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntPatch_SequenceOfIWLineOfTheIWalking::new();
        assert!(seq.is_empty());
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_append() {
        let mut seq = IntPatch_SequenceOfIWLineOfTheIWalking::new();
        seq.append(1);
        seq.append(2);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.line(0), Some(1));
        assert_eq!(seq.line(1), Some(2));
    }

    #[test]
    fn test_clear() {
        let mut seq = IntPatch_SequenceOfIWLineOfTheIWalking::new();
        seq.append(1);
        seq.clear();
        assert!(seq.is_empty());
    }
}

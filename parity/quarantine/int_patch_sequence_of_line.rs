// FILE: int_patch_sequence_of_line.rs
// occt: IntPatch_SequenceOfLine

use std::vec::Vec;

/// Deprecated alias for a sequence of intersection lines.
#[derive(Clone, Debug)]
pub struct IntPatch_SequenceOfLine {
    lines: Vec<u32>,
}

impl IntPatch_SequenceOfLine {
    /// Create a new sequence.
    pub fn new() -> Self {
        IntPatch_SequenceOfLine {
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

impl Default for IntPatch_SequenceOfLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IntPatch_SequenceOfLine::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = IntPatch_SequenceOfLine::new();
        seq.append(10);
        seq.append(20);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.line(0), Some(10));
    }

    #[test]
    fn test_clear() {
        let mut seq = IntPatch_SequenceOfLine::new();
        seq.append(5);
        seq.clear();
        assert!(seq.is_empty());
    }
}

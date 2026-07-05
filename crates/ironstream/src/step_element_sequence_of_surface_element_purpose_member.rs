// FILE: step_element_sequence_of_surface_element_purpose_member.rs
// occt: StepElement_SequenceOfSurfaceElementPurposeMember

/// Rust port of OCCT's deprecated StepElement_SequenceOfSurfaceElementPurposeMember.
/// Sequence is a dynamic sequence wrapping a Vec.
#[derive(Clone, Debug)]
pub struct StepElementSequenceOfSurfaceElementPurposeMember {
    data: Vec<String>,
}

impl StepElementSequenceOfSurfaceElementPurposeMember {
    /// Create a new empty Sequence.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    /// Get the length of the sequence.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Append a value to the sequence.
    pub fn append(&mut self, val: String) {
        self.data.push(val);
    }

    /// Get a reference to the value at the given 1-based index.
    pub fn value(&self, idx: i32) -> Option<&String> {
        if idx < 1 {
            return None;
        }
        self.data.get((idx - 1) as usize)
    }

    /// Get a mutable reference to the value at the given 1-based index.
    pub fn value_mut(&mut self, idx: i32) -> Option<&mut String> {
        if idx < 1 {
            return None;
        }
        self.data.get_mut((idx - 1) as usize)
    }

    /// Set the value at the given 1-based index.
    pub fn set_value(&mut self, idx: i32, val: String) -> bool {
        if let Some(r) = self.value_mut(idx) {
            *r = val;
            true
        } else {
            false
        }
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for StepElementSequenceOfSurfaceElementPurposeMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = StepElementSequenceOfSurfaceElementPurposeMember::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = StepElementSequenceOfSurfaceElementPurposeMember::new();
        seq.append("surf_mem1".to_string());
        seq.append("surf_mem2".to_string());

        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(1), Some(&"surf_mem1".to_string()));
        assert_eq!(seq.value(2), Some(&"surf_mem2".to_string()));
    }

    #[test]
    fn test_sequence_set_value() {
        let mut seq = StepElementSequenceOfSurfaceElementPurposeMember::new();
        seq.append("initial".to_string());
        seq.set_value(1, "replaced".to_string());

        assert_eq!(seq.value(1), Some(&"replaced".to_string()));
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = StepElementSequenceOfSurfaceElementPurposeMember::new();
        seq.append("sm1".to_string());
        seq.append("sm2".to_string());
        seq.clear();

        assert_eq!(seq.length(), 0);
    }
}

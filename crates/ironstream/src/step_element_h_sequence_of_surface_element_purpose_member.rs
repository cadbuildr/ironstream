// FILE: step_element_h_sequence_of_surface_element_purpose_member.rs
// occt: StepElement_HSequenceOfSurfaceElementPurposeMember

/// Rust port of OCCT's deprecated StepElement_HSequenceOfSurfaceElementPurposeMember.
/// HSequence is a heap-allocated dynamic sequence wrapping a Vec.
#[derive(Clone, Debug)]
pub struct StepElementHSequenceOfSurfaceElementPurposeMember {
    data: Vec<String>,
}

impl StepElementHSequenceOfSurfaceElementPurposeMember {
    /// Create a new empty HSequence.
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

impl Default for StepElementHSequenceOfSurfaceElementPurposeMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsequence_creation() {
        let seq = StepElementHSequenceOfSurfaceElementPurposeMember::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_hsequence_append() {
        let mut seq = StepElementHSequenceOfSurfaceElementPurposeMember::new();
        seq.append("surf_mem1".to_string());
        seq.append("surf_mem2".to_string());

        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(1), Some(&"surf_mem1".to_string()));
        assert_eq!(seq.value(2), Some(&"surf_mem2".to_string()));
    }

    #[test]
    fn test_hsequence_set_value() {
        let mut seq = StepElementHSequenceOfSurfaceElementPurposeMember::new();
        seq.append("prev".to_string());
        seq.set_value(1, "current".to_string());

        assert_eq!(seq.value(1), Some(&"current".to_string()));
    }

    #[test]
    fn test_hsequence_clear() {
        let mut seq = StepElementHSequenceOfSurfaceElementPurposeMember::new();
        seq.append("s1".to_string());
        seq.append("s2".to_string());
        seq.clear();

        assert_eq!(seq.length(), 0);
    }
}

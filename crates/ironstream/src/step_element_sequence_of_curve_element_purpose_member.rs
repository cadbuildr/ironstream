// FILE: step_element_sequence_of_curve_element_purpose_member.rs
// occt: StepElement_SequenceOfCurveElementPurposeMember

/// Rust port of OCCT's deprecated StepElement_SequenceOfCurveElementPurposeMember.
/// Sequence is a dynamic sequence wrapping a Vec.
#[derive(Clone, Debug)]
pub struct StepElementSequenceOfCurveElementPurposeMember {
    data: Vec<String>,
}

impl StepElementSequenceOfCurveElementPurposeMember {
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

impl Default for StepElementSequenceOfCurveElementPurposeMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = StepElementSequenceOfCurveElementPurposeMember::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = StepElementSequenceOfCurveElementPurposeMember::new();
        seq.append("first".to_string());
        seq.append("second".to_string());

        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(1), Some(&"first".to_string()));
        assert_eq!(seq.value(2), Some(&"second".to_string()));
    }

    #[test]
    fn test_sequence_set_value() {
        let mut seq = StepElementSequenceOfCurveElementPurposeMember::new();
        seq.append("before".to_string());
        seq.set_value(1, "after".to_string());

        assert_eq!(seq.value(1), Some(&"after".to_string()));
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = StepElementSequenceOfCurveElementPurposeMember::new();
        seq.append("elem".to_string());
        seq.clear();

        assert_eq!(seq.length(), 0);
    }
}

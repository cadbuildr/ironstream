// FILE: step_element_sequence_of_element_material.rs
// occt: StepElement_SequenceOfElementMaterial

/// Rust port of OCCT's deprecated StepElement_SequenceOfElementMaterial.
/// Sequence is a dynamic sequence wrapping a Vec.
#[derive(Clone, Debug)]
pub struct StepElementSequenceOfElementMaterial {
    data: Vec<String>,
}

impl StepElementSequenceOfElementMaterial {
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

impl Default for StepElementSequenceOfElementMaterial {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = StepElementSequenceOfElementMaterial::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = StepElementSequenceOfElementMaterial::new();
        seq.append("steel".to_string());
        seq.append("aluminum".to_string());

        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(1), Some(&"steel".to_string()));
        assert_eq!(seq.value(2), Some(&"aluminum".to_string()));
    }

    #[test]
    fn test_sequence_set_value() {
        let mut seq = StepElementSequenceOfElementMaterial::new();
        seq.append("material1".to_string());
        seq.set_value(1, "material2".to_string());

        assert_eq!(seq.value(1), Some(&"material2".to_string()));
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = StepElementSequenceOfElementMaterial::new();
        seq.append("mat1".to_string());
        seq.append("mat2".to_string());
        seq.clear();

        assert_eq!(seq.length(), 0);
    }
}

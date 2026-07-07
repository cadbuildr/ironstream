// FILE: hatch_sequence_of_parameter.rs
// occt: Hatch_SequenceOfParameter

/// Represents a hatching parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct HatchParameter {
    pub value: f64,
}

/// Sequence of Hatch_Parameter objects.
/// In OCCT, this was NCollection_Sequence<Hatch_Parameter>.
/// This Rust newtype wraps a Vec for faithful behavior.
pub struct HatchSequenceOfParameter {
    items: Vec<HatchParameter>,
}

impl HatchSequenceOfParameter {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        HatchSequenceOfParameter { items: Vec::new() }
    }

    /// Returns the length of the sequence.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Appends an element to the end of the sequence.
    pub fn append(&mut self, value: HatchParameter) {
        self.items.push(value);
    }

    /// Returns a reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value(&self, index: usize) -> Option<&HatchParameter> {
        if index > 0 && index <= self.items.len() {
            self.items.get(index - 1)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value_mut(&mut self, index: usize) -> Option<&mut HatchParameter> {
        if index > 0 && index <= self.items.len() {
            self.items.get_mut(index - 1)
        } else {
            None
        }
    }

    /// Clears all elements from the sequence.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns an iterator over the sequence.
    pub fn iter(&self) -> std::slice::Iter<HatchParameter> {
        self.items.iter()
    }
}

impl Default for HatchSequenceOfParameter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = HatchSequenceOfParameter::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_and_len() {
        let mut seq = HatchSequenceOfParameter::new();
        let param = HatchParameter { value: 0.5 };
        seq.append(param);
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_value_1indexed() {
        let mut seq = HatchSequenceOfParameter::new();
        let param = HatchParameter { value: 1.5 };
        seq.append(param.clone());

        // 1-indexed access
        assert_eq!(seq.value(1), Some(&param));
        // 0-indexed should return None
        assert_eq!(seq.value(0), None);
        // Out of bounds
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn test_value_mut() {
        let mut seq = HatchSequenceOfParameter::new();
        let param = HatchParameter { value: 2.0 };
        seq.append(param);

        if let Some(val) = seq.value_mut(1) {
            val.value = 3.0;
        }

        let retrieved = seq.value(1).unwrap();
        assert_eq!(retrieved.value, 3.0);
    }

    #[test]
    fn test_clear() {
        let mut seq = HatchSequenceOfParameter::new();
        seq.append(HatchParameter { value: 1.0 });
        seq.append(HatchParameter { value: 2.0 });

        assert_eq!(seq.len(), 2);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = HatchSequenceOfParameter::new();
        let param1 = HatchParameter { value: 0.5 };
        let param2 = HatchParameter { value: 1.5 };
        seq.append(param1.clone());
        seq.append(param2.clone());

        let mut iter = seq.iter();
        assert_eq!(iter.next(), Some(&param1));
        assert_eq!(iter.next(), Some(&param2));
        assert_eq!(iter.next(), None);
    }
}

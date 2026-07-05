// FILE: if_select_sequence_of_general_modifier.rs
// occt: IFSelect_SequenceOfGeneralModifier

/// Represents a general modifier entity.
#[derive(Clone, Debug)]
pub struct IFSelectGeneralModifier {
    pub id: usize,
}

/// Sequence of IFSelect_GeneralModifier objects.
/// In OCCT, this was NCollection_Sequence<opencascade::handle<IFSelect_GeneralModifier>>.
/// This Rust newtype wraps a Vec for faithful behavior.
pub struct IFSelectSequenceOfGeneralModifier {
    items: Vec<IFSelectGeneralModifier>,
}

impl IFSelectSequenceOfGeneralModifier {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        IFSelectSequenceOfGeneralModifier { items: Vec::new() }
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
    pub fn append(&mut self, value: IFSelectGeneralModifier) {
        self.items.push(value);
    }

    /// Returns a reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value(&self, index: usize) -> Option<&IFSelectGeneralModifier> {
        if index > 0 && index <= self.items.len() {
            self.items.get(index - 1)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value_mut(&mut self, index: usize) -> Option<&mut IFSelectGeneralModifier> {
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
    pub fn iter(&self) -> std::slice::Iter<IFSelectGeneralModifier> {
        self.items.iter()
    }
}

impl Default for IFSelectSequenceOfGeneralModifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IFSelectSequenceOfGeneralModifier::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_and_len() {
        let mut seq = IFSelectSequenceOfGeneralModifier::new();
        let mod1 = IFSelectGeneralModifier { id: 1 };
        seq.append(mod1);
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_value_1indexed() {
        let mut seq = IFSelectSequenceOfGeneralModifier::new();
        let mod1 = IFSelectGeneralModifier { id: 10 };
        seq.append(mod1.clone());

        // 1-indexed access
        assert_eq!(seq.value(1).unwrap().id, 10);
        // 0-indexed should return None
        assert_eq!(seq.value(0), None);
        // Out of bounds
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn test_value_mut() {
        let mut seq = IFSelectSequenceOfGeneralModifier::new();
        let mod1 = IFSelectGeneralModifier { id: 3 };
        seq.append(mod1);

        if let Some(val) = seq.value_mut(1) {
            val.id = 30;
        }

        let retrieved = seq.value(1).unwrap();
        assert_eq!(retrieved.id, 30);
    }

    #[test]
    fn test_clear() {
        let mut seq = IFSelectSequenceOfGeneralModifier::new();
        seq.append(IFSelectGeneralModifier { id: 1 });
        seq.append(IFSelectGeneralModifier { id: 2 });

        assert_eq!(seq.len(), 2);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = IFSelectSequenceOfGeneralModifier::new();
        let mod1 = IFSelectGeneralModifier { id: 1 };
        let mod2 = IFSelectGeneralModifier { id: 2 };
        seq.append(mod1.clone());
        seq.append(mod2.clone());

        let ids: Vec<usize> = seq.iter().map(|m| m.id).collect();
        assert_eq!(ids, vec![1, 2]);
    }
}

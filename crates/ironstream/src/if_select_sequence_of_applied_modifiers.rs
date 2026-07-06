// FILE: if_select_sequence_of_applied_modifiers.rs
// occt: IFSelect_SequenceOfAppliedModifiers

/// Represents applied modifiers on entities.
#[derive(Clone, Debug, PartialEq)]
pub struct IFSelectAppliedModifiers {
    pub id: usize,
}

/// Sequence of IFSelect_AppliedModifiers objects.
/// In OCCT, this was NCollection_Sequence<opencascade::handle<IFSelect_AppliedModifiers>>.
/// This Rust newtype wraps a Vec for faithful behavior.
pub struct IFSelectSequenceOfAppliedModifiers {
    items: Vec<IFSelectAppliedModifiers>,
}

impl IFSelectSequenceOfAppliedModifiers {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        IFSelectSequenceOfAppliedModifiers { items: Vec::new() }
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
    pub fn append(&mut self, value: IFSelectAppliedModifiers) {
        self.items.push(value);
    }

    /// Returns a reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value(&self, index: usize) -> Option<&IFSelectAppliedModifiers> {
        if index > 0 && index <= self.items.len() {
            self.items.get(index - 1)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value_mut(&mut self, index: usize) -> Option<&mut IFSelectAppliedModifiers> {
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
    pub fn iter(&self) -> std::slice::Iter<IFSelectAppliedModifiers> {
        self.items.iter()
    }
}

impl Default for IFSelectSequenceOfAppliedModifiers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IFSelectSequenceOfAppliedModifiers::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_and_len() {
        let mut seq = IFSelectSequenceOfAppliedModifiers::new();
        let mod1 = IFSelectAppliedModifiers { id: 1 };
        seq.append(mod1);
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_value_1indexed() {
        let mut seq = IFSelectSequenceOfAppliedModifiers::new();
        let mod1 = IFSelectAppliedModifiers { id: 5 };
        seq.append(mod1.clone());

        // 1-indexed access
        assert_eq!(seq.value(1).unwrap().id, 5);
        // 0-indexed should return None
        assert_eq!(seq.value(0), None);
        // Out of bounds
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn test_value_mut() {
        let mut seq = IFSelectSequenceOfAppliedModifiers::new();
        let mod1 = IFSelectAppliedModifiers { id: 7 };
        seq.append(mod1);

        if let Some(val) = seq.value_mut(1) {
            val.id = 20;
        }

        let retrieved = seq.value(1).unwrap();
        assert_eq!(retrieved.id, 20);
    }

    #[test]
    fn test_clear() {
        let mut seq = IFSelectSequenceOfAppliedModifiers::new();
        seq.append(IFSelectAppliedModifiers { id: 1 });
        seq.append(IFSelectAppliedModifiers { id: 2 });

        assert_eq!(seq.len(), 2);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = IFSelectSequenceOfAppliedModifiers::new();
        let mod1 = IFSelectAppliedModifiers { id: 1 };
        let mod2 = IFSelectAppliedModifiers { id: 2 };
        seq.append(mod1.clone());
        seq.append(mod2.clone());

        let ids: Vec<usize> = seq.iter().map(|m| m.id).collect();
        assert_eq!(ids, vec![1, 2]);
    }
}

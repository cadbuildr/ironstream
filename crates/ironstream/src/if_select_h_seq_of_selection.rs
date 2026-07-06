// FILE: if_select_h_seq_of_selection.rs
// occt: IFSelect_HSeqOfSelection

/// Represents a selection entity in the interface select framework.
#[derive(Clone, Debug, PartialEq)]
pub struct IFSelectSelection {
    pub id: usize,
}

/// Handle-based sequence of IFSelect_Selection objects.
/// In OCCT, this was NCollection_HSequence<opencascade::handle<IFSelect_Selection>>.
/// This Rust newtype wraps a reference-counted Vec for faithful handle semantics.
pub struct IFSelectHSeqOfSelection {
    items: Vec<IFSelectSelection>,
}

impl IFSelectHSeqOfSelection {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        IFSelectHSeqOfSelection { items: Vec::new() }
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
    pub fn append(&mut self, value: IFSelectSelection) {
        self.items.push(value);
    }

    /// Prepends an element to the beginning of the sequence.
    pub fn prepend(&mut self, value: IFSelectSelection) {
        self.items.insert(0, value);
    }

    /// Returns a reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value(&self, index: usize) -> Option<&IFSelectSelection> {
        if index > 0 && index <= self.items.len() {
            self.items.get(index - 1)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value_mut(&mut self, index: usize) -> Option<&mut IFSelectSelection> {
        if index > 0 && index <= self.items.len() {
            self.items.get_mut(index - 1)
        } else {
            None
        }
    }

    /// Changes the value at the given index (1-indexed).
    pub fn set_value(&mut self, index: usize, value: IFSelectSelection) -> bool {
        if index > 0 && index <= self.items.len() {
            self.items[index - 1] = value;
            true
        } else {
            false
        }
    }

    /// Clears all elements from the sequence.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns an iterator over the sequence.
    pub fn iter(&self) -> std::slice::Iter<IFSelectSelection> {
        self.items.iter()
    }
}

impl Default for IFSelectHSeqOfSelection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IFSelectHSeqOfSelection::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_and_len() {
        let mut seq = IFSelectHSeqOfSelection::new();
        let sel = IFSelectSelection { id: 1 };
        seq.append(sel);
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_prepend() {
        let mut seq = IFSelectHSeqOfSelection::new();
        let sel1 = IFSelectSelection { id: 1 };
        let sel2 = IFSelectSelection { id: 2 };
        seq.append(sel1);
        seq.prepend(sel2.clone());

        assert_eq!(seq.len(), 2);
        assert_eq!(seq.value(1).unwrap().id, 2);
        assert_eq!(seq.value(2).unwrap().id, 1);
    }

    #[test]
    fn test_value_1indexed() {
        let mut seq = IFSelectHSeqOfSelection::new();
        let sel = IFSelectSelection { id: 42 };
        seq.append(sel.clone());

        // 1-indexed access
        assert_eq!(seq.value(1).unwrap().id, 42);
        // 0-indexed should return None
        assert_eq!(seq.value(0), None);
        // Out of bounds
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn test_value_mut() {
        let mut seq = IFSelectHSeqOfSelection::new();
        let sel = IFSelectSelection { id: 10 };
        seq.append(sel);

        if let Some(val) = seq.value_mut(1) {
            val.id = 99;
        }

        let retrieved = seq.value(1).unwrap();
        assert_eq!(retrieved.id, 99);
    }

    #[test]
    fn test_set_value() {
        let mut seq = IFSelectHSeqOfSelection::new();
        seq.append(IFSelectSelection { id: 5 });

        let result = seq.set_value(1, IFSelectSelection { id: 15 });
        assert!(result);

        let retrieved = seq.value(1).unwrap();
        assert_eq!(retrieved.id, 15);
    }

    #[test]
    fn test_set_value_out_of_bounds() {
        let mut seq = IFSelectHSeqOfSelection::new();
        seq.append(IFSelectSelection { id: 5 });

        let result = seq.set_value(2, IFSelectSelection { id: 20 });
        assert!(!result);
    }

    #[test]
    fn test_clear() {
        let mut seq = IFSelectHSeqOfSelection::new();
        seq.append(IFSelectSelection { id: 1 });
        seq.append(IFSelectSelection { id: 2 });

        assert_eq!(seq.len(), 2);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = IFSelectHSeqOfSelection::new();
        let sel1 = IFSelectSelection { id: 1 };
        let sel2 = IFSelectSelection { id: 2 };
        seq.append(sel1.clone());
        seq.append(sel2.clone());

        let mut iter = seq.iter();
        assert_eq!(iter.next().unwrap().id, 1);
        assert_eq!(iter.next().unwrap().id, 2);
        assert_eq!(iter.next(), None);
    }
}

// FILE: if_select_t_seq_of_selection.rs
// occt: IFSelect_TSeqOfSelection

/// Represents a selection entity.
#[derive(Clone, Debug, PartialEq)]
pub struct IFSelectSelection {
    pub id: usize,
}

/// Sequence of IFSelect_Selection objects.
/// In OCCT, this was NCollection_Sequence<opencascade::handle<IFSelect_Selection>>.
/// This Rust newtype wraps a Vec for faithful behavior.
pub struct IFSelectTSeqOfSelection {
    items: Vec<IFSelectSelection>,
}

impl IFSelectTSeqOfSelection {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        IFSelectTSeqOfSelection { items: Vec::new() }
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

    /// Clears all elements from the sequence.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns an iterator over the sequence.
    pub fn iter(&self) -> std::slice::Iter<IFSelectSelection> {
        self.items.iter()
    }
}

impl Default for IFSelectTSeqOfSelection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IFSelectTSeqOfSelection::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_and_len() {
        let mut seq = IFSelectTSeqOfSelection::new();
        let sel = IFSelectSelection { id: 1 };
        seq.append(sel);
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_value_1indexed() {
        let mut seq = IFSelectTSeqOfSelection::new();
        let sel = IFSelectSelection { id: 15 };
        seq.append(sel.clone());

        // 1-indexed access
        assert_eq!(seq.value(1).unwrap().id, 15);
        // 0-indexed should return None
        assert_eq!(seq.value(0), None);
        // Out of bounds
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn test_value_mut() {
        let mut seq = IFSelectTSeqOfSelection::new();
        let sel = IFSelectSelection { id: 6 };
        seq.append(sel);

        if let Some(val) = seq.value_mut(1) {
            val.id = 60;
        }

        let retrieved = seq.value(1).unwrap();
        assert_eq!(retrieved.id, 60);
    }

    #[test]
    fn test_clear() {
        let mut seq = IFSelectTSeqOfSelection::new();
        seq.append(IFSelectSelection { id: 1 });
        seq.append(IFSelectSelection { id: 2 });

        assert_eq!(seq.len(), 2);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = IFSelectTSeqOfSelection::new();
        let sel1 = IFSelectSelection { id: 1 };
        let sel2 = IFSelectSelection { id: 2 };
        seq.append(sel1.clone());
        seq.append(sel2.clone());

        let ids: Vec<usize> = seq.iter().map(|s| s.id).collect();
        assert_eq!(ids, vec![1, 2]);
    }
}

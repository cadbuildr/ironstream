// FILE: if_select_t_seq_of_dispatch.rs
// occt: IFSelect_TSeqOfDispatch

/// Represents a dispatch entity.
#[derive(Clone, Debug, PartialEq)]
pub struct IFSelectDispatch {
    pub id: usize,
}

/// Sequence of IFSelect_Dispatch objects.
/// In OCCT, this was NCollection_Sequence<opencascade::handle<IFSelect_Dispatch>>.
/// This Rust newtype wraps a Vec for faithful behavior.
pub struct IFSelectTSeqOfDispatch {
    items: Vec<IFSelectDispatch>,
}

impl IFSelectTSeqOfDispatch {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        IFSelectTSeqOfDispatch { items: Vec::new() }
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
    pub fn append(&mut self, value: IFSelectDispatch) {
        self.items.push(value);
    }

    /// Returns a reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value(&self, index: usize) -> Option<&IFSelectDispatch> {
        if index > 0 && index <= self.items.len() {
            self.items.get(index - 1)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value_mut(&mut self, index: usize) -> Option<&mut IFSelectDispatch> {
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
    pub fn iter(&self) -> std::slice::Iter<IFSelectDispatch> {
        self.items.iter()
    }
}

impl Default for IFSelectTSeqOfDispatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IFSelectTSeqOfDispatch::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_and_len() {
        let mut seq = IFSelectTSeqOfDispatch::new();
        let disp = IFSelectDispatch { id: 1 };
        seq.append(disp);
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_value_1indexed() {
        let mut seq = IFSelectTSeqOfDispatch::new();
        let disp = IFSelectDispatch { id: 12 };
        seq.append(disp.clone());

        // 1-indexed access
        assert_eq!(seq.value(1).unwrap().id, 12);
        // 0-indexed should return None
        assert_eq!(seq.value(0), None);
        // Out of bounds
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn test_value_mut() {
        let mut seq = IFSelectTSeqOfDispatch::new();
        let disp = IFSelectDispatch { id: 4 };
        seq.append(disp);

        if let Some(val) = seq.value_mut(1) {
            val.id = 40;
        }

        let retrieved = seq.value(1).unwrap();
        assert_eq!(retrieved.id, 40);
    }

    #[test]
    fn test_clear() {
        let mut seq = IFSelectTSeqOfDispatch::new();
        seq.append(IFSelectDispatch { id: 1 });
        seq.append(IFSelectDispatch { id: 2 });

        assert_eq!(seq.len(), 2);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = IFSelectTSeqOfDispatch::new();
        let disp1 = IFSelectDispatch { id: 1 };
        let disp2 = IFSelectDispatch { id: 2 };
        seq.append(disp1.clone());
        seq.append(disp2.clone());

        let ids: Vec<usize> = seq.iter().map(|d| d.id).collect();
        assert_eq!(ids, vec![1, 2]);
    }
}

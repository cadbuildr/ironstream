// FILE: select_mgr_sequence_of_selection.rs
// occt: SelectMgr_SequenceOfSelection, SelectMgr_SequenceIteratorOfSequenceOfSelection

/// Deprecated typedef for backward compatibility.
/// A sequence of selection objects using 1-based indexing (OCCT semantics).
/// Corresponds to NCollection_Sequence<opencascade::handle<SelectMgr_Selection>>
pub struct SelectMgrSequenceOfSelection {
    // 1-based indexing: store data starting at index 1, leave index 0 empty
    data: Vec<Option<String>>,
}

impl SelectMgrSequenceOfSelection {
    /// Create a new empty sequence.
    pub fn new() -> Self {
        SelectMgrSequenceOfSelection {
            data: vec![None], // Reserve index 0 (unused) to support 1-based indexing
        }
    }

    /// Append an element to the sequence (at the end).
    pub fn append(&mut self, value: String) {
        self.data.push(Some(value));
    }

    /// Prepend an element to the sequence (at the beginning, after the dummy index 0).
    pub fn prepend(&mut self, value: String) {
        self.data.insert(1, Some(value));
    }

    /// Get an element by 1-based index.
    pub fn value(&self, index: usize) -> Option<&String> {
        if index > 0 && index < self.data.len() {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    /// Set an element at a 1-based index.
    pub fn set_value(&mut self, index: usize, value: String) -> bool {
        if index > 0 && index < self.data.len() {
            self.data[index] = Some(value);
            true
        } else {
            false
        }
    }

    /// Remove an element at a 1-based index.
    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index > 0 && index < self.data.len() {
            self.data.remove(index).flatten()
        } else {
            None
        }
    }

    /// Get the length of the sequence (1-based, so returns actual count).
    pub fn len(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1 // Exclude the dummy index 0
        }
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear all elements from the sequence.
    pub fn clear(&mut self) {
        self.data.clear();
        self.data.push(None); // Restore dummy index 0
    }

    /// Get the lower bound (always 1 in OCCT semantics).
    pub fn lower(&self) -> usize {
        1
    }

    /// Get the upper bound (1-based index of last element).
    pub fn upper(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }
}

impl Default for SelectMgrSequenceOfSelection {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for the deprecated sequence type.
/// Corresponds to SelectMgr_SequenceIteratorOfSequenceOfSelection
pub struct SelectMgrSequenceIteratorOfSequenceOfSelection {
    data: Vec<String>,
    index: usize,
}

impl SelectMgrSequenceIteratorOfSequenceOfSelection {
    /// Create a new iterator from a sequence.
    pub fn new(seq: &SelectMgrSequenceOfSelection) -> Self {
        let mut data = Vec::new();
        for i in 1..seq.data.len() {
            if let Some(ref val) = seq.data[i] {
                data.push(val.clone());
            }
        }
        SelectMgrSequenceIteratorOfSequenceOfSelection { data, index: 0 }
    }

    /// Check if there are more elements.
    pub fn more(&self) -> bool {
        self.index < self.data.len()
    }

    /// Move to the next element.
    pub fn next(&mut self) {
        if self.more() {
            self.index += 1;
        }
    }

    /// Get the current element.
    pub fn value(&self) -> Option<&String> {
        if self.more() {
            Some(&self.data[self.index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1based_indexing() {
        let mut seq = SelectMgrSequenceOfSelection::new();
        assert!(seq.is_empty());
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 0);

        seq.append("item1".to_string());
        assert_eq!(seq.len(), 1);
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 1);
        assert_eq!(seq.value(1), Some(&"item1".to_string()));
        assert_eq!(seq.value(0), None); // Index 0 doesn't exist in OCCT semantics
    }

    #[test]
    fn test_append_and_prepend() {
        let mut seq = SelectMgrSequenceOfSelection::new();
        seq.append("second".to_string());
        seq.prepend("first".to_string());
        seq.append("third".to_string());

        assert_eq!(seq.len(), 3);
        assert_eq!(seq.value(1), Some(&"first".to_string()));
        assert_eq!(seq.value(2), Some(&"second".to_string()));
        assert_eq!(seq.value(3), Some(&"third".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut seq = SelectMgrSequenceOfSelection::new();
        seq.append("a".to_string());
        seq.append("b".to_string());
        seq.append("c".to_string());

        let removed = seq.remove(2);
        assert_eq!(removed, Some("b".to_string()));
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.value(2), Some(&"c".to_string()));
    }

    #[test]
    fn test_iterator() {
        let mut seq = SelectMgrSequenceOfSelection::new();
        seq.append("x".to_string());
        seq.append("y".to_string());
        seq.append("z".to_string());

        let mut iter = SelectMgrSequenceIteratorOfSequenceOfSelection::new(&seq);
        let mut count = 0;

        while iter.more() {
            assert!(iter.value().is_some());
            count += 1;
            iter.next();
        }

        assert_eq!(count, 3);
    }
}

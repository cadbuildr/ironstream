// FILE: transfer_h_sequence_of_finder.rs
// occt: Transfer_HSequenceOfFinder

use std::fmt;

/// Placeholder for Transfer_Finder handle (shared pointer)
#[derive(Clone, Debug)]
pub struct TransferFinderHandle {
    // In real implementation, would contain reference-counted pointer to Transfer_Finder
}

/// Deprecated typedef: NCollection_HSequence<opencascade::handle<Transfer_Finder>>
///
/// A handle-based sequence of Transfer_Finder objects.
/// H-prefix in OCCT indicates a Handle-based class (shared reference).
/// Implements 1-based indexing as per OCCT sequence semantics.
#[derive(Clone, Debug)]
pub struct TransferHSequenceOfFinder {
    items: Vec<TransferFinderHandle>,
    lower: i32,
}

impl TransferHSequenceOfFinder {
    /// Create an empty sequence.
    pub fn new() -> Self {
        TransferHSequenceOfFinder {
            items: Vec::new(),
            lower: 1,
        }
    }

    /// Create a sequence with initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        TransferHSequenceOfFinder {
            items: Vec::with_capacity(capacity),
            lower: 1,
        }
    }

    /// Append an element to the sequence.
    pub fn append(&mut self, elem: TransferFinderHandle) {
        self.items.push(elem);
    }

    /// Insert element at position (1-based OCCT indexing).
    pub fn insert(&mut self, index: i32, elem: TransferFinderHandle) {
        let idx = self.to_usize_index(index);
        assert!(
            idx <= self.items.len(),
            "insert index {} out of valid range [1, {}]",
            index,
            self.items.len() + 1
        );
        self.items.insert(idx, elem);
    }

    /// Get element at position (1-based OCCT indexing).
    pub fn value(&self, index: i32) -> Option<&TransferFinderHandle> {
        if index < self.lower {
            return None;
        }
        self.items.get((index - self.lower) as usize)
    }

    /// Get mutable reference to element (1-based OCCT indexing).
    pub fn change_value(&mut self, index: i32) -> Option<&mut TransferFinderHandle> {
        if index < self.lower {
            return None;
        }
        self.items.get_mut((index - self.lower) as usize)
    }

    /// Remove element at position (1-based OCCT indexing).
    pub fn remove(&mut self, index: i32) {
        let idx = self.to_usize_index(index);
        assert!(idx < self.items.len(), "remove index {} out of bounds", index);
        self.items.remove(idx);
    }

    /// Get the lower index (always 1 in standard OCCT sequences).
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// Get the upper index (1-based).
    pub fn upper(&self) -> i32 {
        self.lower + self.items.len() as i32 - 1
    }

    /// Get the size of the sequence.
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Check if sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get iterator.
    pub fn iter(&self) -> impl Iterator<Item = &TransferFinderHandle> {
        self.items.iter()
    }

    /// Get mutable iterator.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TransferFinderHandle> {
        self.items.iter_mut()
    }

    /// Convert 1-based OCCT index to 0-based Rust index.
    fn to_usize_index(&self, idx: i32) -> usize {
        assert!(
            idx >= self.lower,
            "index {} out of valid range [{}, {}]",
            idx,
            self.lower,
            self.upper()
        );
        (idx - self.lower) as usize
    }
}

impl Default for TransferHSequenceOfFinder {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TransferHSequenceOfFinder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TransferHSequenceOfFinder(size={})", self.size())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_sequence() {
        let seq = TransferHSequenceOfFinder::new();
        assert!(seq.is_empty());
        assert_eq!(seq.size(), 0);
        assert_eq!(seq.lower(), 1);
    }

    #[test]
    fn test_append_elements() {
        let mut seq = TransferHSequenceOfFinder::new();
        let elem = TransferFinderHandle {};
        seq.append(elem.clone());
        assert_eq!(seq.size(), 1);
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 1);
    }

    #[test]
    fn test_value_access() {
        let mut seq = TransferHSequenceOfFinder::new();
        let elem = TransferFinderHandle {};
        seq.append(elem);
        assert!(seq.value(1).is_some());
        assert!(seq.value(2).is_none());
        assert!(seq.value(0).is_none());
    }

    #[test]
    fn test_occt_1based_indexing() {
        let mut seq = TransferHSequenceOfFinder::new();
        seq.append(TransferFinderHandle {});
        seq.append(TransferFinderHandle {});
        seq.append(TransferFinderHandle {});

        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 3);
        assert!(seq.value(1).is_some());
        assert!(seq.value(3).is_some());
        assert!(seq.value(4).is_none());
    }

    #[test]
    fn test_insert_element() {
        let mut seq = TransferHSequenceOfFinder::new();
        seq.append(TransferFinderHandle {});
        seq.append(TransferFinderHandle {});
        seq.insert(2, TransferFinderHandle {});
        assert_eq!(seq.size(), 3);
    }

    #[test]
    fn test_remove_element() {
        let mut seq = TransferHSequenceOfFinder::new();
        seq.append(TransferFinderHandle {});
        seq.append(TransferFinderHandle {});
        seq.remove(1);
        assert_eq!(seq.size(), 1);
    }

    #[test]
    fn test_change_value() {
        let mut seq = TransferHSequenceOfFinder::new();
        seq.append(TransferFinderHandle {});
        assert!(seq.change_value(1).is_some());
        assert!(seq.change_value(2).is_none());
    }

    #[test]
    fn test_clear_sequence() {
        let mut seq = TransferHSequenceOfFinder::new();
        seq.append(TransferFinderHandle {});
        seq.append(TransferFinderHandle {});
        assert_eq!(seq.size(), 2);
        seq.clear();
        assert_eq!(seq.size(), 0);
    }

    #[test]
    fn test_iteration() {
        let mut seq = TransferHSequenceOfFinder::new();
        seq.append(TransferFinderHandle {});
        seq.append(TransferFinderHandle {});
        let count = seq.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_display() {
        let seq = TransferHSequenceOfFinder::new();
        let s = format!("{}", seq);
        assert!(s.contains("size=0"));
    }

    #[test]
    fn test_invalid_index() {
        // OCCT NCollection_Sequence::Value raises Standard_OutOfRange uniformly for
        // any index outside [1, Length()] -- index 0 is treated exactly like an
        // index above Length(). This Option-returning port maps that exception to
        // None, so all out-of-range indices yield None (never a panic).
        let seq = TransferHSequenceOfFinder::new();
        assert!(seq.value(0).is_none());
        assert!(seq.value(-1).is_none());
        assert!(seq.value(1).is_none());
    }
}

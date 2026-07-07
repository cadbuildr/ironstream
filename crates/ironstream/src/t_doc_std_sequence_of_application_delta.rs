// FILE: t_doc_std_sequence_of_application_delta.rs
// occt: TDocStd_SequenceOfApplicationDelta

//! Deprecated typedef for TDocStd_SequenceOfApplicationDelta.
//!
//! In OCCT, this was a sequence of TDocStd_ApplicationDelta handles.
//! We implement a minimal sequence structure using Vec with 1-based indexing semantics.

use std::fmt;

/// TDocStd_SequenceOfApplicationDelta: A sequence of ApplicationDelta handles (deprecated typedef).
/// Wraps a Vec and provides 1-based indexing semantics matching NCollection_Sequence behavior.
#[derive(Clone)]
pub struct TDocStdSequenceOfApplicationDelta {
    items: Vec<i32>,  // Placeholder: would be Vec<Handle<TDocStdApplicationDelta>> in full port
}

impl TDocStdSequenceOfApplicationDelta {
    /// Create a new empty sequence.
    pub fn new() -> Self {
        TDocStdSequenceOfApplicationDelta { items: Vec::new() }
    }

    /// Append an item to the sequence.
    pub fn append(&mut self, item: i32) {
        self.items.push(item);
    }

    /// Prepend an item to the sequence.
    pub fn prepend(&mut self, item: i32) {
        self.items.insert(0, item);
    }

    /// Insert an item at a 1-based position.
    pub fn insert_at(&mut self, index: usize, item: i32) -> bool {
        if index >= 1 && index <= self.items.len() + 1 {
            self.items.insert(index - 1, item);
            true
        } else {
            false
        }
    }

    /// Remove an item at a 1-based position.
    pub fn remove_at(&mut self, index: usize) -> bool {
        if index >= 1 && index <= self.items.len() {
            self.items.remove(index - 1);
            true
        } else {
            false
        }
    }

    /// Get value at 1-based index.
    pub fn value(&self, index: usize) -> Option<i32> {
        if index >= 1 && index <= self.items.len() {
            Some(self.items[index - 1])
        } else {
            None
        }
    }

    /// Return the size of the sequence.
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Return an iterator over the sequence.
    pub fn iter(&self) -> TDocStdSequenceOfApplicationDeltaIterator {
        TDocStdSequenceOfApplicationDeltaIterator {
            items: self.items.clone(),
            current: 0,
        }
    }
}

impl Default for TDocStdSequenceOfApplicationDelta {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TDocStdSequenceOfApplicationDelta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TDocStdSequenceOfApplicationDelta")
            .field("size", &self.items.len())
            .finish()
    }
}

/// Iterator for TDocStd_SequenceOfApplicationDelta.
pub struct TDocStdSequenceOfApplicationDeltaIterator {
    items: Vec<i32>,
    current: usize,
}

impl TDocStdSequenceOfApplicationDeltaIterator {
    /// Check if there is a more item.
    pub fn more(&self) -> bool {
        self.current < self.items.len()
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        if self.current < self.items.len() {
            self.current += 1;
        }
    }

    /// Get the current value.
    pub fn value(&self) -> Option<i32> {
        if self.current < self.items.len() {
            Some(self.items[self.current])
        } else {
            None
        }
    }
}

impl Iterator for TDocStdSequenceOfApplicationDeltaIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.value();
        TDocStdSequenceOfApplicationDeltaIterator::next(self);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_sequence() {
        let seq = TDocStdSequenceOfApplicationDelta::new();
        assert_eq!(seq.size(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = TDocStdSequenceOfApplicationDelta::new();
        seq.append(1);
        seq.append(2);
        seq.append(3);

        assert_eq!(seq.size(), 3);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_prepend() {
        let mut seq = TDocStdSequenceOfApplicationDelta::new();
        seq.append(2);
        seq.prepend(1);

        let values: Vec<i32> = seq.iter().collect();
        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn test_1based_indexing() {
        let mut seq = TDocStdSequenceOfApplicationDelta::new();
        seq.append(10);
        seq.append(20);
        seq.append(30);

        assert_eq!(seq.value(1), Some(10));
        assert_eq!(seq.value(2), Some(20));
        assert_eq!(seq.value(3), Some(30));
        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(4), None);
    }

    #[test]
    fn test_insert_at() {
        let mut seq = TDocStdSequenceOfApplicationDelta::new();
        seq.append(1);
        seq.append(3);

        assert!(seq.insert_at(2, 2));
        let values: Vec<i32> = seq.iter().collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_insert_at_bounds() {
        let mut seq = TDocStdSequenceOfApplicationDelta::new();
        seq.append(1);
        seq.append(2);

        assert!(seq.insert_at(1, 0));  // at beginning
        assert!(seq.insert_at(4, 99)); // at end
        assert!(!seq.insert_at(0, 10));  // invalid
        assert!(!seq.insert_at(6, 10));  // out of bounds
    }

    #[test]
    fn test_remove_at() {
        let mut seq = TDocStdSequenceOfApplicationDelta::new();
        seq.append(1);
        seq.append(2);
        seq.append(3);

        assert!(seq.remove_at(2));
        let values: Vec<i32> = seq.iter().collect();
        assert_eq!(values, vec![1, 3]);
    }

    #[test]
    fn test_remove_at_bounds() {
        let mut seq = TDocStdSequenceOfApplicationDelta::new();
        seq.append(1);
        seq.append(2);

        assert!(!seq.remove_at(0));
        assert!(!seq.remove_at(3));
        assert!(seq.remove_at(1));
        assert_eq!(seq.size(), 1);
    }

    #[test]
    fn test_iterator() {
        let mut seq = TDocStdSequenceOfApplicationDelta::new();
        seq.append(5);
        seq.append(10);
        seq.append(15);

        let mut iter = seq.iter();
        assert!(iter.more());
        assert_eq!(iter.value(), Some(5));
        iter.next();

        assert!(iter.more());
        assert_eq!(iter.value(), Some(10));
        iter.next();

        assert!(iter.more());
        assert_eq!(iter.value(), Some(15));
        iter.next();

        assert!(!iter.more());
    }

    #[test]
    fn test_clear() {
        let mut seq = TDocStdSequenceOfApplicationDelta::new();
        seq.append(1);
        seq.append(2);
        assert_eq!(seq.size(), 2);

        seq.clear();
        assert_eq!(seq.size(), 0);
        assert!(seq.is_empty());
    }
}

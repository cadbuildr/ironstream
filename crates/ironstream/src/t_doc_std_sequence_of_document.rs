// FILE: t_doc_std_sequence_of_document.rs
// occt: TDocStd_SequenceOfDocument

//! Deprecated typedef for TDocStd_SequenceOfDocument.
//!
//! In OCCT, this was a sequence of TDocStd_Document handles.
//! We implement a minimal sequence structure using Vec with 1-based indexing semantics.

use std::fmt;

/// TDocStd_SequenceOfDocument: A sequence of Document handles (deprecated typedef).
/// Wraps a Vec and provides 1-based indexing semantics matching NCollection_Sequence behavior.
#[derive(Clone)]
pub struct TDocStdSequenceOfDocument {
    items: Vec<String>,  // Placeholder: would be Vec<Handle<TDocStdDocument>> in full port
}

impl TDocStdSequenceOfDocument {
    /// Create a new empty sequence.
    pub fn new() -> Self {
        TDocStdSequenceOfDocument { items: Vec::new() }
    }

    /// Append an item to the sequence.
    pub fn append(&mut self, item: String) {
        self.items.push(item);
    }

    /// Prepend an item to the sequence.
    pub fn prepend(&mut self, item: String) {
        self.items.insert(0, item);
    }

    /// Insert an item at a 1-based position.
    pub fn insert_at(&mut self, index: usize, item: String) -> bool {
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
    pub fn value(&self, index: usize) -> Option<String> {
        if index >= 1 && index <= self.items.len() {
            Some(self.items[index - 1].clone())
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
    pub fn iter(&self) -> TDocStdSequenceOfDocumentIterator {
        TDocStdSequenceOfDocumentIterator {
            items: self.items.clone(),
            current: 0,
        }
    }
}

impl Default for TDocStdSequenceOfDocument {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TDocStdSequenceOfDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TDocStdSequenceOfDocument")
            .field("size", &self.items.len())
            .finish()
    }
}

/// Iterator for TDocStd_SequenceOfDocument.
pub struct TDocStdSequenceOfDocumentIterator {
    items: Vec<String>,
    current: usize,
}

impl TDocStdSequenceOfDocumentIterator {
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
    pub fn value(&self) -> Option<String> {
        if self.current < self.items.len() {
            Some(self.items[self.current].clone())
        } else {
            None
        }
    }
}

impl Iterator for TDocStdSequenceOfDocumentIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.value();
        TDocStdSequenceOfDocumentIterator::next(self);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_sequence() {
        let seq = TDocStdSequenceOfDocument::new();
        assert_eq!(seq.size(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = TDocStdSequenceOfDocument::new();
        seq.append("doc1".to_string());
        seq.append("doc2".to_string());
        seq.append("doc3".to_string());

        assert_eq!(seq.size(), 3);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_prepend() {
        let mut seq = TDocStdSequenceOfDocument::new();
        seq.append("doc2".to_string());
        seq.prepend("doc1".to_string());

        let values: Vec<String> = seq.iter().collect();
        assert_eq!(values, vec!["doc1", "doc2"]);
    }

    #[test]
    fn test_1based_indexing() {
        let mut seq = TDocStdSequenceOfDocument::new();
        seq.append("a".to_string());
        seq.append("b".to_string());
        seq.append("c".to_string());

        assert_eq!(seq.value(1), Some("a".to_string()));
        assert_eq!(seq.value(2), Some("b".to_string()));
        assert_eq!(seq.value(3), Some("c".to_string()));
        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(4), None);
    }

    #[test]
    fn test_insert_at() {
        let mut seq = TDocStdSequenceOfDocument::new();
        seq.append("first".to_string());
        seq.append("third".to_string());

        assert!(seq.insert_at(2, "second".to_string()));
        let values: Vec<String> = seq.iter().collect();
        assert_eq!(values, vec!["first", "second", "third"]);
    }

    #[test]
    fn test_insert_at_bounds() {
        let mut seq = TDocStdSequenceOfDocument::new();
        seq.append("b".to_string());

        assert!(seq.insert_at(1, "a".to_string()));  // at beginning
        assert!(seq.insert_at(3, "c".to_string())); // at end
        assert!(!seq.insert_at(0, "x".to_string()));  // invalid
        assert!(!seq.insert_at(10, "x".to_string()));  // out of bounds
    }

    #[test]
    fn test_remove_at() {
        let mut seq = TDocStdSequenceOfDocument::new();
        seq.append("a".to_string());
        seq.append("b".to_string());
        seq.append("c".to_string());

        assert!(seq.remove_at(2));
        let values: Vec<String> = seq.iter().collect();
        assert_eq!(values, vec!["a", "c"]);
    }

    #[test]
    fn test_remove_at_bounds() {
        let mut seq = TDocStdSequenceOfDocument::new();
        seq.append("x".to_string());
        seq.append("y".to_string());

        assert!(!seq.remove_at(0));
        assert!(!seq.remove_at(3));
        assert!(seq.remove_at(1));
        assert_eq!(seq.size(), 1);
    }

    #[test]
    fn test_iterator() {
        let mut seq = TDocStdSequenceOfDocument::new();
        seq.append("doc1".to_string());
        seq.append("doc2".to_string());
        seq.append("doc3".to_string());

        let mut iter = seq.iter();
        assert!(iter.more());
        assert_eq!(iter.value(), Some("doc1".to_string()));
        iter.next();

        assert!(iter.more());
        assert_eq!(iter.value(), Some("doc2".to_string()));
        iter.next();

        assert!(iter.more());
        assert_eq!(iter.value(), Some("doc3".to_string()));
        iter.next();

        assert!(!iter.more());
    }

    #[test]
    fn test_clear() {
        let mut seq = TDocStdSequenceOfDocument::new();
        seq.append("d1".to_string());
        seq.append("d2".to_string());
        assert_eq!(seq.size(), 2);

        seq.clear();
        assert_eq!(seq.size(), 0);
        assert!(seq.is_empty());
    }
}

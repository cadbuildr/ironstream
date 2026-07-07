// FILE: tdf_label_list.rs
// occt: TDF_LabelList, TDF_ListIteratorOfLabelList

//! Deprecated typedef for TDF_LabelList.
//!
//! In OCCT, this was a list of TDF_Label items. We implement a minimal list
//! structure using Vec with NCollection_List-style iteration semantics.

use std::fmt;

/// TDF_LabelList: A list container for TDF_Label items (deprecated typedef).
/// Wraps a Vec and provides iteration semantics matching NCollection_List.
#[derive(Clone)]
pub struct TdfLabelList {
    items: Vec<String>,  // Placeholder: would be Vec<TdfLabel> in full port
}

impl TdfLabelList {
    /// Create a new empty list.
    pub fn new() -> Self {
        TdfLabelList { items: Vec::new() }
    }

    /// Append an item to the list.
    pub fn append(&mut self, item: String) {
        self.items.push(item);
    }

    /// Prepend an item to the list.
    pub fn prepend(&mut self, item: String) {
        self.items.insert(0, item);
    }

    /// Return the size of the list.
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Return an iterator over the list.
    pub fn iter(&self) -> TdfListIteratorOfLabelList {
        TdfListIteratorOfLabelList {
            list: &self.items,
            current: 0,
        }
    }
}

impl Default for TdfLabelList {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TdfLabelList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TdfLabelList")
            .field("size", &self.items.len())
            .finish()
    }
}

/// Iterator for TDF_LabelList.
pub struct TdfListIteratorOfLabelList<'a> {
    list: &'a Vec<String>,
    current: usize,
}

impl<'a> TdfListIteratorOfLabelList<'a> {
    /// Check if there is a more item.
    pub fn more(&self) -> bool {
        self.current < self.list.len()
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        if self.current < self.list.len() {
            self.current += 1;
        }
    }

    /// Get the current value.
    pub fn value(&self) -> Option<String> {
        if self.current < self.list.len() {
            Some(self.list[self.current].clone())
        } else {
            None
        }
    }
}

impl<'a> Iterator for TdfListIteratorOfLabelList<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.value();
        TdfListIteratorOfLabelList::next(self);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list = TdfLabelList::new();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list = TdfLabelList::new();
        list.append("label1".to_string());
        list.append("label2".to_string());
        list.append("label3".to_string());

        assert_eq!(list.size(), 3);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_prepend() {
        let mut list = TdfLabelList::new();
        list.append("b".to_string());
        list.prepend("a".to_string());

        let values: Vec<String> = list.iter().collect();
        assert_eq!(values, vec!["a", "b"]);
    }

    #[test]
    fn test_iterator() {
        let mut list = TdfLabelList::new();
        list.append("l1".to_string());
        list.append("l2".to_string());
        list.append("l3".to_string());

        let mut iter = list.iter();
        assert!(iter.more());
        assert_eq!(iter.value(), Some("l1".to_string()));
        iter.next();

        assert!(iter.more());
        assert_eq!(iter.value(), Some("l2".to_string()));
        iter.next();

        assert!(iter.more());
        assert_eq!(iter.value(), Some("l3".to_string()));
        iter.next();

        assert!(!iter.more());
        assert_eq!(iter.value(), None);
    }

    #[test]
    fn test_iterator_as_rust_iterator() {
        let mut list = TdfLabelList::new();
        list.append("first".to_string());
        list.append("second".to_string());
        list.append("third".to_string());

        let values: Vec<String> = list.iter().collect();
        assert_eq!(values, vec!["first", "second", "third"]);
    }

    #[test]
    fn test_clear() {
        let mut list = TdfLabelList::new();
        list.append("l1".to_string());
        list.append("l2".to_string());
        assert_eq!(list.size(), 2);

        list.clear();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }
}

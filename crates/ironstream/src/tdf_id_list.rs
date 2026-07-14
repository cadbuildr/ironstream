// FILE: tdf_id_list.rs
// occt: TDF_IDList
// occt-ref: TDF_ListIteratorOfIDList

//! Deprecated typedef for TDF_IDList.
//!
//! In OCCT, this was a list of Standard_GUID items. We implement a minimal list
//! structure using Vec with NCollection_List-style iteration semantics.

use std::fmt;

/// TDF_IDList: A list container for Standard_GUID items (deprecated typedef).
/// Wraps a Vec and provides iteration semantics matching NCollection_List.
#[derive(Clone)]
pub struct TdfIdList {
    items: Vec<String>,  // Placeholder: would be Vec<StandardGuid> in full port
}

impl TdfIdList {
    /// Create a new empty list.
    pub fn new() -> Self {
        TdfIdList { items: Vec::new() }
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
    pub fn iter(&self) -> TdfListIteratorOfIdList {
        TdfListIteratorOfIdList {
            list: &self.items,
            current: 0,
        }
    }
}

impl Default for TdfIdList {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TdfIdList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TdfIdList")
            .field("size", &self.items.len())
            .finish()
    }
}

/// Iterator for TDF_IDList.
pub struct TdfListIteratorOfIdList<'a> {
    list: &'a Vec<String>,
    current: usize,
}

impl<'a> TdfListIteratorOfIdList<'a> {
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

impl<'a> Iterator for TdfListIteratorOfIdList<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.value();
        TdfListIteratorOfIdList::next(self);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list = TdfIdList::new();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list = TdfIdList::new();
        list.append("guid1".to_string());
        list.append("guid2".to_string());
        list.append("guid3".to_string());

        assert_eq!(list.size(), 3);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_prepend() {
        let mut list = TdfIdList::new();
        list.append("b".to_string());
        list.prepend("a".to_string());

        let values: Vec<String> = list.iter().collect();
        assert_eq!(values, vec!["a", "b"]);
    }

    #[test]
    fn test_iterator() {
        let mut list = TdfIdList::new();
        list.append("id1".to_string());
        list.append("id2".to_string());
        list.append("id3".to_string());

        let mut iter = list.iter();
        assert!(iter.more());
        assert_eq!(iter.value(), Some("id1".to_string()));
        iter.next();

        assert!(iter.more());
        assert_eq!(iter.value(), Some("id2".to_string()));
        iter.next();

        assert!(iter.more());
        assert_eq!(iter.value(), Some("id3".to_string()));
        iter.next();

        assert!(!iter.more());
        assert_eq!(iter.value(), None);
    }

    #[test]
    fn test_iterator_as_rust_iterator() {
        let mut list = TdfIdList::new();
        list.append("x".to_string());
        list.append("y".to_string());
        list.append("z".to_string());

        let values: Vec<String> = list.iter().collect();
        assert_eq!(values, vec!["x", "y", "z"]);
    }

    #[test]
    fn test_clear() {
        let mut list = TdfIdList::new();
        list.append("id1".to_string());
        list.append("id2".to_string());
        assert_eq!(list.size(), 2);

        list.clear();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }
}

// FILE: tdf_delta_list.rs
// occt: TDF_DeltaList
// occt-ref: TDF_ListIteratorOfDeltaList

//! Deprecated typedef for TDF_DeltaList.
//!
//! In OCCT, this was a list of TDF_Delta handles. Since TDF_Delta is not yet ported,
//! we provide a minimal newtype wrapper over Vec that demonstrates the 1-based indexing
//! pattern and iteration behavior consistent with NCollection_List semantics.

use std::fmt;

/// TDF_DeltaList: A list container for TDF_Delta items (deprecated typedef).
/// Wraps a Vec and provides 1-based indexing semantics matching OCCT.
#[derive(Clone)]
pub struct TdfDeltaList {
    items: Vec<i32>, // Placeholder: would be Vec<Handle<TdfDelta>> in full port
}

impl TdfDeltaList {
    /// Create a new empty list.
    pub fn new() -> Self {
        TdfDeltaList { items: Vec::new() }
    }

    /// Append an item to the list.
    pub fn append(&mut self, item: i32) {
        self.items.push(item);
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
    pub fn iter(&self) -> TdfListIteratorOfDeltaList {
        TdfListIteratorOfDeltaList {
            list: &self.items,
            current: 0,
        }
    }
}

impl Default for TdfDeltaList {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TdfDeltaList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TdfDeltaList")
            .field("size", &self.items.len())
            .finish()
    }
}

/// Iterator for TDF_DeltaList, matching NCollection_List iterator semantics.
pub struct TdfListIteratorOfDeltaList<'a> {
    list: &'a Vec<i32>,
    current: usize,
}

impl<'a> TdfListIteratorOfDeltaList<'a> {
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
    pub fn value(&self) -> Option<i32> {
        if self.current < self.list.len() {
            Some(self.list[self.current])
        } else {
            None
        }
    }
}

impl<'a> Iterator for TdfListIteratorOfDeltaList<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.value();
        TdfListIteratorOfDeltaList::next(self);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list = TdfDeltaList::new();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_append_and_size() {
        let mut list = TdfDeltaList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.size(), 3);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut list = TdfDeltaList::new();
        list.append(10);
        list.append(20);
        list.append(30);

        let mut iter = list.iter();
        assert!(iter.more());
        assert_eq!(iter.value(), Some(10));
        iter.next();

        assert!(iter.more());
        assert_eq!(iter.value(), Some(20));
        iter.next();

        assert!(iter.more());
        assert_eq!(iter.value(), Some(30));
        iter.next();

        assert!(!iter.more());
        assert_eq!(iter.value(), None);
    }

    #[test]
    fn test_iterator_as_rust_iterator() {
        let mut list = TdfDeltaList::new();
        list.append(5);
        list.append(15);
        list.append(25);

        let values: Vec<i32> = list.iter().collect();
        assert_eq!(values, vec![5, 15, 25]);
    }

    #[test]
    fn test_clear() {
        let mut list = TdfDeltaList::new();
        list.append(1);
        list.append(2);
        assert_eq!(list.size(), 2);

        list.clear();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_debug() {
        let mut list = TdfDeltaList::new();
        list.append(1);
        let debug_str = format!("{:?}", list);
        assert!(debug_str.contains("size"));
    }
}

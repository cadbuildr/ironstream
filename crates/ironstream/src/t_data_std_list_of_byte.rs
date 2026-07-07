// FILE: t_data_std_list_of_byte.rs
// occt: TDataStd_ListOfByte, TDataStd_ListIteratorOfListOfByte

//! Deprecated typedef for TDataStd_ListOfByte.
//!
//! In OCCT, this was a list of uint8_t items. We implement a minimal list
//! structure using Vec with NCollection_List-style iteration semantics.

use std::fmt;

/// TDataStd_ListOfByte: A list container for uint8_t items (deprecated typedef).
/// Wraps a Vec and provides iteration semantics matching NCollection_List.
#[derive(Clone)]
pub struct TDataStdListOfByte {
    items: Vec<u8>,
}

impl TDataStdListOfByte {
    /// Create a new empty list.
    pub fn new() -> Self {
        TDataStdListOfByte { items: Vec::new() }
    }

    /// Append an item to the list.
    pub fn append(&mut self, item: u8) {
        self.items.push(item);
    }

    /// Prepend an item to the list.
    pub fn prepend(&mut self, item: u8) {
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
    pub fn iter(&self) -> TDataStdListIteratorOfListOfByte {
        TDataStdListIteratorOfListOfByte {
            list: &self.items,
            current: 0,
        }
    }
}

impl Default for TDataStdListOfByte {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TDataStdListOfByte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TDataStdListOfByte")
            .field("size", &self.items.len())
            .finish()
    }
}

/// Iterator for TDataStd_ListOfByte.
pub struct TDataStdListIteratorOfListOfByte<'a> {
    list: &'a Vec<u8>,
    current: usize,
}

impl<'a> TDataStdListIteratorOfListOfByte<'a> {
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
    pub fn value(&self) -> Option<u8> {
        if self.current < self.list.len() {
            Some(self.list[self.current])
        } else {
            None
        }
    }
}

impl<'a> Iterator for TDataStdListIteratorOfListOfByte<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.value();
        TDataStdListIteratorOfListOfByte::next(self);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list = TDataStdListOfByte::new();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list = TDataStdListOfByte::new();
        list.append(10);
        list.append(20);
        list.append(255);

        assert_eq!(list.size(), 3);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_prepend() {
        let mut list = TDataStdListOfByte::new();
        list.append(2);
        list.prepend(1);

        let values: Vec<u8> = list.iter().collect();
        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn test_iterator() {
        let mut list = TDataStdListOfByte::new();
        list.append(5);
        list.append(10);
        list.append(15);

        let mut iter = list.iter();
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
        assert_eq!(iter.value(), None);
    }

    #[test]
    fn test_iterator_as_rust_iterator() {
        let mut list = TDataStdListOfByte::new();
        list.append(0);
        list.append(128);
        list.append(255);

        let values: Vec<u8> = list.iter().collect();
        assert_eq!(values, vec![0, 128, 255]);
    }

    #[test]
    fn test_clear() {
        let mut list = TDataStdListOfByte::new();
        list.append(1);
        list.append(2);
        assert_eq!(list.size(), 2);

        list.clear();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_byte_range() {
        let mut list = TDataStdListOfByte::new();
        list.append(0);     // min
        list.append(127);   // mid
        list.append(255);   // max

        assert_eq!(list.size(), 3);
        let values: Vec<u8> = list.iter().collect();
        assert_eq!(values, vec![0, 127, 255]);
    }
}

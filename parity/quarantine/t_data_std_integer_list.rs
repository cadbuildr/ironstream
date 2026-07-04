// FILE: t_data_std_integer_list.rs
// occt: TDataStd_IntegerList

use std::collections::VecDeque;

/// A list attribute containing integer values.
/// Provides efficient insertion, removal, and traversal of integers.
#[derive(Clone, Debug)]
pub struct TDataStd_IntegerList {
    list: VecDeque<i32>,
    id: [u8; 16],
}

impl TDataStd_IntegerList {
    /// Create a new IntegerList attribute.
    pub fn new() -> Self {
        Self {
            list: VecDeque::new(),
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for IntegerList attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_IntegerList
        [
            0x2A, 0xE6, 0x0D, 0xB2, 0x8D, 0xF3, 0x4E, 0x5C, 0xA2, 0x6F, 0x4A, 0x11, 0x77, 0x22,
            0x22, 0x22,
        ]
    }

    /// Set a custom GUID for this attribute.
    pub fn set_id(&mut self, guid: [u8; 16]) {
        self.id = guid;
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Get the number of elements.
    pub fn extent(&self) -> usize {
        self.list.len()
    }

    /// Add a value to the front of the list.
    pub fn prepend(&mut self, value: i32) {
        self.list.push_front(value);
    }

    /// Add a value to the end of the list.
    pub fn append(&mut self, value: i32) {
        self.list.push_back(value);
    }

    /// Insert value before the first occurrence of before_value.
    pub fn insert_before(&mut self, value: i32, before_value: i32) -> bool {
        for (idx, &v) in self.list.iter().enumerate() {
            if v == before_value {
                self.list.insert(idx, value);
                return true;
            }
        }
        false
    }

    /// Insert value at the given index (1-based).
    pub fn insert_before_by_index(&mut self, index: usize, value: i32) -> bool {
        if index == 0 || index > self.list.len() {
            return false;
        }
        self.list.insert(index - 1, value);
        true
    }

    /// Insert value after the first occurrence of after_value.
    pub fn insert_after(&mut self, value: i32, after_value: i32) -> bool {
        for (idx, &v) in self.list.iter().enumerate() {
            if v == after_value {
                self.list.insert(idx + 1, value);
                return true;
            }
        }
        false
    }

    /// Insert value after the given index (1-based).
    pub fn insert_after_by_index(&mut self, index: usize, value: i32) -> bool {
        if index == 0 || index > self.list.len() {
            return false;
        }
        self.list.insert(index, value);
        true
    }

    /// Remove the first occurrence of value.
    pub fn remove(&mut self, value: i32) -> bool {
        if let Some(pos) = self.list.iter().position(|&v| v == value) {
            self.list.remove(pos);
            true
        } else {
            false
        }
    }

    /// Remove value at the given index (1-based).
    pub fn remove_by_index(&mut self, index: usize) -> bool {
        if index == 0 || index > self.list.len() {
            return false;
        }
        self.list.remove(index - 1);
        true
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.list.clear();
    }

    /// Get the first element.
    pub fn first(&self) -> Option<i32> {
        self.list.front().copied()
    }

    /// Get the last element.
    pub fn last(&self) -> Option<i32> {
        self.list.back().copied()
    }

    /// Get the list as a slice.
    pub fn list(&self) -> Vec<i32> {
        self.list.iter().copied().collect()
    }
}

impl Default for TDataStd_IntegerList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_list() {
        let list = TDataStd_IntegerList::new();
        assert!(list.is_empty());
        assert_eq!(list.extent(), 0);
    }

    #[test]
    fn test_append() {
        let mut list = TDataStd_IntegerList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.extent(), 3);
        assert_eq!(list.first(), Some(1));
        assert_eq!(list.last(), Some(3));
    }

    #[test]
    fn test_prepend() {
        let mut list = TDataStd_IntegerList::new();
        list.append(2);
        list.prepend(1);
        assert_eq!(list.first(), Some(1));
        assert_eq!(list.extent(), 2);
    }

    #[test]
    fn test_insert_before() {
        let mut list = TDataStd_IntegerList::new();
        list.append(1);
        list.append(3);
        assert!(list.insert_before(2, 3));
        let values = list.list();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_insert_before_by_index() {
        let mut list = TDataStd_IntegerList::new();
        list.append(1);
        list.append(3);
        assert!(list.insert_before_by_index(2, 2));
        let values = list.list();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_insert_after() {
        let mut list = TDataStd_IntegerList::new();
        list.append(1);
        list.append(2);
        assert!(list.insert_after(3, 2));
        let values = list.list();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_insert_after_by_index() {
        let mut list = TDataStd_IntegerList::new();
        list.append(1);
        list.append(2);
        assert!(list.insert_after_by_index(2, 3));
        let values = list.list();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_remove() {
        let mut list = TDataStd_IntegerList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert!(list.remove(2));
        assert_eq!(list.extent(), 2);
        assert_eq!(list.list(), vec![1, 3]);
    }

    #[test]
    fn test_remove_by_index() {
        let mut list = TDataStd_IntegerList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert!(list.remove_by_index(2));
        assert_eq!(list.list(), vec![1, 3]);
    }

    #[test]
    fn test_clear() {
        let mut list = TDataStd_IntegerList::new();
        list.append(1);
        list.append(2);
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_default() {
        let list = TDataStd_IntegerList::default();
        assert!(list.is_empty());
    }
}

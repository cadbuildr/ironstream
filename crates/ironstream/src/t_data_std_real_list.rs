// FILE: t_data_std_real_list.rs
// occt: TDataStd_RealList

use std::collections::VecDeque;

/// A list attribute containing real (f64) values.
/// Provides efficient insertion, removal, and traversal of floating-point numbers.
#[derive(Clone, Debug)]
pub struct TDataStd_RealList {
    list: VecDeque<f64>,
    id: [u8; 16],
}

impl TDataStd_RealList {
    /// Create a new RealList attribute.
    pub fn new() -> Self {
        Self {
            list: VecDeque::new(),
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for RealList attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_RealList
        [
            0x4B, 0x1F, 0xE8, 0xC7, 0x3A, 0x9D, 0x4F, 0x22, 0xB5, 0x8C, 0x7F, 0x44, 0x55, 0x22,
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
    pub fn prepend(&mut self, value: f64) {
        self.list.push_front(value);
    }

    /// Add a value to the end of the list.
    pub fn append(&mut self, value: f64) {
        self.list.push_back(value);
    }

    /// Insert value before the first occurrence of before_value.
    pub fn insert_before(&mut self, value: f64, before_value: f64) -> bool {
        for (idx, &v) in self.list.iter().enumerate() {
            if (v - before_value).abs() < f64::EPSILON {
                self.list.insert(idx, value);
                return true;
            }
        }
        false
    }

    /// Insert value at the given index (1-based).
    pub fn insert_before_by_index(&mut self, index: usize, value: f64) -> bool {
        if index == 0 || index > self.list.len() {
            return false;
        }
        self.list.insert(index - 1, value);
        true
    }

    /// Insert value after the first occurrence of after_value.
    pub fn insert_after(&mut self, value: f64, after_value: f64) -> bool {
        for (idx, &v) in self.list.iter().enumerate() {
            if (v - after_value).abs() < f64::EPSILON {
                self.list.insert(idx + 1, value);
                return true;
            }
        }
        false
    }

    /// Insert value after the given index (1-based).
    pub fn insert_after_by_index(&mut self, index: usize, value: f64) -> bool {
        if index == 0 || index > self.list.len() {
            return false;
        }
        self.list.insert(index, value);
        true
    }

    /// Remove the first occurrence of value.
    pub fn remove(&mut self, value: f64) -> bool {
        if let Some(pos) = self
            .list
            .iter()
            .position(|&v| (v - value).abs() < f64::EPSILON)
        {
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
    pub fn first(&self) -> Option<f64> {
        self.list.front().copied()
    }

    /// Get the last element.
    pub fn last(&self) -> Option<f64> {
        self.list.back().copied()
    }

    /// Get the list as a vector.
    pub fn list(&self) -> Vec<f64> {
        self.list.iter().copied().collect()
    }
}

impl Default for TDataStd_RealList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_list() {
        let list = TDataStd_RealList::new();
        assert!(list.is_empty());
        assert_eq!(list.extent(), 0);
    }

    #[test]
    fn test_append() {
        let mut list = TDataStd_RealList::new();
        list.append(1.5);
        list.append(2.5);
        list.append(3.5);
        assert_eq!(list.extent(), 3);
        assert_eq!(list.first(), Some(1.5));
        assert_eq!(list.last(), Some(3.5));
    }

    #[test]
    fn test_prepend() {
        let mut list = TDataStd_RealList::new();
        list.append(2.0);
        list.prepend(1.0);
        assert_eq!(list.first(), Some(1.0));
        assert_eq!(list.extent(), 2);
    }

    #[test]
    fn test_insert_before() {
        let mut list = TDataStd_RealList::new();
        list.append(1.0);
        list.append(3.0);
        assert!(list.insert_before(2.0, 3.0));
        let values = list.list();
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn test_insert_after() {
        let mut list = TDataStd_RealList::new();
        list.append(1.0);
        list.append(2.0);
        assert!(list.insert_after(3.0, 2.0));
        let values = list.list();
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn test_remove() {
        let mut list = TDataStd_RealList::new();
        list.append(1.5);
        list.append(2.5);
        list.append(3.5);
        assert!(list.remove(2.5));
        assert_eq!(list.extent(), 2);
    }

    #[test]
    fn test_remove_by_index() {
        let mut list = TDataStd_RealList::new();
        list.append(1.5);
        list.append(2.5);
        list.append(3.5);
        assert!(list.remove_by_index(2));
        assert_eq!(list.extent(), 2);
    }

    #[test]
    fn test_clear() {
        let mut list = TDataStd_RealList::new();
        list.append(1.0);
        list.append(2.0);
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_default() {
        let list = TDataStd_RealList::default();
        assert!(list.is_empty());
    }
}

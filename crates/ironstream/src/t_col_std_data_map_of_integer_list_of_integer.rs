// FILE: t_col_std_data_map_of_integer_list_of_integer.rs
// occt: TColStd_DataMapOfIntegerListOfInteger

use std::collections::HashMap;

/// TColStd_DataMapOfIntegerListOfInteger is a deprecated alias for a data map with i32 keys and Vec<i32> values.
/// This is a Rust port implementing OCCT's data map semantics.
pub struct TColStdDataMapOfIntegerListOfInteger {
    data: HashMap<i32, Vec<i32>>,
}

impl TColStdDataMapOfIntegerListOfInteger {
    /// Creates a new empty data map.
    pub fn new() -> Self {
        TColStdDataMapOfIntegerListOfInteger {
            data: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the map (creates empty list if key doesn't exist).
    pub fn insert(&mut self, key: i32) -> bool {
        if self.data.contains_key(&key) {
            false
        } else {
            self.data.insert(key, Vec::new());
            true
        }
    }

    /// Appends a value to the list at the given key.
    pub fn append(&mut self, key: i32, value: i32) {
        self.data.entry(key).or_insert_with(Vec::new).push(value);
    }

    /// Gets a reference to a list by key.
    pub fn at(&self, key: i32) -> Option<&Vec<i32>> {
        self.data.get(&key)
    }

    /// Gets a mutable reference to a list by key.
    pub fn at_mut(&mut self, key: i32) -> Option<&mut Vec<i32>> {
        self.data.get_mut(&key)
    }

    /// Removes a key from the map.
    pub fn remove(&mut self, key: i32) -> bool {
        self.data.remove(&key).is_some()
    }

    /// Returns the number of elements in the map.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Checks if the map contains a key.
    pub fn contains(&self, key: i32) -> bool {
        self.data.contains_key(&key)
    }
}

impl Default for TColStdDataMapOfIntegerListOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_at() {
        let mut map = TColStdDataMapOfIntegerListOfInteger::new();
        map.insert(1);
        map.insert(2);

        assert_eq!(map.at(1), Some(&vec![]));
        assert_eq!(map.at(2), Some(&vec![]));
    }

    #[test]
    fn test_append() {
        let mut map = TColStdDataMapOfIntegerListOfInteger::new();
        map.insert(1);
        map.append(1, 10);
        map.append(1, 20);

        assert_eq!(map.at(1), Some(&vec![10, 20]));
    }

    #[test]
    fn test_append_creates_entry() {
        let mut map = TColStdDataMapOfIntegerListOfInteger::new();
        map.append(5, 50);

        assert_eq!(map.at(5), Some(&vec![50]));
    }

    #[test]
    fn test_size() {
        let mut map = TColStdDataMapOfIntegerListOfInteger::new();
        assert_eq!(map.size(), 0);

        map.insert(1);
        assert_eq!(map.size(), 1);

        map.insert(2);
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdDataMapOfIntegerListOfInteger::new();
        map.insert(10);
        assert!(map.contains(10));

        assert!(map.remove(10));
        assert!(!map.contains(10));
    }
}

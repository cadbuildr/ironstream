// FILE: t_col_std_data_map_of_integer_integer.rs
// occt: TColStd_DataMapOfIntegerInteger

use std::collections::HashMap;

/// TColStd_DataMapOfIntegerInteger is a deprecated alias for a data map with i32 keys and values.
/// This is a Rust port implementing OCCT's data map semantics.
pub struct TColStdDataMapOfIntegerInteger {
    data: HashMap<i32, i32>,
}

impl TColStdDataMapOfIntegerInteger {
    /// Creates a new empty data map.
    pub fn new() -> Self {
        TColStdDataMapOfIntegerInteger {
            data: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: i32, value: i32) -> bool {
        self.data.insert(key, value).is_none()
    }

    /// Gets a reference to a value by key.
    pub fn at(&self, key: i32) -> Option<&i32> {
        self.data.get(&key)
    }

    /// Gets a mutable reference to a value by key.
    pub fn at_mut(&mut self, key: i32) -> Option<&mut i32> {
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

impl Default for TColStdDataMapOfIntegerInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_at() {
        let mut map = TColStdDataMapOfIntegerInteger::new();
        map.insert(1, 10);
        map.insert(2, 20);

        assert_eq!(map.at(1), Some(&10));
        assert_eq!(map.at(2), Some(&20));
        assert_eq!(map.at(3), None);
    }

    #[test]
    fn test_size() {
        let mut map = TColStdDataMapOfIntegerInteger::new();
        assert_eq!(map.size(), 0);

        map.insert(1, 100);
        assert_eq!(map.size(), 1);

        map.insert(2, 200);
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdDataMapOfIntegerInteger::new();
        map.insert(5, 50);
        assert!(map.contains(5));

        assert!(map.remove(5));
        assert!(!map.contains(5));
    }

    #[test]
    fn test_at_mut() {
        let mut map = TColStdDataMapOfIntegerInteger::new();
        map.insert(10, 100);

        if let Some(val) = map.at_mut(10) {
            *val = 200;
        }

        assert_eq!(map.at(10), Some(&200));
    }
}

// FILE: t_col_std_data_map_of_integer_real.rs
// occt: TColStd_DataMapOfIntegerReal

use std::collections::HashMap;

/// TColStd_DataMapOfIntegerReal is a deprecated alias for a data map with i32 keys and f64 values.
/// This is a Rust port implementing OCCT's data map semantics.
pub struct TColStdDataMapOfIntegerReal {
    data: HashMap<i32, f64>,
}

impl TColStdDataMapOfIntegerReal {
    /// Creates a new empty data map.
    pub fn new() -> Self {
        TColStdDataMapOfIntegerReal {
            data: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: i32, value: f64) -> bool {
        self.data.insert(key, value).is_none()
    }

    /// Gets a reference to a value by key.
    pub fn at(&self, key: i32) -> Option<&f64> {
        self.data.get(&key)
    }

    /// Gets a mutable reference to a value by key.
    pub fn at_mut(&mut self, key: i32) -> Option<&mut f64> {
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

impl Default for TColStdDataMapOfIntegerReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_at() {
        let mut map = TColStdDataMapOfIntegerReal::new();
        map.insert(1, 1.5);
        map.insert(2, 2.5);

        assert_eq!(map.at(1), Some(&1.5));
        assert_eq!(map.at(2), Some(&2.5));
        assert_eq!(map.at(3), None);
    }

    #[test]
    fn test_size() {
        let mut map = TColStdDataMapOfIntegerReal::new();
        assert_eq!(map.size(), 0);

        map.insert(1, 1.0);
        assert_eq!(map.size(), 1);

        map.insert(2, 2.0);
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdDataMapOfIntegerReal::new();
        map.insert(5, 5.5);
        assert!(map.contains(5));

        assert!(map.remove(5));
        assert!(!map.contains(5));
    }

    #[test]
    fn test_at_mut() {
        let mut map = TColStdDataMapOfIntegerReal::new();
        map.insert(10, 1.0);

        if let Some(val) = map.at_mut(10) {
            *val = 2.0;
        }

        assert_eq!(map.at(10), Some(&2.0));
    }
}

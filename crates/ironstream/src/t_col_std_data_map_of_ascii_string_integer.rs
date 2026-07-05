// FILE: t_col_std_data_map_of_ascii_string_integer.rs
// occt: TColStd_DataMapOfAsciiStringInteger

use std::collections::HashMap;

/// TColStd_DataMapOfAsciiStringInteger is a deprecated alias for a data map with String keys and i32 values.
/// This is a Rust port implementing OCCT's data map semantics.
pub struct TColStdDataMapOfAsciiStringInteger {
    data: HashMap<String, i32>,
}

impl TColStdDataMapOfAsciiStringInteger {
    /// Creates a new empty data map.
    pub fn new() -> Self {
        TColStdDataMapOfAsciiStringInteger {
            data: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: String, value: i32) -> bool {
        self.data.insert(key, value).is_none()
    }

    /// Gets a reference to a value by key.
    pub fn at(&self, key: &str) -> Option<&i32> {
        self.data.get(key)
    }

    /// Gets a mutable reference to a value by key.
    pub fn at_mut(&mut self, key: &str) -> Option<&mut i32> {
        self.data.get_mut(key)
    }

    /// Removes a key from the map.
    pub fn remove(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
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
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}

impl Default for TColStdDataMapOfAsciiStringInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_at() {
        let mut map = TColStdDataMapOfAsciiStringInteger::new();
        map.insert("key1".to_string(), 10);
        map.insert("key2".to_string(), 20);

        assert_eq!(map.at("key1"), Some(&10));
        assert_eq!(map.at("key2"), Some(&20));
        assert_eq!(map.at("key3"), None);
    }

    #[test]
    fn test_size() {
        let mut map = TColStdDataMapOfAsciiStringInteger::new();
        assert_eq!(map.size(), 0);

        map.insert("a".to_string(), 1);
        assert_eq!(map.size(), 1);

        map.insert("b".to_string(), 2);
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdDataMapOfAsciiStringInteger::new();
        map.insert("key".to_string(), 42);
        assert!(map.contains("key"));

        assert!(map.remove("key"));
        assert!(!map.contains("key"));
        assert!(!map.remove("key"));
    }

    #[test]
    fn test_clear() {
        let mut map = TColStdDataMapOfAsciiStringInteger::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_at_mut() {
        let mut map = TColStdDataMapOfAsciiStringInteger::new();
        map.insert("key".to_string(), 5);

        if let Some(val) = map.at_mut("key") {
            *val = 10;
        }

        assert_eq!(map.at("key"), Some(&10));
    }
}

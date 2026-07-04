// FILE: t_data_std_h_data_map_of_string_integer.rs
// occt: TDataStd_HDataMapOfStringInteger

use std::collections::HashMap;

/// A handle-based data map of String to Integer.
/// Extension of NCollection_DataMap manipulated by handle (shared pointer).
#[derive(Clone, Debug)]
pub struct TDataStd_HDataMapOfStringInteger {
    map: HashMap<String, i32>,
}

impl TDataStd_HDataMapOfStringInteger {
    /// Create a new map with specified number of buckets.
    pub fn new(nb_buckets: usize) -> Self {
        Self {
            map: HashMap::with_capacity(nb_buckets),
        }
    }

    /// Create a map from existing data.
    pub fn from_map(data: HashMap<String, i32>) -> Self {
        Self { map: data }
    }

    /// Get a reference to the underlying map.
    pub fn map(&self) -> &HashMap<String, i32> {
        &self.map
    }

    /// Get mutable access to the underlying map.
    pub fn change_map(&mut self) -> &mut HashMap<String, i32> {
        &mut self.map
    }

    /// Insert a key-value pair.
    pub fn insert(&mut self, key: String, value: i32) -> Option<i32> {
        self.map.insert(key, value)
    }

    /// Get a value by key.
    pub fn get(&self, key: &str) -> Option<i32> {
        self.map.get(key).copied()
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// Remove a key-value pair.
    pub fn remove(&mut self, key: &str) -> Option<i32> {
        self.map.remove(key)
    }

    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Clear the map.
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl Default for TDataStd_HDataMapOfStringInteger {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_map() {
        let map = TDataStd_HDataMapOfStringInteger::new(10);
        assert!(map.is_empty());
    }

    #[test]
    fn test_insert_and_get() {
        let mut map = TDataStd_HDataMapOfStringInteger::new(10);
        map.insert("key1".to_string(), 42);
        assert_eq!(map.get("key1"), Some(42));
    }

    #[test]
    fn test_contains() {
        let mut map = TDataStd_HDataMapOfStringInteger::new(10);
        map.insert("exists".to_string(), 1);
        assert!(map.contains("exists"));
        assert!(!map.contains("not_exists"));
    }

    #[test]
    fn test_remove() {
        let mut map = TDataStd_HDataMapOfStringInteger::new(10);
        map.insert("key".to_string(), 100);
        assert_eq!(map.remove("key"), Some(100));
        assert!(!map.contains("key"));
    }

    #[test]
    fn test_len() {
        let mut map = TDataStd_HDataMapOfStringInteger::new(10);
        assert_eq!(map.len(), 0);
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut map = TDataStd_HDataMapOfStringInteger::new(10);
        map.insert("a".to_string(), 1);
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_default() {
        let map = TDataStd_HDataMapOfStringInteger::default();
        assert!(map.is_empty());
    }
}

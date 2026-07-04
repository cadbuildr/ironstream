// FILE: t_data_std_h_data_map_of_string_real.rs
// occt: TDataStd_HDataMapOfStringReal

use std::collections::HashMap;

/// A handle-based data map of String to Real (f64).
/// Extension of NCollection_DataMap manipulated by handle (shared pointer).
#[derive(Clone, Debug)]
pub struct TDataStd_HDataMapOfStringReal {
    map: HashMap<String, f64>,
}

impl TDataStd_HDataMapOfStringReal {
    /// Create a new map with specified number of buckets.
    pub fn new(nb_buckets: usize) -> Self {
        Self {
            map: HashMap::with_capacity(nb_buckets),
        }
    }

    /// Create a map from existing data.
    pub fn from_map(data: HashMap<String, f64>) -> Self {
        Self { map: data }
    }

    /// Get a reference to the underlying map.
    pub fn map(&self) -> &HashMap<String, f64> {
        &self.map
    }

    /// Get mutable access to the underlying map.
    pub fn change_map(&mut self) -> &mut HashMap<String, f64> {
        &mut self.map
    }

    /// Insert a key-value pair.
    pub fn insert(&mut self, key: String, value: f64) -> Option<f64> {
        self.map.insert(key, value)
    }

    /// Get a value by key.
    pub fn get(&self, key: &str) -> Option<f64> {
        self.map.get(key).copied()
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// Remove a key-value pair.
    pub fn remove(&mut self, key: &str) -> Option<f64> {
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

impl Default for TDataStd_HDataMapOfStringReal {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_map() {
        let map = TDataStd_HDataMapOfStringReal::new(10);
        assert!(map.is_empty());
    }

    #[test]
    fn test_insert_and_get() {
        let mut map = TDataStd_HDataMapOfStringReal::new(10);
        map.insert("key1".to_string(), 3.14);
        assert_eq!(map.get("key1"), Some(3.14));
    }

    #[test]
    fn test_contains() {
        let mut map = TDataStd_HDataMapOfStringReal::new(10);
        map.insert("exists".to_string(), 1.5);
        assert!(map.contains("exists"));
        assert!(!map.contains("not_exists"));
    }

    #[test]
    fn test_remove() {
        let mut map = TDataStd_HDataMapOfStringReal::new(10);
        map.insert("key".to_string(), 2.71);
        assert_eq!(map.remove("key"), Some(2.71));
        assert!(!map.contains("key"));
    }

    #[test]
    fn test_len() {
        let mut map = TDataStd_HDataMapOfStringReal::new(10);
        assert_eq!(map.len(), 0);
        map.insert("a".to_string(), 1.0);
        map.insert("b".to_string(), 2.0);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut map = TDataStd_HDataMapOfStringReal::new(10);
        map.insert("a".to_string(), 1.0);
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_default() {
        let map = TDataStd_HDataMapOfStringReal::default();
        assert!(map.is_empty());
    }
}

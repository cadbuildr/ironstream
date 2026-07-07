// FILE: t_col_std_data_map_of_transient_transient.rs
// occt: TColStd_DataMapOfTransientTransient

use std::collections::HashMap;

/// TColStd_DataMapOfTransientTransient is a deprecated alias for a data map with String (transient) keys and values.
/// This is a Rust port implementing OCCT's data map semantics.
pub struct TColStdDataMapOfTransientTransient {
    data: HashMap<String, String>,
}

impl TColStdDataMapOfTransientTransient {
    /// Creates a new empty data map.
    pub fn new() -> Self {
        TColStdDataMapOfTransientTransient {
            data: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: String, value: String) -> bool {
        self.data.insert(key, value).is_none()
    }

    /// Gets a reference to a value by key.
    pub fn at(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// Gets a mutable reference to a value by key.
    pub fn at_mut(&mut self, key: &str) -> Option<&mut String> {
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

impl Default for TColStdDataMapOfTransientTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_at() {
        let mut map = TColStdDataMapOfTransientTransient::new();
        map.insert("key1".to_string(), "val1".to_string());
        map.insert("key2".to_string(), "val2".to_string());

        assert_eq!(map.at("key1"), Some(&"val1".to_string()));
        assert_eq!(map.at("key2"), Some(&"val2".to_string()));
        assert_eq!(map.at("key3"), None);
    }

    #[test]
    fn test_size() {
        let mut map = TColStdDataMapOfTransientTransient::new();
        assert_eq!(map.size(), 0);

        map.insert("k1".to_string(), "v1".to_string());
        assert_eq!(map.size(), 1);

        map.insert("k2".to_string(), "v2".to_string());
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdDataMapOfTransientTransient::new();
        map.insert("key".to_string(), "value".to_string());
        assert!(map.contains("key"));

        assert!(map.remove("key"));
        assert!(!map.contains("key"));
    }

    #[test]
    fn test_at_mut() {
        let mut map = TColStdDataMapOfTransientTransient::new();
        map.insert("key".to_string(), "old".to_string());

        if let Some(val) = map.at_mut("key") {
            *val = "new".to_string();
        }

        assert_eq!(map.at("key"), Some(&"new".to_string()));
    }
}

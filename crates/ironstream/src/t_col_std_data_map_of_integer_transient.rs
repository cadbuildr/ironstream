// FILE: t_col_std_data_map_of_integer_transient.rs
// occt: TColStd_DataMapOfIntegerTransient

use std::collections::HashMap;

/// TColStd_DataMapOfIntegerTransient is a deprecated alias for a data map with i32 keys and String (transient) values.
/// This is a Rust port implementing OCCT's data map semantics.
pub struct TColStdDataMapOfIntegerTransient {
    data: HashMap<i32, String>,
}

impl TColStdDataMapOfIntegerTransient {
    /// Creates a new empty data map.
    pub fn new() -> Self {
        TColStdDataMapOfIntegerTransient {
            data: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: i32, value: String) -> bool {
        self.data.insert(key, value).is_none()
    }

    /// Gets a reference to a value by key.
    pub fn at(&self, key: i32) -> Option<&String> {
        self.data.get(&key)
    }

    /// Gets a mutable reference to a value by key.
    pub fn at_mut(&mut self, key: i32) -> Option<&mut String> {
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

impl Default for TColStdDataMapOfIntegerTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_at() {
        let mut map = TColStdDataMapOfIntegerTransient::new();
        map.insert(1, "obj1".to_string());
        map.insert(2, "obj2".to_string());

        assert_eq!(map.at(1), Some(&"obj1".to_string()));
        assert_eq!(map.at(2), Some(&"obj2".to_string()));
        assert_eq!(map.at(3), None);
    }

    #[test]
    fn test_size() {
        let mut map = TColStdDataMapOfIntegerTransient::new();
        assert_eq!(map.size(), 0);

        map.insert(1, "a".to_string());
        assert_eq!(map.size(), 1);

        map.insert(2, "b".to_string());
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdDataMapOfIntegerTransient::new();
        map.insert(5, "item".to_string());
        assert!(map.contains(5));

        assert!(map.remove(5));
        assert!(!map.contains(5));
    }

    #[test]
    fn test_at_mut() {
        let mut map = TColStdDataMapOfIntegerTransient::new();
        map.insert(10, "val".to_string());

        if let Some(s) = map.at_mut(10) {
            *s = "modified".to_string();
        }

        assert_eq!(map.at(10), Some(&"modified".to_string()));
    }
}

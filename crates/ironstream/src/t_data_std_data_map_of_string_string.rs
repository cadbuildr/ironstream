// FILE: t_data_std_data_map_of_string_string.rs
// occt: TDataStd_DataMapOfStringString
// occt-ref: TDataStd_DataMapIteratorOfDataMapOfStringString

//! Deprecated typedef for TDataStd_DataMapOfStringString.
//!
//! In OCCT, this was a data map from ExtendedString to ExtendedString.
//! We implement a minimal map structure using HashMap with NCollection_DataMap semantics.

use std::collections::HashMap;
use std::fmt;

/// TDataStd_DataMapOfStringString: A data map from String to String (deprecated typedef).
/// Wraps a HashMap for O(1) lookups with String keys and String values.
#[derive(Clone)]
pub struct TDataStdDataMapOfStringString {
    data: HashMap<String, String>,
}

impl TDataStdDataMapOfStringString {
    /// Create a new empty map.
    pub fn new() -> Self {
        TDataStdDataMapOfStringString {
            data: HashMap::new(),
        }
    }

    /// Bind a key to a value in the map.
    pub fn bind(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Find a value by key.
    pub fn find(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    /// Check if a key is in the map.
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Return the size of the map.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Check if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear the map.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Return an iterator over the map.
    pub fn iter(&self) -> TDataStdDataMapIteratorOfDataMapOfStringString {
        let pairs: Vec<(String, String)> = self
            .data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        TDataStdDataMapIteratorOfDataMapOfStringString {
            pairs,
            current: 0,
        }
    }
}

impl Default for TDataStdDataMapOfStringString {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TDataStdDataMapOfStringString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TDataStdDataMapOfStringString")
            .field("size", &self.data.len())
            .finish()
    }
}

/// Iterator for TDataStd_DataMapOfStringString.
pub struct TDataStdDataMapIteratorOfDataMapOfStringString {
    pairs: Vec<(String, String)>,
    current: usize,
}

impl TDataStdDataMapIteratorOfDataMapOfStringString {
    /// Check if there is a more item.
    pub fn more(&self) -> bool {
        self.current < self.pairs.len()
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        if self.current < self.pairs.len() {
            self.current += 1;
        }
    }

    /// Get the current key.
    pub fn key(&self) -> Option<String> {
        if self.current < self.pairs.len() {
            Some(self.pairs[self.current].0.clone())
        } else {
            None
        }
    }

    /// Get the current value.
    pub fn value(&self) -> Option<String> {
        if self.current < self.pairs.len() {
            Some(self.pairs[self.current].1.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_map() {
        let map = TDataStdDataMapOfStringString::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TDataStdDataMapOfStringString::new();
        map.bind("key1".to_string(), "value1".to_string());
        map.bind("key2".to_string(), "value2".to_string());

        assert_eq!(map.size(), 2);
        assert_eq!(map.find("key1"), Some("value1".to_string()));
        assert_eq!(map.find("key2"), Some("value2".to_string()));
        assert_eq!(map.find("key3"), None);
    }

    #[test]
    fn test_contains() {
        let mut map = TDataStdDataMapOfStringString::new();
        map.bind("a".to_string(), "x".to_string());
        map.bind("b".to_string(), "y".to_string());

        assert!(map.contains("a"));
        assert!(map.contains("b"));
        assert!(!map.contains("c"));
    }

    #[test]
    fn test_rebind() {
        let mut map = TDataStdDataMapOfStringString::new();
        map.bind("key".to_string(), "old".to_string());
        assert_eq!(map.find("key"), Some("old".to_string()));

        map.bind("key".to_string(), "new".to_string());
        assert_eq!(map.size(), 1);
        assert_eq!(map.find("key"), Some("new".to_string()));
    }

    #[test]
    fn test_iterator() {
        let mut map = TDataStdDataMapOfStringString::new();
        map.bind("x".to_string(), "1".to_string());
        map.bind("y".to_string(), "2".to_string());

        let mut iter = map.iter();
        assert!(iter.more());
        assert!(iter.key().is_some());
        assert!(iter.value().is_some());
        iter.next();

        assert!(iter.more());
        iter.next();

        assert!(!iter.more());
    }

    #[test]
    fn test_clear() {
        let mut map = TDataStdDataMapOfStringString::new();
        map.bind("k1".to_string(), "v1".to_string());
        map.bind("k2".to_string(), "v2".to_string());
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_empty_values() {
        let mut map = TDataStdDataMapOfStringString::new();
        map.bind("empty".to_string(), "".to_string());
        map.bind("nonempty".to_string(), "value".to_string());

        assert_eq!(map.find("empty"), Some("".to_string()));
        assert_eq!(map.find("nonempty"), Some("value".to_string()));
    }

    #[test]
    fn test_debug() {
        let mut map = TDataStdDataMapOfStringString::new();
        map.bind("test".to_string(), "data".to_string());
        let debug_str = format!("{:?}", map);
        assert!(debug_str.contains("size"));
    }
}

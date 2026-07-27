// FILE: t_data_std_data_map_of_string_real.rs
// occt: TDataStd_DataMapOfStringReal
// occt-ref: TDataStd_DataMapIteratorOfDataMapOfStringReal

//! Deprecated typedef for TDataStd_DataMapOfStringReal.
//!
//! In OCCT, this was a data map from ExtendedString to double.
//! We implement a minimal map structure using HashMap with NCollection_DataMap semantics.

use std::collections::HashMap;
use std::fmt;

/// TDataStd_DataMapOfStringReal: A data map from String to f64 (deprecated typedef).
/// Wraps a HashMap for O(1) lookups with String keys and double values.
#[derive(Clone)]
pub struct TDataStdDataMapOfStringReal {
    data: HashMap<String, f64>,
}

impl TDataStdDataMapOfStringReal {
    /// Create a new empty map.
    pub fn new() -> Self {
        TDataStdDataMapOfStringReal {
            data: HashMap::new(),
        }
    }

    /// Bind a key to a real value in the map.
    pub fn bind(&mut self, key: String, value: f64) {
        self.data.insert(key, value);
    }

    /// Find a value by key.
    pub fn find(&self, key: &str) -> Option<f64> {
        self.data.get(key).copied()
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
    pub fn iter(&self) -> TDataStdDataMapIteratorOfDataMapOfStringReal {
        let pairs: Vec<(String, f64)> = self
            .data
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        TDataStdDataMapIteratorOfDataMapOfStringReal {
            pairs,
            current: 0,
        }
    }
}

impl Default for TDataStdDataMapOfStringReal {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TDataStdDataMapOfStringReal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TDataStdDataMapOfStringReal")
            .field("size", &self.data.len())
            .finish()
    }
}

/// Iterator for TDataStd_DataMapOfStringReal.
pub struct TDataStdDataMapIteratorOfDataMapOfStringReal {
    pairs: Vec<(String, f64)>,
    current: usize,
}

impl TDataStdDataMapIteratorOfDataMapOfStringReal {
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
    pub fn value(&self) -> Option<f64> {
        if self.current < self.pairs.len() {
            Some(self.pairs[self.current].1)
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
        let map = TDataStdDataMapOfStringReal::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TDataStdDataMapOfStringReal::new();
        map.bind("key1".to_string(), 3.14);
        map.bind("key2".to_string(), 2.71);

        assert_eq!(map.size(), 2);
        assert_eq!(map.find("key1"), Some(3.14));
        assert_eq!(map.find("key2"), Some(2.71));
        assert_eq!(map.find("key3"), None);
    }

    #[test]
    fn test_contains() {
        let mut map = TDataStdDataMapOfStringReal::new();
        map.bind("a".to_string(), 1.5);
        map.bind("b".to_string(), 2.5);

        assert!(map.contains("a"));
        assert!(map.contains("b"));
        assert!(!map.contains("c"));
    }

    #[test]
    fn test_real_values() {
        let mut map = TDataStdDataMapOfStringReal::new();
        map.bind("zero".to_string(), 0.0);
        map.bind("negative".to_string(), -1.5);
        map.bind("positive".to_string(), 99.99);

        assert_eq!(map.find("zero"), Some(0.0));
        assert_eq!(map.find("negative"), Some(-1.5));
        assert_eq!(map.find("positive"), Some(99.99));
    }

    #[test]
    fn test_rebind() {
        let mut map = TDataStdDataMapOfStringReal::new();
        map.bind("key".to_string(), 10.5);
        assert_eq!(map.find("key"), Some(10.5));

        map.bind("key".to_string(), 20.5);
        assert_eq!(map.size(), 1);
        assert_eq!(map.find("key"), Some(20.5));
    }

    #[test]
    fn test_iterator() {
        let mut map = TDataStdDataMapOfStringReal::new();
        map.bind("x".to_string(), 1.1);
        map.bind("y".to_string(), 2.2);

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
        let mut map = TDataStdDataMapOfStringReal::new();
        map.bind("k1".to_string(), 1.1);
        map.bind("k2".to_string(), 2.2);
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_debug() {
        let mut map = TDataStdDataMapOfStringReal::new();
        map.bind("test".to_string(), 7.5);
        let debug_str = format!("{:?}", map);
        assert!(debug_str.contains("size"));
    }
}

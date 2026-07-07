// FILE: t_data_std_data_map_of_string_byte.rs
// occt: TDataStd_DataMapOfStringByte, TDataStd_DataMapIteratorOfDataMapOfStringByte

//! Deprecated typedef for TDataStd_DataMapOfStringByte.
//!
//! In OCCT, this was a data map from ExtendedString to uint8_t.
//! We implement a minimal map structure using HashMap with NCollection_DataMap semantics.

use std::collections::HashMap;
use std::fmt;

/// TDataStd_DataMapOfStringByte: A data map from String to uint8_t (deprecated typedef).
/// Wraps a HashMap for O(1) lookups with String keys and byte values.
#[derive(Clone)]
pub struct TDataStdDataMapOfStringByte {
    data: HashMap<String, u8>,
}

impl TDataStdDataMapOfStringByte {
    /// Create a new empty map.
    pub fn new() -> Self {
        TDataStdDataMapOfStringByte {
            data: HashMap::new(),
        }
    }

    /// Bind a key to a byte value in the map.
    pub fn bind(&mut self, key: String, value: u8) {
        self.data.insert(key, value);
    }

    /// Find a value by key.
    pub fn find(&self, key: &str) -> Option<u8> {
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
    pub fn iter(&self) -> TDataStdDataMapIteratorOfDataMapOfStringByte {
        let pairs: Vec<(String, u8)> = self
            .data
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        TDataStdDataMapIteratorOfDataMapOfStringByte {
            pairs,
            current: 0,
        }
    }
}

impl Default for TDataStdDataMapOfStringByte {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TDataStdDataMapOfStringByte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TDataStdDataMapOfStringByte")
            .field("size", &self.data.len())
            .finish()
    }
}

/// Iterator for TDataStd_DataMapOfStringByte.
pub struct TDataStdDataMapIteratorOfDataMapOfStringByte {
    pairs: Vec<(String, u8)>,
    current: usize,
}

impl TDataStdDataMapIteratorOfDataMapOfStringByte {
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
    pub fn value(&self) -> Option<u8> {
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
        let map = TDataStdDataMapOfStringByte::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TDataStdDataMapOfStringByte::new();
        map.bind("key1".to_string(), 42);
        map.bind("key2".to_string(), 255);

        assert_eq!(map.size(), 2);
        assert_eq!(map.find("key1"), Some(42));
        assert_eq!(map.find("key2"), Some(255));
        assert_eq!(map.find("key3"), None);
    }

    #[test]
    fn test_contains() {
        let mut map = TDataStdDataMapOfStringByte::new();
        map.bind("a".to_string(), 10);
        map.bind("b".to_string(), 20);

        assert!(map.contains("a"));
        assert!(map.contains("b"));
        assert!(!map.contains("c"));
    }

    #[test]
    fn test_byte_values() {
        let mut map = TDataStdDataMapOfStringByte::new();
        map.bind("zero".to_string(), 0);
        map.bind("max".to_string(), 255);
        map.bind("mid".to_string(), 128);

        assert_eq!(map.find("zero"), Some(0));
        assert_eq!(map.find("max"), Some(255));
        assert_eq!(map.find("mid"), Some(128));
    }

    #[test]
    fn test_rebind() {
        let mut map = TDataStdDataMapOfStringByte::new();
        map.bind("key".to_string(), 10);
        assert_eq!(map.find("key"), Some(10));

        map.bind("key".to_string(), 20);
        assert_eq!(map.size(), 1);
        assert_eq!(map.find("key"), Some(20));
    }

    #[test]
    fn test_iterator() {
        let mut map = TDataStdDataMapOfStringByte::new();
        map.bind("x".to_string(), 1);
        map.bind("y".to_string(), 2);

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
        let mut map = TDataStdDataMapOfStringByte::new();
        map.bind("k1".to_string(), 1);
        map.bind("k2".to_string(), 2);
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_debug() {
        let mut map = TDataStdDataMapOfStringByte::new();
        map.bind("test".to_string(), 99);
        let debug_str = format!("{:?}", map);
        assert!(debug_str.contains("size"));
    }
}

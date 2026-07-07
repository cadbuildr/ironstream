// FILE: tdf_label_integer_map.rs
// occt: TDF_LabelIntegerMap, TDF_DataMapIteratorOfLabelIntegerMap

//! Deprecated typedef for TDF_LabelIntegerMap.
//!
//! In OCCT, this was a data map from TDF_Label to int.
//! We implement a minimal map structure using HashMap with NCollection_DataMap semantics.

use std::collections::HashMap;
use std::fmt;

/// TDF_LabelIntegerMap: A data map from TDF_Label to int (deprecated typedef).
/// Wraps a HashMap for O(1) lookups with TDF_Label keys and integer values.
#[derive(Clone)]
pub struct TdfLabelIntegerMap {
    data: HashMap<String, i32>,  // Placeholder: would be HashMap<TdfLabel, i32> in full port
}

impl TdfLabelIntegerMap {
    /// Create a new empty map.
    pub fn new() -> Self {
        TdfLabelIntegerMap {
            data: HashMap::new(),
        }
    }

    /// Bind a key to an integer value in the map.
    pub fn bind(&mut self, key: String, value: i32) {
        self.data.insert(key, value);
    }

    /// Find a value by key.
    pub fn find(&self, key: &str) -> Option<i32> {
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
    pub fn iter(&self) -> TdfDataMapIteratorOfLabelIntegerMap {
        let pairs: Vec<(String, i32)> = self
            .data
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        TdfDataMapIteratorOfLabelIntegerMap {
            pairs,
            current: 0,
        }
    }
}

impl Default for TdfLabelIntegerMap {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TdfLabelIntegerMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TdfLabelIntegerMap")
            .field("size", &self.data.len())
            .finish()
    }
}

/// Iterator for TDF_LabelIntegerMap.
pub struct TdfDataMapIteratorOfLabelIntegerMap {
    pairs: Vec<(String, i32)>,
    current: usize,
}

impl TdfDataMapIteratorOfLabelIntegerMap {
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
    pub fn value(&self) -> Option<i32> {
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
        let map = TdfLabelIntegerMap::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TdfLabelIntegerMap::new();
        map.bind("label1".to_string(), 10);
        map.bind("label2".to_string(), 20);

        assert_eq!(map.size(), 2);
        assert_eq!(map.find("label1"), Some(10));
        assert_eq!(map.find("label2"), Some(20));
        assert_eq!(map.find("label3"), None);
    }

    #[test]
    fn test_contains() {
        let mut map = TdfLabelIntegerMap::new();
        map.bind("k1".to_string(), 1);
        map.bind("k2".to_string(), 2);

        assert!(map.contains("k1"));
        assert!(map.contains("k2"));
        assert!(!map.contains("k3"));
    }

    #[test]
    fn test_rebind() {
        let mut map = TdfLabelIntegerMap::new();
        map.bind("key".to_string(), 5);
        assert_eq!(map.find("key"), Some(5));

        map.bind("key".to_string(), 15);
        assert_eq!(map.size(), 1);
        assert_eq!(map.find("key"), Some(15));
    }

    #[test]
    fn test_iterator() {
        let mut map = TdfLabelIntegerMap::new();
        map.bind("a".to_string(), 100);
        map.bind("b".to_string(), 200);

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
    fn test_integer_values() {
        let mut map = TdfLabelIntegerMap::new();
        map.bind("negative".to_string(), -42);
        map.bind("positive".to_string(), 42);
        map.bind("zero".to_string(), 0);

        assert_eq!(map.find("negative"), Some(-42));
        assert_eq!(map.find("positive"), Some(42));
        assert_eq!(map.find("zero"), Some(0));
    }

    #[test]
    fn test_clear() {
        let mut map = TdfLabelIntegerMap::new();
        map.bind("k1".to_string(), 1);
        map.bind("k2".to_string(), 2);
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_debug() {
        let mut map = TdfLabelIntegerMap::new();
        map.bind("x".to_string(), 99);
        let debug_str = format!("{:?}", map);
        assert!(debug_str.contains("size"));
    }
}

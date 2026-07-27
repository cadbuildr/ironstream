// FILE: tdf_label_data_map.rs
// occt: TDF_LabelDataMap
// occt-ref: TDF_DataMapIteratorOfLabelDataMap

//! Deprecated typedef for TDF_LabelDataMap.
//!
//! In OCCT, this was a data map from TDF_Label to TDF_Label.
//! We implement a minimal map structure using HashMap with NCollection_DataMap semantics.

use std::collections::HashMap;
use std::fmt;

/// TDF_LabelDataMap: A data map from TDF_Label to TDF_Label (deprecated typedef).
/// Wraps a HashMap for O(1) lookups with TDF_Label keys and values.
#[derive(Clone)]
pub struct TdfLabelDataMap {
    data: HashMap<String, String>,  // Placeholder: would be HashMap<TdfLabel, TdfLabel> in full port
}

impl TdfLabelDataMap {
    /// Create a new empty map.
    pub fn new() -> Self {
        TdfLabelDataMap {
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
    pub fn iter(&self) -> TdfDataMapIteratorOfLabelDataMap {
        let pairs: Vec<(String, String)> = self
            .data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        TdfDataMapIteratorOfLabelDataMap {
            pairs,
            current: 0,
        }
    }
}

impl Default for TdfLabelDataMap {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TdfLabelDataMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TdfLabelDataMap")
            .field("size", &self.data.len())
            .finish()
    }
}

/// Iterator for TDF_LabelDataMap.
pub struct TdfDataMapIteratorOfLabelDataMap {
    pairs: Vec<(String, String)>,
    current: usize,
}

impl TdfDataMapIteratorOfLabelDataMap {
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
        let map = TdfLabelDataMap::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TdfLabelDataMap::new();
        map.bind("label1".to_string(), "value1".to_string());
        map.bind("label2".to_string(), "value2".to_string());

        assert_eq!(map.size(), 2);
        assert_eq!(map.find("label1"), Some("value1".to_string()));
        assert_eq!(map.find("label2"), Some("value2".to_string()));
        assert_eq!(map.find("label3"), None);
    }

    #[test]
    fn test_contains() {
        let mut map = TdfLabelDataMap::new();
        map.bind("k1".to_string(), "v1".to_string());
        map.bind("k2".to_string(), "v2".to_string());

        assert!(map.contains("k1"));
        assert!(map.contains("k2"));
        assert!(!map.contains("k3"));
    }

    #[test]
    fn test_rebind() {
        let mut map = TdfLabelDataMap::new();
        map.bind("key".to_string(), "value1".to_string());
        assert_eq!(map.find("key"), Some("value1".to_string()));

        map.bind("key".to_string(), "value2".to_string());
        assert_eq!(map.size(), 1);
        assert_eq!(map.find("key"), Some("value2".to_string()));
    }

    #[test]
    fn test_iterator() {
        let mut map = TdfLabelDataMap::new();
        map.bind("a".to_string(), "1".to_string());
        map.bind("b".to_string(), "2".to_string());

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
        let mut map = TdfLabelDataMap::new();
        map.bind("k1".to_string(), "v1".to_string());
        map.bind("k2".to_string(), "v2".to_string());
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_debug() {
        let mut map = TdfLabelDataMap::new();
        map.bind("x".to_string(), "y".to_string());
        let debug_str = format!("{:?}", map);
        assert!(debug_str.contains("size"));
    }
}

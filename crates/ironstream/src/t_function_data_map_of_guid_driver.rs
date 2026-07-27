// FILE: t_function_data_map_of_guid_driver.rs
// occt: TFunction_DataMapOfGUIDDriver
// occt-ref: TFunction_DataMapIteratorOfDataMapOfGUIDDriver

//! Deprecated typedef for TFunction_DataMapOfGUIDDriver.
//!
//! In OCCT, this was a data map from Standard_GUID to a handle of TFunction_Driver.
//! We implement a minimal map structure using HashMap with NCollection_DataMap semantics.

use std::collections::HashMap;
use std::fmt;

/// TFunction_DataMapOfGUIDDriver: A data map from GUID to Driver (deprecated typedef).
/// Wraps a HashMap for O(1) lookups with GUID keys and driver ID values.
#[derive(Clone)]
pub struct TFunctionDataMapOfGuidDriver {
    data: HashMap<String, i32>,  // Placeholder: would be HashMap<StandardGuid, Handle<TFunctionDriver>> in full port
}

impl TFunctionDataMapOfGuidDriver {
    /// Create a new empty map.
    pub fn new() -> Self {
        TFunctionDataMapOfGuidDriver {
            data: HashMap::new(),
        }
    }

    /// Bind a GUID key to a driver value in the map.
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
    pub fn iter(&self) -> TFunctionDataMapIteratorOfDataMapOfGuidDriver {
        let pairs: Vec<(String, i32)> = self
            .data
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        TFunctionDataMapIteratorOfDataMapOfGuidDriver {
            pairs,
            current: 0,
        }
    }
}

impl Default for TFunctionDataMapOfGuidDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TFunctionDataMapOfGuidDriver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TFunctionDataMapOfGuidDriver")
            .field("size", &self.data.len())
            .finish()
    }
}

/// Iterator for TFunction_DataMapOfGUIDDriver.
pub struct TFunctionDataMapIteratorOfDataMapOfGuidDriver {
    pairs: Vec<(String, i32)>,
    current: usize,
}

impl TFunctionDataMapIteratorOfDataMapOfGuidDriver {
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
        let map = TFunctionDataMapOfGuidDriver::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TFunctionDataMapOfGuidDriver::new();
        map.bind("guid1".to_string(), 10);
        map.bind("guid2".to_string(), 20);

        assert_eq!(map.size(), 2);
        assert_eq!(map.find("guid1"), Some(10));
        assert_eq!(map.find("guid2"), Some(20));
        assert_eq!(map.find("guid3"), None);
    }

    #[test]
    fn test_contains() {
        let mut map = TFunctionDataMapOfGuidDriver::new();
        map.bind("g1".to_string(), 1);
        map.bind("g2".to_string(), 2);

        assert!(map.contains("g1"));
        assert!(map.contains("g2"));
        assert!(!map.contains("g3"));
    }

    #[test]
    fn test_rebind() {
        let mut map = TFunctionDataMapOfGuidDriver::new();
        map.bind("guid".to_string(), 5);
        assert_eq!(map.find("guid"), Some(5));

        map.bind("guid".to_string(), 15);
        assert_eq!(map.size(), 1);
        assert_eq!(map.find("guid"), Some(15));
    }

    #[test]
    fn test_iterator() {
        let mut map = TFunctionDataMapOfGuidDriver::new();
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
        let mut map = TFunctionDataMapOfGuidDriver::new();
        map.bind("k1".to_string(), 1);
        map.bind("k2".to_string(), 2);
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_debug() {
        let mut map = TFunctionDataMapOfGuidDriver::new();
        map.bind("test".to_string(), 42);
        let debug_str = format!("{:?}", map);
        assert!(debug_str.contains("size"));
    }
}

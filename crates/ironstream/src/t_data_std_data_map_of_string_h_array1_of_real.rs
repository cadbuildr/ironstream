// FILE: t_data_std_data_map_of_string_h_array1_of_real.rs
// occt: TDataStd_DataMapOfStringHArray1OfReal, TDataStd_DataMapIteratorOfDataMapOfStringHArray1OfReal

//! Deprecated typedef for TDataStd_DataMapOfStringHArray1OfReal.
//!
//! In OCCT, this was a data map from ExtendedString to a handle of HArray1OfReal.
//! We implement a map using HashMap with Vec<f64> as placeholder for array handles.

use std::collections::HashMap;
use std::fmt;

/// TDataStd_DataMapOfStringHArray1OfReal: A data map from String to array of reals (deprecated typedef).
/// Wraps a HashMap for O(1) lookups with String keys and Vec<f64> values.
#[derive(Clone)]
pub struct TDataStdDataMapOfStringHArray1OfReal {
    data: HashMap<String, Vec<f64>>,
}

impl TDataStdDataMapOfStringHArray1OfReal {
    /// Create a new empty map.
    pub fn new() -> Self {
        TDataStdDataMapOfStringHArray1OfReal {
            data: HashMap::new(),
        }
    }

    /// Bind a key to an array value in the map.
    pub fn bind(&mut self, key: String, value: Vec<f64>) {
        self.data.insert(key, value);
    }

    /// Find a value by key.
    pub fn find(&self, key: &str) -> Option<Vec<f64>> {
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
    pub fn iter(&self) -> TDataStdDataMapIteratorOfDataMapOfStringHArray1OfReal {
        let pairs: Vec<(String, Vec<f64>)> = self
            .data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        TDataStdDataMapIteratorOfDataMapOfStringHArray1OfReal {
            pairs,
            current: 0,
        }
    }
}

impl Default for TDataStdDataMapOfStringHArray1OfReal {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TDataStdDataMapOfStringHArray1OfReal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TDataStdDataMapOfStringHArray1OfReal")
            .field("size", &self.data.len())
            .finish()
    }
}

/// Iterator for TDataStd_DataMapOfStringHArray1OfReal.
pub struct TDataStdDataMapIteratorOfDataMapOfStringHArray1OfReal {
    pairs: Vec<(String, Vec<f64>)>,
    current: usize,
}

impl TDataStdDataMapIteratorOfDataMapOfStringHArray1OfReal {
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

    /// Get the current value (array).
    pub fn value(&self) -> Option<Vec<f64>> {
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
        let map = TDataStdDataMapOfStringHArray1OfReal::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TDataStdDataMapOfStringHArray1OfReal::new();
        let arr1 = vec![1.5, 2.5, 3.5];
        let arr2 = vec![4.0, 5.0];

        map.bind("key1".to_string(), arr1.clone());
        map.bind("key2".to_string(), arr2.clone());

        assert_eq!(map.size(), 2);
        assert_eq!(map.find("key1"), Some(arr1));
        assert_eq!(map.find("key2"), Some(arr2));
        assert_eq!(map.find("key3"), None);
    }

    #[test]
    fn test_contains() {
        let mut map = TDataStdDataMapOfStringHArray1OfReal::new();
        map.bind("a".to_string(), vec![1.0]);
        map.bind("b".to_string(), vec![2.0]);

        assert!(map.contains("a"));
        assert!(map.contains("b"));
        assert!(!map.contains("c"));
    }

    #[test]
    fn test_empty_array() {
        let mut map = TDataStdDataMapOfStringHArray1OfReal::new();
        map.bind("empty".to_string(), vec![]);

        assert_eq!(map.find("empty"), Some(vec![]));
    }

    #[test]
    fn test_rebind() {
        let mut map = TDataStdDataMapOfStringHArray1OfReal::new();
        let arr1 = vec![1.1, 2.2];
        let arr2 = vec![3.3, 4.4, 5.5];

        map.bind("key".to_string(), arr1);
        assert_eq!(map.size(), 1);
        assert_eq!(map.find("key"), Some(vec![1.1, 2.2]));

        map.bind("key".to_string(), arr2);
        assert_eq!(map.size(), 1);
        assert_eq!(map.find("key"), Some(vec![3.3, 4.4, 5.5]));
    }

    #[test]
    fn test_iterator() {
        let mut map = TDataStdDataMapOfStringHArray1OfReal::new();
        map.bind("x".to_string(), vec![1.5, 2.5]);
        map.bind("y".to_string(), vec![3.5, 4.5]);

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
        let mut map = TDataStdDataMapOfStringHArray1OfReal::new();
        map.bind("k1".to_string(), vec![1.0, 2.0, 3.0]);
        map.bind("k2".to_string(), vec![4.0, 5.0]);
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }
}

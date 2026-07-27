// FILE: t_function_data_map_of_label_list_of_label.rs
// occt: TFunction_DataMapOfLabelListOfLabel
// occt-ref: TFunction_DataMapIteratorOfDataMapOfLabelListOfLabel

//! Deprecated typedef for TFunction_DataMapOfLabelListOfLabel.
//!
//! In OCCT, this was a data map from TDF_Label to TDF_LabelList.
//! We implement a minimal map structure using HashMap with Vec<String> as label lists.

use std::collections::HashMap;
use std::fmt;

/// TFunction_DataMapOfLabelListOfLabel: A data map from Label to LabelList (deprecated typedef).
/// Wraps a HashMap for O(1) lookups with Label keys and LabelList values.
#[derive(Clone)]
pub struct TFunctionDataMapOfLabelListOfLabel {
    data: HashMap<String, Vec<String>>,  // label -> list of labels
}

impl TFunctionDataMapOfLabelListOfLabel {
    /// Create a new empty map.
    pub fn new() -> Self {
        TFunctionDataMapOfLabelListOfLabel {
            data: HashMap::new(),
        }
    }

    /// Bind a key to a label list value in the map.
    pub fn bind(&mut self, key: String, value: Vec<String>) {
        self.data.insert(key, value);
    }

    /// Find a value by key.
    pub fn find(&self, key: &str) -> Option<Vec<String>> {
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
    pub fn iter(&self) -> TFunctionDataMapIteratorOfDataMapOfLabelListOfLabel {
        let pairs: Vec<(String, Vec<String>)> = self
            .data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        TFunctionDataMapIteratorOfDataMapOfLabelListOfLabel {
            pairs,
            current: 0,
        }
    }
}

impl Default for TFunctionDataMapOfLabelListOfLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TFunctionDataMapOfLabelListOfLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TFunctionDataMapOfLabelListOfLabel")
            .field("size", &self.data.len())
            .finish()
    }
}

/// Iterator for TFunction_DataMapOfLabelListOfLabel.
pub struct TFunctionDataMapIteratorOfDataMapOfLabelListOfLabel {
    pairs: Vec<(String, Vec<String>)>,
    current: usize,
}

impl TFunctionDataMapIteratorOfDataMapOfLabelListOfLabel {
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
    pub fn value(&self) -> Option<Vec<String>> {
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
        let map = TFunctionDataMapOfLabelListOfLabel::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TFunctionDataMapOfLabelListOfLabel::new();
        let list1 = vec!["a".to_string(), "b".to_string()];
        let list2 = vec!["c".to_string()];

        map.bind("label1".to_string(), list1.clone());
        map.bind("label2".to_string(), list2.clone());

        assert_eq!(map.size(), 2);
        assert_eq!(map.find("label1"), Some(list1));
        assert_eq!(map.find("label2"), Some(list2));
        assert_eq!(map.find("label3"), None);
    }

    #[test]
    fn test_contains() {
        let mut map = TFunctionDataMapOfLabelListOfLabel::new();
        map.bind("l1".to_string(), vec!["x".to_string()]);

        assert!(map.contains("l1"));
        assert!(!map.contains("l2"));
    }

    #[test]
    fn test_empty_list() {
        let mut map = TFunctionDataMapOfLabelListOfLabel::new();
        map.bind("key".to_string(), vec![]);

        assert_eq!(map.find("key"), Some(vec![]));
    }

    #[test]
    fn test_rebind() {
        let mut map = TFunctionDataMapOfLabelListOfLabel::new();
        let list1 = vec!["old".to_string()];
        let list2 = vec!["new1".to_string(), "new2".to_string()];

        map.bind("key".to_string(), list1);
        assert_eq!(map.size(), 1);

        map.bind("key".to_string(), list2.clone());
        assert_eq!(map.size(), 1);
        assert_eq!(map.find("key"), Some(list2));
    }

    #[test]
    fn test_iterator() {
        let mut map = TFunctionDataMapOfLabelListOfLabel::new();
        map.bind("x".to_string(), vec!["1".to_string()]);
        map.bind("y".to_string(), vec!["2".to_string()]);

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
        let mut map = TFunctionDataMapOfLabelListOfLabel::new();
        map.bind("k1".to_string(), vec!["a".to_string()]);
        map.bind("k2".to_string(), vec!["b".to_string()]);
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }
}

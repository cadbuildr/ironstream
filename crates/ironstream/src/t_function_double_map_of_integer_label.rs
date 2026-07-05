// FILE: t_function_double_map_of_integer_label.rs
// occt: TFunction_DoubleMapOfIntegerLabel, TFunction_DoubleMapIteratorOfDoubleMapOfIntegerLabel

//! Deprecated typedef for TFunction_DoubleMapOfIntegerLabel.
//!
//! In OCCT, this was a bidirectional map from int to TDF_Label.
//! We implement a minimal double-map structure with forward and reverse lookups.

use std::collections::HashMap;
use std::fmt;

/// TFunction_DoubleMapOfIntegerLabel: A bidirectional map between int and Label (deprecated typedef).
/// Uses two HashMaps to maintain bidirectional lookups.
#[derive(Clone)]
pub struct TFunctionDoubleMapOfIntegerLabel {
    forward: HashMap<i32, String>,  // int -> label
    reverse: HashMap<String, i32>,  // label -> int
}

impl TFunctionDoubleMapOfIntegerLabel {
    /// Create a new empty map.
    pub fn new() -> Self {
        TFunctionDoubleMapOfIntegerLabel {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    /// Bind two values bidirectionally.
    pub fn bind(&mut self, int_val: i32, label: String) {
        // Remove old associations if they exist
        if let Some(old_label) = self.forward.remove(&int_val) {
            self.reverse.remove(&old_label);
        }
        if let Some(old_int) = self.reverse.remove(&label) {
            self.forward.remove(&old_int);
        }

        // Add new bidirectional binding
        self.forward.insert(int_val, label.clone());
        self.reverse.insert(label, int_val);
    }

    /// Find the label from the int.
    pub fn find1(&self, int_val: i32) -> Option<String> {
        self.forward.get(&int_val).cloned()
    }

    /// Find the int from the label.
    pub fn find2(&self, label: &str) -> Option<i32> {
        self.reverse.get(label).copied()
    }

    /// Return the size of the map.
    pub fn size(&self) -> usize {
        self.forward.len()
    }

    /// Check if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }

    /// Clear the map.
    pub fn clear(&mut self) {
        self.forward.clear();
        self.reverse.clear();
    }

    /// Return an iterator over the map.
    pub fn iter(&self) -> TFunctionDoubleMapIteratorOfDoubleMapOfIntegerLabel {
        let pairs: Vec<(i32, String)> = self
            .forward
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();
        TFunctionDoubleMapIteratorOfDoubleMapOfIntegerLabel {
            pairs,
            current: 0,
        }
    }
}

impl Default for TFunctionDoubleMapOfIntegerLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TFunctionDoubleMapOfIntegerLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TFunctionDoubleMapOfIntegerLabel")
            .field("size", &self.forward.len())
            .finish()
    }
}

/// Iterator for TFunction_DoubleMapOfIntegerLabel.
pub struct TFunctionDoubleMapIteratorOfDoubleMapOfIntegerLabel {
    pairs: Vec<(i32, String)>,
    current: usize,
}

impl TFunctionDoubleMapIteratorOfDoubleMapOfIntegerLabel {
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

    /// Get the current key (int).
    pub fn key(&self) -> Option<i32> {
        if self.current < self.pairs.len() {
            Some(self.pairs[self.current].0)
        } else {
            None
        }
    }

    /// Get the current value (label).
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
        let map = TFunctionDoubleMapOfIntegerLabel::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TFunctionDoubleMapOfIntegerLabel::new();
        map.bind(1, "label1".to_string());
        map.bind(2, "label2".to_string());

        assert_eq!(map.size(), 2);
        assert_eq!(map.find1(1), Some("label1".to_string()));
        assert_eq!(map.find1(2), Some("label2".to_string()));
        assert_eq!(map.find2("label1"), Some(1));
        assert_eq!(map.find2("label2"), Some(2));
    }

    #[test]
    fn test_bidirectional_lookup() {
        let mut map = TFunctionDoubleMapOfIntegerLabel::new();
        map.bind(42, "label".to_string());

        assert_eq!(map.find1(42), Some("label".to_string()));
        assert_eq!(map.find2("label"), Some(42));
    }

    #[test]
    fn test_rebind() {
        let mut map = TFunctionDoubleMapOfIntegerLabel::new();
        map.bind(1, "old".to_string());
        assert_eq!(map.size(), 1);

        map.bind(1, "new".to_string());
        assert_eq!(map.size(), 1);
        assert_eq!(map.find1(1), Some("new".to_string()));
        assert_eq!(map.find2("old"), None);
        assert_eq!(map.find2("new"), Some(1));
    }

    #[test]
    fn test_iterator() {
        let mut map = TFunctionDoubleMapOfIntegerLabel::new();
        map.bind(1, "a".to_string());
        map.bind(2, "b".to_string());

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
        let mut map = TFunctionDoubleMapOfIntegerLabel::new();
        map.bind(1, "l1".to_string());
        map.bind(2, "l2".to_string());
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_find_not_found() {
        let map = TFunctionDoubleMapOfIntegerLabel::new();
        assert_eq!(map.find1(999), None);
        assert_eq!(map.find2("nonexistent"), None);
    }
}

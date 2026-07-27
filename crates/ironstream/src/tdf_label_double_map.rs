// FILE: tdf_label_double_map.rs
// occt: TDF_LabelDoubleMap
// occt-ref: TDF_DoubleMapIteratorOfLabelDoubleMap

//! Deprecated typedef for TDF_LabelDoubleMap.
//!
//! In OCCT, this was a bidirectional map from TDF_Label to TDF_Label.
//! We implement a minimal double-map structure with forward and reverse lookups.

use std::collections::HashMap;
use std::fmt;

/// TDF_LabelDoubleMap: A bidirectional map between TDF_Label and TDF_Label (deprecated typedef).
/// Uses two HashMaps to maintain bidirectional lookups.
#[derive(Clone)]
pub struct TdfLabelDoubleMap {
    forward: HashMap<String, String>,  // label1 -> label2
    reverse: HashMap<String, String>,  // label2 -> label1
}

impl TdfLabelDoubleMap {
    /// Create a new empty map.
    pub fn new() -> Self {
        TdfLabelDoubleMap {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    /// Bind two labels bidirectionally.
    pub fn bind(&mut self, label1: String, label2: String) {
        // Remove old associations if they exist
        if let Some(old_label2) = self.forward.remove(&label1) {
            self.reverse.remove(&old_label2);
        }
        if let Some(old_label1) = self.reverse.remove(&label2) {
            self.forward.remove(&old_label1);
        }

        // Add new bidirectional binding
        self.forward.insert(label1.clone(), label2.clone());
        self.reverse.insert(label2, label1);
    }

    /// Find the second label from the first.
    pub fn find1(&self, label1: &str) -> Option<String> {
        self.forward.get(label1).cloned()
    }

    /// Find the first label from the second.
    pub fn find2(&self, label2: &str) -> Option<String> {
        self.reverse.get(label2).cloned()
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
    pub fn iter(&self) -> TdfDoubleMapIteratorOfLabelDoubleMap {
        let pairs: Vec<(String, String)> = self
            .forward
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        TdfDoubleMapIteratorOfLabelDoubleMap {
            pairs,
            current: 0,
        }
    }
}

impl Default for TdfLabelDoubleMap {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TdfLabelDoubleMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TdfLabelDoubleMap")
            .field("size", &self.forward.len())
            .finish()
    }
}

/// Iterator for TDF_LabelDoubleMap.
pub struct TdfDoubleMapIteratorOfLabelDoubleMap {
    pairs: Vec<(String, String)>,
    current: usize,
}

impl TdfDoubleMapIteratorOfLabelDoubleMap {
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

    /// Get the current key (first label).
    pub fn key(&self) -> Option<String> {
        if self.current < self.pairs.len() {
            Some(self.pairs[self.current].0.clone())
        } else {
            None
        }
    }

    /// Get the current value (second label).
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
        let map = TdfLabelDoubleMap::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TdfLabelDoubleMap::new();
        map.bind("label1".to_string(), "label2".to_string());
        map.bind("label3".to_string(), "label4".to_string());

        assert_eq!(map.size(), 2);
        assert_eq!(map.find1("label1"), Some("label2".to_string()));
        assert_eq!(map.find1("label3"), Some("label4".to_string()));
        assert_eq!(map.find2("label2"), Some("label1".to_string()));
        assert_eq!(map.find2("label4"), Some("label3".to_string()));
    }

    #[test]
    fn test_bidirectional_lookup() {
        let mut map = TdfLabelDoubleMap::new();
        map.bind("a".to_string(), "x".to_string());

        assert_eq!(map.find1("a"), Some("x".to_string()));
        assert_eq!(map.find2("x"), Some("a".to_string()));
    }

    #[test]
    fn test_rebind() {
        let mut map = TdfLabelDoubleMap::new();
        map.bind("l1".to_string(), "l2".to_string());
        assert_eq!(map.size(), 1);

        // Rebind the same first label to a new second label
        map.bind("l1".to_string(), "l3".to_string());
        assert_eq!(map.size(), 1);
        assert_eq!(map.find1("l1"), Some("l3".to_string()));
        assert_eq!(map.find2("l2"), None);
        assert_eq!(map.find2("l3"), Some("l1".to_string()));
    }

    #[test]
    fn test_iterator() {
        let mut map = TdfLabelDoubleMap::new();
        map.bind("a".to_string(), "x".to_string());
        map.bind("b".to_string(), "y".to_string());

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
        let mut map = TdfLabelDoubleMap::new();
        map.bind("l1".to_string(), "l2".to_string());
        map.bind("l3".to_string(), "l4".to_string());
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_find_not_found() {
        let map = TdfLabelDoubleMap::new();
        assert_eq!(map.find1("nonexistent"), None);
        assert_eq!(map.find2("nonexistent"), None);
    }
}

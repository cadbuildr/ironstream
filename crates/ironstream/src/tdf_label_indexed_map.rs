// FILE: tdf_label_indexed_map.rs
// occt: TDF_LabelIndexedMap

//! Deprecated typedef for TDF_LabelIndexedMap.
//!
//! In OCCT, this was an indexed map for TDF_Label items.
//! We implement a structure using Vec with 1-based indexing for O(1) lookups by index.

use std::collections::HashMap;
use std::fmt;

/// TDF_LabelIndexedMap: An indexed map for TDF_Label items (deprecated typedef).
/// Maintains both a Vec for 1-based index access and a HashMap for fast lookup.
#[derive(Clone)]
pub struct TdfLabelIndexedMap {
    items: Vec<String>,              // Placeholder: would be Vec<TdfLabel> in full port
    index_map: HashMap<String, usize>,  // label -> 1-based index
}

impl TdfLabelIndexedMap {
    /// Create a new empty indexed map.
    pub fn new() -> Self {
        TdfLabelIndexedMap {
            items: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    /// Add an item to the map, returning its 1-based index.
    pub fn add(&mut self, item: String) -> usize {
        if self.index_map.contains_key(&item) {
            return *self.index_map.get(&item).unwrap();
        }
        let idx = self.items.len() + 1;  // 1-based index
        self.items.push(item.clone());
        self.index_map.insert(item, idx);
        idx
    }

    /// Find the 1-based index of an item.
    pub fn find_index(&self, item: &str) -> Option<usize> {
        self.index_map.get(item).copied()
    }

    /// Get the item at a 1-based index.
    pub fn find_from_index(&self, index: usize) -> Option<String> {
        if index >= 1 && index <= self.items.len() {
            Some(self.items[index - 1].clone())
        } else {
            None
        }
    }

    /// Check if an item is in the map.
    pub fn contains(&self, item: &str) -> bool {
        self.index_map.contains_key(item)
    }

    /// Return the size of the map.
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Check if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear the map.
    pub fn clear(&mut self) {
        self.items.clear();
        self.index_map.clear();
    }

    /// Return an iterator over the map.
    pub fn iter(&self) -> TdfLabelIndexedMapIterator {
        TdfLabelIndexedMapIterator {
            items: self.items.clone(),
            current: 0,
        }
    }
}

impl Default for TdfLabelIndexedMap {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TdfLabelIndexedMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TdfLabelIndexedMap")
            .field("size", &self.items.len())
            .finish()
    }
}

/// Iterator for TDF_LabelIndexedMap.
pub struct TdfLabelIndexedMapIterator {
    items: Vec<String>,
    current: usize,
}

impl TdfLabelIndexedMapIterator {
    /// Check if there is a more item.
    pub fn more(&self) -> bool {
        self.current < self.items.len()
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        if self.current < self.items.len() {
            self.current += 1;
        }
    }

    /// Get the current value.
    pub fn value(&self) -> Option<String> {
        if self.current < self.items.len() {
            Some(self.items[self.current].clone())
        } else {
            None
        }
    }
}

impl Iterator for TdfLabelIndexedMapIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.value();
        TdfLabelIndexedMapIterator::next(self);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_map() {
        let map = TdfLabelIndexedMap::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_add() {
        let mut map = TdfLabelIndexedMap::new();
        let idx1 = map.add("label1".to_string());
        let idx2 = map.add("label2".to_string());
        let idx3 = map.add("label3".to_string());

        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
        assert_eq!(idx3, 3);
        assert_eq!(map.size(), 3);
    }

    #[test]
    fn test_add_duplicate() {
        let mut map = TdfLabelIndexedMap::new();
        let idx1 = map.add("label1".to_string());
        let idx2 = map.add("label1".to_string());

        assert_eq!(idx1, idx2);
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_find_index() {
        let mut map = TdfLabelIndexedMap::new();
        map.add("a".to_string());
        map.add("b".to_string());
        map.add("c".to_string());

        assert_eq!(map.find_index("a"), Some(1));
        assert_eq!(map.find_index("b"), Some(2));
        assert_eq!(map.find_index("c"), Some(3));
        assert_eq!(map.find_index("d"), None);
    }

    #[test]
    fn test_find_from_index() {
        let mut map = TdfLabelIndexedMap::new();
        map.add("x".to_string());
        map.add("y".to_string());
        map.add("z".to_string());

        assert_eq!(map.find_from_index(1), Some("x".to_string()));
        assert_eq!(map.find_from_index(2), Some("y".to_string()));
        assert_eq!(map.find_from_index(3), Some("z".to_string()));
        assert_eq!(map.find_from_index(0), None);
        assert_eq!(map.find_from_index(4), None);
    }

    #[test]
    fn test_contains() {
        let mut map = TdfLabelIndexedMap::new();
        map.add("label1".to_string());
        map.add("label2".to_string());

        assert!(map.contains("label1"));
        assert!(map.contains("label2"));
        assert!(!map.contains("label3"));
    }

    #[test]
    fn test_iterator() {
        let mut map = TdfLabelIndexedMap::new();
        map.add("first".to_string());
        map.add("second".to_string());
        map.add("third".to_string());

        let values: Vec<String> = map.iter().collect();
        assert_eq!(values, vec!["first", "second", "third"]);
    }

    #[test]
    fn test_clear() {
        let mut map = TdfLabelIndexedMap::new();
        map.add("l1".to_string());
        map.add("l2".to_string());
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }
}

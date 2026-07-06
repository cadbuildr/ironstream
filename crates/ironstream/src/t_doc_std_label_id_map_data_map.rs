// FILE: t_doc_std_label_id_map_data_map.rs
// occt: TDocStd_LabelIDMapDataMap, TDocStd_DataMapIteratorOfLabelIDMapDataMap

//! Deprecated typedef for TDocStd_LabelIDMapDataMap.
//!
//! In OCCT, this was a data map from TDF_Label to TDF_IDMap.
//! We implement a minimal map structure using HashMap with placeholder ID maps.

use std::collections::{HashMap, HashSet};
use std::fmt;

/// Placeholder IDMap: a set-like map for unique string IDs.
#[derive(Clone, PartialEq, Eq)]
pub struct IdMap {
    items: HashSet<String>,
}

impl IdMap {
    pub fn new() -> Self {
        IdMap { items: HashSet::new() }
    }

    pub fn add(&mut self, item: String) -> bool {
        self.items.insert(item)
    }

    pub fn contains(&self, item: &str) -> bool {
        self.items.contains(item)
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

impl Default for IdMap {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for IdMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IdMap")
            .field("size", &self.items.len())
            .finish()
    }
}

/// TDocStd_LabelIDMapDataMap: A data map from Label to IDMap (deprecated typedef).
/// Wraps a HashMap for O(1) lookups with Label keys and IDMap values.
#[derive(Clone)]
pub struct TDocStdLabelIdMapDataMap {
    data: HashMap<String, IdMap>,  // Placeholder: would be HashMap<TdfLabel, IdMap> in full port
}

impl TDocStdLabelIdMapDataMap {
    /// Create a new empty map.
    pub fn new() -> Self {
        TDocStdLabelIdMapDataMap {
            data: HashMap::new(),
        }
    }

    /// Bind a label key to an IDMap value in the map.
    pub fn bind(&mut self, key: String, value: IdMap) {
        self.data.insert(key, value);
    }

    /// Find a value by key.
    pub fn find(&self, key: &str) -> Option<IdMap> {
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
    pub fn iter(&self) -> TDocStdDataMapIteratorOfLabelIdMapDataMap {
        let pairs: Vec<(String, IdMap)> = self
            .data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        TDocStdDataMapIteratorOfLabelIdMapDataMap {
            pairs,
            current: 0,
        }
    }
}

impl Default for TDocStdLabelIdMapDataMap {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TDocStdLabelIdMapDataMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TDocStdLabelIdMapDataMap")
            .field("size", &self.data.len())
            .finish()
    }
}

/// Iterator for TDocStd_LabelIDMapDataMap.
pub struct TDocStdDataMapIteratorOfLabelIdMapDataMap {
    pairs: Vec<(String, IdMap)>,
    current: usize,
}

impl TDocStdDataMapIteratorOfLabelIdMapDataMap {
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
    pub fn value(&self) -> Option<IdMap> {
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
        let map = TDocStdLabelIdMapDataMap::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TDocStdLabelIdMapDataMap::new();
        let mut id_map1 = IdMap::new();
        id_map1.add("id1".to_string());
        id_map1.add("id2".to_string());

        let mut id_map2 = IdMap::new();
        id_map2.add("id3".to_string());

        map.bind("label1".to_string(), id_map1.clone());
        map.bind("label2".to_string(), id_map2.clone());

        assert_eq!(map.size(), 2);
        assert_eq!(map.find("label1").unwrap().size(), 2);
        assert_eq!(map.find("label2").unwrap().size(), 1);
        assert_eq!(map.find("label3"), None);
    }

    #[test]
    fn test_contains() {
        let mut map = TDocStdLabelIdMapDataMap::new();
        let id_map = IdMap::new();
        map.bind("l1".to_string(), id_map);

        assert!(map.contains("l1"));
        assert!(!map.contains("l2"));
    }

    #[test]
    fn test_rebind() {
        let mut map = TDocStdLabelIdMapDataMap::new();
        let mut id_map1 = IdMap::new();
        id_map1.add("a".to_string());

        let mut id_map2 = IdMap::new();
        id_map2.add("b".to_string());
        id_map2.add("c".to_string());

        map.bind("key".to_string(), id_map1);
        assert_eq!(map.find("key").unwrap().size(), 1);

        map.bind("key".to_string(), id_map2);
        assert_eq!(map.size(), 1);
        assert_eq!(map.find("key").unwrap().size(), 2);
    }

    #[test]
    fn test_iterator() {
        let mut map = TDocStdLabelIdMapDataMap::new();
        let id_map = IdMap::new();
        map.bind("x".to_string(), id_map.clone());
        map.bind("y".to_string(), id_map);

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
        let mut map = TDocStdLabelIdMapDataMap::new();
        let id_map = IdMap::new();
        map.bind("k1".to_string(), id_map.clone());
        map.bind("k2".to_string(), id_map);
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_id_map_operations() {
        let mut id_map = IdMap::new();
        assert!(id_map.add("id1".to_string()));
        assert!(!id_map.add("id1".to_string()));  // duplicate
        assert_eq!(id_map.size(), 1);
        assert!(id_map.contains("id1"));
        assert!(!id_map.contains("id2"));
    }
}

// FILE: tdf_id_map.rs
// occt: TDF_IDMap, TDF_MapIteratorOfIDMap

//! Deprecated typedef for TDF_IDMap.
//!
//! In OCCT, this was a set-like map for Standard_GUID items.
//! We implement a minimal set structure using HashSet with iteration semantics.

use std::collections::HashSet;
use std::fmt;

/// TDF_IDMap: A set-like map for Standard_GUID items (deprecated typedef).
/// Wraps a HashSet for unique element storage and O(1) lookups.
#[derive(Clone)]
pub struct TdfIdMap {
    items: HashSet<String>,  // Placeholder: would be HashSet<StandardGuid> in full port
}

impl TdfIdMap {
    /// Create a new empty map.
    pub fn new() -> Self {
        TdfIdMap {
            items: HashSet::new(),
        }
    }

    /// Add an item to the map.
    pub fn add(&mut self, item: String) -> bool {
        self.items.insert(item)
    }

    /// Remove an item from the map.
    pub fn remove(&mut self, item: &str) -> bool {
        self.items.remove(item)
    }

    /// Check if an item is in the map.
    pub fn contains(&self, item: &str) -> bool {
        self.items.contains(item)
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
    }

    /// Return an iterator over the map.
    pub fn iter(&self) -> TdfMapIteratorOfIdMap {
        let items: Vec<String> = self.items.iter().cloned().collect();
        TdfMapIteratorOfIdMap {
            items,
            current: 0,
        }
    }
}

impl Default for TdfIdMap {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TdfIdMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TdfIdMap")
            .field("size", &self.items.len())
            .finish()
    }
}

/// Iterator for TDF_IDMap.
pub struct TdfMapIteratorOfIdMap {
    items: Vec<String>,
    current: usize,
}

impl TdfMapIteratorOfIdMap {
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

impl Iterator for TdfMapIteratorOfIdMap {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.value();
        TdfMapIteratorOfIdMap::next(self);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_map() {
        let map = TdfIdMap::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_add() {
        let mut map = TdfIdMap::new();
        assert!(map.add("guid1".to_string()));
        assert!(map.add("guid2".to_string()));
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_add_duplicate() {
        let mut map = TdfIdMap::new();
        assert!(map.add("guid1".to_string()));
        assert!(!map.add("guid1".to_string()));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_contains() {
        let mut map = TdfIdMap::new();
        map.add("id1".to_string());
        map.add("id2".to_string());

        assert!(map.contains("id1"));
        assert!(map.contains("id2"));
        assert!(!map.contains("id3"));
    }

    #[test]
    fn test_remove() {
        let mut map = TdfIdMap::new();
        map.add("id1".to_string());
        map.add("id2".to_string());
        assert_eq!(map.size(), 2);

        assert!(map.remove("id1"));
        assert_eq!(map.size(), 1);
        assert!(!map.contains("id1"));
        assert!(map.contains("id2"));

        assert!(!map.remove("id1"));
    }

    #[test]
    fn test_iterator() {
        let mut map = TdfIdMap::new();
        map.add("a".to_string());
        map.add("b".to_string());
        map.add("c".to_string());

        let mut iter = map.iter();
        assert!(iter.more());
        let val1 = iter.value();
        assert!(val1.is_some());
        iter.next();

        assert!(iter.more());
        iter.next();

        assert!(iter.more());
        iter.next();

        assert!(!iter.more());
        assert_eq!(iter.value(), None);
    }

    #[test]
    fn test_iterator_as_rust_iterator() {
        let mut map = TdfIdMap::new();
        map.add("x".to_string());
        map.add("y".to_string());
        map.add("z".to_string());

        let collected: Vec<String> = map.iter().collect();
        assert_eq!(collected.len(), 3);
        assert!(collected.contains(&"x".to_string()));
        assert!(collected.contains(&"y".to_string()));
        assert!(collected.contains(&"z".to_string()));
    }

    #[test]
    fn test_clear() {
        let mut map = TdfIdMap::new();
        map.add("id1".to_string());
        map.add("id2".to_string());
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }
}

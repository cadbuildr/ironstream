// FILE: tdf_guid_prog_id_map.rs
// occt: TDF_GUIDProgIDMap, TDF_DoubleMapIteratorOfGUIDProgIDMap

//! Deprecated typedef for TDF_GUIDProgIDMap.
//!
//! In OCCT, this was a bidirectional map from Standard_GUID to TCollection_ExtendedString.
//! We implement a minimal double-map structure using HashMap with forward and reverse lookups.

use std::collections::HashMap;
use std::fmt;

/// TDF_GUIDProgIDMap: A bidirectional map between GUID and ExtendedString (deprecated typedef).
/// Uses two HashMaps to maintain O(1) forward and reverse lookups.
#[derive(Clone)]
pub struct TdfGuidProgIdMap {
    forward: HashMap<String, String>,  // guid_str -> ext_string
    reverse: HashMap<String, String>,  // ext_string -> guid_str
}

impl TdfGuidProgIdMap {
    /// Create a new empty map.
    pub fn new() -> Self {
        TdfGuidProgIdMap {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    /// Bind a GUID to an extended string (bidirectional).
    pub fn bind(&mut self, guid: String, prog_id: String) {
        // Remove old associations if they exist
        if let Some(old_prog_id) = self.forward.remove(&guid) {
            self.reverse.remove(&old_prog_id);
        }
        if let Some(old_guid) = self.reverse.remove(&prog_id) {
            self.forward.remove(&old_guid);
        }

        // Add new binding
        self.forward.insert(guid.clone(), prog_id.clone());
        self.reverse.insert(prog_id, guid);
    }

    /// Find a prog_id by GUID.
    pub fn find1(&self, guid: &str) -> Option<String> {
        self.forward.get(guid).cloned()
    }

    /// Find a GUID by prog_id.
    pub fn find2(&self, prog_id: &str) -> Option<String> {
        self.reverse.get(prog_id).cloned()
    }

    /// Return the number of bindings.
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
    pub fn iter(&self) -> TdfDoubleMapIteratorOfGuidProgIdMap {
        let pairs: Vec<(String, String)> = self
            .forward
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        TdfDoubleMapIteratorOfGuidProgIdMap {
            pairs,
            current: 0,
        }
    }
}

impl Default for TdfGuidProgIdMap {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TdfGuidProgIdMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TdfGuidProgIdMap")
            .field("size", &self.forward.len())
            .finish()
    }
}

/// Iterator for TDF_GUIDProgIDMap.
pub struct TdfDoubleMapIteratorOfGuidProgIdMap {
    pairs: Vec<(String, String)>,
    current: usize,
}

impl TdfDoubleMapIteratorOfGuidProgIdMap {
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

    /// Get the current key (GUID).
    pub fn key(&self) -> Option<String> {
        if self.current < self.pairs.len() {
            Some(self.pairs[self.current].0.clone())
        } else {
            None
        }
    }

    /// Get the current value (prog_id).
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
        let map = TdfGuidProgIdMap::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TdfGuidProgIdMap::new();
        map.bind("guid1".to_string(), "ProgID1".to_string());
        map.bind("guid2".to_string(), "ProgID2".to_string());

        assert_eq!(map.size(), 2);
        assert_eq!(map.find1("guid1"), Some("ProgID1".to_string()));
        assert_eq!(map.find1("guid2"), Some("ProgID2".to_string()));
        assert_eq!(map.find2("ProgID1"), Some("guid1".to_string()));
        assert_eq!(map.find2("ProgID2"), Some("guid2".to_string()));
    }

    #[test]
    fn test_rebind() {
        let mut map = TdfGuidProgIdMap::new();
        map.bind("guid1".to_string(), "ProgID1".to_string());
        assert_eq!(map.size(), 1);

        // Rebind the same GUID to a new ProgID
        map.bind("guid1".to_string(), "ProgID2".to_string());
        assert_eq!(map.size(), 1);
        assert_eq!(map.find1("guid1"), Some("ProgID2".to_string()));
        assert_eq!(map.find2("ProgID1"), None);
        assert_eq!(map.find2("ProgID2"), Some("guid1".to_string()));
    }

    #[test]
    fn test_iterator() {
        let mut map = TdfGuidProgIdMap::new();
        map.bind("g1".to_string(), "p1".to_string());
        map.bind("g2".to_string(), "p2".to_string());

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
        let mut map = TdfGuidProgIdMap::new();
        map.bind("g1".to_string(), "p1".to_string());
        assert_eq!(map.size(), 1);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_find_not_found() {
        let map = TdfGuidProgIdMap::new();
        assert_eq!(map.find1("nonexistent"), None);
        assert_eq!(map.find2("nonexistent"), None);
    }
}

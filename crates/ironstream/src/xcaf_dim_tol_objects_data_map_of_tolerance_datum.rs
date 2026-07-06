// FILE: xcaf_dim_tol_objects_data_map_of_tolerance_datum.rs
// occt: XCAFDimTolObjects_DataMapOfToleranceDatum
//
// Faithful port of OCCT XCAFDimTolObjects_DataMapOfToleranceDatum
// (Deprecated/NCollectionAliases/XCAFDimTolObjects_DataMapOfToleranceDatum.hxx/.cxx):
// a data map (key-value collection) for tolerance datum associations.
// NCollection alias for mapping keys to tolerance datum objects.

use std::collections::HashMap;

/// Local representation of a tolerance datum reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ToleranceDatum {
    id: u32,
}

impl ToleranceDatum {
    pub fn new(id: u32) -> Self {
        ToleranceDatum { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Port of XCAFDimTolObjects_DataMapOfToleranceDatum.
#[derive(Debug, Clone, PartialEq)]
pub struct XcafDimTolObjectsDataMapOfToleranceDatum {
    map: HashMap<u32, ToleranceDatum>,
}

impl XcafDimTolObjectsDataMapOfToleranceDatum {
    /// Create an empty map.
    pub fn new() -> Self {
        XcafDimTolObjectsDataMapOfToleranceDatum {
            map: HashMap::new(),
        }
    }

    /// Bind a key to a datum.
    pub fn bind(&mut self, key: u32, datum: ToleranceDatum) {
        self.map.insert(key, datum);
    }

    /// Find a datum by key.
    pub fn find(&self, key: u32) -> Option<&ToleranceDatum> {
        self.map.get(&key)
    }

    /// Check if key exists.
    pub fn contains(&self, key: u32) -> bool {
        self.map.contains_key(&key)
    }

    /// Remove entry by key.
    pub fn remove(&mut self, key: u32) -> Option<ToleranceDatum> {
        self.map.remove(&key)
    }

    /// Get number of entries.
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Get all keys.
    pub fn keys(&self) -> Vec<u32> {
        self.map.keys().copied().collect()
    }

    /// Get all values.
    pub fn values(&self) -> Vec<&ToleranceDatum> {
        self.map.values().collect()
    }
}

impl Default for XcafDimTolObjectsDataMapOfToleranceDatum {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_empty() {
        let map = XcafDimTolObjectsDataMapOfToleranceDatum::new();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn tolerance_datum_creation() {
        let datum = ToleranceDatum::new(42);
        assert_eq!(datum.id(), 42);
    }

    #[test]
    fn bind_and_find() {
        let mut map = XcafDimTolObjectsDataMapOfToleranceDatum::new();
        let datum = ToleranceDatum::new(1);
        map.bind(100, datum.clone());
        assert_eq!(map.find(100), Some(&datum));
    }

    #[test]
    fn find_nonexistent() {
        let map = XcafDimTolObjectsDataMapOfToleranceDatum::new();
        assert_eq!(map.find(99), None);
    }

    #[test]
    fn contains() {
        let mut map = XcafDimTolObjectsDataMapOfToleranceDatum::new();
        map.bind(1, ToleranceDatum::new(10));
        assert!(map.contains(1));
        assert!(!map.contains(2));
    }

    #[test]
    fn remove() {
        let mut map = XcafDimTolObjectsDataMapOfToleranceDatum::new();
        let datum = ToleranceDatum::new(5);
        map.bind(1, datum.clone());
        assert_eq!(map.size(), 1);
        let removed = map.remove(1);
        assert_eq!(removed, Some(datum));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn clear() {
        let mut map = XcafDimTolObjectsDataMapOfToleranceDatum::new();
        map.bind(1, ToleranceDatum::new(10));
        map.bind(2, ToleranceDatum::new(20));
        assert_eq!(map.size(), 2);
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn keys() {
        let mut map = XcafDimTolObjectsDataMapOfToleranceDatum::new();
        map.bind(1, ToleranceDatum::new(10));
        map.bind(2, ToleranceDatum::new(20));
        let keys = map.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
    }

    #[test]
    fn values() {
        let mut map = XcafDimTolObjectsDataMapOfToleranceDatum::new();
        let d1 = ToleranceDatum::new(10);
        let d2 = ToleranceDatum::new(20);
        map.bind(1, d1.clone());
        map.bind(2, d2.clone());
        let vals = map.values();
        assert_eq!(vals.len(), 2);
    }

    #[test]
    fn multiple_operations() {
        let mut map = XcafDimTolObjectsDataMapOfToleranceDatum::new();
        map.bind(1, ToleranceDatum::new(100));
        map.bind(2, ToleranceDatum::new(200));
        map.bind(3, ToleranceDatum::new(300));
        assert_eq!(map.size(), 3);
        map.remove(2);
        assert_eq!(map.size(), 2);
        assert!(!map.contains(2));
    }
}

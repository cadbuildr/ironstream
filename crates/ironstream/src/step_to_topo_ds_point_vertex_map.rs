// FILE: step_to_topo_ds_point_vertex_map.rs
// occt: StepToTopoDS_PointVertexMap, StepToTopoDS_DataMapIteratorOfPointVertexMap

use std::collections::HashMap;

/// StepToTopoDS_PointVertexMap: a newtype wrapper over HashMap
/// mapping StepGeom_CartesianPoint handles to TopoDS_Vertex.
///
/// This is a deprecated OCCT typedef for backward compatibility.
/// In Rust, we model this as a HashMap with cartesian point keys
/// and vertex values, preserving the map semantics.
#[derive(Debug, Clone)]
pub struct StepToTopoDS_PointVertexMap {
    inner: HashMap<u64, u64>, // (point_id -> vertex_id)
}

impl StepToTopoDS_PointVertexMap {
    /// Create a new empty map.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Insert a (CartesianPoint, Vertex) pair.
    /// Returns the previous value if the key was already present.
    pub fn insert(&mut self, point_key: u64, vertex_val: u64) -> Option<u64> {
        self.inner.insert(point_key, vertex_val)
    }

    /// Remove a key from the map.
    /// Returns the value if the key was present.
    pub fn remove(&mut self, point_key: &u64) -> Option<u64> {
        self.inner.remove(point_key)
    }

    /// Lookup a vertex by cartesian point key.
    pub fn find(&self, point_key: &u64) -> Option<u64> {
        self.inner.get(point_key).copied()
    }

    /// Check if a key exists in the map.
    pub fn contains(&self, point_key: &u64) -> bool {
        self.inner.contains_key(point_key)
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Return the number of entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get an iterator over (key, value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&u64, &u64)> {
        self.inner.iter()
    }

    /// Get a mutable iterator over (key, value) pairs.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&u64, &mut u64)> {
        self.inner.iter_mut()
    }
}

impl Default for StepToTopoDS_PointVertexMap {
    fn default() -> Self {
        Self::new()
    }
}

/// StepToTopoDS_DataMapIteratorOfPointVertexMap: an iterator adapter
/// for walking the map entries.
#[derive(Debug)]
pub struct StepToTopoDS_DataMapIteratorOfPointVertexMap {
    keys: Vec<u64>,
    values: Vec<u64>,
    index: usize,
}

impl StepToTopoDS_DataMapIteratorOfPointVertexMap {
    /// Create an iterator from the map.
    pub fn new(map: &StepToTopoDS_PointVertexMap) -> Self {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        for (k, v) in map.inner.iter() {
            keys.push(*k);
            values.push(*v);
        }
        Self {
            keys,
            values,
            index: 0,
        }
    }

    /// Check if there are more entries to iterate.
    pub fn more(&self) -> bool {
        self.index < self.keys.len()
    }

    /// Move to the next entry.
    pub fn next(&mut self) {
        if self.index < self.keys.len() {
            self.index += 1;
        }
    }

    /// Get the current key.
    pub fn key(&self) -> Option<u64> {
        if self.index < self.keys.len() {
            Some(self.keys[self.index])
        } else {
            None
        }
    }

    /// Get the current value.
    pub fn value(&self) -> Option<u64> {
        if self.index < self.values.len() {
            Some(self.values[self.index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_vertex_map_insert_and_find() {
        let mut map = StepToTopoDS_PointVertexMap::new();

        // Insert a point-vertex pair
        let prev = map.insert(1, 100);
        assert_eq!(prev, None);

        // Find the vertex
        assert_eq!(map.find(&1), Some(100));

        // Insert another pair
        map.insert(2, 200);
        assert_eq!(map.find(&2), Some(200));
    }

    #[test]
    fn test_point_vertex_map_remove() {
        let mut map = StepToTopoDS_PointVertexMap::new();
        map.insert(1, 100);
        map.insert(2, 200);

        // Remove an entry
        let removed = map.remove(&1);
        assert_eq!(removed, Some(100));

        // Verify it's gone
        assert_eq!(map.find(&1), None);
        assert_eq!(map.find(&2), Some(200));
    }

    #[test]
    fn test_point_vertex_map_len_and_empty() {
        let mut map = StepToTopoDS_PointVertexMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);

        map.insert(1, 100);
        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);

        map.insert(2, 200);
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_point_vertex_map_contains() {
        let mut map = StepToTopoDS_PointVertexMap::new();
        map.insert(1, 100);

        assert!(map.contains(&1));
        assert!(!map.contains(&99));
    }

    #[test]
    fn test_point_vertex_map_iterator() {
        let mut map = StepToTopoDS_PointVertexMap::new();
        map.insert(1, 100);
        map.insert(2, 200);
        map.insert(3, 300);

        let mut iter = StepToTopoDS_DataMapIteratorOfPointVertexMap::new(&map);

        let mut found_entries = 0;
        while iter.more() {
            if let (Some(k), Some(v)) = (iter.key(), iter.value()) {
                assert!(map.contains(&k));
                assert_eq!(map.find(&k), Some(v));
                found_entries += 1;
            }
            iter.next();
        }
        assert_eq!(found_entries, 3);
    }

    #[test]
    fn test_point_vertex_map_default() {
        let map: StepToTopoDS_PointVertexMap = Default::default();
        assert!(map.is_empty());
    }
}

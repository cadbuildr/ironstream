// FILE: bop_tools_indexed_data_map_of_set_shape.rs
// occt: BOPTools_IndexedDataMapOfSetShape

use std::collections::BTreeMap;

/// Represents a set of shapes used in Boolean operations.
/// Mirrors BOPTools_Set from OCCT.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Set {
    shapes: Vec<usize>,        // Indices of shapes
    primary_shape: usize,      // Primary shape ID
    nb_shapes: i32,            // Number of shapes
    sum: u64,                  // Hash sum for equality
}

impl Set {
    /// Creates a new empty set.
    fn new() -> Self {
        Set {
            shapes: Vec::new(),
            primary_shape: 0,
            nb_shapes: 0,
            sum: 0,
        }
    }

    /// Adds a shape to the set.
    fn add(&mut self, shape_id: usize) {
        if !self.shapes.contains(&shape_id) {
            self.shapes.push(shape_id);
            self.nb_shapes += 1;
            self.sum = self.sum.wrapping_add(shape_id as u64);
        }
    }

    /// Returns the number of shapes in the set.
    fn nb_shapes(&self) -> i32 {
        self.nb_shapes
    }

    /// Checks if two sets are equal.
    fn is_equal(&self, other: &Set) -> bool {
        if self.nb_shapes != other.nb_shapes {
            return false;
        }
        // Check if all shapes match
        for shape_id in &self.shapes {
            if !other.shapes.contains(shape_id) {
                return false;
            }
        }
        true
    }

    /// Returns the hash sum of the set.
    fn get_sum(&self) -> u64 {
        self.sum
    }
}

impl std::hash::Hash for Set {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sum.hash(state);
    }
}

/// Indexed data map associating a Set key to a shape value.
/// Mirrors NCollection_IndexedDataMap<BOPTools_Set, TopoDS_Shape>.
pub struct BoptoolsIndexedDataMapOfSetShape {
    // Internal storage: map from index to (key, value) pair
    data: BTreeMap<usize, (Set, usize)>,
    next_index: usize,
}

impl BoptoolsIndexedDataMapOfSetShape {
    /// Creates a new empty indexed data map.
    pub fn new() -> Self {
        BoptoolsIndexedDataMapOfSetShape {
            data: BTreeMap::new(),
            next_index: 1,  // Indexed maps typically start at 1
        }
    }

    /// Adds a key-value pair to the map, returning its index.
    pub fn add(&mut self, key: Set, value: usize) -> usize {
        let index = self.next_index;
        self.data.insert(index, (key, value));
        self.next_index += 1;
        index
    }

    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Looks up the value associated with a key by index.
    pub fn find_index(&self, index: usize) -> Option<(&Set, &usize)> {
        self.data.get(&index).map(|(k, v)| (k, v))
    }

    /// Looks up the index of a key in the map.
    pub fn find_key(&self, key: &Set) -> Option<usize> {
        for (&idx, (k, _)) in &self.data {
            if k == key {
                return Some(idx);
            }
        }
        None
    }

    /// Looks up the value associated with a key.
    pub fn find(&self, key: &Set) -> Option<&usize> {
        for (_, (k, v)) in &self.data {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    /// Clears all entries from the map.
    pub fn clear(&mut self) {
        self.data.clear();
        self.next_index = 1;
    }

    /// Removes an entry by index.
    pub fn remove(&mut self, index: usize) -> Option<(Set, usize)> {
        self.data.remove(&index)
    }
}

impl Default for BoptoolsIndexedDataMapOfSetShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_creation() {
        let set = Set::new();
        assert_eq!(set.nb_shapes(), 0);
        assert_eq!(set.get_sum(), 0);
    }

    #[test]
    fn test_set_add() {
        let mut set = Set::new();
        set.add(1);
        set.add(2);
        set.add(3);
        assert_eq!(set.nb_shapes(), 3);
        assert!(set.shapes.contains(&1));
        assert!(set.shapes.contains(&2));
        assert!(set.shapes.contains(&3));
    }

    #[test]
    fn test_set_duplicate_add() {
        let mut set = Set::new();
        set.add(1);
        let initial_count = set.nb_shapes();
        set.add(1);  // Try adding same shape again
        assert_eq!(set.nb_shapes(), initial_count);
    }

    #[test]
    fn test_set_equality() {
        let mut set1 = Set::new();
        set1.add(1);
        set1.add(2);

        let mut set2 = Set::new();
        set2.add(2);
        set2.add(1);

        assert!(set1.is_equal(&set2));
    }

    #[test]
    fn test_set_inequality() {
        let mut set1 = Set::new();
        set1.add(1);
        set1.add(2);

        let mut set2 = Set::new();
        set2.add(1);
        set2.add(3);

        assert!(!set1.is_equal(&set2));
    }

    #[test]
    fn test_map_creation() {
        let map = BoptoolsIndexedDataMapOfSetShape::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_add_and_find() {
        let mut map = BoptoolsIndexedDataMapOfSetShape::new();
        let mut set = Set::new();
        set.add(1);
        set.add(2);

        let idx = map.add(set.clone(), 42);
        assert_eq!(idx, 1);
        assert_eq!(map.len(), 1);

        let (found_set, value) = map.find_index(idx).unwrap();
        assert_eq!(found_set, &set);
        assert_eq!(*value, 42);
    }

    #[test]
    fn test_map_multiple_entries() {
        let mut map = BoptoolsIndexedDataMapOfSetShape::new();

        let mut set1 = Set::new();
        set1.add(1);
        let idx1 = map.add(set1.clone(), 10);

        let mut set2 = Set::new();
        set2.add(2);
        let idx2 = map.add(set2.clone(), 20);

        assert_eq!(map.len(), 2);
        assert_eq!(*map.find_index(idx1).unwrap().1, 10);
        assert_eq!(*map.find_index(idx2).unwrap().1, 20);
    }

    #[test]
    fn test_map_find_key() {
        let mut map = BoptoolsIndexedDataMapOfSetShape::new();
        let mut set = Set::new();
        set.add(5);

        let idx = map.add(set.clone(), 100);
        let found_idx = map.find_key(&set).unwrap();
        assert_eq!(found_idx, idx);
    }

    #[test]
    fn test_map_find_value() {
        let mut map = BoptoolsIndexedDataMapOfSetShape::new();
        let mut set = Set::new();
        set.add(1);

        map.add(set.clone(), 999);
        let value = map.find(&set).unwrap();
        assert_eq!(*value, 999);
    }

    #[test]
    fn test_map_remove() {
        let mut map = BoptoolsIndexedDataMapOfSetShape::new();
        let mut set = Set::new();
        set.add(1);

        let idx = map.add(set, 50);
        assert_eq!(map.len(), 1);

        let removed = map.remove(idx);
        assert!(removed.is_some());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_clear() {
        let mut map = BoptoolsIndexedDataMapOfSetShape::new();
        let mut set1 = Set::new();
        set1.add(1);
        let mut set2 = Set::new();
        set2.add(2);

        map.add(set1, 10);
        map.add(set2, 20);
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_set_hash() {
        let mut set1 = Set::new();
        set1.add(1);
        set1.add(2);

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        set1.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher2 = DefaultHasher::new();
        set1.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }
}

// FILE: bop_tools_map_of_set.rs
// occt: BOPTools_MapOfSet

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

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

impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sum.hash(state);
    }
}

/// Map of sets used in Boolean operations.
/// Mirrors NCollection_Map<BOPTools_Set>.
pub struct BoptoolsMapOfSet {
    sets: HashSet<u64>,           // Store hash values to detect duplicates
    data: Vec<Set>,               // Actual set data indexed by hash
}

impl BoptoolsMapOfSet {
    /// Creates a new empty map.
    pub fn new() -> Self {
        BoptoolsMapOfSet {
            sets: HashSet::new(),
            data: Vec::new(),
        }
    }

    /// Adds a set to the map if not already present.
    /// Returns true if the set was added, false if it was already there.
    pub fn add(&mut self, set: Set) -> bool {
        let hash = set.get_sum();
        if self.sets.insert(hash) {
            self.data.push(set);
            true
        } else {
            false
        }
    }

    /// Checks if a set is in the map.
    pub fn contains(&self, set: &Set) -> bool {
        let hash = set.get_sum();
        self.sets.contains(&hash)
    }

    /// Returns the number of sets in the map.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clears all sets from the map.
    pub fn clear(&mut self) {
        self.sets.clear();
        self.data.clear();
    }

    /// Returns an iterator over the sets.
    pub fn iter(&self) -> impl Iterator<Item = &Set> {
        self.data.iter()
    }

    /// Removes a set from the map by hash.
    pub fn remove(&mut self, set: &Set) -> bool {
        let hash = set.get_sum();
        if self.sets.remove(&hash) {
            self.data.retain(|s| s != set);
            true
        } else {
            false
        }
    }

    /// Finds a set in the map that matches the given set.
    pub fn find(&self, set: &Set) -> Option<&Set> {
        self.data.iter().find(|s| s == set)
    }
}

impl Default for BoptoolsMapOfSet {
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
    }

    #[test]
    fn test_set_duplicate_add() {
        let mut set = Set::new();
        set.add(1);
        let initial_count = set.nb_shapes();
        set.add(1);
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
    fn test_set_hash() {
        let mut set1 = Set::new();
        set1.add(1);
        set1.add(2);

        let mut hasher = DefaultHasher::new();
        set1.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher2 = DefaultHasher::new();
        set1.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_map_creation() {
        let map = BoptoolsMapOfSet::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_add() {
        let mut map = BoptoolsMapOfSet::new();
        let mut set = Set::new();
        set.add(1);
        set.add(2);

        let added = map.add(set.clone());
        assert!(added);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_map_add_duplicate() {
        let mut map = BoptoolsMapOfSet::new();
        let mut set = Set::new();
        set.add(1);

        let added1 = map.add(set.clone());
        let added2 = map.add(set.clone());

        assert!(added1);
        assert!(!added2);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_map_contains() {
        let mut map = BoptoolsMapOfSet::new();
        let mut set = Set::new();
        set.add(5);

        assert!(!map.contains(&set));
        map.add(set.clone());
        assert!(map.contains(&set));
    }

    #[test]
    fn test_map_multiple_sets() {
        let mut map = BoptoolsMapOfSet::new();

        let mut set1 = Set::new();
        set1.add(1);
        let mut set2 = Set::new();
        set2.add(2);

        map.add(set1.clone());
        map.add(set2.clone());

        assert_eq!(map.len(), 2);
        assert!(map.contains(&set1));
        assert!(map.contains(&set2));
    }

    #[test]
    fn test_map_remove() {
        let mut map = BoptoolsMapOfSet::new();
        let mut set = Set::new();
        set.add(1);

        map.add(set.clone());
        assert_eq!(map.len(), 1);

        let removed = map.remove(&set);
        assert!(removed);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_clear() {
        let mut map = BoptoolsMapOfSet::new();
        let mut set1 = Set::new();
        set1.add(1);
        let mut set2 = Set::new();
        set2.add(2);

        map.add(set1);
        map.add(set2);
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_find() {
        let mut map = BoptoolsMapOfSet::new();
        let mut set = Set::new();
        set.add(1);

        map.add(set.clone());
        let found = map.find(&set);
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &set);
    }

    #[test]
    fn test_map_iterator() {
        let mut map = BoptoolsMapOfSet::new();
        for i in 0..5 {
            let mut set = Set::new();
            set.add(i);
            map.add(set);
        }

        let count = map.iter().count();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_map_find_none() {
        let mut map = BoptoolsMapOfSet::new();
        let mut set1 = Set::new();
        set1.add(1);
        map.add(set1);

        let mut set2 = Set::new();
        set2.add(2);
        let found = map.find(&set2);
        assert!(found.is_none());
    }
}

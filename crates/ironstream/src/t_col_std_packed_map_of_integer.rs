// FILE: t_col_std_packed_map_of_integer.rs
// occt: TColStd_PackedMapOfInteger

use std::collections::HashSet;

/// A packed map of integers (memory-efficient set)
/// Maps to `typedef NCollection_PackedMap<int> TColStd_PackedMapOfInteger` in OCCT
/// This is a wrapper around a HashSet for simplicity in pure Rust
#[derive(Debug, Clone)]
pub struct TColStdPackedMapOfInteger {
    data: HashSet<i32>,
}

impl TColStdPackedMapOfInteger {
    /// Create a new empty packed map
    pub fn new() -> Self {
        TColStdPackedMapOfInteger {
            data: HashSet::new(),
        }
    }

    /// Create a packed map with initial bucket count
    pub fn with_capacity(capacity: usize) -> Self {
        TColStdPackedMapOfInteger {
            data: HashSet::with_capacity(capacity),
        }
    }

    /// Add an integer to the map
    pub fn add(&mut self, value: i32) -> bool {
        self.data.insert(value)
    }

    /// Remove an integer from the map
    pub fn remove(&mut self, value: i32) -> bool {
        self.data.remove(&value)
    }

    /// Check if an integer is in the map
    pub fn contains(&self, value: i32) -> bool {
        self.data.contains(&value)
    }

    /// Get the number of elements in the map
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear the map
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get an iterator over the values
    pub fn iter(&self) -> std::collections::hash_set::Iter<i32> {
        self.data.iter()
    }

    /// Get values as a vector (sorted)
    pub fn to_vec(&self) -> Vec<i32> {
        let mut vec: Vec<i32> = self.data.iter().copied().collect();
        vec.sort();
        vec
    }
}

impl Default for TColStdPackedMapOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TColStdPackedMapOfInteger {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_map_new() {
        let map = TColStdPackedMapOfInteger::new();
        assert!(map.is_empty());
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_packed_map_add() {
        let mut map = TColStdPackedMapOfInteger::new();
        assert!(map.add(42));
        assert!(!map.add(42)); // Adding duplicate returns false
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_packed_map_contains() {
        let mut map = TColStdPackedMapOfInteger::new();
        map.add(10);
        map.add(20);
        assert!(map.contains(10));
        assert!(map.contains(20));
        assert!(!map.contains(30));
    }

    #[test]
    fn test_packed_map_remove() {
        let mut map = TColStdPackedMapOfInteger::new();
        map.add(5);
        assert_eq!(map.size(), 1);
        assert!(map.remove(5));
        assert_eq!(map.size(), 0);
        assert!(!map.remove(5)); // Removing non-existent returns false
    }

    #[test]
    fn test_packed_map_clear() {
        let mut map = TColStdPackedMapOfInteger::new();
        map.add(1);
        map.add(2);
        map.add(3);
        assert_eq!(map.size(), 3);
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_packed_map_to_vec() {
        let mut map = TColStdPackedMapOfInteger::new();
        map.add(3);
        map.add(1);
        map.add(2);
        let vec = map.to_vec();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_packed_map_clone() {
        let mut map1 = TColStdPackedMapOfInteger::new();
        map1.add(100);
        let map2 = map1.clone();
        assert_eq!(map1, map2);
    }

    #[test]
    fn test_packed_map_default() {
        let map = TColStdPackedMapOfInteger::default();
        assert!(map.is_empty());
    }

    #[test]
    fn test_packed_map_with_capacity() {
        let map = TColStdPackedMapOfInteger::with_capacity(100);
        assert!(map.is_empty());
    }
}

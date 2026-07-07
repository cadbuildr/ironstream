// FILE: t_col_std_map_of_integer.rs
// occt: TColStd_MapOfInteger

use std::collections::HashSet;

/// TColStd_MapOfInteger is a deprecated alias for a set of integers.
/// This is a Rust port implementing OCCT's map semantics (unordered set).
pub struct TColStdMapOfInteger {
    data: HashSet<i32>,
}

impl TColStdMapOfInteger {
    /// Creates a new empty map.
    pub fn new() -> Self {
        TColStdMapOfInteger {
            data: HashSet::new(),
        }
    }

    /// Adds an element to the map.
    pub fn add(&mut self, value: i32) -> bool {
        self.data.insert(value)
    }

    /// Returns the size of the map.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Checks if the map contains a value.
    pub fn contains(&self, value: i32) -> bool {
        self.data.contains(&value)
    }

    /// Removes a value from the map.
    pub fn remove(&mut self, value: i32) -> bool {
        self.data.remove(&value)
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for TColStdMapOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_contains() {
        let mut map = TColStdMapOfInteger::new();
        assert!(map.add(10));
        assert!(!map.add(10)); // Already exists

        assert!(map.contains(10));
        assert!(!map.contains(20));
    }

    #[test]
    fn test_size() {
        let mut map = TColStdMapOfInteger::new();
        assert_eq!(map.size(), 0);

        map.add(1);
        assert_eq!(map.size(), 1);

        map.add(2);
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdMapOfInteger::new();
        map.add(42);
        assert!(map.contains(42));

        assert!(map.remove(42));
        assert!(!map.contains(42));
    }

    #[test]
    fn test_clear() {
        let mut map = TColStdMapOfInteger::new();
        map.add(1);
        map.add(2);
        map.clear();
        assert_eq!(map.size(), 0);
    }
}

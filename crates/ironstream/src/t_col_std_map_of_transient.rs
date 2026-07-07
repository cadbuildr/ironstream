// FILE: t_col_std_map_of_transient.rs
// occt: TColStd_MapOfTransient

use std::collections::HashSet;

/// TColStd_MapOfTransient is a deprecated alias for a set of transient objects.
/// This is a Rust port implementing OCCT's map semantics (unordered set).
pub struct TColStdMapOfTransient {
    data: HashSet<String>,
}

impl TColStdMapOfTransient {
    /// Creates a new empty map.
    pub fn new() -> Self {
        TColStdMapOfTransient {
            data: HashSet::new(),
        }
    }

    /// Adds an element to the map.
    pub fn add(&mut self, value: String) -> bool {
        self.data.insert(value)
    }

    /// Returns the size of the map.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Checks if the map contains a value.
    pub fn contains(&self, value: &str) -> bool {
        self.data.contains(value)
    }

    /// Removes a value from the map.
    pub fn remove(&mut self, value: &str) -> bool {
        self.data.remove(value)
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for TColStdMapOfTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_contains() {
        let mut map = TColStdMapOfTransient::new();
        assert!(map.add("obj1".to_string()));
        assert!(!map.add("obj1".to_string())); // Already exists

        assert!(map.contains("obj1"));
        assert!(!map.contains("obj2"));
    }

    #[test]
    fn test_size() {
        let mut map = TColStdMapOfTransient::new();
        assert_eq!(map.size(), 0);

        map.add("a".to_string());
        assert_eq!(map.size(), 1);

        map.add("b".to_string());
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdMapOfTransient::new();
        map.add("item".to_string());
        assert!(map.contains("item"));

        assert!(map.remove("item"));
        assert!(!map.contains("item"));
    }

    #[test]
    fn test_clear() {
        let mut map = TColStdMapOfTransient::new();
        map.add("x".to_string());
        map.add("y".to_string());
        map.clear();
        assert_eq!(map.size(), 0);
    }
}

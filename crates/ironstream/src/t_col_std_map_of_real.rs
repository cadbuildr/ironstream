// FILE: t_col_std_map_of_real.rs
// occt: TColStd_MapOfReal

use std::collections::HashSet;

/// TColStd_MapOfReal is a deprecated alias for a set of real numbers.
/// This is a Rust port implementing OCCT's map semantics (unordered set).
pub struct TColStdMapOfReal {
    data: HashSet<u64>,
}

impl TColStdMapOfReal {
    /// Creates a new empty map.
    pub fn new() -> Self {
        TColStdMapOfReal {
            data: HashSet::new(),
        }
    }

    /// Adds an element to the map.
    /// Note: f64 values are converted to u64 for hashing purposes.
    pub fn add(&mut self, value: f64) -> bool {
        self.data.insert(value.to_bits())
    }

    /// Returns the size of the map.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Checks if the map contains a value.
    pub fn contains(&self, value: f64) -> bool {
        self.data.contains(&value.to_bits())
    }

    /// Removes a value from the map.
    pub fn remove(&mut self, value: f64) -> bool {
        self.data.remove(&value.to_bits())
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for TColStdMapOfReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_contains() {
        let mut map = TColStdMapOfReal::new();
        assert!(map.add(1.5));
        assert!(!map.add(1.5)); // Already exists

        assert!(map.contains(1.5));
        assert!(!map.contains(2.5));
    }

    #[test]
    fn test_size() {
        let mut map = TColStdMapOfReal::new();
        assert_eq!(map.size(), 0);

        map.add(1.0);
        assert_eq!(map.size(), 1);

        map.add(2.0);
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdMapOfReal::new();
        map.add(3.14);
        assert!(map.contains(3.14));

        assert!(map.remove(3.14));
        assert!(!map.contains(3.14));
    }

    #[test]
    fn test_clear() {
        let mut map = TColStdMapOfReal::new();
        map.add(1.0);
        map.add(2.0);
        map.clear();
        assert_eq!(map.size(), 0);
    }
}

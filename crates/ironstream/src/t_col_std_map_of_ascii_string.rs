// FILE: t_col_std_map_of_ascii_string.rs
// occt: TColStd_MapOfAsciiString

use std::collections::HashSet;

/// TColStd_MapOfAsciiString is a deprecated alias for a set of ASCII strings.
/// This is a Rust port implementing OCCT's map semantics (unordered set).
pub struct TColStdMapOfAsciiString {
    data: HashSet<String>,
}

impl TColStdMapOfAsciiString {
    /// Creates a new empty map.
    pub fn new() -> Self {
        TColStdMapOfAsciiString {
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

impl Default for TColStdMapOfAsciiString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_contains() {
        let mut map = TColStdMapOfAsciiString::new();
        assert!(map.add("hello".to_string()));
        assert!(!map.add("hello".to_string())); // Already exists

        assert!(map.contains("hello"));
        assert!(!map.contains("world"));
    }

    #[test]
    fn test_size() {
        let mut map = TColStdMapOfAsciiString::new();
        assert_eq!(map.size(), 0);

        map.add("a".to_string());
        assert_eq!(map.size(), 1);

        map.add("b".to_string());
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdMapOfAsciiString::new();
        map.add("key".to_string());
        assert!(map.contains("key"));

        assert!(map.remove("key"));
        assert!(!map.contains("key"));
        assert!(!map.remove("key"));
    }

    #[test]
    fn test_clear() {
        let mut map = TColStdMapOfAsciiString::new();
        map.add("a".to_string());
        map.add("b".to_string());
        map.clear();
        assert_eq!(map.size(), 0);
    }
}

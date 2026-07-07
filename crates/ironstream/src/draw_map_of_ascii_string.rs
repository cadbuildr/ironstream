// FILE: draw_map_of_ascii_string.rs
// occt: Draw_MapOfAsciiString

//! Deprecated: Use standard Rust collections directly.
//! This is a type alias for backward compatibility.

use std::collections::BTreeMap;

/// Deprecated: Use BTreeMap<String, String> directly instead.
/// This is a map of ASCII strings compatible with the old Draw API.
pub type DrawMapOfAsciiString = BTreeMap<usize, String>;

/// Create a new Draw_MapOfAsciiString
pub fn new() -> DrawMapOfAsciiString {
    BTreeMap::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: DrawMapOfAsciiString = new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_insert_and_get() {
        let mut map: DrawMapOfAsciiString = new();
        map.insert(1, "test".to_string());
        assert_eq!(map.get(&1), Some(&"test".to_string()));
    }

    #[test]
    fn test_map_contains() {
        let mut map: DrawMapOfAsciiString = new();
        map.insert(0, "hello".to_string());
        assert!(map.contains_key(&0));
        assert!(!map.contains_key(&1));
    }
}

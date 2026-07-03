// FILE: t_col_std_h_packed_map_of_integer.rs
// occt: TColStd_HPackedMapOfInteger

use crate::t_col_std_packed_map_of_integer::TColStdPackedMapOfInteger;

/// Handle wrapper for packed map of integers (deprecated wrapper)
/// Maps to `class TColStd_HPackedMapOfInteger : public Standard_Transient` in OCCT
/// This wraps TColStd_PackedMapOfInteger in a handle-like structure
#[derive(Debug, Clone)]
pub struct TColStdHPackedMapOfInteger {
    map: TColStdPackedMapOfInteger,
}

impl TColStdHPackedMapOfInteger {
    /// Create a new empty handle
    pub fn new() -> Self {
        TColStdHPackedMapOfInteger {
            map: TColStdPackedMapOfInteger::new(),
        }
    }

    /// Create with initial bucket count
    pub fn with_capacity(capacity: usize) -> Self {
        TColStdHPackedMapOfInteger {
            map: TColStdPackedMapOfInteger::with_capacity(capacity),
        }
    }

    /// Create from an existing packed map
    pub fn from_map(map: TColStdPackedMapOfInteger) -> Self {
        TColStdHPackedMapOfInteger { map }
    }

    /// Get const reference to the underlying map
    pub fn map(&self) -> &TColStdPackedMapOfInteger {
        &self.map
    }

    /// Get mutable reference to the underlying map
    pub fn change_map(&mut self) -> &mut TColStdPackedMapOfInteger {
        &mut self.map
    }

    /// Add a value to the map
    pub fn add(&mut self, value: i32) -> bool {
        self.map.add(value)
    }

    /// Check if value is in map
    pub fn contains(&self, value: i32) -> bool {
        self.map.contains(value)
    }

    /// Get the size
    pub fn size(&self) -> usize {
        self.map.size()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Clear the map
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl Default for TColStdHPackedMapOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TColStdHPackedMapOfInteger {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_packed_map_new() {
        let h_map = TColStdHPackedMapOfInteger::new();
        assert!(h_map.is_empty());
        assert_eq!(h_map.size(), 0);
    }

    #[test]
    fn test_h_packed_map_add() {
        let mut h_map = TColStdHPackedMapOfInteger::new();
        assert!(h_map.add(5));
        assert!(!h_map.add(5));
        assert_eq!(h_map.size(), 1);
    }

    #[test]
    fn test_h_packed_map_contains() {
        let mut h_map = TColStdHPackedMapOfInteger::new();
        h_map.add(10);
        assert!(h_map.contains(10));
        assert!(!h_map.contains(20));
    }

    #[test]
    fn test_h_packed_map_map_methods() {
        let mut h_map = TColStdHPackedMapOfInteger::new();
        h_map.add(15);
        let map = h_map.map();
        assert!(map.contains(15));
    }

    #[test]
    fn test_h_packed_map_change_map() {
        let mut h_map = TColStdHPackedMapOfInteger::new();
        {
            let map = h_map.change_map();
            map.add(25);
        }
        assert!(h_map.contains(25));
    }

    #[test]
    fn test_h_packed_map_clone() {
        let mut h_map1 = TColStdHPackedMapOfInteger::new();
        h_map1.add(7);
        let h_map2 = h_map1.clone();
        assert_eq!(h_map1, h_map2);
    }

    #[test]
    fn test_h_packed_map_clear() {
        let mut h_map = TColStdHPackedMapOfInteger::new();
        h_map.add(1);
        h_map.add(2);
        h_map.clear();
        assert!(h_map.is_empty());
    }

    #[test]
    fn test_h_packed_map_from_map() {
        let mut map = TColStdPackedMapOfInteger::new();
        map.add(99);
        let h_map = TColStdHPackedMapOfInteger::from_map(map);
        assert!(h_map.contains(99));
    }
}

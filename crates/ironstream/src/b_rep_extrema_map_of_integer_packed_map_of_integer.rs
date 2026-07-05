// FILE: b_rep_extrema_map_of_integer_packed_map_of_integer.rs
// occt: BRepExtrema_MapOfIntegerPackedMapOfInteger

//! Deprecated type alias for backward compatibility.
//! Maps integers to packed maps of integers.
//! This is a direct type alias equivalent to NCollection_DataMap<i32, PackedMapOfInteger>.

use std::collections::HashMap;

/// A data map from integers to packed maps of integers.
/// Deprecated since OCCT 8.0.0; use std::collections::HashMap directly.
pub type BRepExtremaMapOfIntegerPackedMapOfInteger = HashMap<i32, PackedMapOfInteger>;

/// A simple packed map of integers using a bit set approach.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackedMapOfInteger {
    bits: Vec<u64>,
}

impl PackedMapOfInteger {
    pub fn new() -> Self {
        Self { bits: Vec::new() }
    }

    pub fn add(&mut self, value: i32) {
        if value < 0 {
            return;
        }
        let idx = (value as usize) / 64;
        let bit = (value as usize) % 64;
        if idx >= self.bits.len() {
            self.bits.resize(idx + 1, 0);
        }
        self.bits[idx] |= 1u64 << bit;
    }

    pub fn contains(&self, value: i32) -> bool {
        if value < 0 {
            return false;
        }
        let idx = (value as usize) / 64;
        let bit = (value as usize) % 64;
        if idx >= self.bits.len() {
            false
        } else {
            (self.bits[idx] & (1u64 << bit)) != 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|&b| b == 0)
    }
}

impl Default for PackedMapOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_map_basic() {
        let mut map = PackedMapOfInteger::new();
        assert!(map.is_empty());

        map.add(0);
        assert!(map.contains(0));
        assert!(!map.contains(1));

        map.add(64);
        assert!(map.contains(64));
        assert!(!map.contains(63));
    }

    #[test]
    fn test_packed_map_multiple() {
        let mut map = PackedMapOfInteger::new();
        for i in 0..100 {
            if i % 2 == 0 {
                map.add(i);
            }
        }

        for i in 0..100 {
            if i % 2 == 0 {
                assert!(map.contains(i), "Should contain {}", i);
            } else {
                assert!(!map.contains(i), "Should not contain {}", i);
            }
        }
    }

    #[test]
    fn test_data_map() {
        let mut map: BRepExtremaMapOfIntegerPackedMapOfInteger = HashMap::new();
        let mut packed = PackedMapOfInteger::new();
        packed.add(5);
        packed.add(10);

        map.insert(1, packed.clone());

        assert!(map.contains_key(&1));
        let retrieved = map.get(&1).unwrap();
        assert!(retrieved.contains(5));
        assert!(retrieved.contains(10));
    }
}

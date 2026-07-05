// FILE: b_rep_mat2d_data_map_of_basic_elt_shape.rs
// occt: BRepMAT2d_DataMapOfBasicEltShape

//! Deprecated type alias for backward compatibility.
//! Maps basic elements to shapes.

use std::collections::HashMap;

/// A data map from basic element identifiers to shape identifiers.
pub type BRepMAT2dDataMapOfBasicEltShape = HashMap<usize, usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let mut map: BRepMAT2dDataMapOfBasicEltShape = HashMap::new();
        assert!(map.is_empty());

        map.insert(1, 100);
        assert_eq!(map.len(), 1);
        assert_eq!(map[&1], 100);
    }

    #[test]
    fn test_map_multiple_entries() {
        let mut map: BRepMAT2dDataMapOfBasicEltShape = HashMap::new();

        for i in 0..10 {
            map.insert(i, i * 100);
        }

        assert_eq!(map.len(), 10);
        assert_eq!(map[&5], 500);
        assert_eq!(map[&9], 900);
    }

    #[test]
    fn test_map_contains() {
        let mut map: BRepMAT2dDataMapOfBasicEltShape = HashMap::new();
        map.insert(1, 100);

        assert!(map.contains_key(&1));
        assert!(!map.contains_key(&2));
    }
}

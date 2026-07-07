// FILE: b_rep_fill_data_map_of_node_shape.rs
// occt: BRepFill_DataMapOfNodeShape

//! Deprecated type alias for backward compatibility.
//! Maps nodes to shapes.

use std::collections::HashMap;

/// A data map from node pointers to shape identifiers.
pub type BRepFillDataMapOfNodeShape = HashMap<usize, usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let mut map: BRepFillDataMapOfNodeShape = HashMap::new();
        assert!(map.is_empty());

        map.insert(1, 100);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), Some(&100));
    }

    #[test]
    fn test_map_multiple_entries() {
        let mut map: BRepFillDataMapOfNodeShape = HashMap::new();

        for i in 0..10 {
            map.insert(i, i * 10);
        }

        assert_eq!(map.len(), 10);
        assert_eq!(map[&5], 50);
        assert_eq!(map[&9], 90);
    }

    #[test]
    fn test_map_override() {
        let mut map: BRepFillDataMapOfNodeShape = HashMap::new();
        map.insert(1, 10);
        assert_eq!(map[&1], 10);

        map.insert(1, 20);
        assert_eq!(map[&1], 20);
    }
}

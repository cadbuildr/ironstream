// FILE: b_rep_fill_data_map_of_node_data_map_of_shape_shape.rs
// occt: BRepFill_DataMapOfNodeDataMapOfShapeShape

//! Deprecated type alias for backward compatibility.
//! Maps nodes to data maps of shape pairs.

use std::collections::HashMap;

/// A data map from node pointers to maps of shape pairs.
pub type BRepFillDataMapOfNodeDataMapOfShapeShape = HashMap<usize, HashMap<usize, usize>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_map_creation() {
        let mut outer: BRepFillDataMapOfNodeDataMapOfShapeShape = HashMap::new();
        let mut inner: HashMap<usize, usize> = HashMap::new();

        inner.insert(1, 10);
        inner.insert(2, 20);

        outer.insert(100, inner);

        assert!(outer.contains_key(&100));
        let retrieved = outer.get(&100).unwrap();
        assert_eq!(retrieved.get(&1), Some(&10));
        assert_eq!(retrieved.get(&2), Some(&20));
    }

    #[test]
    fn test_nested_map_multiple_levels() {
        let mut outer: BRepFillDataMapOfNodeDataMapOfShapeShape = HashMap::new();

        for i in 0..5 {
            let mut inner: HashMap<usize, usize> = HashMap::new();
            for j in 0..3 {
                inner.insert(j, i * 10 + j);
            }
            outer.insert(i, inner);
        }

        assert_eq!(outer.len(), 5);
        assert_eq!(outer[&2].len(), 3);
        assert_eq!(outer[&2][&1], 21);
    }
}

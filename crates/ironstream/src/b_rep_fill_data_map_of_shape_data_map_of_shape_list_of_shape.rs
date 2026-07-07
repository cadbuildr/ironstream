// FILE: b_rep_fill_data_map_of_shape_data_map_of_shape_list_of_shape.rs
// occt: BRepFill_DataMapOfShapeDataMapOfShapeListOfShape

//! Deprecated type alias for backward compatibility.
//! Maps shapes to data maps of shape lists.

use std::collections::HashMap;

/// A data map from shape identifiers to maps of shape to shape lists.
pub type BRepFillDataMapOfShapeDataMapOfShapeListOfShape = HashMap<usize, HashMap<usize, Vec<usize>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_map_creation() {
        let mut outer: BRepFillDataMapOfShapeDataMapOfShapeListOfShape = HashMap::new();
        let mut inner: HashMap<usize, Vec<usize>> = HashMap::new();

        inner.insert(1, vec![10, 20]);
        inner.insert(2, vec![30, 40, 50]);

        outer.insert(100, inner);

        assert!(outer.contains_key(&100));
        let retrieved = outer.get(&100).unwrap();
        assert_eq!(retrieved[&1], vec![10, 20]);
        assert_eq!(retrieved[&2], vec![30, 40, 50]);
    }

    #[test]
    fn test_nested_map_multiple_levels() {
        let mut outer: BRepFillDataMapOfShapeDataMapOfShapeListOfShape = HashMap::new();

        for i in 0..3 {
            let mut inner: HashMap<usize, Vec<usize>> = HashMap::new();
            for j in 0..2 {
                let mut list = Vec::new();
                for k in 0..2 {
                    list.push(i * 100 + j * 10 + k);
                }
                inner.insert(j, list);
            }
            outer.insert(i, inner);
        }

        assert_eq!(outer.len(), 3);
        assert_eq!(outer[&1].len(), 2);
        assert_eq!(outer[&1][&0], vec![100, 101]);
    }
}

// FILE: b_rep_fill_indexed_data_map_of_oriented_shape_list_of_shape.rs
// occt: BRepFill_IndexedDataMapOfOrientedShapeListOfShape

//! Deprecated type alias for backward compatibility.
//! An indexed data map from oriented shapes to lists of shapes.

use std::collections::HashMap;

/// An indexed data map where keys are integers mapped to oriented shape lists.
pub type BRepFillIndexedDataMapOfOrientedShapeListOfShape = Vec<(usize, Vec<usize>)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_map_creation() {
        let mut map: BRepFillIndexedDataMapOfOrientedShapeListOfShape = Vec::new();
        map.push((1, vec![10, 20]));
        map.push((2, vec![30, 40, 50]));

        assert_eq!(map.len(), 2);
        assert_eq!(map[0].0, 1);
        assert_eq!(map[0].1, vec![10, 20]);
    }

    #[test]
    fn test_indexed_map_retrieval() {
        let mut map: BRepFillIndexedDataMapOfOrientedShapeListOfShape = Vec::new();

        for i in 0..5 {
            let mut list = Vec::new();
            for j in 0..3 {
                list.push(i * 10 + j);
            }
            map.push((i, list));
        }

        assert_eq!(map.len(), 5);
        assert_eq!(map[2].0, 2);
        assert_eq!(map[2].1, vec![20, 21, 22]);
    }

    #[test]
    fn test_indexed_map_find() {
        let mut map: BRepFillIndexedDataMapOfOrientedShapeListOfShape = Vec::new();
        map.push((100, vec![1, 2, 3]));
        map.push((200, vec![4, 5, 6]));

        let found = map.iter().find(|(k, _)| *k == 200);
        assert!(found.is_some());
        assert_eq!(found.unwrap().1, vec![4, 5, 6]);
    }
}

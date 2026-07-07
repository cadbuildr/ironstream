// FILE: b_rep_fill_data_map_of_oriented_shape_list_of_shape.rs
// occt: BRepFill_DataMapOfOrientedShapeListOfShape

//! Deprecated type alias for backward compatibility.
//! Maps oriented shapes to lists of shapes.

use std::collections::HashMap;

/// A data map from oriented shape identifiers to lists of shape identifiers.
pub type BRepFillDataMapOfOrientedShapeListOfShape = HashMap<usize, Vec<usize>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let mut map: BRepFillDataMapOfOrientedShapeListOfShape = HashMap::new();
        assert!(map.is_empty());

        map.insert(1, vec![10, 20, 30]);
        assert_eq!(map.len(), 1);
        assert_eq!(map[&1], vec![10, 20, 30]);
    }

    #[test]
    fn test_map_multiple_entries() {
        let mut map: BRepFillDataMapOfOrientedShapeListOfShape = HashMap::new();

        for i in 0..5 {
            let mut list = Vec::new();
            for j in 0..i + 1 {
                list.push((i * 10 + j) as usize);
            }
            map.insert(i, list);
        }

        assert_eq!(map.len(), 5);
        assert_eq!(map[&3].len(), 4);
        assert_eq!(map[&3][2], 32);
    }

    #[test]
    fn test_map_append_to_list() {
        let mut map: BRepFillDataMapOfOrientedShapeListOfShape = HashMap::new();
        map.insert(1, vec![10, 20]);

        if let Some(list) = map.get_mut(&1) {
            list.push(30);
        }

        assert_eq!(map[&1], vec![10, 20, 30]);
    }
}

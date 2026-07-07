// FILE: b_rep_fill_data_map_of_shape_h_array2_of_shape.rs
// occt: BRepFill_DataMapOfShapeHArray2OfShape

//! Deprecated type alias for backward compatibility.
//! Maps shapes to 2D arrays of shapes.

use std::collections::HashMap;

/// A 2D array of shape identifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HArray2OfShape {
    pub data: Vec<usize>,
    pub rows: usize,
    pub cols: usize,
}

impl HArray2OfShape {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: usize) {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = value;
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.rows && col < self.cols {
            Some(self.data[row * self.cols + col])
        } else {
            None
        }
    }
}

/// A data map from shape identifiers to 2D arrays of shapes.
pub type BRepFillDataMapOfShapeHArray2OfShape = HashMap<usize, HArray2OfShape>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_array2_creation() {
        let arr = HArray2OfShape::new(3, 4);
        assert_eq!(arr.rows, 3);
        assert_eq!(arr.cols, 4);
        assert_eq!(arr.get(0, 0), Some(0));
    }

    #[test]
    fn test_h_array2_set_get() {
        let mut arr = HArray2OfShape::new(2, 3);
        arr.set(0, 1, 42);
        arr.set(1, 2, 99);

        assert_eq!(arr.get(0, 1), Some(42));
        assert_eq!(arr.get(1, 2), Some(99));
        assert_eq!(arr.get(0, 0), Some(0));
        assert_eq!(arr.get(2, 0), None);
    }

    #[test]
    fn test_map_creation() {
        let mut map: BRepFillDataMapOfShapeHArray2OfShape = HashMap::new();
        let mut arr = HArray2OfShape::new(2, 2);
        arr.set(0, 0, 10);
        arr.set(0, 1, 20);
        arr.set(1, 0, 30);
        arr.set(1, 1, 40);

        map.insert(1, arr);

        assert_eq!(map.len(), 1);
        assert_eq!(map[&1].get(0, 0), Some(10));
        assert_eq!(map[&1].get(1, 1), Some(40));
    }
}

// FILE: step_geom_array2_of_cartesian_point.rs
// occt: StepGeom_Array2OfCartesianPoint

use std::vec::Vec;

/// Deprecated typedef alias for NCollection_Array2<StepGeom_CartesianPoint>.
/// Provides 2D array with 1-based indexing semantics.
pub struct StepGeomArray2OfCartesianPoint {
    data: Vec<Option<String>>,
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    col_count: usize,
}

impl StepGeomArray2OfCartesianPoint {
    /// Create a 2D array with specified bounds (1-based).
    pub fn new(row_lower: usize, row_upper: usize, col_lower: usize, col_upper: usize) -> Self {
        let row_count = if row_lower > row_upper { 0 } else { row_upper - row_lower + 1 };
        let col_count = if col_lower > col_upper { 0 } else { col_upper - col_lower + 1 };
        let total_size = row_count * col_count;

        Self {
            data: vec![None; total_size],
            row_lower,
            row_upper,
            col_lower,
            col_upper,
            col_count,
        }
    }

    /// Get the lower row bound.
    pub fn row_lower(&self) -> usize {
        self.row_lower
    }

    /// Get the upper row bound.
    pub fn row_upper(&self) -> usize {
        self.row_upper
    }

    /// Get the lower column bound.
    pub fn col_lower(&self) -> usize {
        self.col_lower
    }

    /// Get the upper column bound.
    pub fn col_upper(&self) -> usize {
        self.col_upper
    }

    /// Get the total number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower || col > self.col_upper {
            return None;
        }
        let row_offset = row - self.row_lower;
        let col_offset = col - self.col_lower;
        Some(row_offset * self.col_count + col_offset)
    }

    /// Get an element by 1-based row/column indices.
    pub fn value(&self, row: usize, col: usize) -> Option<&Option<String>> {
        let idx = self.index(row, col)?;
        self.data.get(idx)
    }

    /// Set an element by 1-based row/column indices. Returns false if out of bounds.
    pub fn set_value(&mut self, row: usize, col: usize, value: Option<String>) -> bool {
        if let Some(idx) = self.index(row, col) {
            if let Some(elem) = self.data.get_mut(idx) {
                *elem = value;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array_2d() {
        let arr = StepGeomArray2OfCartesianPoint::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
        assert_eq!(arr.len(), 12);
    }

    #[test]
    fn test_2d_indexing() {
        let mut arr = StepGeomArray2OfCartesianPoint::new(1, 2, 1, 3);

        arr.set_value(1, 1, Some("a11".to_string()));
        arr.set_value(1, 2, Some("a12".to_string()));
        arr.set_value(2, 1, Some("a21".to_string()));
        arr.set_value(2, 3, Some("a23".to_string()));

        assert_eq!(arr.value(1, 1), Some(&Some("a11".to_string())));
        assert_eq!(arr.value(1, 2), Some(&Some("a12".to_string())));
        assert_eq!(arr.value(2, 1), Some(&Some("a21".to_string())));
        assert_eq!(arr.value(2, 3), Some(&Some("a23".to_string())));
    }

    #[test]
    fn test_bounds_checking() {
        let arr = StepGeomArray2OfCartesianPoint::new(1, 2, 1, 3);
        assert_eq!(arr.value(0, 1), None);
        assert_eq!(arr.value(3, 1), None);
        assert_eq!(arr.value(1, 0), None);
        assert_eq!(arr.value(1, 4), None);
    }
}

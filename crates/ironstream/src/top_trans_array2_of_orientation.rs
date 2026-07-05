// FILE: top_trans_array2_of_orientation.rs
// occt: TopTrans_Array2OfOrientation

/// TopAbs_Orientation: orientation of topological shapes.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsOrientation {
    Forward = 0,
    Reversed = 1,
    Internal = 2,
    External = 3,
}

/// Deprecated typedef: NCollection_Array2<TopAbs_Orientation>
///
/// 2D array of orientation values.
/// Maintains OCCT semantics with optional lower bounds (1-based or custom).
#[derive(Clone, Debug)]
pub struct TopTransArray2OfOrientation {
    data: Vec<TopAbsOrientation>,
    rows: usize,
    cols: usize,
    row_low: i32,
    col_low: i32,
}

impl TopTransArray2OfOrientation {
    /// Create a 2D array with given dimensions (1-based indexing by default).
    pub fn new(row_low: i32, row_high: i32, col_low: i32, col_high: i32) -> Self {
        let rows = (row_high - row_low + 1) as usize;
        let cols = (col_high - col_low + 1) as usize;
        TopTransArray2OfOrientation {
            data: vec![TopAbsOrientation::Forward; rows * cols],
            rows,
            cols,
            row_low,
            col_low,
        }
    }

    /// Create a 2D array with given dimensions and default value.
    pub fn new_with_value(
        row_low: i32,
        row_high: i32,
        col_low: i32,
        col_high: i32,
        value: TopAbsOrientation,
    ) -> Self {
        let rows = (row_high - row_low + 1) as usize;
        let cols = (col_high - col_low + 1) as usize;
        TopTransArray2OfOrientation {
            data: vec![value; rows * cols],
            rows,
            cols,
            row_low,
            col_low,
        }
    }

    /// Get value at (row, col) with OCCT-style indexing.
    pub fn value(&self, row: i32, col: i32) -> TopAbsOrientation {
        let idx = self.linear_index(row, col);
        self.data[idx]
    }

    /// Set value at (row, col) with OCCT-style indexing.
    pub fn set_value(&mut self, row: i32, col: i32, val: TopAbsOrientation) {
        let idx = self.linear_index(row, col);
        self.data[idx] = val;
    }

    /// Get the lower row bound.
    pub fn row_lower(&self) -> i32 {
        self.row_low
    }

    /// Get the upper row bound.
    pub fn row_upper(&self) -> i32 {
        self.row_low + self.rows as i32 - 1
    }

    /// Get the lower column bound.
    pub fn col_lower(&self) -> i32 {
        self.col_low
    }

    /// Get the upper column bound.
    pub fn col_upper(&self) -> i32 {
        self.col_low + self.cols as i32 - 1
    }

    /// Get number of rows.
    pub fn nb_rows(&self) -> usize {
        self.rows
    }

    /// Get number of columns.
    pub fn nb_cols(&self) -> usize {
        self.cols
    }

    /// Compute linear index from 2D indices with bounds checking.
    fn linear_index(&self, row: i32, col: i32) -> usize {
        assert!(
            row >= self.row_low && row < self.row_low + self.rows as i32,
            "row {} out of bounds [{}, {}]",
            row,
            self.row_low,
            self.row_upper()
        );
        assert!(
            col >= self.col_low && col < self.col_low + self.cols as i32,
            "col {} out of bounds [{}, {}]",
            col,
            self.col_low,
            self.col_upper()
        );
        let r = (row - self.row_low) as usize;
        let c = (col - self.col_low) as usize;
        r * self.cols + c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_array() {
        let arr = TopTransArray2OfOrientation::new(1, 3, 1, 4);
        assert_eq!(arr.nb_rows(), 3);
        assert_eq!(arr.nb_cols(), 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
    }

    #[test]
    fn test_default_value() {
        let arr = TopTransArray2OfOrientation::new(1, 2, 1, 2);
        assert_eq!(arr.value(1, 1), TopAbsOrientation::Forward);
        assert_eq!(arr.value(2, 2), TopAbsOrientation::Forward);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = TopTransArray2OfOrientation::new(1, 2, 1, 2);
        arr.set_value(1, 1, TopAbsOrientation::Reversed);
        assert_eq!(arr.value(1, 1), TopAbsOrientation::Reversed);
        assert_eq!(arr.value(1, 2), TopAbsOrientation::Forward);
    }

    #[test]
    fn test_custom_initial_value() {
        let arr = TopTransArray2OfOrientation::new_with_value(
            0,
            2,
            0,
            2,
            TopAbsOrientation::Internal,
        );
        assert_eq!(arr.value(0, 0), TopAbsOrientation::Internal);
        assert_eq!(arr.value(2, 2), TopAbsOrientation::Internal);
    }

    #[test]
    fn test_zero_based_indexing() {
        let mut arr = TopTransArray2OfOrientation::new(0, 1, 0, 1);
        arr.set_value(0, 0, TopAbsOrientation::External);
        arr.set_value(1, 1, TopAbsOrientation::Reversed);
        assert_eq!(arr.value(0, 0), TopAbsOrientation::External);
        assert_eq!(arr.value(1, 1), TopAbsOrientation::Reversed);
    }

    #[test]
    fn test_full_grid_modification() {
        let mut arr = TopTransArray2OfOrientation::new(1, 2, 1, 3);
        let values = [
            TopAbsOrientation::Forward,
            TopAbsOrientation::Reversed,
            TopAbsOrientation::Internal,
            TopAbsOrientation::External,
            TopAbsOrientation::Forward,
            TopAbsOrientation::Reversed,
        ];
        let mut idx = 0;
        for r in 1..=2 {
            for c in 1..=3 {
                arr.set_value(r, c, values[idx]);
                idx += 1;
            }
        }
        assert_eq!(arr.value(1, 1), TopAbsOrientation::Forward);
        assert_eq!(arr.value(1, 2), TopAbsOrientation::Reversed);
        assert_eq!(arr.value(2, 3), TopAbsOrientation::Reversed);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_bounds_check_row() {
        let arr = TopTransArray2OfOrientation::new(1, 2, 1, 2);
        let _ = arr.value(3, 1); // Out of bounds
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_bounds_check_col() {
        let arr = TopTransArray2OfOrientation::new(1, 2, 1, 2);
        let _ = arr.value(1, 3); // Out of bounds
    }
}

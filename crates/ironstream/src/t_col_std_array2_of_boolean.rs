// FILE: t_col_std_array2_of_boolean.rs
// occt: TColStd_Array2OfBoolean

use std::ops::Index;

/// TColStd_Array2OfBoolean is a deprecated alias for a 2D 1-based array of booleans.
/// This is a Rust port implementing OCCT's 1-based indexing semantics.
pub struct TColStdArray2OfBoolean {
    data: Vec<bool>,
    row_lower: i32,
    row_upper: i32,
    col_lower: i32,
    col_upper: i32,
}

impl TColStdArray2OfBoolean {
    /// Creates a new 2D 1-based array with the given bounds.
    pub fn new(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32) -> Self {
        if row_lower > row_upper || col_lower > col_upper {
            panic!(
                "Invalid bounds: rows [{}, {}], cols [{}, {}]",
                row_lower, row_upper, col_lower, col_upper
            );
        }
        let rows = (row_upper - row_lower + 1) as usize;
        let cols = (col_upper - col_lower + 1) as usize;
        let size = rows * cols;
        TColStdArray2OfBoolean {
            data: vec![false; size],
            row_lower,
            row_upper,
            col_lower,
            col_upper,
        }
    }

    /// Returns the lower row bound.
    pub fn row_lower(&self) -> i32 {
        self.row_lower
    }

    /// Returns the upper row bound.
    pub fn row_upper(&self) -> i32 {
        self.row_upper
    }

    /// Returns the lower column bound.
    pub fn col_lower(&self) -> i32 {
        self.col_lower
    }

    /// Returns the upper column bound.
    pub fn col_upper(&self) -> i32 {
        self.col_upper
    }

    /// Sets a value at the given 1-based row, column indices.
    pub fn set(&mut self, row: i32, col: i32, value: bool) {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower
            || col > self.col_upper
        {
            panic!(
                "Index ({}, {}) out of bounds",
                row, col
            );
        }
        let cols = (self.col_upper - self.col_lower + 1) as usize;
        let row_pos = (row - self.row_lower) as usize;
        let col_pos = (col - self.col_lower) as usize;
        let idx = row_pos * cols + col_pos;
        self.data[idx] = value;
    }

    /// Gets a reference to the value at the given 1-based indices.
    pub fn at(&self, row: i32, col: i32) -> Option<&bool> {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower
            || col > self.col_upper
        {
            return None;
        }
        let cols = (self.col_upper - self.col_lower + 1) as usize;
        let row_pos = (row - self.row_lower) as usize;
        let col_pos = (col - self.col_lower) as usize;
        let idx = row_pos * cols + col_pos;
        Some(&self.data[idx])
    }

    /// Gets a mutable reference to the value at the given 1-based indices.
    pub fn at_mut(&mut self, row: i32, col: i32) -> Option<&mut bool> {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower
            || col > self.col_upper
        {
            return None;
        }
        let cols = (self.col_upper - self.col_lower + 1) as usize;
        let row_pos = (row - self.row_lower) as usize;
        let col_pos = (col - self.col_lower) as usize;
        let idx = row_pos * cols + col_pos;
        Some(&mut self.data[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_and_bounds() {
        let arr = TColStdArray2OfBoolean::new(1, 3, 1, 2);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 2);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = TColStdArray2OfBoolean::new(1, 2, 1, 2);
        arr.set(1, 1, true);
        arr.set(1, 2, false);
        arr.set(2, 1, false);
        arr.set(2, 2, true);

        assert_eq!(arr.at(1, 1), Some(&true));
        assert_eq!(arr.at(1, 2), Some(&false));
        assert_eq!(arr.at(2, 1), Some(&false));
        assert_eq!(arr.at(2, 2), Some(&true));
    }

    #[test]
    fn test_at_out_of_bounds() {
        let arr = TColStdArray2OfBoolean::new(1, 2, 1, 2);
        assert_eq!(arr.at(5, 5), None);
    }
}

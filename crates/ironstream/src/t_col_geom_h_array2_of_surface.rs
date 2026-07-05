// FILE: t_col_geom_h_array2_of_surface.rs
// occt: TColGeom_HArray2OfSurface

use std::ops::Index;

/// TColGeom_HArray2OfSurface is a deprecated alias for a 2D 1-based array of surfaces.
/// This is a Rust port implementing OCCT's 1-based indexing semantics.
pub struct TColGeomHArray2OfSurface {
    data: Vec<Option<String>>,
    row_lower: i32,
    row_upper: i32,
    col_lower: i32,
    col_upper: i32,
}

impl TColGeomHArray2OfSurface {
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
        TColGeomHArray2OfSurface {
            data: vec![None; size],
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
    pub fn set(&mut self, row: i32, col: i32, value: Option<String>) {
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
    pub fn at(&self, row: i32, col: i32) -> Option<&Option<String>> {
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
    pub fn at_mut(&mut self, row: i32, col: i32) -> Option<&mut Option<String>> {
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
        let arr = TColGeomHArray2OfSurface::new(1, 3, 1, 2);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 2);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = TColGeomHArray2OfSurface::new(1, 2, 1, 2);
        arr.set(1, 1, Some("s11".to_string()));
        arr.set(1, 2, Some("s12".to_string()));
        arr.set(2, 1, Some("s21".to_string()));
        arr.set(2, 2, None);

        assert_eq!(arr.at(1, 1), Some(&Some("s11".to_string())));
        assert_eq!(arr.at(1, 2), Some(&Some("s12".to_string())));
        assert_eq!(arr.at(2, 1), Some(&Some("s21".to_string())));
        assert_eq!(arr.at(2, 2), Some(&None));
    }

    #[test]
    fn test_at_out_of_bounds() {
        let arr = TColGeomHArray2OfSurface::new(1, 2, 1, 2);
        assert_eq!(arr.at(5, 5), None);
        assert_eq!(arr.at(1, 5), None);
    }
}

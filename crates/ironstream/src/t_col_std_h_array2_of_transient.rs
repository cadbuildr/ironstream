// FILE: t_col_std_h_array2_of_transient.rs
// occt: TColStd_HArray2OfTransient

/// TColStd_HArray2OfTransient is a deprecated alias for a handle (heap-allocated) 2D 1-based array of transient objects.
/// This is a Rust port implementing OCCT's 1-based indexing semantics.
pub struct TColStdHArray2OfTransient {
    data: Vec<Option<String>>,
    row_lower: i32,
    row_upper: i32,
    col_lower: i32,
    col_upper: i32,
}

impl TColStdHArray2OfTransient {
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
        TColStdHArray2OfTransient {
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
        let arr = TColStdHArray2OfTransient::new(1, 3, 1, 2);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 2);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = TColStdHArray2OfTransient::new(1, 2, 1, 2);
        arr.set(1, 1, Some("obj1".to_string()));
        arr.set(1, 2, None);
        arr.set(2, 1, Some("obj3".to_string()));
        arr.set(2, 2, None);

        assert_eq!(arr.at(1, 1), Some(&Some("obj1".to_string())));
        assert_eq!(arr.at(1, 2), Some(&None));
        assert_eq!(arr.at(2, 1), Some(&Some("obj3".to_string())));
        assert_eq!(arr.at(2, 2), Some(&None));
    }

    #[test]
    fn test_at_out_of_bounds() {
        let arr = TColStdHArray2OfTransient::new(1, 2, 1, 2);
        assert_eq!(arr.at(5, 5), None);
    }
}

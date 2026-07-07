// FILE: shape_persistent_h_array2.rs
// occt: ShapePersistent_HArray2

/// Dynamic 2D array of persistent objects
pub struct HArray2 {
    rows: usize,
    cols: usize,
    data: Vec<Option<String>>,
}

impl HArray2 {
    /// Create a new 2D dynamic array
    pub fn new(rows: usize, cols: usize) -> Self {
        let size = rows * cols;
        HArray2 {
            rows,
            cols,
            data: vec![None; size],
        }
    }

    /// Get number of rows
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get number of columns
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get element at (row, col)
    pub fn get(&self, row: usize, col: usize) -> Option<&Option<String>> {
        if row < self.rows && col < self.cols {
            self.data.get(row * self.cols + col)
        } else {
            None
        }
    }

    /// Set element at (row, col)
    pub fn set(&mut self, row: usize, col: usize, value: Option<String>) {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let arr = HArray2::new(3, 4);
        assert_eq!(arr.rows(), 3);
        assert_eq!(arr.cols(), 4);
    }

    #[test]
    fn test_get_set() {
        let mut arr = HArray2::new(3, 3);
        arr.set(1, 2, Some("value".to_string()));
        assert_eq!(arr.get(1, 2), Some(&Some("value".to_string())));
    }
}

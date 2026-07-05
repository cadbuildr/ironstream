// FILE: step_element_h_array2_of_surface_element_purpose.rs
// occt: StepElement_HArray2OfSurfaceElementPurpose

/// Rust port of OCCT's deprecated StepElement_HArray2OfSurfaceElementPurpose.
/// A 2D 1-based array wrapping a Vec in row-major order, mimicking NCollection_HArray2 semantics.
#[derive(Clone, Debug)]
pub struct StepElementHArray2OfSurfaceElementPurpose {
    data: Vec<i32>,
    row_lower: i32,
    row_upper: i32,
    col_lower: i32,
    col_upper: i32,
}

impl StepElementHArray2OfSurfaceElementPurpose {
    /// Create a new HArray2 with the given row and column bounds.
    pub fn new(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32) -> Self {
        let row_count = (row_upper - row_lower + 1) as usize;
        let col_count = (col_upper - col_lower + 1) as usize;
        Self {
            data: vec![0; row_count * col_count],
            row_lower,
            row_upper,
            col_lower,
            col_upper,
        }
    }

    /// Get the row lower bound.
    pub fn row_lower(&self) -> i32 {
        self.row_lower
    }

    /// Get the row upper bound.
    pub fn row_upper(&self) -> i32 {
        self.row_upper
    }

    /// Get the column lower bound.
    pub fn col_lower(&self) -> i32 {
        self.col_lower
    }

    /// Get the column upper bound.
    pub fn col_upper(&self) -> i32 {
        self.col_upper
    }

    /// Get a reference to the value at the given 1-based indices.
    pub fn value(&self, row: i32, col: i32) -> Option<&i32> {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower || col > self.col_upper {
            return None;
        }
        let row_idx = (row - self.row_lower) as usize;
        let col_idx = (col - self.col_lower) as usize;
        let col_count = (self.col_upper - self.col_lower + 1) as usize;
        self.data.get(row_idx * col_count + col_idx)
    }

    /// Get a mutable reference to the value at the given 1-based indices.
    pub fn value_mut(&mut self, row: i32, col: i32) -> Option<&mut i32> {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower || col > self.col_upper {
            return None;
        }
        let row_idx = (row - self.row_lower) as usize;
        let col_idx = (col - self.col_lower) as usize;
        let col_count = (self.col_upper - self.col_lower + 1) as usize;
        self.data.get_mut(row_idx * col_count + col_idx)
    }

    /// Set the value at the given 1-based indices.
    pub fn set_value(&mut self, row: i32, col: i32, val: i32) -> bool {
        if let Some(r) = self.value_mut(row, col) {
            *r = val;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harray2_creation() {
        let arr = StepElementHArray2OfSurfaceElementPurpose::new(1, 3, 1, 2);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
    }

    #[test]
    fn test_harray2_set_get() {
        let mut arr = StepElementHArray2OfSurfaceElementPurpose::new(1, 2, 1, 2);
        arr.set_value(1, 1, 11);
        arr.set_value(2, 2, 22);

        assert_eq!(arr.value(1, 1), Some(&11));
        assert_eq!(arr.value(2, 2), Some(&22));
    }

    #[test]
    fn test_harray2_bounds() {
        let arr = StepElementHArray2OfSurfaceElementPurpose::new(1, 2, 1, 2);
        assert_eq!(arr.value(3, 1), None);
        assert_eq!(arr.value(1, 3), None);
    }
}

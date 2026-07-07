// FILE: step_element_array2_of_curve_element_purpose_member.rs
// occt: StepElement_Array2OfCurveElementPurposeMember

/// Rust port of OCCT's deprecated StepElement_Array2OfCurveElementPurposeMember.
/// A 2D 1-based array wrapping a Vec in row-major order, mimicking NCollection_Array2 semantics.
#[derive(Clone, Debug)]
pub struct StepElementArray2OfCurveElementPurposeMember {
    data: Vec<String>,
    row_lower: i32,
    row_upper: i32,
    col_lower: i32,
    col_upper: i32,
}

impl StepElementArray2OfCurveElementPurposeMember {
    /// Create a new Array2 with the given row and column bounds.
    pub fn new(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32) -> Self {
        let row_count = (row_upper - row_lower + 1) as usize;
        let col_count = (col_upper - col_lower + 1) as usize;
        Self {
            data: vec![String::new(); row_count * col_count],
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
    pub fn value(&self, row: i32, col: i32) -> Option<&String> {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower || col > self.col_upper {
            return None;
        }
        let row_idx = (row - self.row_lower) as usize;
        let col_idx = (col - self.col_lower) as usize;
        let col_count = (self.col_upper - self.col_lower + 1) as usize;
        self.data.get(row_idx * col_count + col_idx)
    }

    /// Get a mutable reference to the value at the given 1-based indices.
    pub fn value_mut(&mut self, row: i32, col: i32) -> Option<&mut String> {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower || col > self.col_upper {
            return None;
        }
        let row_idx = (row - self.row_lower) as usize;
        let col_idx = (col - self.col_lower) as usize;
        let col_count = (self.col_upper - self.col_lower + 1) as usize;
        self.data.get_mut(row_idx * col_count + col_idx)
    }

    /// Set the value at the given 1-based indices.
    pub fn set_value(&mut self, row: i32, col: i32, val: String) -> bool {
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
    fn test_array2_creation() {
        let arr = StepElementArray2OfCurveElementPurposeMember::new(1, 3, 1, 2);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 2);
    }

    #[test]
    fn test_array2_access() {
        let mut arr = StepElementArray2OfCurveElementPurposeMember::new(1, 2, 1, 2);
        arr.set_value(1, 1, "a".to_string());
        arr.set_value(1, 2, "b".to_string());
        arr.set_value(2, 1, "c".to_string());
        arr.set_value(2, 2, "d".to_string());

        assert_eq!(arr.value(1, 1), Some(&"a".to_string()));
        assert_eq!(arr.value(1, 2), Some(&"b".to_string()));
        assert_eq!(arr.value(2, 1), Some(&"c".to_string()));
        assert_eq!(arr.value(2, 2), Some(&"d".to_string()));
    }

    #[test]
    fn test_array2_bounds() {
        let arr = StepElementArray2OfCurveElementPurposeMember::new(1, 2, 1, 2);
        assert_eq!(arr.value(0, 1), None);
        assert_eq!(arr.value(3, 1), None);
        assert_eq!(arr.value(1, 0), None);
        assert_eq!(arr.value(1, 3), None);
    }
}

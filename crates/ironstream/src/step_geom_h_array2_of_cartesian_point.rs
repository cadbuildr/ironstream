// FILE: step_geom_h_array2_of_cartesian_point.rs
// occt: StepGeom_HArray2OfCartesianPoint

use std::vec::Vec;

pub struct StepGeomHArray2OfCartesianPoint {
    data: Vec<Option<String>>,
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    col_count: usize,
}

impl StepGeomHArray2OfCartesianPoint {
    pub fn new(row_lower: usize, row_upper: usize, col_lower: usize, col_upper: usize) -> Self {
        let row_count = if row_lower > row_upper { 0 } else { row_upper - row_lower + 1 };
        let col_count = if col_lower > col_upper { 0 } else { col_upper - col_lower + 1 };
        Self {
            data: vec![None; row_count * col_count],
            row_lower,
            row_upper,
            col_lower,
            col_upper,
            col_count,
        }
    }

    pub fn row_lower(&self) -> usize { self.row_lower }
    pub fn row_upper(&self) -> usize { self.row_upper }
    pub fn col_lower(&self) -> usize { self.col_lower }
    pub fn col_upper(&self) -> usize { self.col_upper }
    pub fn len(&self) -> usize { self.data.len() }

    fn index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower || col > self.col_upper {
            return None;
        }
        Some((row - self.row_lower) * self.col_count + (col - self.col_lower))
    }

    pub fn value(&self, row: usize, col: usize) -> Option<&Option<String>> {
        self.index(row, col).and_then(|idx| self.data.get(idx))
    }

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
    fn test_basic() {
        let arr = StepGeomHArray2OfCartesianPoint::new(1, 2, 1, 3);
        assert_eq!(arr.len(), 6);
        assert_eq!(arr.row_lower(), 1);
    }

    #[test]
    fn test_indexing() {
        let mut arr = StepGeomHArray2OfCartesianPoint::new(1, 2, 1, 2);
        arr.set_value(1, 1, Some("a".to_string()));
        assert_eq!(arr.value(1, 1), Some(&Some("a".to_string())));
    }
}

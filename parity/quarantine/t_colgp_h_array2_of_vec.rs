// FILE: t_colgp_h_array2_of_vec.rs
// occt: TColgp_HArray2OfVec

use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Vec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone)]
pub struct TColgpHArray2OfVec {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    items: Vec<Vec>,
}

impl TColgpHArray2OfVec {
    pub fn new(row_lower: usize, row_upper: usize, col_lower: usize, col_upper: usize) -> Self {
        if row_lower > row_upper || col_lower > col_upper { panic!("Invalid bounds"); }
        let rows = row_upper - row_lower + 1;
        let cols = col_upper - col_lower + 1;
        TColgpHArray2OfVec {
            data: Arc::new(Data {
                row_lower, row_upper, col_lower, col_upper,
                items: vec![Vec { x: 0.0, y: 0.0, z: 0.0 }; rows * cols],
            }),
        }
    }
    pub fn row_lower(&self) -> usize { self.data.row_lower }
    pub fn row_upper(&self) -> usize { self.data.row_upper }
    pub fn col_lower(&self) -> usize { self.data.col_lower }
    pub fn col_upper(&self) -> usize { self.data.col_upper }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _arr = TColgpHArray2OfVec::new(1, 2, 1, 2); }
}

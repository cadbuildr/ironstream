// FILE: t_colgp_h_array2_of_pnt2d.rs
// occt: TColgp_HArray2OfPnt2d

use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Pnt2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct TColgpHArray2OfPnt2d {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    items: Vec<Pnt2d>,
}

impl TColgpHArray2OfPnt2d {
    pub fn new(row_lower: usize, row_upper: usize, col_lower: usize, col_upper: usize) -> Self {
        if row_lower > row_upper || col_lower > col_upper { panic!("Invalid bounds"); }
        let rows = row_upper - row_lower + 1;
        let cols = col_upper - col_lower + 1;
        TColgpHArray2OfPnt2d {
            data: Arc::new(Data {
                row_lower, row_upper, col_lower, col_upper,
                items: vec![Pnt2d { x: 0.0, y: 0.0 }; rows * cols],
            }),
        }
    }
    pub fn row_lower(&self) -> usize { self.data.row_lower }
    pub fn row_upper(&self) -> usize { self.data.row_upper }
    pub fn col_lower(&self) -> usize { self.data.col_lower }
    pub fn col_upper(&self) -> usize { self.data.col_upper }
    pub fn get(&self, row: usize, col: usize) -> Pnt2d {
        if row < self.data.row_lower || row > self.data.row_upper || col < self.data.col_lower || col > self.data.col_upper { panic!("Out of bounds"); }
        let cols = self.data.col_upper - self.data.col_lower + 1;
        self.data.items[(row - self.data.row_lower) * cols + (col - self.data.col_lower)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _arr = TColgpHArray2OfPnt2d::new(1, 2, 1, 2); }
}

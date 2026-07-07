// FILE: t_colgp_h_array2_of_pnt.rs
// occt: TColgp_HArray2OfPnt

use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Pnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Pnt { x, y, z }
    }
}

#[derive(Debug, Clone)]
pub struct TColgpHArray2OfPnt {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    items: Vec<Pnt>,
}

impl TColgpHArray2OfPnt {
    pub fn new(row_lower: usize, row_upper: usize, col_lower: usize, col_upper: usize) -> Self {
        if row_lower > row_upper || col_lower > col_upper {
            panic!("Invalid bounds");
        }
        let rows = row_upper - row_lower + 1;
        let cols = col_upper - col_lower + 1;
        TColgpHArray2OfPnt {
            data: Arc::new(Data {
                row_lower,
                row_upper,
                col_lower,
                col_upper,
                items: vec![Pnt { x: 0.0, y: 0.0, z: 0.0 }; rows * cols],
            }),
        }
    }

    pub fn row_lower(&self) -> usize { self.data.row_lower }
    pub fn row_upper(&self) -> usize { self.data.row_upper }
    pub fn col_lower(&self) -> usize { self.data.col_lower }
    pub fn col_upper(&self) -> usize { self.data.col_upper }

    pub fn get(&self, row: usize, col: usize) -> Pnt {
        if row < self.data.row_lower || row > self.data.row_upper || col < self.data.col_lower || col > self.data.col_upper {
            panic!("Index out of bounds");
        }
        let cols = self.data.col_upper - self.data.col_lower + 1;
        let idx = (row - self.data.row_lower) * cols + (col - self.data.col_lower);
        self.data.items[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() { let _arr = TColgpHArray2OfPnt::new(1, 2, 1, 2); }

    #[test]
    fn test_clone() {
        let arr1 = TColgpHArray2OfPnt::new(1, 2, 1, 2);
        let arr2 = arr1.clone();
        assert_eq!(arr1.row_lower(), arr2.row_lower());
    }
}

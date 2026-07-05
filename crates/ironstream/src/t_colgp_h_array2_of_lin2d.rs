// FILE: t_colgp_h_array2_of_lin2d.rs
// occt: TColgp_HArray2OfLin2d

use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lin2d {
    pub px: f64,
    pub py: f64,
    pub dx: f64,
    pub dy: f64,
}

impl Lin2d {
    pub fn new(px: f64, py: f64, dx: f64, dy: f64) -> Self {
        let mag = (dx * dx + dy * dy).sqrt();
        if mag == 0.0 {
            panic!("Zero direction");
        }
        Lin2d {
            px,
            py,
            dx: dx / mag,
            dy: dy / mag,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TColgpHArray2OfLin2d {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    items: Vec<Lin2d>,
}

impl TColgpHArray2OfLin2d {
    pub fn new(row_lower: usize, row_upper: usize, col_lower: usize, col_upper: usize) -> Self {
        if row_lower > row_upper || col_lower > col_upper {
            panic!("Invalid bounds");
        }
        let rows = row_upper - row_lower + 1;
        let cols = col_upper - col_lower + 1;
        TColgpHArray2OfLin2d {
            data: Arc::new(Data {
                row_lower,
                row_upper,
                col_lower,
                col_upper,
                items: vec![Lin2d { px: 0.0, py: 0.0, dx: 1.0, dy: 0.0 }; rows * cols],
            }),
        }
    }

    pub fn row_lower(&self) -> usize { self.data.row_lower }
    pub fn row_upper(&self) -> usize { self.data.row_upper }
    pub fn col_lower(&self) -> usize { self.data.col_lower }
    pub fn col_upper(&self) -> usize { self.data.col_upper }

    pub fn get(&self, row: usize, col: usize) -> Lin2d {
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
    fn test_harray2_creation() {
        let arr = TColgpHArray2OfLin2d::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
    }

    #[test]
    fn test_harray2_clone() {
        let arr1 = TColgpHArray2OfLin2d::new(1, 2, 1, 2);
        let arr2 = arr1.clone();
        assert_eq!(arr1.row_lower(), arr2.row_lower());
    }
}

// FILE: t_colgp_h_array2_of_dir2d.rs
// occt: TColgp_HArray2OfDir2d

use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dir2d {
    pub x: f64,
    pub y: f64,
}

impl Dir2d {
    pub fn new(x: f64, y: f64) -> Self {
        let mag = (x * x + y * y).sqrt();
        if mag == 0.0 {
            panic!("Zero vector");
        }
        Dir2d {
            x: x / mag,
            y: y / mag,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TColgpHArray2OfDir2d {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    items: Vec<Dir2d>,
}

impl TColgpHArray2OfDir2d {
    pub fn new(row_lower: usize, row_upper: usize, col_lower: usize, col_upper: usize) -> Self {
        if row_lower > row_upper || col_lower > col_upper {
            panic!("Invalid bounds");
        }
        let rows = row_upper - row_lower + 1;
        let cols = col_upper - col_lower + 1;
        TColgpHArray2OfDir2d {
            data: Arc::new(Data {
                row_lower,
                row_upper,
                col_lower,
                col_upper,
                items: vec![Dir2d { x: 1.0, y: 0.0 }; rows * cols],
            }),
        }
    }

    pub fn row_lower(&self) -> usize {
        self.data.row_lower
    }
    pub fn row_upper(&self) -> usize {
        self.data.row_upper
    }
    pub fn col_lower(&self) -> usize {
        self.data.col_lower
    }
    pub fn col_upper(&self) -> usize {
        self.data.col_upper
    }

    pub fn get(&self, row: usize, col: usize) -> Dir2d {
        if row < self.data.row_lower || row > self.data.row_upper || col < self.data.col_lower
            || col > self.data.col_upper
        {
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
        let arr = TColgpHArray2OfDir2d::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
    }

    #[test]
    fn test_harray2_get() {
        let arr = TColgpHArray2OfDir2d::new(0, 2, 0, 2);
        let d = arr.get(1, 1);
        assert_eq!(d.x, 1.0);
    }

    #[test]
    fn test_harray2_clone() {
        let arr1 = TColgpHArray2OfDir2d::new(1, 2, 1, 2);
        let arr2 = arr1.clone();
        assert_eq!(arr1.row_lower(), arr2.row_lower());
    }
}

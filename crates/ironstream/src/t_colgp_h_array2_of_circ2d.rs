// FILE: t_colgp_h_array2_of_circ2d.rs
// occt: TColgp_HArray2OfCirc2d

use std::sync::Arc;

/// A simple 2D circle representation (gp_Circ2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circ2d {
    pub cx: f64,
    pub cy: f64,
    pub radius: f64,
}

impl Circ2d {
    pub fn new(cx: f64, cy: f64, radius: f64) -> Self {
        Circ2d { cx, cy, radius }
    }
}

/// Handle-based 2D array of 2D circles.
#[derive(Debug, Clone)]
pub struct TColgpHArray2OfCirc2d {
    data: Arc<TColgpArray2OfCirc2dData>,
}

#[derive(Debug)]
struct TColgpArray2OfCirc2dData {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    items: Vec<Circ2d>,
}

impl TColgpHArray2OfCirc2d {
    pub fn new(row_lower: usize, row_upper: usize, col_lower: usize, col_upper: usize) -> Self {
        if row_lower > row_upper || col_lower > col_upper {
            panic!("Invalid bounds");
        }
        let rows = row_upper - row_lower + 1;
        let cols = col_upper - col_lower + 1;
        TColgpHArray2OfCirc2d {
            data: Arc::new(TColgpArray2OfCirc2dData {
                row_lower,
                row_upper,
                col_lower,
                col_upper,
                items: vec![Circ2d { cx: 0.0, cy: 0.0, radius: 0.0 }; rows * cols],
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

    pub fn get(&self, row: usize, col: usize) -> Circ2d {
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
        let arr = TColgpHArray2OfCirc2d::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
    }

    #[test]
    fn test_harray2_get() {
        let arr = TColgpHArray2OfCirc2d::new(0, 2, 0, 2);
        let c = arr.get(1, 1);
        assert_eq!(c.cx, 0.0);
    }

    #[test]
    fn test_harray2_clone_shares_data() {
        let arr1 = TColgpHArray2OfCirc2d::new(1, 2, 1, 2);
        let arr2 = arr1.clone();
        assert_eq!(arr1.row_lower(), arr2.row_lower());
    }
}

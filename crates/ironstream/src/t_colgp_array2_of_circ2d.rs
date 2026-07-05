// FILE: t_colgp_array2_of_circ2d.rs
// occt: TColgp_Array2OfCirc2d

/// A simple 2D circle representation (gp_Circ2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circ2d {
    /// Center X
    pub cx: f64,
    /// Center Y
    pub cy: f64,
    /// Radius
    pub radius: f64,
}

impl Circ2d {
    pub fn new(cx: f64, cy: f64, radius: f64) -> Self {
        Circ2d { cx, cy, radius }
    }
}

/// 2D array (matrix) of 2D circles with bounds [row_lower..row_upper, col_lower..col_upper].
#[derive(Debug, Clone)]
pub struct TColgpArray2OfCirc2d {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    data: Vec<Circ2d>,
}

impl TColgpArray2OfCirc2d {
    /// Creates a 2D array with bounds.
    pub fn new(
        row_lower: usize,
        row_upper: usize,
        col_lower: usize,
        col_upper: usize,
    ) -> Self {
        if row_lower > row_upper || col_lower > col_upper {
            panic!(
                "Invalid bounds: rows [{}, {}], cols [{}, {}]",
                row_lower, row_upper, col_lower, col_upper
            );
        }
        let rows = row_upper - row_lower + 1;
        let cols = col_upper - col_lower + 1;
        TColgpArray2OfCirc2d {
            row_lower,
            row_upper,
            col_lower,
            col_upper,
            data: vec![Circ2d { cx: 0.0, cy: 0.0, radius: 0.0 }; rows * cols],
        }
    }

    /// Returns row lower bound.
    pub fn row_lower(&self) -> usize {
        self.row_lower
    }

    /// Returns row upper bound.
    pub fn row_upper(&self) -> usize {
        self.row_upper
    }

    /// Returns column lower bound.
    pub fn col_lower(&self) -> usize {
        self.col_lower
    }

    /// Returns column upper bound.
    pub fn col_upper(&self) -> usize {
        self.col_upper
    }

    /// Gets element at (row, col).
    pub fn get(&self, row: usize, col: usize) -> Circ2d {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower || col > self.col_upper {
            panic!(
                "Index ({}, {}) out of bounds [{},{}] x [{},{}]",
                row, col, self.row_lower, self.row_upper, self.col_lower, self.col_upper
            );
        }
        let cols = self.col_upper - self.col_lower + 1;
        let idx = (row - self.row_lower) * cols + (col - self.col_lower);
        self.data[idx]
    }

    /// Sets element at (row, col).
    pub fn set(&mut self, row: usize, col: usize, value: Circ2d) {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower || col > self.col_upper {
            panic!(
                "Index ({}, {}) out of bounds [{},{}] x [{},{}]",
                row, col, self.row_lower, self.row_upper, self.col_lower, self.col_upper
            );
        }
        let cols = self.col_upper - self.col_lower + 1;
        let idx = (row - self.row_lower) * cols + (col - self.col_lower);
        self.data[idx] = value;
    }

    /// Gets reference to element at (row, col).
    pub fn at(&self, row: usize, col: usize) -> &Circ2d {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower || col > self.col_upper {
            panic!(
                "Index ({}, {}) out of bounds [{},{}] x [{},{}]",
                row, col, self.row_lower, self.row_upper, self.col_lower, self.col_upper
            );
        }
        let cols = self.col_upper - self.col_lower + 1;
        let idx = (row - self.row_lower) * cols + (col - self.col_lower);
        &self.data[idx]
    }

    /// Gets mutable reference to element at (row, col).
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut Circ2d {
        if row < self.row_lower || row > self.row_upper || col < self.col_lower || col > self.col_upper {
            panic!(
                "Index ({}, {}) out of bounds [{},{}] x [{},{}]",
                row, col, self.row_lower, self.row_upper, self.col_lower, self.col_upper
            );
        }
        let cols = self.col_upper - self.col_lower + 1;
        let idx = (row - self.row_lower) * cols + (col - self.col_lower);
        &mut self.data[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array2_creation() {
        let arr = TColgpArray2OfCirc2d::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
    }

    #[test]
    fn test_array2_set_and_get() {
        let mut arr = TColgpArray2OfCirc2d::new(0, 2, 0, 2);
        let circ = Circ2d::new(5.0, 10.0, 2.5);
        arr.set(1, 1, circ);

        assert_eq!(arr.get(1, 1), circ);
    }

    #[test]
    fn test_array2_at_mutable() {
        let mut arr = TColgpArray2OfCirc2d::new(1, 2, 1, 2);
        arr.at_mut(1, 1).cx = 3.5;
        arr.at_mut(1, 1).radius = 1.0;

        let c = arr.get(1, 1);
        assert_eq!(c.cx, 3.5);
        assert_eq!(c.radius, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_array2_get_out_of_bounds() {
        let arr = TColgpArray2OfCirc2d::new(1, 2, 1, 2);
        let _ = arr.get(3, 1);
    }

    #[test]
    #[should_panic]
    fn test_array2_invalid_bounds() {
        let _ = TColgpArray2OfCirc2d::new(2, 1, 1, 2);
    }
}

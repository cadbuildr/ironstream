// FILE: t_colgp_array2_of_lin2d.rs
// occt: TColgp_Array2OfLin2d

/// A 2D line representation (gp_Lin2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lin2d {
    /// Point X on line
    pub px: f64,
    /// Point Y on line
    pub py: f64,
    /// Direction X (normalized)
    pub dx: f64,
    /// Direction Y (normalized)
    pub dy: f64,
}

impl Lin2d {
    /// Creates a 2D line from a point and a direction.
    pub fn new(px: f64, py: f64, dx: f64, dy: f64) -> Self {
        let mag = (dx * dx + dy * dy).sqrt();
        if mag == 0.0 {
            panic!("Cannot create line from zero direction vector");
        }
        Lin2d {
            px,
            py,
            dx: dx / mag,
            dy: dy / mag,
        }
    }
}

/// 2D array (matrix) of 2D lines with bounds [row_lower..row_upper, col_lower..col_upper].
#[derive(Debug, Clone)]
pub struct TColgpArray2OfLin2d {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    data: Vec<Lin2d>,
}

impl TColgpArray2OfLin2d {
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
        TColgpArray2OfLin2d {
            row_lower,
            row_upper,
            col_lower,
            col_upper,
            data: vec![Lin2d { px: 0.0, py: 0.0, dx: 1.0, dy: 0.0 }; rows * cols],
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
    pub fn get(&self, row: usize, col: usize) -> Lin2d {
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
    pub fn set(&mut self, row: usize, col: usize, value: Lin2d) {
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
    pub fn at(&self, row: usize, col: usize) -> &Lin2d {
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
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut Lin2d {
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
    fn test_lin2d_creation() {
        let l = Lin2d::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(l.px, 1.0);
        assert_eq!(l.py, 2.0);
    }

    #[test]
    fn test_array2_creation() {
        let arr = TColgpArray2OfLin2d::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
    }

    #[test]
    fn test_array2_set_and_get() {
        let mut arr = TColgpArray2OfLin2d::new(0, 2, 0, 2);
        let line = Lin2d::new(5.0, 6.0, 1.0, 0.0);
        arr.set(1, 1, line);

        let retrieved = arr.get(1, 1);
        assert_eq!(retrieved.px, 5.0);
        assert_eq!(retrieved.py, 6.0);
    }

    #[test]
    fn test_array2_at_mutable() {
        let mut arr = TColgpArray2OfLin2d::new(1, 2, 1, 2);
        arr.at_mut(1, 1).px = 10.0;
        arr.at_mut(1, 1).py = 20.0;

        let retrieved = arr.get(1, 1);
        assert_eq!(retrieved.px, 10.0);
        assert_eq!(retrieved.py, 20.0);
    }

    #[test]
    #[should_panic]
    fn test_array2_get_out_of_bounds() {
        let arr = TColgpArray2OfLin2d::new(1, 2, 1, 2);
        let _ = arr.get(3, 1);
    }
}

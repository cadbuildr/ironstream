// FILE: t_colgp_array2_of_vec2d.rs
// occt: TColgp_Array2OfVec2d

/// A 2D vector (gp_Vec2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2d {
    /// X component
    pub x: f64,
    /// Y component
    pub y: f64,
}

impl Vec2d {
    /// Creates a 2D vector.
    pub fn new(x: f64, y: f64) -> Self {
        Vec2d { x, y }
    }

    /// Returns the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// 2D array (matrix) of 2D vectors with bounds [row_lower..row_upper, col_lower..col_upper].
#[derive(Debug, Clone)]
pub struct TColgpArray2OfVec2d {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    data: Vec<Vec2d>,
}

impl TColgpArray2OfVec2d {
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
        TColgpArray2OfVec2d {
            row_lower,
            row_upper,
            col_lower,
            col_upper,
            data: vec![Vec2d { x: 0.0, y: 0.0 }; rows * cols],
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
    pub fn get(&self, row: usize, col: usize) -> Vec2d {
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
    pub fn set(&mut self, row: usize, col: usize, value: Vec2d) {
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
    pub fn at(&self, row: usize, col: usize) -> &Vec2d {
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
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut Vec2d {
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
    fn test_vec2d_magnitude() {
        let v = Vec2d::new(3.0, 4.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_array2_creation() {
        let arr = TColgpArray2OfVec2d::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
    }

    #[test]
    fn test_array2_set_and_get() {
        let mut arr = TColgpArray2OfVec2d::new(0, 2, 0, 2);
        let vec = Vec2d::new(7.0, 9.0);
        arr.set(1, 1, vec);

        assert_eq!(arr.get(1, 1), vec);
    }

    #[test]
    fn test_array2_at_mutable() {
        let mut arr = TColgpArray2OfVec2d::new(1, 2, 1, 2);
        arr.at_mut(1, 1).x = 11.0;
        arr.at_mut(1, 1).y = 13.0;

        let retrieved = arr.get(1, 1);
        assert_eq!(retrieved.x, 11.0);
        assert_eq!(retrieved.y, 13.0);
    }

    #[test]
    #[should_panic]
    fn test_array2_get_out_of_bounds() {
        let arr = TColgpArray2OfVec2d::new(1, 2, 1, 2);
        let _ = arr.get(3, 1);
    }
}

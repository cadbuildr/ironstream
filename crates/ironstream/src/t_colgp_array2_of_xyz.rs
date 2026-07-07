// FILE: t_colgp_array2_of_xyz.rs
// occt: TColgp_Array2OfXYZ

/// A 3D coordinate triplet (gp_XYZ in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZ {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

impl XYZ {
    /// Creates a 3D coordinate.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        XYZ { x, y, z }
    }

    /// Returns the magnitude (length) of the vector from origin.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

/// 2D array (matrix) of 3D coordinates with bounds [row_lower..row_upper, col_lower..col_upper].
#[derive(Debug, Clone)]
pub struct TColgpArray2OfXYZ {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    data: Vec<XYZ>,
}

impl TColgpArray2OfXYZ {
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
        TColgpArray2OfXYZ {
            row_lower,
            row_upper,
            col_lower,
            col_upper,
            data: vec![XYZ { x: 0.0, y: 0.0, z: 0.0 }; rows * cols],
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
    pub fn get(&self, row: usize, col: usize) -> XYZ {
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
    pub fn set(&mut self, row: usize, col: usize, value: XYZ) {
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
    pub fn at(&self, row: usize, col: usize) -> &XYZ {
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
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut XYZ {
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
    fn test_xyz_creation() {
        let xyz = XYZ::new(1.0, 2.0, 3.0);
        assert_eq!(xyz.x, 1.0);
        assert_eq!(xyz.y, 2.0);
        assert_eq!(xyz.z, 3.0);
    }

    #[test]
    fn test_xyz_magnitude() {
        let xyz = XYZ::new(1.0, 2.0, 2.0);
        assert_eq!(xyz.magnitude(), 3.0);
    }

    #[test]
    fn test_array2_creation() {
        let arr = TColgpArray2OfXYZ::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
    }

    #[test]
    fn test_array2_set_and_get() {
        let mut arr = TColgpArray2OfXYZ::new(0, 2, 0, 2);
        let xyz = XYZ::new(1.5, 2.5, 3.5);
        arr.set(1, 1, xyz);

        assert_eq!(arr.get(1, 1), xyz);
    }

    #[test]
    fn test_array2_at_mutable() {
        let mut arr = TColgpArray2OfXYZ::new(1, 2, 1, 2);
        arr.at_mut(1, 1).x = 10.0;
        arr.at_mut(1, 1).y = 20.0;
        arr.at_mut(1, 1).z = 30.0;

        let retrieved = arr.get(1, 1);
        assert_eq!(retrieved.x, 10.0);
        assert_eq!(retrieved.y, 20.0);
        assert_eq!(retrieved.z, 30.0);
    }

    #[test]
    #[should_panic]
    fn test_array2_get_out_of_bounds() {
        let arr = TColgpArray2OfXYZ::new(1, 2, 1, 2);
        let _ = arr.get(3, 1);
    }
}

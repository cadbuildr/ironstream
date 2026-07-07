// FILE: t_colgp_array2_of_dir.rs
// occt: TColgp_Array2OfDir

/// A 3D direction vector (gp_Dir in OCCT), normalized.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dir {
    /// X component (normalized)
    pub x: f64,
    /// Y component (normalized)
    pub y: f64,
    /// Z component (normalized)
    pub z: f64,
}

impl Dir {
    /// Creates a normalized 3D direction vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let mag = (x * x + y * y + z * z).sqrt();
        if mag == 0.0 {
            panic!("Cannot create direction from zero vector");
        }
        Dir {
            x: x / mag,
            y: y / mag,
            z: z / mag,
        }
    }
}

/// 2D array (matrix) of 3D direction vectors with bounds [row_lower..row_upper, col_lower..col_upper].
#[derive(Debug, Clone)]
pub struct TColgpArray2OfDir {
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
    data: Vec<Dir>,
}

impl TColgpArray2OfDir {
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
        TColgpArray2OfDir {
            row_lower,
            row_upper,
            col_lower,
            col_upper,
            data: vec![Dir { x: 1.0, y: 0.0, z: 0.0 }; rows * cols],
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
    pub fn get(&self, row: usize, col: usize) -> Dir {
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
    pub fn set(&mut self, row: usize, col: usize, value: Dir) {
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
    pub fn at(&self, row: usize, col: usize) -> &Dir {
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
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut Dir {
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
    fn test_dir_normalization() {
        let d = Dir::new(1.0, 1.0, 1.0);
        let mag = (d.x * d.x + d.y * d.y + d.z * d.z).sqrt();
        assert!((mag - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_array2_creation() {
        let arr = TColgpArray2OfDir::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
    }

    #[test]
    fn test_array2_set_and_get() {
        let mut arr = TColgpArray2OfDir::new(0, 2, 0, 2);
        let dir = Dir::new(1.0, 1.0, 1.0);
        arr.set(1, 1, dir);

        let retrieved = arr.get(1, 1);
        assert!((retrieved.x - dir.x).abs() < 1e-10);
        assert!((retrieved.y - dir.y).abs() < 1e-10);
        assert!((retrieved.z - dir.z).abs() < 1e-10);
    }

    #[test]
    fn test_array2_at_mutable() {
        let mut arr = TColgpArray2OfDir::new(1, 2, 1, 2);
        {
            let d = arr.at_mut(1, 1);
            d.x = 0.707;
            d.y = 0.707;
        }

        let retrieved = arr.get(1, 1);
        assert!((retrieved.x - 0.707).abs() < 1e-6);
    }

    #[test]
    #[should_panic]
    fn test_array2_get_out_of_bounds() {
        let arr = TColgpArray2OfDir::new(1, 2, 1, 2);
        let _ = arr.get(3, 1);
    }
}

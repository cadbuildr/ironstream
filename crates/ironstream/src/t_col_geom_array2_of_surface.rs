// FILE: t_col_geom_array2_of_surface.rs
// occt: TColGeom_Array2OfSurface

/// TColGeom_Array2OfSurface: a 2D array of 3D surface handles.
///
/// This is a 2-indexed array with Lower/Upper bounds for both dimensions.
#[derive(Debug, Clone)]
pub struct TColGeom_Array2OfSurface {
    row_lower: i32,
    row_upper: i32,
    col_lower: i32,
    col_upper: i32,
    data: Vec<u64>,
}

impl TColGeom_Array2OfSurface {
    pub fn new(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32) -> Self {
        let row_size = (row_upper - row_lower + 1) as usize;
        let col_size = (col_upper - col_lower + 1) as usize;
        let total_size = row_size * col_size;

        Self {
            row_lower,
            row_upper,
            col_lower,
            col_upper,
            data: vec![0; total_size],
        }
    }

    pub fn row_lower(&self) -> i32 {
        self.row_lower
    }

    pub fn row_upper(&self) -> i32 {
        self.row_upper
    }

    pub fn col_lower(&self) -> i32 {
        self.col_lower
    }

    pub fn col_upper(&self) -> i32 {
        self.col_upper
    }

    pub fn at(&self, row: i32, col: i32) -> u64 {
        assert!(row >= self.row_lower && row <= self.row_upper, "Row index out of bounds");
        assert!(col >= self.col_lower && col <= self.col_upper, "Column index out of bounds");

        let col_size = (self.col_upper - self.col_lower + 1) as usize;
        let row_offset = (row - self.row_lower) as usize;
        let col_offset = (col - self.col_lower) as usize;
        let index = row_offset * col_size + col_offset;

        self.data[index]
    }

    pub fn set(&mut self, row: i32, col: i32, value: u64) {
        assert!(row >= self.row_lower && row <= self.row_upper, "Row index out of bounds");
        assert!(col >= self.col_lower && col <= self.col_upper, "Column index out of bounds");

        let col_size = (self.col_upper - self.col_lower + 1) as usize;
        let row_offset = (row - self.row_lower) as usize;
        let col_offset = (col - self.col_lower) as usize;
        let index = row_offset * col_size + col_offset;

        self.data[index] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array2_bounds() {
        let arr = TColGeom_Array2OfSurface::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
    }

    #[test]
    fn test_array2_at_and_set() {
        let mut arr = TColGeom_Array2OfSurface::new(1, 3, 1, 3);
        arr.set(1, 1, 100);
        arr.set(3, 3, 200);

        assert_eq!(arr.at(1, 1), 100);
        assert_eq!(arr.at(3, 3), 200);
    }
}

// FILE: mat2d_array2_of_connexion.rs
// occt: MAT2d_Array2OfConnexion

/// Deprecated alias for NCollection_Array2<opencascade::handle<MAT2d_Connexion>>.
/// 2D array with row and column bounds (OCCT-style).
pub struct MAT2dArray2OfConnexion {
    items: Vec<u32>,   // Placeholder for MAT2d_Connexion (opaque type)
    lower_row: usize,  // OCCT array lower row bound
    upper_row: usize,  // OCCT array upper row bound
    lower_col: usize,  // OCCT array lower column bound
    upper_col: usize,  // OCCT array upper column bound
}

impl MAT2dArray2OfConnexion {
    pub fn new(lower_row: usize, upper_row: usize, lower_col: usize, upper_col: usize) -> Self {
        let row_size = if upper_row >= lower_row {
            upper_row - lower_row + 1
        } else {
            0
        };
        let col_size = if upper_col >= lower_col {
            upper_col - lower_col + 1
        } else {
            0
        };
        let size = row_size * col_size;
        Self {
            items: vec![0; size],
            lower_row,
            upper_row,
            lower_col,
            upper_col,
        }
    }

    pub fn lower_row(&self) -> usize {
        self.lower_row
    }

    pub fn upper_row(&self) -> usize {
        self.upper_row
    }

    pub fn lower_col(&self) -> usize {
        self.lower_col
    }

    pub fn upper_col(&self) -> usize {
        self.upper_col
    }

    fn row_size(&self) -> usize {
        if self.upper_row >= self.lower_row {
            self.upper_row - self.lower_row + 1
        } else {
            0
        }
    }

    fn col_size(&self) -> usize {
        if self.upper_col >= self.lower_col {
            self.upper_col - self.lower_col + 1
        } else {
            0
        }
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: u32) {
        if row >= self.lower_row && row <= self.upper_row && col >= self.lower_col && col <= self.upper_col {
            let row_idx = row - self.lower_row;
            let col_idx = col - self.lower_col;
            let idx = row_idx * self.col_size() + col_idx;
            if idx < self.items.len() {
                self.items[idx] = value;
            }
        }
    }

    pub fn value_at(&self, row: usize, col: usize) -> Option<u32> {
        if row >= self.lower_row && row <= self.upper_row && col >= self.lower_col && col <= self.upper_col {
            let row_idx = row - self.lower_row;
            let col_idx = col - self.lower_col;
            let idx = row_idx * self.col_size() + col_idx;
            if idx < self.items.len() {
                return Some(self.items[idx]);
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array2_construction() {
        let arr = MAT2dArray2OfConnexion::new(1, 3, 1, 4);
        assert_eq!(arr.lower_row(), 1);
        assert_eq!(arr.upper_row(), 3);
        assert_eq!(arr.lower_col(), 1);
        assert_eq!(arr.upper_col(), 4);
    }

    #[test]
    fn test_array2_set_and_get() {
        let mut arr = MAT2dArray2OfConnexion::new(1, 2, 1, 2);
        arr.set_value(1, 1, 10);
        arr.set_value(1, 2, 20);
        arr.set_value(2, 1, 30);
        arr.set_value(2, 2, 40);

        assert_eq!(arr.value_at(1, 1), Some(10));
        assert_eq!(arr.value_at(1, 2), Some(20));
        assert_eq!(arr.value_at(2, 1), Some(30));
        assert_eq!(arr.value_at(2, 2), Some(40));
    }

    #[test]
    fn test_array2_bounds() {
        let arr = MAT2dArray2OfConnexion::new(0, 1, 0, 2);
        assert_eq!(arr.lower_row(), 0);
        assert_eq!(arr.upper_row(), 1);
        assert_eq!(arr.lower_col(), 0);
        assert_eq!(arr.upper_col(), 2);
    }

    #[test]
    fn test_array2_out_of_bounds() {
        let arr = MAT2dArray2OfConnexion::new(1, 2, 1, 2);
        assert_eq!(arr.value_at(0, 0), None);
        assert_eq!(arr.value_at(3, 3), None);
    }

    #[test]
    fn test_empty_array2() {
        let arr = MAT2dArray2OfConnexion::new(1, 0, 1, 0);
        assert!(arr.is_empty());
    }
}

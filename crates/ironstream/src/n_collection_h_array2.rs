// FILE: n_collection_h_array2.rs
// occt: NCollection_HArray2

/// Handle to 2D array.
pub struct NCollectionHArray2<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> NCollectionHArray2<T> {
    pub fn new(rows: usize, cols: usize, init_value: T) -> Self {
        Self {
            data: vec![init_value; rows * cols],
            rows,
            cols,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> bool {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = value;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr = NCollectionHArray2::new(3, 4, 0i32);
        assert_eq!(arr.rows(), 3);
        assert_eq!(arr.cols(), 4);
    }

    #[test]
    fn test_get_set() {
        let mut arr = NCollectionHArray2::new(3, 4, 0i32);
        arr.set(1, 2, 42);
        assert_eq!(*arr.get(1, 2).unwrap(), 42);
    }
}

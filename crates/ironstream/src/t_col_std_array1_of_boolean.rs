// FILE: t_col_std_array1_of_boolean.rs
// occt: TColStd_Array1OfBoolean

use std::ops::Index;

/// TColStd_Array1OfBoolean is a deprecated alias for a 1-based array of booleans.
/// This is a Rust port implementing OCCT's 1-based indexing semantics.
pub struct TColStdArray1OfBoolean {
    data: Vec<bool>,
    lower: i32,
    upper: i32,
}

impl TColStdArray1OfBoolean {
    /// Creates a new 1-based array with the given bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        if lower > upper {
            panic!("Lower bound {} must be <= upper bound {}", lower, upper);
        }
        let size = (upper - lower + 1) as usize;
        TColStdArray1OfBoolean {
            data: vec![false; size],
            lower,
            upper,
        }
    }

    /// Returns the lower bound of the array (1-based indexing).
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// Returns the upper bound of the array (1-based indexing).
    pub fn upper(&self) -> i32 {
        self.upper
    }

    /// Returns the length of the array.
    pub fn length(&self) -> i32 {
        self.upper - self.lower + 1
    }

    /// Sets a value at the given 1-based index.
    pub fn set(&mut self, idx: i32, value: bool) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        let pos = (idx - self.lower) as usize;
        self.data[pos] = value;
    }

    /// Gets a reference to the value at the given 1-based index.
    pub fn at(&self, idx: i32) -> Option<&bool> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&self.data[pos])
    }

    /// Gets a mutable reference to the value at the given 1-based index.
    pub fn at_mut(&mut self, idx: i32) -> Option<&mut bool> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&mut self.data[pos])
    }
}

impl Index<i32> for TColStdArray1OfBoolean {
    type Output = bool;

    fn index(&self, idx: i32) -> &Self::Output {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        let pos = (idx - self.lower) as usize;
        &self.data[pos]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_and_bounds() {
        let arr = TColStdArray1OfBoolean::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = TColStdArray1OfBoolean::new(1, 3);
        arr.set(1, true);
        arr.set(2, false);
        arr.set(3, true);

        assert_eq!(arr.at(1), Some(&true));
        assert_eq!(arr.at(2), Some(&false));
        assert_eq!(arr.at(3), Some(&true));
    }

    #[test]
    fn test_index_operator() {
        let mut arr = TColStdArray1OfBoolean::new(1, 2);
        arr.set(1, true);
        assert_eq!(arr[1], true);
        assert_eq!(arr[2], false);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let arr = TColStdArray1OfBoolean::new(1, 3);
        let _ = arr[5];
    }
}

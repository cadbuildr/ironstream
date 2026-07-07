// FILE: t_col_std_array1_of_transient.rs
// occt: TColStd_Array1OfTransient

use std::ops::Index;

/// TColStd_Array1OfTransient is a deprecated alias for a 1-based array of transient objects.
/// This is a Rust port implementing OCCT's 1-based indexing semantics.
pub struct TColStdArray1OfTransient {
    data: Vec<Option<String>>,
    lower: i32,
    upper: i32,
}

impl TColStdArray1OfTransient {
    /// Creates a new 1-based array with the given bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        if lower > upper {
            panic!("Lower bound {} must be <= upper bound {}", lower, upper);
        }
        let size = (upper - lower + 1) as usize;
        TColStdArray1OfTransient {
            data: vec![None; size],
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
    pub fn set(&mut self, idx: i32, value: Option<String>) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        let pos = (idx - self.lower) as usize;
        self.data[pos] = value;
    }

    /// Gets a reference to the value at the given 1-based index.
    pub fn at(&self, idx: i32) -> Option<&Option<String>> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&self.data[pos])
    }

    /// Gets a mutable reference to the value at the given 1-based index.
    pub fn at_mut(&mut self, idx: i32) -> Option<&mut Option<String>> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&mut self.data[pos])
    }
}

impl Index<i32> for TColStdArray1OfTransient {
    type Output = Option<String>;

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
        let arr = TColStdArray1OfTransient::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = TColStdArray1OfTransient::new(1, 3);
        arr.set(1, Some("obj1".to_string()));
        arr.set(2, None);
        arr.set(3, Some("obj3".to_string()));

        assert_eq!(arr.at(1), Some(&Some("obj1".to_string())));
        assert_eq!(arr.at(2), Some(&None));
        assert_eq!(arr.at(3), Some(&Some("obj3".to_string())));
    }

    #[test]
    fn test_index_operator() {
        let mut arr = TColStdArray1OfTransient::new(1, 2);
        arr.set(1, Some("transient".to_string()));
        assert_eq!(arr[1], Some("transient".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let arr = TColStdArray1OfTransient::new(1, 3);
        let _ = arr[5];
    }
}

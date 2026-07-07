// FILE: t_col_std_array1_of_ascii_string.rs
// occt: TColStd_Array1OfAsciiString

use std::ops::Index;

/// TColStd_Array1OfAsciiString is a deprecated alias for a 1-based array of ASCII strings.
/// This is a Rust port implementing OCCT's 1-based indexing semantics.
pub struct TColStdArray1OfAsciiString {
    data: Vec<String>,
    lower: i32,
    upper: i32,
}

impl TColStdArray1OfAsciiString {
    /// Creates a new 1-based array with the given bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        if lower > upper {
            panic!("Lower bound {} must be <= upper bound {}", lower, upper);
        }
        let size = (upper - lower + 1) as usize;
        TColStdArray1OfAsciiString {
            data: vec![String::new(); size],
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
    pub fn set(&mut self, idx: i32, value: String) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        let pos = (idx - self.lower) as usize;
        self.data[pos] = value;
    }

    /// Gets a reference to the value at the given 1-based index.
    pub fn at(&self, idx: i32) -> Option<&String> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&self.data[pos])
    }

    /// Gets a mutable reference to the value at the given 1-based index.
    pub fn at_mut(&mut self, idx: i32) -> Option<&mut String> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&mut self.data[pos])
    }
}

impl Index<i32> for TColStdArray1OfAsciiString {
    type Output = String;

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
        let arr = TColStdArray1OfAsciiString::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = TColStdArray1OfAsciiString::new(1, 3);
        arr.set(1, "hello".to_string());
        arr.set(2, "world".to_string());

        assert_eq!(arr.at(1), Some(&"hello".to_string()));
        assert_eq!(arr.at(2), Some(&"world".to_string()));
    }

    #[test]
    fn test_index_operator() {
        let mut arr = TColStdArray1OfAsciiString::new(1, 2);
        arr.set(1, "test".to_string());
        assert_eq!(arr[1], "test".to_string());
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let arr = TColStdArray1OfAsciiString::new(1, 3);
        let _ = arr[5];
    }
}

// FILE: t_col_std_h_array1_of_byte.rs
// occt: TColStd_HArray1OfByte

use std::ops::Index;

/// TColStd_HArray1OfByte is a deprecated alias for a handle (heap-allocated) 1-based array of bytes.
/// This is a Rust port implementing OCCT's 1-based indexing semantics.
pub struct TColStdHArray1OfByte {
    data: Vec<u8>,
    lower: i32,
    upper: i32,
}

impl TColStdHArray1OfByte {
    /// Creates a new 1-based array with the given bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        if lower > upper {
            panic!("Lower bound {} must be <= upper bound {}", lower, upper);
        }
        let size = (upper - lower + 1) as usize;
        TColStdHArray1OfByte {
            data: vec![0u8; size],
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
    pub fn set(&mut self, idx: i32, value: u8) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        let pos = (idx - self.lower) as usize;
        self.data[pos] = value;
    }

    /// Gets a reference to the value at the given 1-based index.
    pub fn at(&self, idx: i32) -> Option<&u8> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&self.data[pos])
    }

    /// Gets a mutable reference to the value at the given 1-based index.
    pub fn at_mut(&mut self, idx: i32) -> Option<&mut u8> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&mut self.data[pos])
    }
}

impl Index<i32> for TColStdHArray1OfByte {
    type Output = u8;

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
        let arr = TColStdHArray1OfByte::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = TColStdHArray1OfByte::new(1, 3);
        arr.set(1, 10u8);
        arr.set(2, 20u8);
        arr.set(3, 255u8);

        assert_eq!(arr.at(1), Some(&10u8));
        assert_eq!(arr.at(2), Some(&20u8));
        assert_eq!(arr.at(3), Some(&255u8));
    }

    #[test]
    fn test_index_operator() {
        let mut arr = TColStdHArray1OfByte::new(1, 2);
        arr.set(1, 42u8);
        assert_eq!(arr[1], 42u8);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let arr = TColStdHArray1OfByte::new(1, 3);
        let _ = arr[5];
    }
}

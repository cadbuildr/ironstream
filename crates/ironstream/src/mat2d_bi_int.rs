// FILE: mat2d_bi_int.rs
// occt: MAT2d_BiInt

/// A pair of two integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mat2dBiInt {
    first_index: i32,
    second_index: i32,
}

impl Mat2dBiInt {
    /// Create a pair of integers.
    pub fn new(i1: i32, i2: i32) -> Self {
        Mat2dBiInt {
            first_index: i1,
            second_index: i2,
        }
    }

    /// Return the first index.
    pub fn first_index(&self) -> i32 {
        self.first_index
    }

    /// Return the second index.
    pub fn second_index(&self) -> i32 {
        self.second_index
    }

    /// Set the first index.
    pub fn set_first_index(&mut self, i1: i32) {
        self.first_index = i1;
    }

    /// Set the second index.
    pub fn set_second_index(&mut self, i2: i32) {
        self.second_index = i2;
    }

    /// Test equality.
    pub fn is_equal(&self, other: &Mat2dBiInt) -> bool {
        self.first_index == other.first_index && self.second_index == other.second_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bi_int_creation() {
        let bi = Mat2dBiInt::new(5, 10);
        assert_eq!(bi.first_index(), 5);
        assert_eq!(bi.second_index(), 10);
    }

    #[test]
    fn test_bi_int_setters() {
        let mut bi = Mat2dBiInt::new(1, 2);
        bi.set_first_index(3);
        bi.set_second_index(4);
        assert_eq!(bi.first_index(), 3);
        assert_eq!(bi.second_index(), 4);
    }

    #[test]
    fn test_bi_int_equality() {
        let bi1 = Mat2dBiInt::new(7, 8);
        let bi2 = Mat2dBiInt::new(7, 8);
        let bi3 = Mat2dBiInt::new(7, 9);
        assert!(bi1.is_equal(&bi2));
        assert!(!bi1.is_equal(&bi3));
    }

    #[test]
    fn test_bi_int_hash() {
        let bi1 = Mat2dBiInt::new(1, 2);
        let bi2 = Mat2dBiInt::new(1, 2);
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(bi1);
        assert!(set.contains(&bi2));
    }
}

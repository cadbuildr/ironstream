// FILE: iges_solid_h_array1_of_face.rs
// occt: IGESSolid_HArray1OfFace

use std::vec::Vec;

/// Deprecated alias for a 1D array of IGESSolid_Face objects.
/// This is a legacy wrapper over a Vec for OCCT compatibility.
#[derive(Clone, Debug)]
pub struct IGESSolid_HArray1OfFace {
    data: Vec<i32>,
    lower: i32,
}

impl IGESSolid_HArray1OfFace {
    /// Create a new array with specified bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        IGESSolid_HArray1OfFace {
            data: vec![0; size],
            lower,
        }
    }

    /// Set value at given index.
    pub fn set_value(&mut self, index: i32, value: i32) {
        let pos = (index - self.lower) as usize;
        if pos < self.data.len() {
            self.data[pos] = value;
        }
    }

    /// Get value at given index.
    pub fn value(&self, index: i32) -> Option<i32> {
        let pos = (index - self.lower) as usize;
        self.data.get(pos).copied()
    }

    /// Get lower bound.
    pub fn lower_bound(&self) -> i32 {
        self.lower
    }

    /// Get upper bound.
    pub fn upper_bound(&self) -> i32 {
        self.lower + self.data.len() as i32 - 1
    }

    /// Get array length.
    pub fn length(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array() {
        let arr = IGESSolid_HArray1OfFace::new(1, 5);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_set_and_get_value() {
        let mut arr = IGESSolid_HArray1OfFace::new(0, 3);
        arr.set_value(0, 10);
        arr.set_value(2, 20);
        assert_eq!(arr.value(0), Some(10));
        assert_eq!(arr.value(2), Some(20));
        assert_eq!(arr.value(1), Some(0));
    }

    #[test]
    fn test_bounds_offset() {
        let mut arr = IGESSolid_HArray1OfFace::new(5, 9);
        arr.set_value(5, 100);
        arr.set_value(9, 200);
        assert_eq!(arr.value(5), Some(100));
        assert_eq!(arr.value(9), Some(200));
    }
}

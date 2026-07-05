// FILE: iges_solid_h_array1_of_loop.rs
// occt: IGESSolid_HArray1OfLoop

use std::vec::Vec;

/// Deprecated alias for a 1D array of IGESSolid_Loop objects.
/// This is a legacy wrapper over a Vec for OCCT compatibility.
#[derive(Clone, Debug)]
pub struct IGESSolid_HArray1OfLoop {
    data: Vec<i32>,
    lower: i32,
}

impl IGESSolid_HArray1OfLoop {
    /// Create a new array with specified bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        IGESSolid_HArray1OfLoop {
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
        let arr = IGESSolid_HArray1OfLoop::new(1, 4);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 4);
        assert_eq!(arr.length(), 4);
    }

    #[test]
    fn test_set_and_get_value() {
        let mut arr = IGESSolid_HArray1OfLoop::new(0, 2);
        arr.set_value(0, 5);
        arr.set_value(2, 15);
        assert_eq!(arr.value(0), Some(5));
        assert_eq!(arr.value(2), Some(15));
        assert_eq!(arr.value(1), Some(0));
    }
}

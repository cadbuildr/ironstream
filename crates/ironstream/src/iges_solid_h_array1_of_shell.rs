// FILE: iges_solid_h_array1_of_shell.rs
// occt: IGESSolid_HArray1OfShell

use std::vec::Vec;

/// Deprecated alias for a 1D array of IGESSolid_Shell objects.
/// This is a legacy wrapper over a Vec for OCCT compatibility.
#[derive(Clone, Debug)]
pub struct IGESSolid_HArray1OfShell {
    data: Vec<i32>,
    lower: i32,
}

impl IGESSolid_HArray1OfShell {
    /// Create a new array with specified bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        IGESSolid_HArray1OfShell {
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
        let arr = IGESSolid_HArray1OfShell::new(1, 3);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 3);
        assert_eq!(arr.length(), 3);
    }

    #[test]
    fn test_set_and_get_value() {
        let mut arr = IGESSolid_HArray1OfShell::new(0, 1);
        arr.set_value(0, 42);
        arr.set_value(1, 99);
        assert_eq!(arr.value(0), Some(42));
        assert_eq!(arr.value(1), Some(99));
    }
}

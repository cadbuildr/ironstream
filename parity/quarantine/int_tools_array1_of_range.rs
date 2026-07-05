// FILE: int_tools_array1_of_range.rs
// occt: IntTools_Array1OfRange

use std::vec::Vec;

/// Range specification: (lower, upper)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Range {
    pub lower: f64,
    pub upper: f64,
}

impl Range {
    /// Create a new range.
    pub fn new(lower: f64, upper: f64) -> Self {
        Range { lower, upper }
    }

    /// Get the range width.
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }

    /// Check if a value is in the range.
    pub fn contains(&self, value: f64) -> bool {
        value >= self.lower && value <= self.upper
    }
}

/// Deprecated alias for a 1D array of ranges.
#[derive(Clone, Debug)]
pub struct IntTools_Array1OfRange {
    data: Vec<Range>,
    lower: i32,
}

impl IntTools_Array1OfRange {
    /// Create a new array with specified bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        IntTools_Array1OfRange {
            data: vec![Range::new(0.0, 0.0); size],
            lower,
        }
    }

    /// Set value at given index.
    pub fn set_value(&mut self, index: i32, range: Range) {
        let pos = (index - self.lower) as usize;
        if pos < self.data.len() {
            self.data[pos] = range;
        }
    }

    /// Get value at given index.
    pub fn value(&self, index: i32) -> Option<Range> {
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
    fn test_range_new() {
        let range = Range::new(1.0, 5.0);
        assert_eq!(range.lower, 1.0);
        assert_eq!(range.upper, 5.0);
    }

    #[test]
    fn test_range_width() {
        let range = Range::new(1.0, 5.0);
        assert_eq!(range.width(), 4.0);
    }

    #[test]
    fn test_range_contains() {
        let range = Range::new(1.0, 5.0);
        assert!(range.contains(3.0));
        assert!(!range.contains(6.0));
    }

    #[test]
    fn test_array_new() {
        let arr = IntTools_Array1OfRange::new(0, 3);
        assert_eq!(arr.lower_bound(), 0);
        assert_eq!(arr.upper_bound(), 3);
        assert_eq!(arr.length(), 4);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = IntTools_Array1OfRange::new(0, 2);
        arr.set_value(0, Range::new(1.0, 2.0));
        arr.set_value(2, Range::new(5.0, 6.0));
        assert_eq!(arr.value(0), Some(Range::new(1.0, 2.0)));
        assert_eq!(arr.value(2), Some(Range::new(5.0, 6.0)));
    }
}

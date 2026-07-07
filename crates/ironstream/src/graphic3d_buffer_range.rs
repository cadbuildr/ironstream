// FILE: graphic3d_buffer_range.rs
// occt: Graphic3d_BufferRange

//! Range of values defined as Start + Length pair.
//!
//! Represents a contiguous range of buffer elements, defined by a starting position
//! and the number of elements.

/// Range of buffer values defined as start position and length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferRange {
    /// First element within the range.
    pub start: i32,
    /// Number of elements within the range.
    pub length: i32,
}

impl BufferRange {
    /// Creates an empty buffer range.
    pub fn new() -> Self {
        BufferRange {
            start: 0,
            length: 0,
        }
    }

    /// Creates a buffer range with specified start and length.
    pub fn with_bounds(start: i32, length: i32) -> Self {
        BufferRange { start, length }
    }

    /// Returns true if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the upper (last) element index within the range.
    /// Formula: start + length - 1
    pub fn upper(&self) -> i32 {
        self.start + self.length - 1
    }

    /// Clears the range, setting start to 0 and length to 0.
    pub fn clear(&mut self) {
        self.start = 0;
        self.length = 0;
    }

    /// Unites another range with this one, expanding to cover both ranges.
    ///
    /// If this range is empty, it becomes equal to the other range.
    /// If the other range is empty, this range remains unchanged.
    /// Otherwise, both ranges are combined into a single larger range.
    pub fn unite(&mut self, other: &BufferRange) {
        if self.is_empty() {
            *self = *other;
            return;
        }

        if other.is_empty() {
            return;
        }

        let new_start = std::cmp::min(self.start, other.start);
        let new_last = std::cmp::max(self.upper(), other.upper());
        self.start = new_start;
        self.length = new_last - new_start + 1;
    }
}

impl Default for BufferRange {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_range_new() {
        let range = BufferRange::new();
        assert!(range.is_empty());
        assert_eq!(range.start, 0);
        assert_eq!(range.length, 0);
    }

    #[test]
    fn test_buffer_range_with_bounds() {
        let range = BufferRange::with_bounds(10, 5);
        assert!(!range.is_empty());
        assert_eq!(range.start, 10);
        assert_eq!(range.length, 5);
    }

    #[test]
    fn test_buffer_range_upper() {
        let range = BufferRange::with_bounds(10, 5);
        // upper should be 10 + 5 - 1 = 14
        assert_eq!(range.upper(), 14);
    }

    #[test]
    fn test_buffer_range_upper_single_element() {
        let range = BufferRange::with_bounds(10, 1);
        assert_eq!(range.upper(), 10);
    }

    #[test]
    fn test_buffer_range_clear() {
        let mut range = BufferRange::with_bounds(10, 5);
        assert!(!range.is_empty());
        range.clear();
        assert!(range.is_empty());
        assert_eq!(range.start, 0);
        assert_eq!(range.length, 0);
    }

    #[test]
    fn test_buffer_range_unite_empty_with_nonempty() {
        let mut range1 = BufferRange::new();
        let range2 = BufferRange::with_bounds(5, 10);
        range1.unite(&range2);
        assert_eq!(range1.start, 5);
        assert_eq!(range1.length, 10);
    }

    #[test]
    fn test_buffer_range_unite_nonempty_with_empty() {
        let mut range1 = BufferRange::with_bounds(5, 10);
        let range2 = BufferRange::new();
        range1.unite(&range2);
        assert_eq!(range1.start, 5);
        assert_eq!(range1.length, 10);
    }

    #[test]
    fn test_buffer_range_unite_overlapping() {
        let mut range1 = BufferRange::with_bounds(0, 10);
        let range2 = BufferRange::with_bounds(5, 10);
        range1.unite(&range2);
        // range1: [0, 9], range2: [5, 14]
        // Result should be: [0, 14]
        assert_eq!(range1.start, 0);
        assert_eq!(range1.upper(), 14);
        assert_eq!(range1.length, 15);
    }

    #[test]
    fn test_buffer_range_unite_disjoint() {
        let mut range1 = BufferRange::with_bounds(0, 5);
        let range2 = BufferRange::with_bounds(10, 5);
        range1.unite(&range2);
        // range1: [0, 4], range2: [10, 14]
        // Result should be: [0, 14]
        assert_eq!(range1.start, 0);
        assert_eq!(range1.upper(), 14);
        assert_eq!(range1.length, 15);
    }

    #[test]
    fn test_buffer_range_unite_one_inside_other() {
        let mut range1 = BufferRange::with_bounds(0, 20);
        let range2 = BufferRange::with_bounds(5, 5);
        range1.unite(&range2);
        // range1 should remain: [0, 19]
        assert_eq!(range1.start, 0);
        assert_eq!(range1.upper(), 19);
    }

    #[test]
    fn test_buffer_range_default() {
        let range = BufferRange::default();
        assert!(range.is_empty());
    }
}

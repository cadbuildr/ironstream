// FILE: int_tools_marked_range_set.rs
// occt: IntTools_MarkedRangeSet

use super::int_tools_range::IntToolsRange;

/// A set of ranges marked with flags.
#[derive(Clone, Debug)]
pub struct IntToolsMarkedRangeSet {
    boundaries: Vec<f64>,
    flags: Vec<i32>,
}

impl IntToolsMarkedRangeSet {
    /// Create an empty marked range set.
    pub fn new() -> Self {
        IntToolsMarkedRangeSet {
            boundaries: Vec::new(),
            flags: Vec::new(),
        }
    }

    /// Create with initial boundaries.
    pub fn with_boundaries(first: f64, last: f64, init_flag: i32) -> Self {
        let mut set = IntToolsMarkedRangeSet::new();
        set.set_boundaries(first, last, init_flag);
        set
    }

    /// Set the boundaries.
    pub fn set_boundaries(&mut self, first: f64, last: f64, init_flag: i32) {
        self.boundaries.clear();
        self.flags.clear();
        self.boundaries.push(first);
        self.boundaries.push(last);
        self.flags.push(init_flag);
    }

    /// Insert a new range.
    pub fn insert_range(&mut self, first: f64, last: f64, flag: i32) -> bool {
        if self.boundaries.is_empty() {
            return false;
        }
        if first < self.boundaries[0] || last > self.boundaries[self.boundaries.len() - 1] {
            return false;
        }
        // Simplified insertion logic
        let mut pos = 0;
        while pos < self.boundaries.len() - 1 && self.boundaries[pos + 1] <= first {
            pos += 2;
        }
        self.boundaries.insert(pos, first);
        self.boundaries.insert(pos + 1, last);
        self.flags.insert(pos / 2, flag);
        true
    }

    /// Get the number of ranges.
    pub fn length(&self) -> i32 {
        if self.boundaries.is_empty() {
            0
        } else {
            (self.boundaries.len() / 2) as i32
        }
    }

    /// Get the range at index.
    pub fn range(&self, index: i32) -> Option<IntToolsRange> {
        if index < 1 || index as usize > self.boundaries.len() / 2 {
            return None;
        }
        let idx = (index - 1) as usize;
        if idx * 2 + 1 < self.boundaries.len() {
            Some(IntToolsRange::with_bounds(
                self.boundaries[idx * 2],
                self.boundaries[idx * 2 + 1],
            ))
        } else {
            None
        }
    }

    /// Set flag for a range.
    pub fn set_flag(&mut self, index: i32, flag: i32) {
        if index >= 1 && (index as usize) <= self.flags.len() {
            self.flags[(index - 1) as usize] = flag;
        }
    }

    /// Get flag for a range.
    pub fn flag(&self, index: i32) -> i32 {
        if index >= 1 && (index as usize) <= self.flags.len() {
            self.flags[(index - 1) as usize]
        } else {
            0
        }
    }
}

impl Default for IntToolsMarkedRangeSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set = IntToolsMarkedRangeSet::new();
        assert_eq!(set.length(), 0);
    }

    #[test]
    fn test_with_boundaries() {
        let set = IntToolsMarkedRangeSet::with_boundaries(0.0, 10.0, 1);
        assert_eq!(set.length(), 1);
        assert_eq!(set.flag(1), 1);
    }

    #[test]
    fn test_set_flag() {
        let mut set = IntToolsMarkedRangeSet::with_boundaries(0.0, 10.0, 1);
        set.set_flag(1, 5);
        assert_eq!(set.flag(1), 5);
    }

    #[test]
    fn test_range() {
        let set = IntToolsMarkedRangeSet::with_boundaries(0.0, 10.0, 1);
        let range = set.range(1).unwrap();
        assert_eq!(range.first(), 0.0);
        assert_eq!(range.last(), 10.0);
    }
}

// FILE: int_tools_range.rs
// occt: IntTools_Range

/// Represents a 1-D range [first, last].
#[derive(Clone, Copy, Debug)]
pub struct IntToolsRange {
    first: f64,
    last: f64,
}

impl IntToolsRange {
    /// Create an empty range (0.0, 0.0).
    pub fn new() -> Self {
        IntToolsRange {
            first: 0.0,
            last: 0.0,
        }
    }

    /// Create a range with boundaries.
    pub fn with_bounds(first: f64, last: f64) -> Self {
        IntToolsRange { first, last }
    }

    /// Set the first boundary.
    pub fn set_first(&mut self, first: f64) {
        self.first = first;
    }

    /// Set the last boundary.
    pub fn set_last(&mut self, last: f64) {
        self.last = last;
    }

    /// Get the first boundary.
    pub fn first(&self) -> f64 {
        self.first
    }

    /// Get the last boundary.
    pub fn last(&self) -> f64 {
        self.last
    }

    /// Get both boundaries.
    pub fn range(&self) -> (f64, f64) {
        (self.first, self.last)
    }
}

impl Default for IntToolsRange {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_range() {
        let r = IntToolsRange::new();
        assert_eq!(r.first(), 0.0);
        assert_eq!(r.last(), 0.0);
    }

    #[test]
    fn test_with_bounds() {
        let r = IntToolsRange::with_bounds(1.5, 3.7);
        assert_eq!(r.first(), 1.5);
        assert_eq!(r.last(), 3.7);
    }

    #[test]
    fn test_set_boundaries() {
        let mut r = IntToolsRange::new();
        r.set_first(2.0);
        r.set_last(5.0);
        let (f, l) = r.range();
        assert_eq!(f, 2.0);
        assert_eq!(l, 5.0);
    }

    #[test]
    fn test_clone() {
        let r1 = IntToolsRange::with_bounds(1.0, 2.0);
        let r2 = r1;
        assert_eq!(r1.first(), r2.first());
        assert_eq!(r1.last(), r2.last());
    }
}

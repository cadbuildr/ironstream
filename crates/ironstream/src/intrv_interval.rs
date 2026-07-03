// FILE: intrv_interval.rs
// occt: Intrv_Interval

use crate::intrv_position::IntrvPosition;

/// Represents a real interval with start and end bounds plus tolerances.
/// Tolerances at boundaries allow fuzzy interval comparison.
#[derive(Clone, Debug)]
pub struct IntrvInterval {
    start: f64,
    end: f64,
    tol_start: f32,
    tol_end: f32,
}

impl IntrvInterval {
    /// Creates a void (empty) interval.
    pub fn new() -> Self {
        IntrvInterval {
            start: 0.0,
            end: 0.0,
            tol_start: 0.0,
            tol_end: 0.0,
        }
    }

    /// Creates an interval from start to end with zero tolerances.
    pub fn from_bounds(start: f64, end: f64) -> Self {
        IntrvInterval {
            start,
            end,
            tol_start: 0.0,
            tol_end: 0.0,
        }
    }

    /// Creates an interval with explicit tolerances.
    pub fn with_tolerances(start: f64, tol_start: f32, end: f64, tol_end: f32) -> Self {
        IntrvInterval {
            start,
            end,
            tol_start,
            tol_end,
        }
    }

    pub fn start(&self) -> f64 {
        self.start
    }

    pub fn end(&self) -> f64 {
        self.end
    }

    pub fn tol_start(&self) -> f32 {
        self.tol_start
    }

    pub fn tol_end(&self) -> f32 {
        self.tol_end
    }

    pub fn bounds(&self) -> (f64, f32, f64, f32) {
        (self.start, self.tol_start, self.end, self.tol_end)
    }

    pub fn set_start(&mut self, start: f64, tol_start: f32) {
        self.start = start;
        self.tol_start = tol_start;
    }

    /// Fuses (extends) this interval at the start with a new start bound.
    pub fn fuse_at_start(&mut self, start: f64, tol_start: f32) {
        if start < self.start {
            self.start = start;
            self.tol_start = tol_start;
        }
    }

    /// Cuts (narrows) this interval at the start.
    pub fn cut_at_start(&mut self, start: f64, tol_start: f32) {
        if start > self.start {
            self.start = start;
            self.tol_start = tol_start;
        }
    }

    pub fn set_end(&mut self, end: f64, tol_end: f32) {
        self.end = end;
        self.tol_end = tol_end;
    }

    /// Fuses (extends) this interval at the end with a new end bound.
    pub fn fuse_at_end(&mut self, end: f64, tol_end: f32) {
        if end > self.end {
            self.end = end;
            self.tol_end = tol_end;
        }
    }

    /// Cuts (narrows) this interval at the end.
    pub fn cut_at_end(&mut self, end: f64, tol_end: f32) {
        if end < self.end {
            self.end = end;
            self.tol_end = tol_end;
        }
    }

    /// Returns true if interval is probably empty (tolerance exceeds bounds).
    pub fn is_probably_empty(&self) -> bool {
        self.start + (self.tol_start as f64) > self.end - (self.tol_end as f64)
            || self.end + (self.tol_end as f64) > self.start - (self.tol_start as f64)
    }

    /// Determines the position of this interval relative to another.
    pub fn position(&self, other: &IntrvInterval) -> IntrvPosition {
        if self.is_after(other) {
            IntrvPosition::After
        } else if self.is_just_after(other) {
            IntrvPosition::JustAfter
        } else if self.is_overlapping_at_end(other) {
            IntrvPosition::OverlappingAtEnd
        } else if self.is_just_overlapping_at_end(other) {
            IntrvPosition::JustOverlappingAtEnd
        } else if self.is_inside(other) {
            IntrvPosition::Inside
        } else if self.is_just_enclosing_at_start(other) {
            IntrvPosition::JustEnclosingAtStart
        } else if self.is_similar(other) {
            IntrvPosition::Similar
        } else if self.is_just_overlapping_at_start(other) {
            IntrvPosition::JustOverlappingAtStart
        } else if self.is_enclosing(other) {
            IntrvPosition::Enclosing
        } else if self.is_just_enclosing_at_end(other) {
            IntrvPosition::JustEnclosingAtEnd
        } else if self.is_overlapping_at_start(other) {
            IntrvPosition::OverlappingAtStart
        } else if self.is_just_before(other) {
            IntrvPosition::JustBefore
        } else {
            IntrvPosition::Before
        }
    }

    pub fn is_before(&self, other: &IntrvInterval) -> bool {
        self.end - (self.tol_end as f64) < other.start + (other.tol_start as f64)
    }

    pub fn is_after(&self, other: &IntrvInterval) -> bool {
        self.start + (self.tol_start as f64) > other.end - (other.tol_end as f64)
    }

    pub fn is_inside(&self, other: &IntrvInterval) -> bool {
        self.start + (self.tol_start as f64) >= other.start + (other.tol_start as f64)
            && self.end - (self.tol_end as f64) <= other.end - (other.tol_end as f64)
    }

    pub fn is_enclosing(&self, other: &IntrvInterval) -> bool {
        self.start + (self.tol_start as f64) <= other.start + (other.tol_start as f64)
            && self.end - (self.tol_end as f64) >= other.end - (other.tol_end as f64)
    }

    pub fn is_just_enclosing_at_start(&self, other: &IntrvInterval) -> bool {
        (self.start + (self.tol_start as f64) - (other.start + (other.tol_start as f64))).abs() < 1e-10
            && self.end - (self.tol_end as f64) >= other.end - (other.tol_end as f64)
    }

    pub fn is_just_enclosing_at_end(&self, other: &IntrvInterval) -> bool {
        self.start + (self.tol_start as f64) <= other.start + (other.tol_start as f64)
            && (self.end - (self.tol_end as f64) - (other.end - (other.tol_end as f64))).abs() < 1e-10
    }

    pub fn is_just_before(&self, other: &IntrvInterval) -> bool {
        (self.end - (self.tol_end as f64) - (other.start + (other.tol_start as f64))).abs() < 1e-10
    }

    pub fn is_just_after(&self, other: &IntrvInterval) -> bool {
        (self.start + (self.tol_start as f64) - (other.end - (other.tol_end as f64))).abs() < 1e-10
    }

    pub fn is_overlapping_at_start(&self, other: &IntrvInterval) -> bool {
        self.start + (self.tol_start as f64) < other.start + (other.tol_start as f64)
            && self.end - (self.tol_end as f64) > other.start + (other.tol_start as f64)
            && self.end - (self.tol_end as f64) < other.end - (other.tol_end as f64)
    }

    pub fn is_overlapping_at_end(&self, other: &IntrvInterval) -> bool {
        self.start + (self.tol_start as f64) > other.start + (other.tol_start as f64)
            && self.start + (self.tol_start as f64) < other.end - (other.tol_end as f64)
            && self.end - (self.tol_end as f64) > other.end - (other.tol_end as f64)
    }

    pub fn is_just_overlapping_at_start(&self, other: &IntrvInterval) -> bool {
        (self.start + (self.tol_start as f64) - (other.start + (other.tol_start as f64))).abs() < 1e-10
            && self.end - (self.tol_end as f64) < other.end - (other.tol_end as f64)
    }

    pub fn is_just_overlapping_at_end(&self, other: &IntrvInterval) -> bool {
        self.start + (self.tol_start as f64) > other.start + (other.tol_start as f64)
            && (self.end - (self.tol_end as f64) - (other.end - (other.tol_end as f64))).abs() < 1e-10
    }

    pub fn is_similar(&self, other: &IntrvInterval) -> bool {
        (self.start + (self.tol_start as f64) - (other.start + (other.tol_start as f64))).abs() < 1e-10
            && (self.end - (self.tol_end as f64) - (other.end - (other.tol_end as f64))).abs() < 1e-10
    }
}

impl Default for IntrvInterval {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_creation() {
        let i = IntrvInterval::from_bounds(1.0, 5.0);
        assert_eq!(i.start(), 1.0);
        assert_eq!(i.end(), 5.0);
    }

    #[test]
    fn test_interval_with_tolerances() {
        let i = IntrvInterval::with_tolerances(1.0, 0.1, 5.0, 0.2);
        assert!((i.tol_start() - 0.1).abs() < 1e-6);
        assert!((i.tol_end() - 0.2).abs() < 1e-6);
    }

    #[test]
    fn test_is_before() {
        let i1 = IntrvInterval::from_bounds(1.0, 3.0);
        let i2 = IntrvInterval::from_bounds(5.0, 7.0);
        assert!(i1.is_before(&i2));
        assert!(!i2.is_before(&i1));
    }

    #[test]
    fn test_is_inside() {
        let i1 = IntrvInterval::from_bounds(2.0, 4.0);
        let i2 = IntrvInterval::from_bounds(1.0, 5.0);
        assert!(i1.is_inside(&i2));
        assert!(!i2.is_inside(&i1));
    }

    #[test]
    fn test_is_enclosing() {
        let i1 = IntrvInterval::from_bounds(1.0, 5.0);
        let i2 = IntrvInterval::from_bounds(2.0, 4.0);
        assert!(i1.is_enclosing(&i2));
        assert!(!i2.is_enclosing(&i1));
    }

    #[test]
    fn test_fuse_at_start() {
        let mut i = IntrvInterval::from_bounds(2.0, 5.0);
        i.fuse_at_start(1.0, 0.0);
        assert_eq!(i.start(), 1.0);
        i.fuse_at_start(3.0, 0.0);
        assert_eq!(i.start(), 1.0); // should not change
    }

    #[test]
    fn test_is_probably_empty() {
        let i = IntrvInterval::with_tolerances(1.0, 1.0, 1.5, 0.6);
        assert!(i.is_probably_empty());
    }
}

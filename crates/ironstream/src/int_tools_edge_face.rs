// FILE: int_tools_edge_face.rs
// occt: IntTools_EdgeFace

use super::int_tools_range::IntToolsRange;

/// Edge/Face intersection algorithm.
#[derive(Clone, Debug)]
pub struct IntToolsEdgeFace {
    fuzzy_value: f64,
    is_done: bool,
    error_status: i32,
    min_distance: f64,
    range: IntToolsRange,
    quick_coincidence_check: bool,
}

impl IntToolsEdgeFace {
    /// Create an empty edge/face intersector.
    pub fn new() -> Self {
        IntToolsEdgeFace {
            fuzzy_value: 1e-6,
            is_done: false,
            error_status: 1, // Not started
            min_distance: f64::MAX,
            range: IntToolsRange::new(),
            quick_coincidence_check: false,
        }
    }

    /// Set the fuzzy value (tolerance).
    pub fn set_fuzzy_value(&mut self, fuzz: f64) {
        self.fuzzy_value = fuzz.max(1e-12);
    }

    /// Get the fuzzy value.
    pub fn fuzzy_value(&self) -> f64 {
        self.fuzzy_value
    }

    /// Set the range for the edge.
    pub fn set_range(&mut self, first: f64, last: f64) {
        self.range.set_first(first);
        self.range.set_last(last);
    }

    /// Get the range.
    pub fn range(&self) -> IntToolsRange {
        self.range
    }

    /// Set quick coincidence check flag.
    pub fn use_quick_coincidence_check(&mut self, flag: bool) {
        self.quick_coincidence_check = flag;
    }

    /// Check if quick coincidence flag is set.
    pub fn is_coincidence_checked_quickly(&self) -> bool {
        self.quick_coincidence_check
    }

    /// Perform the intersection (placeholder).
    pub fn perform(&mut self) {
        self.is_done = true;
        self.error_status = 0;
    }

    /// Check if computation was successful.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Get the error status.
    pub fn error_status(&self) -> i32 {
        self.error_status
    }

    /// Get the minimal distance found.
    pub fn minimal_distance(&self) -> f64 {
        self.min_distance
    }
}

impl Default for IntToolsEdgeFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ef = IntToolsEdgeFace::new();
        assert!(!ef.is_done());
        assert_eq!(ef.error_status(), 1);
    }

    #[test]
    fn test_set_fuzzy_value() {
        let mut ef = IntToolsEdgeFace::new();
        ef.set_fuzzy_value(0.001);
        assert_eq!(ef.fuzzy_value(), 0.001);
    }

    #[test]
    fn test_set_range() {
        let mut ef = IntToolsEdgeFace::new();
        ef.set_range(1.0, 5.0);
        let range = ef.range();
        assert_eq!(range.first(), 1.0);
        assert_eq!(range.last(), 5.0);
    }

    #[test]
    fn test_perform() {
        let mut ef = IntToolsEdgeFace::new();
        ef.perform();
        assert!(ef.is_done());
        assert_eq!(ef.error_status(), 0);
    }
}

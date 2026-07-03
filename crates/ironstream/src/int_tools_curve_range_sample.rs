// FILE: int_tools_curve_range_sample.rs
// occt: IntTools_CurveRangeSample

use super::int_tools_base_range_sample::IntToolsBaseRangeSample;
use super::int_tools_range::IntToolsRange;

/// Sample for curve range indexing.
#[derive(Clone, Copy, Debug)]
pub struct IntToolsCurveRangeSample {
    base: IntToolsBaseRangeSample,
    index: i32,
}

impl IntToolsCurveRangeSample {
    /// Create an empty curve range sample.
    pub fn new() -> Self {
        IntToolsCurveRangeSample {
            base: IntToolsBaseRangeSample::new(),
            index: 0,
        }
    }

    /// Create with an index.
    pub fn with_index(index: i32) -> Self {
        IntToolsCurveRangeSample {
            base: IntToolsBaseRangeSample::new(),
            index,
        }
    }

    /// Set the range index.
    pub fn set_range_index(&mut self, index: i32) {
        self.index = index;
    }

    /// Get the range index.
    pub fn get_range_index(&self) -> i32 {
        self.index
    }

    /// Get depth (from base).
    pub fn get_depth(&self) -> i32 {
        self.base.get_depth()
    }

    /// Set depth (on base).
    pub fn set_depth(&mut self, depth: i32) {
        self.base.set_depth(depth);
    }

    /// Check equality.
    pub fn is_equal(&self, other: &IntToolsCurveRangeSample) -> bool {
        self.index == other.index && self.get_depth() == other.get_depth()
    }

    /// Compute the range from first, last, and number of samples.
    pub fn get_range(&self, first: f64, last: f64, nb_sample: i32) -> IntToolsRange {
        let step = (last - first) / nb_sample as f64;
        let range_first = first + self.index as f64 * step;
        let range_last = range_first + step;
        IntToolsRange::with_bounds(range_first, range_last)
    }

    /// Get the deeper range index.
    pub fn get_range_index_deeper(&self, nb_sample: i32) -> i32 {
        self.index * nb_sample
    }
}

impl Default for IntToolsCurveRangeSample {
    fn default() -> Self {
        Self::new()
    }
}

impl std::cmp::PartialEq for IntToolsCurveRangeSample {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl std::hash::Hash for IntToolsCurveRangeSample {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_depth().hash(state);
        self.index.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sample = IntToolsCurveRangeSample::new();
        assert_eq!(sample.get_range_index(), 0);
        assert_eq!(sample.get_depth(), 0);
    }

    #[test]
    fn test_with_index() {
        let sample = IntToolsCurveRangeSample::with_index(3);
        assert_eq!(sample.get_range_index(), 3);
    }

    #[test]
    fn test_get_range() {
        let mut sample = IntToolsCurveRangeSample::with_index(1);
        sample.set_depth(0);
        let range = sample.get_range(0.0, 10.0, 5);
        assert_eq!(range.first(), 2.0);
        assert_eq!(range.last(), 4.0);
    }

    #[test]
    fn test_is_equal() {
        let s1 = IntToolsCurveRangeSample::with_index(2);
        let s2 = IntToolsCurveRangeSample::with_index(2);
        assert!(s1.is_equal(&s2));
    }

    #[test]
    fn test_get_range_index_deeper() {
        let sample = IntToolsCurveRangeSample::with_index(3);
        assert_eq!(sample.get_range_index_deeper(2), 6);
    }
}

// FILE: int_tools_list_of_curve_range_sample.rs
// occt: IntTools_ListOfCurveRangeSample

use std::vec::Vec;

/// Curve range sample specification
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurveRangeSample {
    pub curve_id: u32,
    pub range_start: f64,
    pub range_end: f64,
}

impl CurveRangeSample {
    /// Create a new curve range sample.
    pub fn new(curve_id: u32, range_start: f64, range_end: f64) -> Self {
        CurveRangeSample {
            curve_id,
            range_start,
            range_end,
        }
    }

    /// Get the range width.
    pub fn width(&self) -> f64 {
        self.range_end - self.range_start
    }
}

/// Deprecated alias for a list of curve range samples.
#[derive(Clone, Debug)]
pub struct IntTools_ListOfCurveRangeSample {
    samples: Vec<CurveRangeSample>,
}

impl IntTools_ListOfCurveRangeSample {
    /// Create a new list.
    pub fn new() -> Self {
        IntTools_ListOfCurveRangeSample {
            samples: Vec::new(),
        }
    }

    /// Append a curve range sample to the list.
    pub fn append(&mut self, sample: CurveRangeSample) {
        self.samples.push(sample);
    }

    /// Get the number of samples.
    pub fn length(&self) -> usize {
        self.samples.len()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Get a sample by index.
    pub fn sample(&self, index: usize) -> Option<CurveRangeSample> {
        self.samples.get(index).copied()
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.samples.clear();
    }
}

impl Default for IntTools_ListOfCurveRangeSample {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_range_sample_new() {
        let sample = CurveRangeSample::new(1, 0.0, 1.0);
        assert_eq!(sample.curve_id, 1);
        assert_eq!(sample.width(), 1.0);
    }

    #[test]
    fn test_list_new() {
        let list = IntTools_ListOfCurveRangeSample::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list = IntTools_ListOfCurveRangeSample::new();
        let sample = CurveRangeSample::new(1, 0.0, 1.0);
        list.append(sample);
        assert_eq!(list.length(), 1);
        assert_eq!(list.sample(0), Some(sample));
    }

    #[test]
    fn test_clear() {
        let mut list = IntTools_ListOfCurveRangeSample::new();
        list.append(CurveRangeSample::new(1, 0.0, 1.0));
        list.clear();
        assert!(list.is_empty());
    }
}

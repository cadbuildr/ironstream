// FILE: int_tools_curve_range_localize_data.rs
// occt: IntTools_CurveRangeLocalizeData

use super::int_tools_curve_range_sample::IntToolsCurveRangeSample;
use std::collections::{HashMap, HashSet};

/// Localized data for curve range (bounding boxes and out-ranges).
#[derive(Clone, Debug)]
pub struct IntToolsCurveRangeLocalizeData {
    nb_sample: i32,
    min_range: f64,
    map_range_out: HashSet<i32>, // Hash of curve range samples
    map_box: HashMap<i32, (f64, f64, f64, f64)>, // Sample index -> bbox
}

impl IntToolsCurveRangeLocalizeData {
    /// Create with sample count and min range.
    pub fn new(nb_sample: i32, min_range: f64) -> Self {
        IntToolsCurveRangeLocalizeData {
            nb_sample,
            min_range,
            map_range_out: HashSet::new(),
            map_box: HashMap::new(),
        }
    }

    /// Get number of samples.
    pub fn get_nb_sample(&self) -> i32 {
        self.nb_sample
    }

    /// Get minimum range.
    pub fn get_min_range(&self) -> f64 {
        self.min_range
    }

    /// Add an out-range.
    pub fn add_out_range(&mut self, sample: &IntToolsCurveRangeSample) {
        let hash = sample.get_range_index() as i32;
        self.map_range_out.insert(hash);
    }

    /// Add a bounding box.
    pub fn add_box(&mut self, sample: &IntToolsCurveRangeSample, bbox: (f64, f64, f64, f64)) {
        let hash = sample.get_range_index() as i32;
        self.map_box.insert(hash, bbox);
    }

    /// Find a bounding box.
    pub fn find_box(&self, sample: &IntToolsCurveRangeSample) -> Option<(f64, f64, f64, f64)> {
        let hash = sample.get_range_index() as i32;
        self.map_box.get(&hash).copied()
    }

    /// Check if range is out.
    pub fn is_range_out(&self, sample: &IntToolsCurveRangeSample) -> bool {
        let hash = sample.get_range_index() as i32;
        self.map_range_out.contains(&hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = IntToolsCurveRangeLocalizeData::new(10, 0.01);
        assert_eq!(data.get_nb_sample(), 10);
        assert_eq!(data.get_min_range(), 0.01);
    }

    #[test]
    fn test_add_out_range() {
        let mut data = IntToolsCurveRangeLocalizeData::new(5, 0.01);
        let sample = IntToolsCurveRangeSample::with_index(2);
        data.add_out_range(&sample);
        assert!(data.is_range_out(&sample));
    }

    #[test]
    fn test_add_find_box() {
        let mut data = IntToolsCurveRangeLocalizeData::new(5, 0.01);
        let sample = IntToolsCurveRangeSample::with_index(1);
        let bbox = (0.0, 1.0, 0.0, 1.0);
        data.add_box(&sample, bbox);
        assert_eq!(data.find_box(&sample), Some(bbox));
    }
}

// FILE: int_tools_surface_range_sample.rs
// occt: IntTools_SurfaceRangeSample

use super::int_tools_curve_range_sample::IntToolsCurveRangeSample;
use super::int_tools_range::IntToolsRange;

/// Surface range sample (2D: U and V).
#[derive(Clone, Copy, Debug)]
pub struct IntToolsSurfaceRangeSample {
    range_u: IntToolsCurveRangeSample,
    range_v: IntToolsCurveRangeSample,
}

impl IntToolsSurfaceRangeSample {
    /// Create an empty surface range sample.
    pub fn new() -> Self {
        IntToolsSurfaceRangeSample {
            range_u: IntToolsCurveRangeSample::new(),
            range_v: IntToolsCurveRangeSample::new(),
        }
    }

    /// Create with indices and depths.
    pub fn with_params(index_u: i32, depth_u: i32, index_v: i32, depth_v: i32) -> Self {
        let mut range_u = IntToolsCurveRangeSample::with_index(index_u);
        range_u.set_depth(depth_u);
        let mut range_v = IntToolsCurveRangeSample::with_index(index_v);
        range_v.set_depth(depth_v);
        IntToolsSurfaceRangeSample { range_u, range_v }
    }

    /// Set ranges.
    pub fn set_ranges(&mut self, range_u: IntToolsCurveRangeSample, range_v: IntToolsCurveRangeSample) {
        self.range_u = range_u;
        self.range_v = range_v;
    }

    /// Get ranges.
    pub fn get_ranges(&self) -> (IntToolsCurveRangeSample, IntToolsCurveRangeSample) {
        (self.range_u, self.range_v)
    }

    /// Set indices.
    pub fn set_indexes(&mut self, index_u: i32, index_v: i32) {
        self.range_u.set_range_index(index_u);
        self.range_v.set_range_index(index_v);
    }

    /// Get indices.
    pub fn get_indexes(&self) -> (i32, i32) {
        (self.range_u.get_range_index(), self.range_v.get_range_index())
    }

    /// Get depths.
    pub fn get_depths(&self) -> (i32, i32) {
        (self.range_u.get_depth(), self.range_v.get_depth())
    }

    /// Set sample range U.
    pub fn set_sample_range_u(&mut self, range: IntToolsCurveRangeSample) {
        self.range_u = range;
    }

    /// Get sample range U.
    pub fn get_sample_range_u(&self) -> IntToolsCurveRangeSample {
        self.range_u
    }

    /// Set sample range V.
    pub fn set_sample_range_v(&mut self, range: IntToolsCurveRangeSample) {
        self.range_v = range;
    }

    /// Get sample range V.
    pub fn get_sample_range_v(&self) -> IntToolsCurveRangeSample {
        self.range_v
    }

    /// Set index U.
    pub fn set_index_u(&mut self, index: i32) {
        self.range_u.set_range_index(index);
    }

    /// Get index U.
    pub fn get_index_u(&self) -> i32 {
        self.range_u.get_range_index()
    }

    /// Set index V.
    pub fn set_index_v(&mut self, index: i32) {
        self.range_v.set_range_index(index);
    }

    /// Get index V.
    pub fn get_index_v(&self) -> i32 {
        self.range_v.get_range_index()
    }

    /// Set depth U.
    pub fn set_depth_u(&mut self, depth: i32) {
        self.range_u.set_depth(depth);
    }

    /// Get depth U.
    pub fn get_depth_u(&self) -> i32 {
        self.range_u.get_depth()
    }

    /// Set depth V.
    pub fn set_depth_v(&mut self, depth: i32) {
        self.range_v.set_depth(depth);
    }

    /// Get depth V.
    pub fn get_depth_v(&self) -> i32 {
        self.range_v.get_depth()
    }

    /// Get range U.
    pub fn get_range_u(&self, first_u: f64, last_u: f64, nb_sample_u: i32) -> IntToolsRange {
        self.range_u.get_range(first_u, last_u, nb_sample_u)
    }

    /// Get range V.
    pub fn get_range_v(&self, first_v: f64, last_v: f64, nb_sample_v: i32) -> IntToolsRange {
        self.range_v.get_range(first_v, last_v, nb_sample_v)
    }

    /// Check equality.
    pub fn is_equal(&self, other: &IntToolsSurfaceRangeSample) -> bool {
        self.range_u.is_equal(&other.range_u) && self.range_v.is_equal(&other.range_v)
    }

    /// Get deeper range index U.
    pub fn get_range_index_u_deeper(&self, nb_sample_u: i32) -> i32 {
        self.range_u.get_range_index_deeper(nb_sample_u)
    }

    /// Get deeper range index V.
    pub fn get_range_index_v_deeper(&self, nb_sample_v: i32) -> i32 {
        self.range_v.get_range_index_deeper(nb_sample_v)
    }
}

impl Default for IntToolsSurfaceRangeSample {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for IntToolsSurfaceRangeSample {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sample = IntToolsSurfaceRangeSample::new();
        assert_eq!(sample.get_index_u(), 0);
        assert_eq!(sample.get_index_v(), 0);
    }

    #[test]
    fn test_with_params() {
        let sample = IntToolsSurfaceRangeSample::with_params(1, 2, 3, 4);
        assert_eq!(sample.get_index_u(), 1);
        assert_eq!(sample.get_depth_u(), 2);
        assert_eq!(sample.get_index_v(), 3);
        assert_eq!(sample.get_depth_v(), 4);
    }

    #[test]
    fn test_get_range_u() {
        let sample = IntToolsSurfaceRangeSample::with_params(1, 0, 0, 0);
        let range = sample.get_range_u(0.0, 10.0, 5);
        assert_eq!(range.first(), 2.0);
        assert_eq!(range.last(), 4.0);
    }
}

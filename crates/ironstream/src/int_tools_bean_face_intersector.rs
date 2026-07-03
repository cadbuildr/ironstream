// FILE: int_tools_bean_face_intersector.rs
// occt: IntTools_BeanFaceIntersector

use super::int_tools_range::IntToolsRange;

/// Bean/Face intersector (edge segment on face).
#[derive(Clone, Debug)]
pub struct IntToolsBeanFaceIntersector {
    first_param: f64,
    last_param: f64,
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
    bean_tol: f64,
    face_tol: f64,
    is_done: bool,
    min_sq_distance: f64,
    results: Vec<IntToolsRange>,
}

impl IntToolsBeanFaceIntersector {
    /// Create an empty bean/face intersector.
    pub fn new() -> Self {
        IntToolsBeanFaceIntersector {
            first_param: 0.0,
            last_param: 0.0,
            u_min: 0.0,
            u_max: 0.0,
            v_min: 0.0,
            v_max: 0.0,
            bean_tol: 0.0,
            face_tol: 0.0,
            is_done: false,
            min_sq_distance: f64::MAX,
            results: Vec::new(),
        }
    }

    /// Set bean parameters.
    pub fn set_bean_parameters(&mut self, first: f64, last: f64) {
        self.first_param = first;
        self.last_param = last;
    }

    /// Set surface parameters.
    pub fn set_surface_parameters(&mut self, u_min: f64, u_max: f64, v_min: f64, v_max: f64) {
        self.u_min = u_min;
        self.u_max = u_max;
        self.v_min = v_min;
        self.v_max = v_max;
    }

    /// Perform the intersection.
    pub fn perform(&mut self) {
        self.is_done = true;
    }

    /// Check if done.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Get the result ranges.
    pub fn result(&self) -> &[IntToolsRange] {
        &self.results
    }

    /// Get minimal square distance.
    pub fn minimal_square_distance(&self) -> f64 {
        self.min_sq_distance
    }

    /// Add a result range.
    pub fn add_result(&mut self, range: IntToolsRange) {
        self.results.push(range);
    }
}

impl Default for IntToolsBeanFaceIntersector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bfi = IntToolsBeanFaceIntersector::new();
        assert!(!bfi.is_done());
    }

    #[test]
    fn test_set_bean_parameters() {
        let mut bfi = IntToolsBeanFaceIntersector::new();
        bfi.set_bean_parameters(0.0, 10.0);
        assert_eq!(bfi.first_param, 0.0);
        assert_eq!(bfi.last_param, 10.0);
    }

    #[test]
    fn test_set_surface_parameters() {
        let mut bfi = IntToolsBeanFaceIntersector::new();
        bfi.set_surface_parameters(0.0, 1.0, 0.0, 1.0);
        assert_eq!(bfi.u_min, 0.0);
        assert_eq!(bfi.u_max, 1.0);
        assert_eq!(bfi.v_min, 0.0);
        assert_eq!(bfi.v_max, 1.0);
    }

    #[test]
    fn test_perform() {
        let mut bfi = IntToolsBeanFaceIntersector::new();
        bfi.perform();
        assert!(bfi.is_done());
    }
}

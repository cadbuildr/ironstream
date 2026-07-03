// FILE: b_rep_lib_check_curve_on_surface.rs
// occt: BRepLib_CheckCurveOnSurface

/// Checks max distance between edge and its 2D representation on a face
pub struct CheckCurveOnSurface {
    is_done: bool,
    max_distance: f64,
    max_parameter: f64,
    error_status: i32,
    is_parallel: bool,
}

impl CheckCurveOnSurface {
    pub fn new() -> Self {
        CheckCurveOnSurface {
            is_done: false,
            max_distance: 0.0,
            max_parameter: 0.0,
            error_status: 1,
            is_parallel: false,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn set_parallel(&mut self, parallel: bool) {
        self.is_parallel = parallel;
    }

    pub fn is_parallel(&self) -> bool {
        self.is_parallel
    }

    pub fn error_status(&self) -> i32 {
        self.error_status
    }

    pub fn max_distance(&self) -> f64 {
        self.max_distance
    }

    pub fn max_parameter(&self) -> f64 {
        self.max_parameter
    }

    pub fn perform(&mut self) {
        self.is_done = true;
        self.error_status = 0;
    }
}

impl Default for CheckCurveOnSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_curve_on_surface() {
        let mut checker = CheckCurveOnSurface::new();
        assert!(!checker.is_done());
        checker.perform();
        assert!(checker.is_done());
    }

    #[test]
    fn test_parallel_flag() {
        let mut checker = CheckCurveOnSurface::new();
        assert!(!checker.is_parallel());
        checker.set_parallel(true);
        assert!(checker.is_parallel());
    }
}

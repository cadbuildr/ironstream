// FILE: approx_fit_and_divide2d.rs
// occt: Approx_FitAndDivide2d

use crate::app_par_curves_constraint::AppParCurvesConstraint;

/// Approximates 2D curves with automatic knot insertion and splitting.
pub struct ApproxFitAndDivide2d {
    degree_min: i32,
    degree_max: i32,
    tolerance_3d: f64,
    tolerance_2d: f64,
    cutting: bool,
    first_constraint: AppParCurvesConstraint,
    last_constraint: AppParCurvesConstraint,
    max_segments: i32,
    inv_order: bool,
    hang_checking: bool,
    is_approximated: bool,
    is_tolerance_reached: bool,
    nb_multi_curves: i32,
}

impl ApproxFitAndDivide2d {
    /// Creates a new approximator with default settings.
    pub fn new() -> Self {
        ApproxFitAndDivide2d {
            degree_min: 3,
            degree_max: 8,
            tolerance_3d: 1.0e-5,
            tolerance_2d: 1.0e-5,
            cutting: false,
            first_constraint: AppParCurvesConstraint::TangencyPoint,
            last_constraint: AppParCurvesConstraint::TangencyPoint,
            max_segments: 10,
            inv_order: true,
            hang_checking: true,
            is_approximated: false,
            is_tolerance_reached: false,
            nb_multi_curves: 0,
        }
    }

    /// Creates with specific degree and tolerance settings.
    pub fn with_settings(
        degree_min: i32,
        degree_max: i32,
        tol_3d: f64,
        tol_2d: f64,
    ) -> Self {
        let mut approx = Self::new();
        approx.degree_min = degree_min;
        approx.degree_max = degree_max;
        approx.tolerance_3d = tol_3d;
        approx.tolerance_2d = tol_2d;
        approx
    }

    /// Sets the degrees of approximation.
    pub fn set_degrees(&mut self, degree_min: i32, degree_max: i32) {
        self.degree_min = degree_min;
        self.degree_max = degree_max;
    }

    /// Sets the tolerances.
    pub fn set_tolerances(&mut self, tol_3d: f64, tol_2d: f64) {
        self.tolerance_3d = tol_3d;
        self.tolerance_2d = tol_2d;
    }

    /// Sets the constraints.
    pub fn set_constraints(
        &mut self,
        first_c: AppParCurvesConstraint,
        last_c: AppParCurvesConstraint,
    ) {
        self.first_constraint = first_c;
        self.last_constraint = last_c;
    }

    /// Sets the maximum number of segments.
    pub fn set_max_segments(&mut self, max_seg: i32) {
        self.max_segments = max_seg;
    }

    /// Sets inverse order of degree selection.
    pub fn set_inv_order(&mut self, inv_order: bool) {
        self.inv_order = inv_order;
    }

    /// Sets hang checking flag.
    pub fn set_hang_checking(&mut self, hang_check: bool) {
        self.hang_checking = hang_check;
    }

    /// Returns False if approximation failed.
    pub fn is_all_approximated(&self) -> bool {
        self.is_approximated
    }

    /// Returns False if tolerance was not reached.
    pub fn is_tolerance_reached(&self) -> bool {
        self.is_tolerance_reached
    }

    /// Returns the number of multi-curves.
    pub fn nb_multi_curves(&self) -> i32 {
        self.nb_multi_curves
    }

    /// Sets the approximation result.
    pub fn set_all_approximated(&mut self, val: bool) {
        self.is_approximated = val;
    }

    /// Sets tolerance reached flag.
    pub fn set_tolerance_reached(&mut self, val: bool) {
        self.is_tolerance_reached = val;
    }

    /// Sets number of multi-curves.
    pub fn set_nb_multi_curves(&mut self, nb: i32) {
        self.nb_multi_curves = nb;
    }
}

impl Default for ApproxFitAndDivide2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let approx = ApproxFitAndDivide2d::new();
        assert_eq!(approx.degree_min, 3);
        assert_eq!(approx.degree_max, 8);
        assert_eq!(approx.tolerance_3d, 1.0e-5);
        assert_eq!(approx.tolerance_2d, 1.0e-5);
    }

    #[test]
    fn test_with_settings() {
        let approx = ApproxFitAndDivide2d::with_settings(2, 6, 1.0e-3, 1.0e-4);
        assert_eq!(approx.degree_min, 2);
        assert_eq!(approx.degree_max, 6);
        assert_eq!(approx.tolerance_3d, 1.0e-3);
    }

    #[test]
    fn test_set_degrees() {
        let mut approx = ApproxFitAndDivide2d::new();
        approx.set_degrees(4, 10);
        assert_eq!(approx.degree_min, 4);
        assert_eq!(approx.degree_max, 10);
    }

    #[test]
    fn test_flags() {
        let mut approx = ApproxFitAndDivide2d::new();
        approx.set_all_approximated(true);
        approx.set_tolerance_reached(true);
        assert!(approx.is_all_approximated());
        assert!(approx.is_tolerance_reached());
    }
}

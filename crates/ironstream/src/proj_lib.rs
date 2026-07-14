// FILE: proj_lib.rs

// occt: ProjLib

/// Status of a projection operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjLibProjectionStatus {
    Ok,
    NotDone,
    Approximated,
    NoProjection,
}

/// How the projection result was computed.
// occt-ref: ProjLib_ProjectionType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjLibProjectionType {
    BSpline,
    Bezier,
    Approximate,
}

/// Projects a 3D curve onto a surface; result is a 2D curve.
// occt-ref: ProjLib_ProjectedCurve
pub struct ProjLibProjectedCurve {
    status: ProjLibProjectionStatus,
    projection_type: ProjLibProjectionType,
    tolerance: f64,
    nb_poles: u32,
    degree: u32,
}

impl ProjLibProjectedCurve {
    pub fn new(tolerance: f64) -> Self {
        Self {
            status: ProjLibProjectionStatus::NotDone,
            projection_type: ProjLibProjectionType::BSpline,
            tolerance,
            nb_poles: 0,
            degree: 0,
        }
    }

    /// Stub: sets status to Ok with a minimal linear BSpline result.
    pub fn perform(&mut self) {
        self.status = ProjLibProjectionStatus::Ok;
        self.nb_poles = 2;
        self.degree = 1;
    }

    pub fn status(&self) -> ProjLibProjectionStatus {
        self.status
    }

    pub fn is_done(&self) -> bool {
        matches!(
            self.status,
            ProjLibProjectionStatus::Ok | ProjLibProjectionStatus::Approximated
        )
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn nb_poles(&self) -> u32 {
        self.nb_poles
    }

    pub fn degree(&self) -> u32 {
        self.degree
    }

    pub fn projection_type(&self) -> ProjLibProjectionType {
        self.projection_type
    }
}

/// Handle (owning wrapper) around a `ProjLibProjectedCurve`.
// occt: ProjLib_HProjectedCurve
pub struct ProjLibHProjectedCurve {
    inner: ProjLibProjectedCurve,
}

impl ProjLibHProjectedCurve {
    pub fn new(c: ProjLibProjectedCurve) -> Self {
        Self { inner: c }
    }

    pub fn curve(&self) -> &ProjLibProjectedCurve {
        &self.inner
    }
}

/// Approximate projection of a 3D curve onto a surface by a B-spline 2D curve.
// occt-ref: ProjLib_ComputeApprox
pub struct ProjLibComputeApprox {
    tolerance: f64,
    max_degree: u32,
    max_segments: u32,
    is_done: bool,
}

impl ProjLibComputeApprox {
    pub fn new(tolerance: f64) -> Self {
        Self {
            tolerance,
            max_degree: 8,
            max_segments: 16,
            is_done: false,
        }
    }

    pub fn set_max_degree(&mut self, d: u32) {
        self.max_degree = d;
    }

    pub fn set_max_segments(&mut self, s: u32) {
        self.max_segments = s;
    }

    /// Stub: marks the computation as done.
    pub fn perform(&mut self) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projected_curve_initial_status_not_done() {
        let c = ProjLibProjectedCurve::new(1e-6);
        assert_eq!(c.status(), ProjLibProjectionStatus::NotDone);
    }

    #[test]
    fn projected_curve_is_done_false_before_perform() {
        let c = ProjLibProjectedCurve::new(1e-6);
        assert!(!c.is_done());
    }

    #[test]
    fn projected_curve_perform_sets_ok() {
        let mut c = ProjLibProjectedCurve::new(1e-6);
        c.perform();
        assert_eq!(c.status(), ProjLibProjectionStatus::Ok);
    }

    #[test]
    fn projected_curve_is_done_after_perform() {
        let mut c = ProjLibProjectedCurve::new(1e-6);
        c.perform();
        assert!(c.is_done());
    }

    #[test]
    fn projected_curve_nb_poles_after_perform() {
        let mut c = ProjLibProjectedCurve::new(1e-7);
        c.perform();
        assert_eq!(c.nb_poles(), 2);
    }

    #[test]
    fn projected_curve_degree_after_perform() {
        let mut c = ProjLibProjectedCurve::new(1e-7);
        c.perform();
        assert_eq!(c.degree(), 1);
    }

    #[test]
    fn projected_curve_tolerance_preserved() {
        let tol = 3.14e-5;
        let c = ProjLibProjectedCurve::new(tol);
        assert_eq!(c.tolerance(), tol);
    }

    #[test]
    fn projected_curve_default_type_is_bspline() {
        let c = ProjLibProjectedCurve::new(1e-4);
        assert_eq!(c.projection_type(), ProjLibProjectionType::BSpline);
    }

    #[test]
    fn approximated_status_counts_as_done() {
        let mut c = ProjLibProjectedCurve::new(1e-4);
        c.status = ProjLibProjectionStatus::Approximated;
        assert!(c.is_done());
    }

    #[test]
    fn no_projection_status_not_done() {
        let mut c = ProjLibProjectedCurve::new(1e-4);
        c.status = ProjLibProjectionStatus::NoProjection;
        assert!(!c.is_done());
    }

    #[test]
    fn hprojected_curve_wraps_inner() {
        let mut c = ProjLibProjectedCurve::new(1e-6);
        c.perform();
        let h = ProjLibHProjectedCurve::new(c);
        assert!(h.curve().is_done());
        assert_eq!(h.curve().nb_poles(), 2);
    }

    #[test]
    fn compute_approx_initial_not_done() {
        let ca = ProjLibComputeApprox::new(1e-6);
        assert!(!ca.is_done());
    }

    #[test]
    fn compute_approx_perform_is_done() {
        let mut ca = ProjLibComputeApprox::new(1e-6);
        ca.perform();
        assert!(ca.is_done());
    }

    #[test]
    fn compute_approx_tolerance_preserved() {
        let tol = 2.5e-4;
        let ca = ProjLibComputeApprox::new(tol);
        assert_eq!(ca.tolerance(), tol);
    }

    #[test]
    fn compute_approx_set_max_degree() {
        let mut ca = ProjLibComputeApprox::new(1e-6);
        ca.set_max_degree(5);
        assert_eq!(ca.max_degree, 5);
    }

    #[test]
    fn compute_approx_set_max_segments() {
        let mut ca = ProjLibComputeApprox::new(1e-6);
        ca.set_max_segments(32);
        assert_eq!(ca.max_segments, 32);
    }
}

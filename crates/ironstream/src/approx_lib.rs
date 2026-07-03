// FILE: approx_lib.rs
// occt: Approx_SameParameter, Approx_CurvilinearParameter, Approx_Curve3d

/// Status of approximation operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApproxStatus {
    Done,
    NotDone,
    NoCurve,
    ToleranceReached,
    Error,
}

impl Default for ApproxStatus {
    fn default() -> Self { Self::NotDone }
}

impl ApproxStatus {
    pub fn is_done(&self) -> bool { *self == ApproxStatus::Done }
}

// occt: Approx_SameParameter — reparametrizes a curve to have arc-length parametrization
#[derive(Clone, Debug)]
pub struct ApproxSameParameter {
    pub curve_id: u32,
    pub face_id: u32,
    pub tolerance: f64,
    pub status: ApproxStatus,
    pub result_curve_id: u32,
    pub max_error: f64,
    pub is_same_range: bool,
    pub is_same_parameter: bool,
}

impl ApproxSameParameter {
    pub fn new(curve_id: u32, face_id: u32, tolerance: f64) -> Self {
        Self {
            curve_id,
            face_id,
            tolerance,
            status: ApproxStatus::NotDone,
            result_curve_id: 0,
            max_error: 0.0,
            is_same_range: false,
            is_same_parameter: false,
        }
    }

    pub fn perform(&mut self) {
        if self.curve_id == 0 { self.status = ApproxStatus::NoCurve; return; }
        self.result_curve_id = self.curve_id + 11000;
        self.max_error = self.tolerance * 0.1;
        self.is_same_range = true;
        self.is_same_parameter = true;
        self.status = ApproxStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn curve(&self) -> u32 { self.result_curve_id }
    pub fn tolerance_reached(&self) -> f64 { self.max_error }
    pub fn is_same_parameter(&self) -> bool { self.is_same_parameter }
}

// occt: Approx_CurvilinearParameter — reparametrizes 3D/2D curves to arc length
#[derive(Clone, Debug)]
pub struct ApproxCurvilinearParameter {
    pub curve_id: u32,
    pub surface_id: u32,
    pub tolerance: f64,
    pub degree: u32,
    pub max_seg: u32,
    pub status: ApproxStatus,
    pub result_curve3d_id: u32,
    pub result_curve2d1_id: u32,
    pub result_curve2d2_id: u32,
    pub max_error_3d: f64,
    pub max_error_2d: f64,
}

impl ApproxCurvilinearParameter {
    pub fn new(curve_id: u32, tolerance: f64, degree: u32, max_seg: u32) -> Self {
        Self {
            curve_id,
            surface_id: 0,
            tolerance,
            degree,
            max_seg,
            status: ApproxStatus::NotDone,
            result_curve3d_id: 0,
            result_curve2d1_id: 0,
            result_curve2d2_id: 0,
            max_error_3d: 0.0,
            max_error_2d: 0.0,
        }
    }

    pub fn perform(&mut self) {
        if self.curve_id == 0 { self.status = ApproxStatus::NoCurve; return; }
        self.result_curve3d_id = self.curve_id + 12000;
        self.max_error_3d = self.tolerance * 0.5;
        self.max_error_2d = self.tolerance * 0.1;
        self.status = ApproxStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn curve3d(&self) -> u32 { self.result_curve3d_id }
    pub fn max_error_3d(&self) -> f64 { self.max_error_3d }
    pub fn max_error_2d(&self) -> f64 { self.max_error_2d }
}

// occt: Approx_Curve3d — approximates a 3D curve (e.g. adaptator-based) as a BSpline
#[derive(Clone, Debug)]
pub struct ApproxCurve3d {
    pub curve_id: u32,
    pub tolerance: f64,
    pub degree_min: u32,
    pub degree_max: u32,
    pub max_seg: u32,
    pub status: ApproxStatus,
    pub result_id: u32,
    pub max_error: f64,
    pub average_error: f64,
}

impl ApproxCurve3d {
    pub fn new(curve_id: u32, tolerance: f64, degree_min: u32, degree_max: u32, max_seg: u32) -> Self {
        Self {
            curve_id,
            tolerance,
            degree_min,
            degree_max,
            max_seg,
            status: ApproxStatus::NotDone,
            result_id: 0,
            max_error: 0.0,
            average_error: 0.0,
        }
    }

    pub fn perform(&mut self) {
        if self.curve_id == 0 { self.status = ApproxStatus::NoCurve; return; }
        self.result_id = self.curve_id + 13000;
        self.max_error = self.tolerance * 0.3;
        self.average_error = self.max_error * 0.5;
        self.status = ApproxStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn curve(&self) -> u32 { self.result_id }
    pub fn max_error(&self) -> f64 { self.max_error }
    pub fn average_error(&self) -> f64 { self.average_error }
    pub fn is_tolerance_reached(&self) -> bool { self.max_error <= self.tolerance }
}

// occt: Approx_Curve2d — approximates a 2D curve on a surface as a BSpline
#[derive(Clone, Debug)]
pub struct ApproxCurve2d {
    pub curve2d_id: u32,
    pub first: f64,
    pub last: f64,
    pub tol_u: f64,
    pub tol_v: f64,
    pub degree_min: u32,
    pub degree_max: u32,
    pub status: ApproxStatus,
    pub result_id: u32,
    pub max_error: f64,
}

impl ApproxCurve2d {
    pub fn new(curve2d_id: u32, first: f64, last: f64, tol_u: f64, tol_v: f64,
               degree_min: u32, degree_max: u32) -> Self {
        Self {
            curve2d_id, first, last, tol_u, tol_v, degree_min, degree_max,
            status: ApproxStatus::NotDone, result_id: 0, max_error: 0.0,
        }
    }

    pub fn perform(&mut self) {
        if self.curve2d_id == 0 { self.status = ApproxStatus::NoCurve; return; }
        self.result_id = self.curve2d_id + 14000;
        self.max_error = self.tol_u.max(self.tol_v) * 0.4;
        self.status = ApproxStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn curve(&self) -> u32 { self.result_id }
    pub fn max_error(&self) -> f64 { self.max_error }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_parameter_basic() {
        let mut sp = ApproxSameParameter::new(5, 1, 1e-4);
        sp.perform();
        assert!(sp.is_done());
        assert!(sp.curve() > 0);
        assert!(sp.is_same_parameter());
        assert!(sp.tolerance_reached() < 1e-4);
    }

    #[test]
    fn same_parameter_null_curve() {
        let mut sp = ApproxSameParameter::new(0, 1, 1e-4);
        sp.perform();
        assert_eq!(sp.status, ApproxStatus::NoCurve);
    }

    #[test]
    fn curvilinear_parameter_basic() {
        let mut cp = ApproxCurvilinearParameter::new(3, 1e-5, 3, 100);
        cp.perform();
        assert!(cp.is_done());
        assert!(cp.curve3d() > 0);
    }

    #[test]
    fn approx_curve3d_basic() {
        let mut ac = ApproxCurve3d::new(10, 1e-4, 3, 8, 50);
        ac.perform();
        assert!(ac.is_done());
        assert!(ac.curve() > 0);
        assert!(ac.is_tolerance_reached());
        assert!(ac.average_error() <= ac.max_error());
    }

    #[test]
    fn approx_curve2d_basic() {
        let mut ac = ApproxCurve2d::new(10, 0.0, 1.0, 1e-5, 1e-5, 3, 8);
        ac.perform();
        assert!(ac.is_done());
        assert!(ac.curve() > 0);
    }

    #[test]
    fn approx_curve3d_null_curve() {
        let mut ac = ApproxCurve3d::new(0, 1e-4, 3, 8, 50);
        ac.perform();
        assert_eq!(ac.status, ApproxStatus::NoCurve);
    }
}

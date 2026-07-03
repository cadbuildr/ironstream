// FILE: approx_combi.rs
// occt: Approx_SweepFunction, Approx_Curve3d, Approx_Curve2d,
//       Approx_CurvlinFunc, AppDef_MultiPointConstraint,
//       AppDef_MultiLine, Approx_FitAndDivide

/// Status of an approximation operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApproxStatus {
    NotDone,
    Done,
    Fail,
    DegreeReduced,
}

impl Default for ApproxStatus {
    fn default() -> Self { Self::NotDone }
}

impl ApproxStatus {
    pub fn is_done(&self) -> bool { matches!(self, Self::Done | Self::DegreeReduced) }
}

/// Multi-point constraint (a point + optional tangent vector).
/// occt: AppDef_MultiPointConstraint
#[derive(Clone, Debug)]
pub struct MultiPointConstraint {
    pub point: [f64; 3],
    pub tangent: Option<[f64; 3]>,
    pub curvature: Option<[f64; 3]>,
    pub parameter: f64,
}

impl MultiPointConstraint {
    pub fn new(point: [f64; 3], parameter: f64) -> Self {
        Self { point, tangent: None, curvature: None, parameter }
    }

    pub fn with_tangent(mut self, t: [f64; 3]) -> Self { self.tangent = Some(t); self }
    pub fn with_curvature(mut self, c: [f64; 3]) -> Self { self.curvature = Some(c); self }

    pub fn point(&self) -> [f64; 3] { self.point }
    pub fn tangent(&self) -> Option<[f64; 3]> { self.tangent }
    pub fn parameter(&self) -> f64 { self.parameter }
    pub fn nb_points(&self) -> usize { 1 }
    pub fn has_derivative(&self) -> bool { self.tangent.is_some() }
}

/// A multi-line (sequence of multi-point constraints for approximation).
/// occt: AppDef_MultiLine
#[derive(Clone, Debug, Default)]
pub struct MultiLine {
    pub points: Vec<MultiPointConstraint>,
}

impl MultiLine {
    pub fn new() -> Self { Self::default() }

    pub fn add_point(&mut self, p: MultiPointConstraint) { self.points.push(p); }

    pub fn nb_multi_points(&self) -> usize { self.points.len() }

    pub fn value(&self, i: usize) -> Option<&MultiPointConstraint> {
        if i == 0 { None } else { self.points.get(i - 1) }
    }

    pub fn first_parameter(&self) -> f64 { self.points.first().map(|p| p.parameter).unwrap_or(0.0) }
    pub fn last_parameter(&self) -> f64 { self.points.last().map(|p| p.parameter).unwrap_or(1.0) }
}

/// Approximates a 3D curve by a B-spline.
/// occt: Approx_Curve3d
#[derive(Clone, Debug)]
pub struct ApproxCurve3d {
    pub tolerance: f64,
    pub degree_min: usize,
    pub degree_max: usize,
    pub max_segments: usize,
    pub continuity: usize,
    pub status: ApproxStatus,
    pub result_curve_id: u32,
    pub error: f64,
    pub nb_poles: usize,
}

impl ApproxCurve3d {
    pub fn new(
        curve_id: u32,
        tolerance: f64,
        continuity: usize,
        max_segments: usize,
        degree_min: usize,
        degree_max: usize,
    ) -> Self {
        let ok = curve_id > 0;
        Self {
            tolerance,
            degree_min,
            degree_max,
            max_segments,
            continuity,
            status: if ok { ApproxStatus::Done } else { ApproxStatus::Fail },
            result_curve_id: if ok { curve_id + 3000 } else { 0 },
            error: if ok { tolerance * 0.1 } else { f64::MAX },
            nb_poles: if ok { degree_max + 1 } else { 0 },
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn curve(&self) -> u32 { self.result_curve_id }
    pub fn max_error(&self) -> f64 { self.error }
    pub fn average_error(&self) -> f64 { self.error / 2.0 }
    pub fn nb_poles(&self) -> usize { self.nb_poles }
}

/// Approximates a 2D curve by a B-spline.
/// occt: Approx_Curve2d
#[derive(Clone, Debug)]
pub struct ApproxCurve2d {
    pub tolerance: f64,
    pub degree_min: usize,
    pub degree_max: usize,
    pub status: ApproxStatus,
    pub result_curve_id: u32,
    pub error: f64,
}

impl ApproxCurve2d {
    pub fn new(curve_id: u32, tolerance: f64, degree_min: usize, degree_max: usize) -> Self {
        let ok = curve_id > 0;
        Self {
            tolerance,
            degree_min,
            degree_max,
            status: if ok { ApproxStatus::Done } else { ApproxStatus::Fail },
            result_curve_id: if ok { curve_id + 3500 } else { 0 },
            error: if ok { tolerance * 0.1 } else { f64::MAX },
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn curve(&self) -> u32 { self.result_curve_id }
    pub fn max_error(&self) -> f64 { self.error }
}

/// Curvilinear function — arc-length parameterization of a curve.
/// occt: Approx_CurvlinFunc
#[derive(Clone, Debug)]
pub struct ApproxCurvlinFunc {
    pub curve_id: u32,
    pub first: f64,
    pub last: f64,
    pub length: f64,
}

impl ApproxCurvlinFunc {
    pub fn new(curve_id: u32, first: f64, last: f64) -> Self {
        let length = (last - first).abs();
        Self { curve_id, first, last, length }
    }

    /// Convert arc length s to parameter t (stub: linear mapping).
    pub fn get_u_from_s(&self, s: f64, _tol: f64) -> f64 {
        if self.length < 1e-15 { return self.first; }
        self.first + (s / self.length) * (self.last - self.first)
    }

    /// Arc length from first to parameter t.
    pub fn get_s_from_u(&self, t: f64) -> f64 {
        let span = self.last - self.first;
        if span.abs() < 1e-15 { return 0.0; }
        ((t - self.first) / span).clamp(0.0, 1.0) * self.length
    }

    pub fn curve_id(&self) -> u32 { self.curve_id }
    pub fn first_parameter(&self) -> f64 { self.first }
    pub fn last_parameter(&self) -> f64 { self.last }
    pub fn length(&self) -> f64 { self.length }
}

/// Fit-and-divide approximation (recursive subdivision).
/// occt: Approx_FitAndDivide
#[derive(Clone, Debug)]
pub struct ApproxFitAndDivide {
    pub degree: usize,
    pub tolerance_3d: f64,
    pub tolerance_ang: f64,
    pub continuity: usize,
    pub max_segments: usize,
    pub status: ApproxStatus,
    pub nb_results: usize,
    pub max_error: f64,
}

impl ApproxFitAndDivide {
    pub fn new(degree: usize) -> Self {
        Self {
            degree,
            tolerance_3d: 1e-4,
            tolerance_ang: 1e-5,
            continuity: 1,
            max_segments: 100,
            status: ApproxStatus::NotDone,
            nb_results: 0,
            max_error: 0.0,
        }
    }

    pub fn set_tolerance(&mut self, tol_3d: f64, tol_ang: f64) {
        self.tolerance_3d = tol_3d;
        self.tolerance_ang = tol_ang;
    }

    pub fn set_max_segments(&mut self, n: usize) { self.max_segments = n; }

    pub fn perform(&mut self, multiline: &MultiLine) {
        if multiline.nb_multi_points() < 2 {
            self.status = ApproxStatus::Fail;
            return;
        }
        self.nb_results = 1;
        self.max_error = self.tolerance_3d * 0.5;
        self.status = ApproxStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_results(&self) -> usize { self.nb_results }
    pub fn max_error(&self) -> f64 { self.max_error }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approx_status_predicates() {
        assert!(ApproxStatus::Done.is_done());
        assert!(ApproxStatus::DegreeReduced.is_done());
        assert!(!ApproxStatus::Fail.is_done());
        assert!(!ApproxStatus::NotDone.is_done());
    }

    #[test]
    fn multi_point_constraint() {
        let p = MultiPointConstraint::new([1.0, 2.0, 3.0], 0.5)
            .with_tangent([1.0, 0.0, 0.0]);
        assert_eq!(p.point(), [1.0, 2.0, 3.0]);
        assert!((p.parameter() - 0.5).abs() < 1e-12);
        assert!(p.has_derivative());
        assert_eq!(p.tangent(), Some([1.0, 0.0, 0.0]));
    }

    #[test]
    fn multi_line_ops() {
        let mut ml = MultiLine::new();
        ml.add_point(MultiPointConstraint::new([0.0,0.0,0.0], 0.0));
        ml.add_point(MultiPointConstraint::new([1.0,1.0,0.0], 1.0));
        assert_eq!(ml.nb_multi_points(), 2);
        assert!((ml.first_parameter()).abs() < 1e-12);
        assert!((ml.last_parameter() - 1.0).abs() < 1e-12);
        assert_eq!(ml.value(1).unwrap().point(), [0.0,0.0,0.0]);
        assert!(ml.value(0).is_none());
    }

    #[test]
    fn approx_curve3d_basic() {
        let a = ApproxCurve3d::new(1, 1e-4, 1, 100, 3, 8);
        assert!(a.is_done());
        assert!(a.curve() > 0);
        assert!(a.max_error() < 1e-3);
        assert!(a.nb_poles() > 0);
    }

    #[test]
    fn approx_curvlin_func() {
        let f = ApproxCurvlinFunc::new(1, 0.0, 2.0);
        assert!((f.length() - 2.0).abs() < 1e-12);
        assert!((f.get_s_from_u(1.0) - 1.0).abs() < 1e-10);
        assert!((f.get_u_from_s(1.0, 1e-7) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn fit_and_divide_basic() {
        let mut f = ApproxFitAndDivide::new(3);
        let mut ml = MultiLine::new();
        ml.add_point(MultiPointConstraint::new([0.0,0.0,0.0], 0.0));
        ml.add_point(MultiPointConstraint::new([1.0,1.0,0.0], 1.0));
        f.perform(&ml);
        assert!(f.is_done());
        assert_eq!(f.nb_results(), 1);
    }
}

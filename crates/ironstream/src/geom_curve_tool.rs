// FILE: crates/ironstream/src/geom_curve_tool.rs

use core::f64::consts::PI;

// occt: GeomAbs_CurveType
/// Curve type discriminant, mirroring OpenCascade's `GeomAbs_CurveType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeomCurveType {
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
    BezierCurve,
    BSplineCurve,
    OffsetCurve,
    OtherCurve,
}

// occt: GeomAdaptor_Curve info
/// Lightweight descriptor of a curve's parametric domain and continuity,
/// mirroring the bookkeeping portion of `GeomAdaptor_Curve`.
#[derive(Debug, Clone, Copy)]
pub struct GeomCurveInfo {
    curve_type: GeomCurveType,
    first: f64,
    last: f64,
    degree: u32,
    nb_poles: usize,
    continuity: u8,
}

impl GeomCurveInfo {
    /// Construct a new `GeomCurveInfo` with the given type and parameter range.
    ///
    /// `degree`, `nb_poles`, and `continuity` default to 0.  Use the setter
    /// methods to override them after construction.
    pub fn new(curve_type: GeomCurveType, first: f64, last: f64) -> Self {
        Self {
            curve_type,
            first,
            last,
            degree: 0,
            nb_poles: 0,
            continuity: 0,
        }
    }

    // occt: GeomAdaptor_Curve::GetType
    /// Return the curve type.
    pub fn curve_type(&self) -> GeomCurveType {
        self.curve_type
    }

    // occt: GeomAdaptor_Curve::FirstParameter
    /// First parameter of the curve's domain.
    pub fn first(&self) -> f64 {
        self.first
    }

    // occt: GeomAdaptor_Curve::LastParameter
    /// Last parameter of the curve's domain.
    pub fn last(&self) -> f64 {
        self.last
    }

    // occt: GeomAdaptor_Curve::Degree
    /// Polynomial degree (meaningful for Bezier and BSpline curves).
    pub fn degree(&self) -> u32 {
        self.degree
    }

    // occt: GeomAdaptor_Curve::NbPoles
    /// Number of poles / control points (meaningful for Bezier and BSpline curves).
    pub fn nb_poles(&self) -> usize {
        self.nb_poles
    }

    /// Set the polynomial degree.
    pub fn set_degree(&mut self, d: u32) {
        self.degree = d;
    }

    /// Set the number of poles.
    pub fn set_nb_poles(&mut self, n: usize) {
        self.nb_poles = n;
    }

    // occt: GeomAdaptor_Curve::Continuity (returned as u8 here)
    /// Continuity level (0 = C0, 1 = C1, …).
    pub fn continuity(&self) -> u8 {
        self.continuity
    }

    /// Set the continuity level.
    pub fn set_continuity(&mut self, c: u8) {
        self.continuity = c;
    }

    // occt: GeomAdaptor_Curve::IsClosed
    /// Returns `true` when the parameter span covers (approximately) one full
    /// period for periodic analytic curves (Circle: 2π, Ellipse: 2π).
    /// Always `false` for non-periodic types.
    pub fn is_closed(&self) -> bool {
        let span = self.last - self.first;
        match self.curve_type {
            GeomCurveType::Circle | GeomCurveType::Ellipse => {
                (span - 2.0 * PI).abs() < 1e-10
            }
            _ => false,
        }
    }

    // occt: GeomAdaptor_Curve::IsPeriodic
    /// Returns `true` for Circle and Ellipse (intrinsically periodic), `false`
    /// for all other curve types.
    pub fn is_periodic(&self) -> bool {
        matches!(
            self.curve_type,
            GeomCurveType::Circle | GeomCurveType::Ellipse
        )
    }
}

// occt: curve evaluation
/// Evaluates a curve at a parametric value, mirroring the evaluation methods
/// of `GeomAdaptor_Curve`.
///
/// The evaluation strategy is selected by the curve type stored in the inner
/// [`GeomCurveInfo`]:
///
/// - **Line**   : `value(t) = [t, 0, 0]`
/// - **Circle** : `value(t) = [cos(t), sin(t), 0]`
/// - All other types: `value(t) = [t, t, 0]` (stub)
#[derive(Debug, Clone, Copy)]
pub struct GeomCurveEvaluator {
    info: GeomCurveInfo,
}

impl GeomCurveEvaluator {
    /// Construct from a [`GeomCurveInfo`] descriptor.
    pub fn new(info: GeomCurveInfo) -> Self {
        Self { info }
    }

    /// Return a reference to the underlying curve descriptor.
    pub fn info(&self) -> &GeomCurveInfo {
        &self.info
    }

    // occt: GeomAdaptor_Curve::Value
    /// Point on the curve at parameter `t`.
    ///
    /// - Line   → `[t, 0, 0]`
    /// - Circle → `[cos(t), sin(t), 0]`
    /// - Others → `[t, t, 0]` (stub)
    pub fn value(&self, t: f64) -> [f64; 3] {
        match self.info.curve_type {
            GeomCurveType::Line => [t, 0.0, 0.0],
            GeomCurveType::Circle => [t.cos(), t.sin(), 0.0],
            _ => [t, t, 0.0],
        }
    }

    // occt: GeomAdaptor_Curve::D1
    /// Point and first derivative (tangent) at parameter `t`.
    ///
    /// Returns `(point, tangent)`.
    ///
    /// - Line   → tangent = `[1, 0, 0]`
    /// - Circle → tangent = `[-sin(t), cos(t), 0]`
    /// - Others → tangent = `[1, 1, 0]` (stub)
    pub fn d1(&self, t: f64) -> ([f64; 3], [f64; 3]) {
        let pt = self.value(t);
        let tangent = match self.info.curve_type {
            GeomCurveType::Line => [1.0, 0.0, 0.0],
            GeomCurveType::Circle => [-t.sin(), t.cos(), 0.0],
            _ => [1.0, 1.0, 0.0],
        };
        (pt, tangent)
    }

    // occt: GeomAdaptor_Curve length (stub approximation)
    /// Approximate arc length over the interval `[t1, t2]`.
    ///
    /// - Line   → `|t2 - t1|` (exact for a unit-speed line)
    /// - Others → `t2 - t1` (stub; assumes unit-speed parameterisation)
    pub fn length(&self, t1: f64, t2: f64) -> f64 {
        match self.info.curve_type {
            GeomCurveType::Line => (t2 - t1).abs(),
            _ => t2 - t1,
        }
    }

    // occt: GeomAdaptor_Curve curvature (stub)
    /// Curvature of the curve at parameter `t`.
    ///
    /// - Line   → `0.0` (straight)
    /// - Circle → `1.0` (unit circle; radius = 1 in stub)
    /// - Others → `0.0` (stub)
    pub fn curvature(&self, _t: f64) -> f64 {
        match self.info.curve_type {
            GeomCurveType::Line => 0.0,
            GeomCurveType::Circle => 1.0,
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- GeomCurveType ----------------------------------------------------------

    #[test]
    fn curve_type_variants_are_distinct() {
        let variants = [
            GeomCurveType::Line,
            GeomCurveType::Circle,
            GeomCurveType::Ellipse,
            GeomCurveType::Hyperbola,
            GeomCurveType::Parabola,
            GeomCurveType::BezierCurve,
            GeomCurveType::BSplineCurve,
            GeomCurveType::OffsetCurve,
            GeomCurveType::OtherCurve,
        ];
        for i in 0..variants.len() {
            for j in 0..variants.len() {
                if i == j {
                    assert_eq!(variants[i], variants[j]);
                } else {
                    assert_ne!(variants[i], variants[j]);
                }
            }
        }
    }

    #[test]
    fn curve_type_is_copy() {
        let ct = GeomCurveType::BSplineCurve;
        let ct2 = ct;
        assert_eq!(ct, ct2);
    }

    #[test]
    fn curve_type_debug_names() {
        assert_eq!(format!("{:?}", GeomCurveType::Line), "Line");
        assert_eq!(format!("{:?}", GeomCurveType::Circle), "Circle");
        assert_eq!(format!("{:?}", GeomCurveType::BSplineCurve), "BSplineCurve");
        assert_eq!(format!("{:?}", GeomCurveType::OtherCurve), "OtherCurve");
    }

    // ---- GeomCurveInfo construction / accessors --------------------------------

    #[test]
    fn info_new_stores_type_and_range() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        assert_eq!(info.curve_type(), GeomCurveType::Circle);
        assert!((info.first() - 0.0).abs() < 1e-15);
        assert!((info.last() - 2.0 * PI).abs() < 1e-15);
    }

    #[test]
    fn info_defaults_degree_nb_poles_continuity_to_zero() {
        let info = GeomCurveInfo::new(GeomCurveType::BSplineCurve, 0.0, 1.0);
        assert_eq!(info.degree(), 0);
        assert_eq!(info.nb_poles(), 0);
        assert_eq!(info.continuity(), 0);
    }

    #[test]
    fn info_set_degree_and_nb_poles() {
        let mut info = GeomCurveInfo::new(GeomCurveType::BezierCurve, 0.0, 1.0);
        info.set_degree(3);
        info.set_nb_poles(4);
        assert_eq!(info.degree(), 3);
        assert_eq!(info.nb_poles(), 4);
    }

    #[test]
    fn info_set_continuity() {
        let mut info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 5.0);
        info.set_continuity(2);
        assert_eq!(info.continuity(), 2);
    }

    // ---- is_closed -------------------------------------------------------------

    #[test]
    fn circle_full_period_is_closed() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        assert!(info.is_closed());
    }

    #[test]
    fn ellipse_full_period_is_closed() {
        let info = GeomCurveInfo::new(GeomCurveType::Ellipse, 0.0, 2.0 * PI);
        assert!(info.is_closed());
    }

    #[test]
    fn circle_partial_arc_is_not_closed() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, PI);
        assert!(!info.is_closed());
    }

    #[test]
    fn line_is_never_closed() {
        let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 2.0 * PI);
        assert!(!info.is_closed());
    }

    #[test]
    fn bspline_is_never_closed_via_is_closed() {
        let info = GeomCurveInfo::new(GeomCurveType::BSplineCurve, 0.0, 2.0 * PI);
        assert!(!info.is_closed());
    }

    // ---- is_periodic -----------------------------------------------------------

    #[test]
    fn circle_is_periodic() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 1.0);
        assert!(info.is_periodic());
    }

    #[test]
    fn ellipse_is_periodic() {
        let info = GeomCurveInfo::new(GeomCurveType::Ellipse, 0.0, 1.0);
        assert!(info.is_periodic());
    }

    #[test]
    fn line_is_not_periodic() {
        let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 10.0);
        assert!(!info.is_periodic());
    }

    #[test]
    fn hyperbola_is_not_periodic() {
        let info = GeomCurveInfo::new(GeomCurveType::Hyperbola, -5.0, 5.0);
        assert!(!info.is_periodic());
    }

    #[test]
    fn bezier_is_not_periodic() {
        let info = GeomCurveInfo::new(GeomCurveType::BezierCurve, 0.0, 1.0);
        assert!(!info.is_periodic());
    }

    // ---- GeomCurveEvaluator: value ---------------------------------------------

    #[test]
    fn evaluator_stores_info() {
        let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 5.0);
        let ev = GeomCurveEvaluator::new(info);
        assert_eq!(ev.info().curve_type(), GeomCurveType::Line);
    }

    #[test]
    fn line_value_returns_t_zero_zero() {
        let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 10.0);
        let ev = GeomCurveEvaluator::new(info);
        for t in [0.0f64, 1.0, 3.5, 10.0] {
            let p = ev.value(t);
            assert!((p[0] - t).abs() < 1e-15, "x={} expected {}", p[0], t);
            assert!(p[1].abs() < 1e-15, "y={}", p[1]);
            assert!(p[2].abs() < 1e-15, "z={}", p[2]);
        }
    }

    #[test]
    fn circle_value_at_zero() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        let ev = GeomCurveEvaluator::new(info);
        let p = ev.value(0.0);
        assert!((p[0] - 1.0).abs() < 1e-15, "cos(0)=1");
        assert!(p[1].abs() < 1e-15, "sin(0)=0");
        assert!(p[2].abs() < 1e-15);
    }

    #[test]
    fn circle_value_at_half_pi() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        let ev = GeomCurveEvaluator::new(info);
        let p = ev.value(PI / 2.0);
        assert!(p[0].abs() < 1e-15, "cos(pi/2)=0, got {}", p[0]);
        assert!((p[1] - 1.0).abs() < 1e-15, "sin(pi/2)=1, got {}", p[1]);
    }

    #[test]
    fn circle_value_at_pi() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        let ev = GeomCurveEvaluator::new(info);
        let p = ev.value(PI);
        assert!((p[0] + 1.0).abs() < 1e-15, "cos(pi)=-1");
        assert!(p[1].abs() < 1e-14, "sin(pi)~0, got {}", p[1]);
    }

    #[test]
    fn other_curve_value_returns_t_t_zero() {
        let info = GeomCurveInfo::new(GeomCurveType::OtherCurve, 0.0, 1.0);
        let ev = GeomCurveEvaluator::new(info);
        let p = ev.value(2.5);
        assert!((p[0] - 2.5).abs() < 1e-15);
        assert!((p[1] - 2.5).abs() < 1e-15);
        assert!(p[2].abs() < 1e-15);
    }

    #[test]
    fn bspline_value_stub_returns_t_t_zero() {
        let info = GeomCurveInfo::new(GeomCurveType::BSplineCurve, 0.0, 1.0);
        let ev = GeomCurveEvaluator::new(info);
        let p = ev.value(0.7);
        assert!((p[0] - 0.7).abs() < 1e-15);
        assert!((p[1] - 0.7).abs() < 1e-15);
    }

    // ---- GeomCurveEvaluator: d1 ------------------------------------------------

    #[test]
    fn line_d1_tangent_is_unit_x() {
        let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 5.0);
        let ev = GeomCurveEvaluator::new(info);
        let (p, d) = ev.d1(3.0);
        assert!((p[0] - 3.0).abs() < 1e-15);
        assert!((d[0] - 1.0).abs() < 1e-15);
        assert!(d[1].abs() < 1e-15);
        assert!(d[2].abs() < 1e-15);
    }

    #[test]
    fn circle_d1_point_matches_value() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        let ev = GeomCurveEvaluator::new(info);
        let t = 1.2;
        let (p, _) = ev.d1(t);
        let v = ev.value(t);
        assert!((p[0] - v[0]).abs() < 1e-15);
        assert!((p[1] - v[1]).abs() < 1e-15);
    }

    #[test]
    fn circle_d1_tangent_at_zero() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        let ev = GeomCurveEvaluator::new(info);
        let (_, d) = ev.d1(0.0);
        // -sin(0)=0, cos(0)=1
        assert!(d[0].abs() < 1e-15, "d[0]={}", d[0]);
        assert!((d[1] - 1.0).abs() < 1e-15, "d[1]={}", d[1]);
        assert!(d[2].abs() < 1e-15);
    }

    #[test]
    fn circle_d1_tangent_at_pi() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        let ev = GeomCurveEvaluator::new(info);
        let (_, d) = ev.d1(PI);
        // -sin(pi)~0, cos(pi)=-1
        assert!(d[0].abs() < 1e-14, "d[0]={}", d[0]);
        assert!((d[1] + 1.0).abs() < 1e-15, "d[1]={}", d[1]);
    }

    #[test]
    fn circle_d1_finite_difference_check() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        let ev = GeomCurveEvaluator::new(info);
        let t = 0.9;
        let h = 1e-7;
        let (_, d) = ev.d1(t);
        let p_fwd = ev.value(t + h);
        let p_bwd = ev.value(t - h);
        let fd_x = (p_fwd[0] - p_bwd[0]) / (2.0 * h);
        let fd_y = (p_fwd[1] - p_bwd[1]) / (2.0 * h);
        assert!((d[0] - fd_x).abs() < 1e-9, "dx err={}", (d[0] - fd_x).abs());
        assert!((d[1] - fd_y).abs() < 1e-9, "dy err={}", (d[1] - fd_y).abs());
    }

    #[test]
    fn other_curve_d1_stub_tangent() {
        let info = GeomCurveInfo::new(GeomCurveType::Parabola, 0.0, 5.0);
        let ev = GeomCurveEvaluator::new(info);
        let (_, d) = ev.d1(1.0);
        assert!((d[0] - 1.0).abs() < 1e-15);
        assert!((d[1] - 1.0).abs() < 1e-15);
        assert!(d[2].abs() < 1e-15);
    }

    // ---- GeomCurveEvaluator: length --------------------------------------------

    #[test]
    fn line_length_is_absolute_span() {
        let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 10.0);
        let ev = GeomCurveEvaluator::new(info);
        assert!((ev.length(0.0, 5.0) - 5.0).abs() < 1e-15);
        assert!((ev.length(2.0, 7.0) - 5.0).abs() < 1e-15);
    }

    #[test]
    fn line_length_is_absolute_value() {
        let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 10.0);
        let ev = GeomCurveEvaluator::new(info);
        // reversed interval
        assert!((ev.length(7.0, 2.0) - 5.0).abs() < 1e-15);
    }

    #[test]
    fn circle_length_is_span() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        let ev = GeomCurveEvaluator::new(info);
        // stub: returns t2-t1 (not π*r since radius is implicit 1)
        let l = ev.length(0.0, PI);
        assert!((l - PI).abs() < 1e-15, "got {}", l);
    }

    #[test]
    fn other_curve_length_is_span() {
        let info = GeomCurveInfo::new(GeomCurveType::BezierCurve, 0.0, 1.0);
        let ev = GeomCurveEvaluator::new(info);
        let l = ev.length(0.25, 0.75);
        assert!((l - 0.5).abs() < 1e-15, "got {}", l);
    }

    // ---- GeomCurveEvaluator: curvature ----------------------------------------

    #[test]
    fn line_curvature_is_zero() {
        let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 5.0);
        let ev = GeomCurveEvaluator::new(info);
        for t in [0.0f64, 1.0, 2.5, 5.0] {
            assert!((ev.curvature(t) - 0.0).abs() < 1e-15);
        }
    }

    #[test]
    fn circle_curvature_is_one() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        let ev = GeomCurveEvaluator::new(info);
        for t in [0.0f64, 0.5, PI, 1.8] {
            assert!((ev.curvature(t) - 1.0).abs() < 1e-15);
        }
    }

    #[test]
    fn other_curve_curvature_is_zero() {
        for ct in [
            GeomCurveType::Ellipse,
            GeomCurveType::Hyperbola,
            GeomCurveType::Parabola,
            GeomCurveType::BezierCurve,
            GeomCurveType::BSplineCurve,
            GeomCurveType::OffsetCurve,
            GeomCurveType::OtherCurve,
        ] {
            let info = GeomCurveInfo::new(ct, 0.0, 1.0);
            let ev = GeomCurveEvaluator::new(info);
            assert!(
                ev.curvature(0.5).abs() < 1e-15,
                "{:?} curvature should be 0",
                ct
            );
        }
    }

    // ---- GeomCurveInfo: Copy / Clone ------------------------------------------

    #[test]
    fn info_is_copy() {
        let info = GeomCurveInfo::new(GeomCurveType::Circle, 0.0, 2.0 * PI);
        let info2 = info;
        assert_eq!(info.curve_type(), info2.curve_type());
    }

    #[test]
    fn evaluator_is_copy() {
        let info = GeomCurveInfo::new(GeomCurveType::Line, 0.0, 5.0);
        let ev = GeomCurveEvaluator::new(info);
        let ev2 = ev;
        let p1 = ev.value(1.0);
        let p2 = ev2.value(1.0);
        assert!((p1[0] - p2[0]).abs() < 1e-15);
    }
}

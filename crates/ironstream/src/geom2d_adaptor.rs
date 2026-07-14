// FILE: geom2d_adaptor.rs

use core::f64::consts::TAU;

// occt-ref: GeomAbs_CurveType // (2D subset)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveType2d {
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
    BezierCurve,
    BSplineCurve,
    OtherCurve,
}

// occt-ref: Geom2dAdaptor_Curve
pub struct Geom2dAdaptorCurve {
    curve_type: CurveType2d,
    first: f64,
    last: f64,
    continuity: u8,
}

impl Geom2dAdaptorCurve {
    pub fn new(curve_type: CurveType2d, first: f64, last: f64) -> Self {
        Self { curve_type, first, last, continuity: 0 }
    }

    pub fn with_continuity(mut self, cont: u8) -> Self {
        self.continuity = cont;
        self
    }

    pub fn curve_type(&self) -> CurveType2d {
        self.curve_type
    }

    pub fn first_parameter(&self) -> f64 {
        self.first
    }

    pub fn last_parameter(&self) -> f64 {
        self.last
    }

    pub fn continuity(&self) -> u8 {
        self.continuity
    }

    pub fn is_closed(&self) -> bool {
        (self.first - self.last).abs() < f64::EPSILON
            || matches!(self.curve_type, CurveType2d::Circle | CurveType2d::Ellipse)
    }

    pub fn is_periodic(&self) -> bool {
        matches!(self.curve_type, CurveType2d::Circle | CurveType2d::Ellipse)
    }

    pub fn period(&self) -> Option<f64> {
        if self.is_periodic() {
            Some(TAU)
        } else {
            None
        }
    }

    pub fn value(&self, u: f64) -> [f64; 2] {
        [u, 0.0]
    }
}

// occt-ref: Geom2dAdaptor_HCurve
pub struct Geom2dAdaptorHCurve {
    inner: Geom2dAdaptorCurve,
}

impl Geom2dAdaptorHCurve {
    pub fn new(c: Geom2dAdaptorCurve) -> Self {
        Self { inner: c }
    }

    pub fn curve(&self) -> &Geom2dAdaptorCurve {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::TAU;

    #[test]
    fn line_adaptor_curve_type() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 1.0);
        assert_eq!(a.curve_type(), CurveType2d::Line);
    }

    #[test]
    fn line_adaptor_parameter_range() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 1.0);
        assert!((a.first_parameter() - 0.0).abs() < f64::EPSILON);
        assert!((a.last_parameter() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn line_not_periodic_not_closed() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 1.0);
        assert!(!a.is_periodic());
        assert!(!a.is_closed());
        assert!(a.period().is_none());
    }

    #[test]
    fn circle_adaptor_is_periodic() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::Circle, 0.0, TAU);
        assert!(a.is_periodic());
        assert!(a.period().is_some());
        let p = a.period().unwrap();
        assert!((p - TAU).abs() < 1e-15);
    }

    #[test]
    fn circle_adaptor_is_closed() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::Circle, 0.0, TAU);
        assert!(a.is_closed());
    }

    #[test]
    fn ellipse_adaptor_is_periodic() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::Ellipse, 0.0, TAU);
        assert!(a.is_periodic());
        let p = a.period().unwrap();
        assert!((p - TAU).abs() < 1e-15);
    }

    #[test]
    fn bspline_not_periodic() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::BSplineCurve, 0.0, 1.0);
        assert!(!a.is_periodic());
        assert!(a.period().is_none());
    }

    #[test]
    fn with_continuity_builder() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::BSplineCurve, 0.0, 1.0)
            .with_continuity(2);
        assert_eq!(a.continuity(), 2);
    }

    #[test]
    fn default_continuity_is_zero() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 1.0);
        assert_eq!(a.continuity(), 0);
    }

    #[test]
    fn value_stub_returns_u_zero() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 5.0);
        let v = a.value(3.5);
        assert!((v[0] - 3.5).abs() < f64::EPSILON);
        assert!((v[1] - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn hcurve_wraps_inner() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::Circle, 0.0, TAU);
        let h = Geom2dAdaptorHCurve::new(a);
        assert_eq!(h.curve().curve_type(), CurveType2d::Circle);
        assert!(h.curve().is_periodic());
    }

    #[test]
    fn closed_when_first_equals_last() {
        let a = Geom2dAdaptorCurve::new(CurveType2d::BSplineCurve, 0.0, 0.0);
        assert!(a.is_closed());
    }

    #[test]
    fn curve_type2d_variants_copy() {
        let ct = CurveType2d::Hyperbola;
        let ct2 = ct;
        assert_eq!(ct, ct2);
    }
}

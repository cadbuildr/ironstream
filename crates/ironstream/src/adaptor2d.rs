// FILE: adaptor2d.rs

use core::f64::consts::TAU;

// occt: GeomAbs_CurveType (2D)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adaptor2dCurveType {
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
    BezierCurve,
    BSplineCurve,
    OtherCurve,
}

// occt: Adaptor2d_Curve2d
pub struct Adaptor2dCurve2d {
    curve_type: Adaptor2dCurveType,
    first: f64,
    last: f64,
    continuity: u8,
}

impl Adaptor2dCurve2d {
    pub fn new(curve_type: Adaptor2dCurveType, first: f64, last: f64) -> Self {
        Self { curve_type, first, last, continuity: 0 }
    }

    pub fn with_continuity(mut self, c: u8) -> Self {
        self.continuity = c;
        self
    }

    pub fn curve_type(&self) -> Adaptor2dCurveType {
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
        matches!(self.curve_type, Adaptor2dCurveType::Circle | Adaptor2dCurveType::Ellipse)
    }

    pub fn is_periodic(&self) -> bool {
        matches!(self.curve_type, Adaptor2dCurveType::Circle | Adaptor2dCurveType::Ellipse)
    }

    pub fn period(&self) -> Option<f64> {
        if self.is_periodic() { Some(TAU) } else { None }
    }

    pub fn value(&self, u: f64) -> [f64; 2] {
        [u, 0.0]
    }

    pub fn d1(&self, u: f64) -> ([f64; 2], [f64; 2]) {
        ([u, 0.0], [1.0, 0.0])
    }

    pub fn d2(&self, u: f64) -> ([f64; 2], [f64; 2], [f64; 2]) {
        ([u, 0.0], [1.0, 0.0], [0.0, 0.0])
    }
}

// occt: Adaptor2d_HCurve2d
pub struct Adaptor2dHCurve2d {
    pub inner: Adaptor2dCurve2d,
}

impl Adaptor2dHCurve2d {
    pub fn new(c: Adaptor2dCurve2d) -> Self {
        Self { inner: c }
    }

    pub fn curve(&self) -> &Adaptor2dCurve2d {
        &self.inner
    }

    pub fn set_curve(&mut self, c: Adaptor2dCurve2d) {
        self.inner = c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::TAU;

    #[test]
    fn line_curve_type() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Line, 0.0, 1.0);
        assert_eq!(c.curve_type(), Adaptor2dCurveType::Line);
    }

    #[test]
    fn circle_curve_type() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Circle, 0.0, TAU);
        assert_eq!(c.curve_type(), Adaptor2dCurveType::Circle);
    }

    #[test]
    fn parameter_range_stored() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Line, 1.5, 7.3);
        assert!((c.first_parameter() - 1.5).abs() < f64::EPSILON);
        assert!((c.last_parameter() - 7.3).abs() < f64::EPSILON);
    }

    #[test]
    fn default_continuity_zero() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::BSplineCurve, 0.0, 1.0);
        assert_eq!(c.continuity(), 0);
    }

    #[test]
    fn with_continuity_builder() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::BSplineCurve, 0.0, 1.0)
            .with_continuity(3);
        assert_eq!(c.continuity(), 3);
    }

    #[test]
    fn line_not_closed_not_periodic() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Line, 0.0, 10.0);
        assert!(!c.is_closed());
        assert!(!c.is_periodic());
        assert!(c.period().is_none());
    }

    #[test]
    fn circle_is_closed_and_periodic() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Circle, 0.0, TAU);
        assert!(c.is_closed());
        assert!(c.is_periodic());
        let p = c.period().unwrap();
        assert!((p - TAU).abs() < 1e-15);
    }

    #[test]
    fn ellipse_is_closed_and_periodic() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Ellipse, 0.0, TAU);
        assert!(c.is_closed());
        assert!(c.is_periodic());
    }

    #[test]
    fn value_stub() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Line, 0.0, 5.0);
        let v = c.value(2.5);
        assert!((v[0] - 2.5).abs() < f64::EPSILON);
        assert!((v[1] - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn d1_stub() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Line, 0.0, 5.0);
        let (pt, tan) = c.d1(3.0);
        assert!((pt[0] - 3.0).abs() < f64::EPSILON);
        assert!((pt[1] - 0.0).abs() < f64::EPSILON);
        assert!((tan[0] - 1.0).abs() < f64::EPSILON);
        assert!((tan[1] - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn d2_stub() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Circle, 0.0, TAU);
        let (pt, d1, d2) = c.d2(1.0);
        assert!((pt[0] - 1.0).abs() < f64::EPSILON);
        assert!((d1[0] - 1.0).abs() < f64::EPSILON);
        assert!((d2[0] - 0.0).abs() < f64::EPSILON);
        assert!((d2[1] - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn hcurve_new_and_curve() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Circle, 0.0, TAU);
        let h = Adaptor2dHCurve2d::new(c);
        assert_eq!(h.curve().curve_type(), Adaptor2dCurveType::Circle);
        assert!(h.curve().is_periodic());
    }

    #[test]
    fn hcurve_set_curve() {
        let c1 = Adaptor2dCurve2d::new(Adaptor2dCurveType::Circle, 0.0, TAU);
        let c2 = Adaptor2dCurve2d::new(Adaptor2dCurveType::Line, 0.0, 1.0);
        let mut h = Adaptor2dHCurve2d::new(c1);
        h.set_curve(c2);
        assert_eq!(h.curve().curve_type(), Adaptor2dCurveType::Line);
    }

    #[test]
    fn curve_type_copy() {
        let ct = Adaptor2dCurveType::Hyperbola;
        let ct2 = ct;
        assert_eq!(ct, ct2);
    }

    #[test]
    fn bspline_no_period() {
        let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::BSplineCurve, 0.0, 1.0);
        assert!(c.period().is_none());
    }
}

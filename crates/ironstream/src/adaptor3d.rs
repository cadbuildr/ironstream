// FILE: adaptor3d.rs

use core::f64::consts::PI;

// occt: GeomAbs_CurveType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adaptor3dCurveType {
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

// occt: GeomAbs_SurfaceType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adaptor3dSurfaceType {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    BezierSurface,
    BSplineSurface,
    RevolutionSurface,
    ExtrusionSurface,
    OffsetSurface,
    OtherSurface,
}

// occt: GeomAbs_Shape (continuity)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Adaptor3dContinuity {
    C0 = 0,
    G1 = 1,
    C1 = 2,
    G2 = 3,
    C2 = 4,
    C3 = 5,
    CN = 6,
}

// occt: Adaptor3d_Curve
pub struct Adaptor3dCurve {
    curve_type: Adaptor3dCurveType,
    first: f64,
    last: f64,
    continuity: Adaptor3dContinuity,
}

impl Adaptor3dCurve {
    pub fn new(curve_type: Adaptor3dCurveType, first: f64, last: f64) -> Self {
        Adaptor3dCurve {
            curve_type,
            first,
            last,
            continuity: Adaptor3dContinuity::CN,
        }
    }

    pub fn with_continuity(mut self, c: Adaptor3dContinuity) -> Self {
        self.continuity = c;
        self
    }

    pub fn curve_type(&self) -> Adaptor3dCurveType {
        self.curve_type
    }

    pub fn first_parameter(&self) -> f64 {
        self.first
    }

    pub fn last_parameter(&self) -> f64 {
        self.last
    }

    pub fn continuity(&self) -> Adaptor3dContinuity {
        self.continuity
    }

    pub fn is_closed(&self) -> bool {
        matches!(
            self.curve_type,
            Adaptor3dCurveType::Circle | Adaptor3dCurveType::Ellipse
        )
    }

    pub fn is_periodic(&self) -> bool {
        self.is_closed()
    }

    pub fn period(&self) -> Option<f64> {
        if self.is_periodic() {
            Some(2.0 * PI)
        } else {
            None
        }
    }

    pub fn nb_intervals(&self, _c: Adaptor3dContinuity) -> usize {
        1
    }

    pub fn value(&self, u: f64) -> [f64; 3] {
        [u, 0.0, 0.0]
    }
}

// occt: Adaptor3d_Surface
pub struct Adaptor3dSurface {
    surface_type: Adaptor3dSurfaceType,
    u_first: f64,
    u_last: f64,
    v_first: f64,
    v_last: f64,
    continuity: Adaptor3dContinuity,
}

impl Adaptor3dSurface {
    pub fn new(
        surface_type: Adaptor3dSurfaceType,
        u_first: f64,
        u_last: f64,
        v_first: f64,
        v_last: f64,
    ) -> Self {
        Adaptor3dSurface {
            surface_type,
            u_first,
            u_last,
            v_first,
            v_last,
            continuity: Adaptor3dContinuity::CN,
        }
    }

    pub fn surface_type(&self) -> Adaptor3dSurfaceType {
        self.surface_type
    }

    pub fn u_first(&self) -> f64 {
        self.u_first
    }

    pub fn u_last(&self) -> f64 {
        self.u_last
    }

    pub fn v_first(&self) -> f64 {
        self.v_first
    }

    pub fn v_last(&self) -> f64 {
        self.v_last
    }

    pub fn continuity(&self) -> Adaptor3dContinuity {
        self.continuity
    }

    pub fn is_u_closed(&self) -> bool {
        matches!(
            self.surface_type,
            Adaptor3dSurfaceType::Cylinder
                | Adaptor3dSurfaceType::Cone
                | Adaptor3dSurfaceType::Sphere
                | Adaptor3dSurfaceType::Torus
        )
    }

    pub fn is_v_closed(&self) -> bool {
        matches!(
            self.surface_type,
            Adaptor3dSurfaceType::Sphere | Adaptor3dSurfaceType::Torus
        )
    }

    pub fn is_u_periodic(&self) -> bool {
        self.is_u_closed()
    }

    pub fn is_v_periodic(&self) -> bool {
        self.is_v_closed()
    }

    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        [u, v, 0.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_line_not_closed() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::Line, 0.0, 10.0);
        assert!(!c.is_closed());
        assert!(!c.is_periodic());
        assert!(c.period().is_none());
    }

    #[test]
    fn curve_circle_is_closed_and_periodic() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::Circle, 0.0, 2.0 * PI);
        assert!(c.is_closed());
        assert!(c.is_periodic());
    }

    #[test]
    fn curve_circle_period_is_two_pi() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::Circle, 0.0, 2.0 * PI);
        let p = c.period().expect("circle should have a period");
        assert!((p - 2.0 * PI).abs() < 1e-15);
    }

    #[test]
    fn curve_ellipse_is_closed_and_periodic() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::Ellipse, 0.0, 2.0 * PI);
        assert!(c.is_closed());
        assert!(c.is_periodic());
        let p = c.period().unwrap();
        assert!((p - 2.0 * PI).abs() < 1e-15);
    }

    #[test]
    fn curve_bspline_not_closed() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::BSplineCurve, 0.0, 1.0);
        assert!(!c.is_closed());
        assert!(!c.is_periodic());
        assert!(c.period().is_none());
    }

    #[test]
    fn curve_parameter_range_stored_correctly() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::Line, -3.5, 7.2);
        assert!((c.first_parameter() - (-3.5)).abs() < 1e-15);
        assert!((c.last_parameter() - 7.2).abs() < 1e-15);
    }

    #[test]
    fn curve_default_continuity_is_cn() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::Line, 0.0, 1.0);
        assert_eq!(c.continuity(), Adaptor3dContinuity::CN);
    }

    #[test]
    fn curve_with_continuity_builder() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::BSplineCurve, 0.0, 1.0)
            .with_continuity(Adaptor3dContinuity::C1);
        assert_eq!(c.continuity(), Adaptor3dContinuity::C1);
    }

    #[test]
    fn curve_nb_intervals_is_one() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::Line, 0.0, 5.0);
        assert_eq!(c.nb_intervals(Adaptor3dContinuity::C0), 1);
        assert_eq!(c.nb_intervals(Adaptor3dContinuity::CN), 1);
    }

    #[test]
    fn curve_value_stub_returns_u_zero_zero() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::Line, 0.0, 10.0);
        let p = c.value(4.5);
        assert!((p[0] - 4.5).abs() < 1e-15);
        assert_eq!(p[1], 0.0);
        assert_eq!(p[2], 0.0);
    }

    #[test]
    fn surface_plane_not_u_closed_not_v_closed() {
        let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
        assert!(!s.is_u_closed());
        assert!(!s.is_v_closed());
        assert!(!s.is_u_periodic());
        assert!(!s.is_v_periodic());
    }

    #[test]
    fn surface_cylinder_u_closed_not_v_closed() {
        let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 5.0);
        assert!(s.is_u_closed());
        assert!(!s.is_v_closed());
        assert!(s.is_u_periodic());
        assert!(!s.is_v_periodic());
    }

    #[test]
    fn surface_sphere_u_closed_and_v_closed() {
        let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Sphere, 0.0, 2.0 * PI, -PI / 2.0, PI / 2.0);
        assert!(s.is_u_closed());
        assert!(s.is_v_closed());
        assert!(s.is_u_periodic());
        assert!(s.is_v_periodic());
    }

    #[test]
    fn surface_torus_u_closed_and_v_closed() {
        let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Torus, 0.0, 2.0 * PI, 0.0, 2.0 * PI);
        assert!(s.is_u_closed());
        assert!(s.is_v_closed());
    }

    #[test]
    fn surface_cone_u_closed_not_v_closed() {
        let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Cone, 0.0, 2.0 * PI, 0.0, 1.0);
        assert!(s.is_u_closed());
        assert!(!s.is_v_closed());
    }

    #[test]
    fn surface_parameter_ranges_stored() {
        let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Plane, -1.0, 2.0, -3.0, 4.0);
        assert!((s.u_first() - (-1.0)).abs() < 1e-15);
        assert!((s.u_last() - 2.0).abs() < 1e-15);
        assert!((s.v_first() - (-3.0)).abs() < 1e-15);
        assert!((s.v_last() - 4.0).abs() < 1e-15);
    }

    #[test]
    fn surface_value_stub_returns_u_v_zero() {
        let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
        let p = s.value(1.5, 2.5);
        assert!((p[0] - 1.5).abs() < 1e-15);
        assert!((p[1] - 2.5).abs() < 1e-15);
        assert_eq!(p[2], 0.0);
    }

    #[test]
    fn continuity_ordering() {
        assert!(Adaptor3dContinuity::C0 < Adaptor3dContinuity::G1);
        assert!(Adaptor3dContinuity::G1 < Adaptor3dContinuity::C1);
        assert!(Adaptor3dContinuity::C1 < Adaptor3dContinuity::G2);
        assert!(Adaptor3dContinuity::G2 < Adaptor3dContinuity::C2);
        assert!(Adaptor3dContinuity::C2 < Adaptor3dContinuity::C3);
        assert!(Adaptor3dContinuity::C3 < Adaptor3dContinuity::CN);
    }

    #[test]
    fn surface_type_accessor() {
        let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::BSplineSurface, 0.0, 1.0, 0.0, 1.0);
        assert_eq!(s.surface_type(), Adaptor3dSurfaceType::BSplineSurface);
    }

    #[test]
    fn curve_type_accessor() {
        let c = Adaptor3dCurve::new(Adaptor3dCurveType::Hyperbola, 0.0, 1.0);
        assert_eq!(c.curve_type(), Adaptor3dCurveType::Hyperbola);
    }
}

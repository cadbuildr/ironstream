// FILE: adapt_curve.rs
// occt: Adaptor3d_Curve, Adaptor2d_Curve2d, GeomAdaptor_Curve

/// Curve type (discriminant for curve geometry).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveType {
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

impl Default for CurveType {
    fn default() -> Self { Self::Line }
}

/// A derivative of a 3D point on a curve.
#[derive(Clone, Debug, Default)]
pub struct CurveD1 {
    pub point: [f64; 3],
    pub d1: [f64; 3],
}

/// Second derivative.
#[derive(Clone, Debug, Default)]
pub struct CurveD2 {
    pub point: [f64; 3],
    pub d1: [f64; 3],
    pub d2: [f64; 3],
}

// occt: Adaptor3d_Curve — abstract 3D curve adaptor interface stub
#[derive(Clone, Debug)]
pub struct Adaptor3dCurve {
    pub curve_id: u32,
    pub curve_type: CurveType,
    pub first_param: f64,
    pub last_param: f64,
    pub is_closed: bool,
    pub is_periodic: bool,
    pub period: f64,
    pub resolution: f64,
}

impl Adaptor3dCurve {
    pub fn new(curve_id: u32, curve_type: CurveType) -> Self {
        Self {
            curve_id,
            curve_type,
            first_param: 0.0,
            last_param: 1.0,
            is_closed: false,
            is_periodic: false,
            period: 0.0,
            resolution: 1e-7,
        }
    }

    pub fn set_param_range(&mut self, first: f64, last: f64) {
        self.first_param = first;
        self.last_param = last;
    }

    pub fn first_parameter(&self) -> f64 { self.first_param }
    pub fn last_parameter(&self) -> f64 { self.last_param }
    pub fn get_type(&self) -> CurveType { self.curve_type }
    pub fn is_closed(&self) -> bool { self.is_closed }
    pub fn is_periodic(&self) -> bool { self.is_periodic }
    pub fn period(&self) -> f64 { self.period }

    pub fn value(&self, t: f64) -> [f64; 3] {
        // Stub: line-like evaluation
        let s = (t - self.first_param) / (self.last_param - self.first_param).max(1e-15);
        [s, 0.0, 0.0]
    }

    pub fn d1(&self, t: f64) -> CurveD1 {
        let s = (t - self.first_param) / (self.last_param - self.first_param).max(1e-15);
        CurveD1 { point: [s, 0.0, 0.0], d1: [1.0 / (self.last_param - self.first_param).max(1e-15), 0.0, 0.0] }
    }

    pub fn d2(&self, t: f64) -> CurveD2 {
        let d1 = self.d1(t);
        CurveD2 { point: d1.point, d1: d1.d1, d2: [0.0; 3] }
    }

    pub fn resolution(&self) -> f64 { self.resolution }

    pub fn trim(&self, first: f64, last: f64) -> Self {
        let mut c = self.clone();
        c.set_param_range(first, last);
        c
    }
}

// occt: GeomAdaptor_Curve — wraps a Geom curve into an Adaptor3d_Curve
#[derive(Clone, Debug)]
pub struct GeomAdaptorCurve {
    pub adaptor: Adaptor3dCurve,
    pub geom_curve_id: u32,
}

impl GeomAdaptorCurve {
    pub fn new(geom_curve_id: u32) -> Self {
        Self {
            adaptor: Adaptor3dCurve::new(geom_curve_id, CurveType::BSplineCurve),
            geom_curve_id,
        }
    }

    pub fn from_curve_and_range(geom_curve_id: u32, first: f64, last: f64) -> Self {
        let mut g = Self::new(geom_curve_id);
        g.adaptor.set_param_range(first, last);
        g
    }

    pub fn get_type(&self) -> CurveType { self.adaptor.get_type() }
    pub fn value(&self, t: f64) -> [f64; 3] { self.adaptor.value(t) }
    pub fn first_parameter(&self) -> f64 { self.adaptor.first_parameter() }
    pub fn last_parameter(&self) -> f64 { self.adaptor.last_parameter() }
}

// occt: Adaptor2d_Curve2d — 2D curve adaptor (in UV space)
#[derive(Clone, Debug)]
pub struct Adaptor2dCurve2d {
    pub curve_id: u32,
    pub curve_type: CurveType,
    pub first_param: f64,
    pub last_param: f64,
}

impl Adaptor2dCurve2d {
    pub fn new(curve_id: u32, curve_type: CurveType) -> Self {
        Self { curve_id, curve_type, first_param: 0.0, last_param: 1.0 }
    }

    pub fn value(&self, t: f64) -> [f64; 2] {
        let s = (t - self.first_param) / (self.last_param - self.first_param).max(1e-15);
        [s, 0.0]
    }

    pub fn first_parameter(&self) -> f64 { self.first_param }
    pub fn last_parameter(&self) -> f64 { self.last_param }
    pub fn get_type(&self) -> CurveType { self.curve_type }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adaptor3d_value_range() {
        let c = Adaptor3dCurve::new(1, CurveType::Line);
        let p = c.value(0.0);
        assert!((p[0] - 0.0).abs() < 1e-10);
        let p1 = c.value(1.0);
        assert!((p1[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn adaptor3d_trim() {
        let c = Adaptor3dCurve::new(1, CurveType::Circle);
        let t = c.trim(0.0, 2.0);
        assert!((t.last_parameter() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn adaptor3d_d1() {
        let c = Adaptor3dCurve::new(1, CurveType::Line);
        let d = c.d1(0.5);
        assert!((d.point[0] - 0.5).abs() < 1e-10);
        assert!((d.d1[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn geom_adaptor_curve() {
        let g = GeomAdaptorCurve::from_curve_and_range(42, -1.0, 3.0);
        assert!((g.first_parameter() + 1.0).abs() < 1e-10);
        assert!((g.last_parameter() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn adaptor2d_value() {
        let c = Adaptor2dCurve2d::new(10, CurveType::Line);
        let p = c.value(0.5);
        assert!((p[0] - 0.5).abs() < 1e-10);
        assert!((p[1] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn adaptor3d_line_type() {
        let c = Adaptor3dCurve::new(5, CurveType::BezierCurve);
        assert_eq!(c.get_type(), CurveType::BezierCurve);
        assert!(!c.is_closed());
        assert!(!c.is_periodic());
    }
}

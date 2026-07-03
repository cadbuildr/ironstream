// FILE: curve_adapt.rs
// occt: Adaptor3d_HCurve, Adaptor3d_CurveOnSurface,
//       Geom2dAdaptor_Curve, BRep_CurveRepresentation

/// The type of parametric entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomAbsShape {
    Point,
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

impl Default for GeomAbsShape {
    fn default() -> Self { Self::BSplineCurve }
}

/// Continuity class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GeomAbsContinuity {
    C0,
    G1,
    C1,
    G2,
    C2,
    C3,
    CN,
}

impl Default for GeomAbsContinuity {
    fn default() -> Self { Self::C0 }
}

// occt: Geom2dAdaptor_Curve — adapts a Geom2d_Curve for algorithm use
#[derive(Clone, Debug)]
pub struct Geom2dAdaptorCurve {
    pub curve_id: u32,
    pub curve_type: GeomAbsShape,
    pub first: f64,
    pub last: f64,
    pub continuity: GeomAbsContinuity,
}

impl Default for Geom2dAdaptorCurve {
    fn default() -> Self {
        Self {
            curve_id: 0,
            curve_type: GeomAbsShape::BSplineCurve,
            first: 0.0,
            last: 1.0,
            continuity: GeomAbsContinuity::C0,
        }
    }
}

impl Geom2dAdaptorCurve {
    pub fn new(curve_id: u32) -> Self {
        Self { curve_id, ..Default::default() }
    }

    pub fn with_trim(curve_id: u32, first: f64, last: f64) -> Self {
        Self { curve_id, first, last, ..Default::default() }
    }

    pub fn curve_id(&self) -> u32 { self.curve_id }
    pub fn first_parameter(&self) -> f64 { self.first }
    pub fn last_parameter(&self) -> f64 { self.last }
    pub fn get_type(&self) -> GeomAbsShape { self.curve_type }
    pub fn continuity(&self) -> GeomAbsContinuity { self.continuity }

    /// Stub: value at parameter u (point on a unit circle arc in 2D)
    pub fn value(&self, u: f64) -> [f64; 2] {
        let t = (u - self.first) / (self.last - self.first + 1e-15);
        [t.cos(), t.sin()]
    }

    pub fn d1(&self, u: f64) -> ([f64; 2], [f64; 2]) {
        let v = self.value(u);
        let d = 1.0 / (self.last - self.first + 1e-15);
        (v, [-v[1] * d, v[0] * d])
    }

    pub fn is_closed(&self) -> bool {
        let p0 = self.value(self.first);
        let p1 = self.value(self.last);
        let dx = p1[0] - p0[0];
        let dy = p1[1] - p0[1];
        (dx * dx + dy * dy).sqrt() < 1e-7
    }

    pub fn is_periodic(&self) -> bool { false }

    pub fn nb_intervals(&self, _continuity: GeomAbsContinuity) -> usize { 1 }
}

// occt: Adaptor3d_CurveOnSurface — a 2D curve mapped onto a surface's parametric space
#[derive(Clone, Debug)]
pub struct AdaptorCurveOnSurface {
    pub curve2d_id: u32,
    pub surface_id: u32,
    pub first: f64,
    pub last: f64,
    pub tolerance: f64,
    pub continuity: GeomAbsContinuity,
}

impl AdaptorCurveOnSurface {
    pub fn new(curve2d_id: u32, surface_id: u32) -> Self {
        Self {
            curve2d_id,
            surface_id,
            first: 0.0,
            last: 1.0,
            tolerance: 1e-7,
            continuity: GeomAbsContinuity::C0,
        }
    }

    pub fn set_trim(&mut self, first: f64, last: f64) {
        self.first = first;
        self.last = last;
    }

    pub fn first_parameter(&self) -> f64 { self.first }
    pub fn last_parameter(&self) -> f64 { self.last }
    pub fn get_type(&self) -> GeomAbsShape { GeomAbsShape::OtherCurve }
    pub fn continuity(&self) -> GeomAbsContinuity { self.continuity }

    /// Stub: evaluates the 3D point by mapping u through the 2D curve onto the surface
    pub fn value(&self, u: f64) -> [f64; 3] {
        let t = (u - self.first) / (self.last - self.first + 1e-15);
        [t, t, 0.0]   // stub: diagonal line
    }

    pub fn d1(&self, u: f64) -> ([f64; 3], [f64; 3]) {
        let v = self.value(u);
        let scale = 1.0 / (self.last - self.first + 1e-15);
        (v, [scale, scale, 0.0])
    }

    pub fn is_closed(&self) -> bool { false }
    pub fn nb_intervals(&self, _continuity: GeomAbsContinuity) -> usize { 1 }
}

/// Type of a BRep curve representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrepCurveRepType {
    Curve3D,
    CurveOnSurface,
    CurveOnClosedSurface,
    Polygon3D,
    PolygonOnSurface,
    PolygonOnClosedSurface,
    PolygonOnTriangulation,
    PolygonOnClosedTriangulation,
}

impl Default for BrepCurveRepType {
    fn default() -> Self { Self::Curve3D }
}

// occt: BRep_CurveRepresentation — stores one representation of an edge's geometry
#[derive(Clone, Debug)]
pub struct BrepCurveRepresentation {
    pub rep_type: BrepCurveRepType,
    pub curve_id: u32,
    pub surface_id: u32,
    pub location_id: u32,
    pub tolerance: f64,
}

impl BrepCurveRepresentation {
    pub fn curve3d(curve_id: u32, tolerance: f64) -> Self {
        Self { rep_type: BrepCurveRepType::Curve3D, curve_id, surface_id: 0, location_id: 0, tolerance }
    }

    pub fn curve_on_surface(curve2d_id: u32, surface_id: u32, tolerance: f64) -> Self {
        Self { rep_type: BrepCurveRepType::CurveOnSurface, curve_id: curve2d_id, surface_id, location_id: 0, tolerance }
    }

    pub fn rep_type(&self) -> BrepCurveRepType { self.rep_type }
    pub fn is_curve3d(&self) -> bool { self.rep_type == BrepCurveRepType::Curve3D }
    pub fn is_curve_on_surface(&self) -> bool {
        matches!(self.rep_type, BrepCurveRepType::CurveOnSurface | BrepCurveRepType::CurveOnClosedSurface)
    }
    pub fn curve_id(&self) -> u32 { self.curve_id }
    pub fn surface_id(&self) -> u32 { self.surface_id }
    pub fn tolerance(&self) -> f64 { self.tolerance }
}

// occt: Adaptor3d_HCurve — handle (wrapper) around a 3D curve adaptor
#[derive(Clone, Debug)]
pub struct Adaptor3dHCurve {
    pub curve_id: u32,
    pub curve_type: GeomAbsShape,
    pub first: f64,
    pub last: f64,
    pub length_stub: f64,  // stub: pre-computed arc length
}

impl Adaptor3dHCurve {
    pub fn new(curve_id: u32, first: f64, last: f64) -> Self {
        Self {
            curve_id,
            curve_type: GeomAbsShape::BSplineCurve,
            first,
            last,
            length_stub: (last - first).abs(),
        }
    }

    pub fn curve_id(&self) -> u32 { self.curve_id }
    pub fn first_parameter(&self) -> f64 { self.first }
    pub fn last_parameter(&self) -> f64 { self.last }
    pub fn get_type(&self) -> GeomAbsShape { self.curve_type }
    pub fn is_null(&self) -> bool { self.curve_id == 0 }

    /// Stub: value on a unit line
    pub fn value(&self, u: f64) -> [f64; 3] {
        let t = (u - self.first) / (self.last - self.first + 1e-15);
        [t, 0.0, 0.0]
    }

    pub fn d1(&self, u: f64) -> ([f64; 3], [f64; 3]) {
        let v = self.value(u);
        let scale = 1.0 / (self.last - self.first + 1e-15);
        (v, [scale, 0.0, 0.0])
    }

    pub fn arc_length(&self) -> f64 { self.length_stub }
    pub fn nb_intervals(&self, _c: GeomAbsContinuity) -> usize { 1 }
    pub fn intervals(&self, _c: GeomAbsContinuity) -> Vec<f64> { vec![self.first, self.last] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geom2d_adaptor_value_d1() {
        let c = Geom2dAdaptorCurve::with_trim(1, 0.0, 1.0);
        let v = c.value(0.0);
        assert!((v[0] - 1.0).abs() < 1e-10);
        let (_, d) = c.d1(0.0);
        assert!(d[0].abs() < 0.1 || d[1].abs() > 0.5);
    }

    #[test]
    fn geom2d_adaptor_trim_range() {
        let c = Geom2dAdaptorCurve::with_trim(5, 0.0, 2.0);
        assert!((c.first_parameter()).abs() < 1e-10);
        assert!((c.last_parameter() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn curve_on_surface_value() {
        let mut c = AdaptorCurveOnSurface::new(1, 10);
        c.set_trim(0.0, 1.0);
        let v = c.value(0.5);
        // stub: t = 0.5 => [0.5, 0.5, 0.0]
        assert!((v[0] - 0.5).abs() < 1e-10);
        assert!((v[1] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn brep_curve_rep_types() {
        let r1 = BrepCurveRepresentation::curve3d(10, 1e-6);
        assert!(r1.is_curve3d());
        assert!(!r1.is_curve_on_surface());

        let r2 = BrepCurveRepresentation::curve_on_surface(5, 20, 1e-6);
        assert!(r2.is_curve_on_surface());
        assert_eq!(r2.surface_id(), 20);
    }

    #[test]
    fn adaptor3d_hcurve_basic() {
        let h = Adaptor3dHCurve::new(3, 0.0, 2.0);
        assert!(!h.is_null());
        assert!((h.arc_length() - 2.0).abs() < 1e-10);
        let v = h.value(1.0);
        assert!((v[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn adaptor3d_hcurve_null() {
        let h = Adaptor3dHCurve::new(0, 0.0, 1.0);
        assert!(h.is_null());
    }
}

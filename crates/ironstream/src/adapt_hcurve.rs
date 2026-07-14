// FILE: adapt_hcurve.rs
// occt-ref: Adaptor3d_HCurve, Adaptor3d_HSurface
//       Adaptor3d_CurveOnSurface, Adaptor2d_HCurve2d

/// Curve type identifier (mirrors GeomAbs_CurveType).
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
    fn default() -> Self { Self::BSplineCurve }
}

/// Surface type identifier (mirrors GeomAbs_SurfaceType).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceType {
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

impl Default for SurfaceType {
    fn default() -> Self { Self::BSplineSurface }
}

// occt-ref: Adaptor3d_Curve // — abstract adaptor wrapping a 3D curve
#[derive(Clone, Debug)]
pub struct Adaptor3dCurve {
    pub curve_id: u32,
    pub curve_type: CurveType,
    pub u_first: f64,
    pub u_last: f64,
    pub continuity: u8,   // 0=C0, 1=C1, 2=C2, ...
}

impl Adaptor3dCurve {
    pub fn new(curve_id: u32, curve_type: CurveType, u0: f64, u1: f64) -> Self {
        Self { curve_id, curve_type, u_first: u0, u_last: u1, continuity: 2 }
    }

    pub fn value(&self, u: f64) -> [f64; 3] {
        let t = (u - self.u_first) / (self.u_last - self.u_first).max(1e-14);
        [t, 0.0, 0.0]
    }

    pub fn d1(&self, u: f64) -> ([f64; 3], [f64; 3]) {
        (self.value(u), [1.0 / (self.u_last - self.u_first).max(1e-14), 0.0, 0.0])
    }

    pub fn period(&self) -> Option<f64> {
        if self.curve_type == CurveType::Circle { Some(2.0 * std::f64::consts::PI) } else { None }
    }

    pub fn is_periodic(&self) -> bool { self.period().is_some() }
    pub fn first_parameter(&self) -> f64 { self.u_first }
    pub fn last_parameter(&self) -> f64 { self.u_last }
    pub fn nb_intervals(&self, _continuity: u8) -> usize { 1 }
}

// occt-note: Adaptor3d_HCurve — handle wrapper around Adaptor3d_Curve
#[derive(Clone, Debug)]
pub struct HCurve {
    pub curve: Adaptor3dCurve,
}

impl HCurve {
    pub fn new(curve: Adaptor3dCurve) -> Self { Self { curve } }
    pub fn curve(&self) -> &Adaptor3dCurve { &self.curve }
    pub fn get_type(&self) -> CurveType { self.curve.curve_type }
    pub fn is_null(&self) -> bool { false }
}

// occt-ref: Adaptor3d_Surface // — abstract adaptor wrapping a 3D surface
#[derive(Clone, Debug)]
pub struct Adaptor3dSurface {
    pub surface_id: u32,
    pub surface_type: SurfaceType,
    pub u_first: f64,
    pub u_last: f64,
    pub v_first: f64,
    pub v_last: f64,
    pub is_u_closed: bool,
    pub is_v_closed: bool,
    pub is_u_periodic: bool,
    pub is_v_periodic: bool,
    pub u_period: f64,
    pub v_period: f64,
}

impl Adaptor3dSurface {
    pub fn new(id: u32, stype: SurfaceType, u0: f64, u1: f64, v0: f64, v1: f64) -> Self {
        Self {
            surface_id: id,
            surface_type: stype,
            u_first: u0, u_last: u1,
            v_first: v0, v_last: v1,
            is_u_closed: false, is_v_closed: false,
            is_u_periodic: false, is_v_periodic: false,
            u_period: 0.0, v_period: 0.0,
        }
    }

    pub fn value(&self, u: f64, v: f64) -> [f64; 3] { [u, v, 0.0] }

    pub fn d1u(&self, _u: f64, _v: f64) -> [f64; 3] { [1.0, 0.0, 0.0] }
    pub fn d1v(&self, _u: f64, _v: f64) -> [f64; 3] { [0.0, 1.0, 0.0] }
    pub fn normal(&self, _u: f64, _v: f64) -> [f64; 3] { [0.0, 0.0, 1.0] }

    pub fn u_period(&self) -> Option<f64> {
        if self.is_u_periodic { Some(self.u_period) } else { None }
    }
    pub fn v_period(&self) -> Option<f64> {
        if self.is_v_periodic { Some(self.v_period) } else { None }
    }

    pub fn nb_u_intervals(&self, _continuity: u8) -> usize { 1 }
    pub fn nb_v_intervals(&self, _continuity: u8) -> usize { 1 }
}

// occt-note: Adaptor3d_HSurface — handle wrapper around Adaptor3d_Surface
#[derive(Clone, Debug)]
pub struct HSurface {
    pub surface: Adaptor3dSurface,
}

impl HSurface {
    pub fn new(surface: Adaptor3dSurface) -> Self { Self { surface } }
    pub fn surface(&self) -> &Adaptor3dSurface { &self.surface }
    pub fn get_type(&self) -> SurfaceType { self.surface.surface_type }
    pub fn is_null(&self) -> bool { false }
}

// occt-ref: Adaptor3d_CurveOnSurface // — 3D curve induced by a 2D pcurve on a surface
#[derive(Clone, Debug)]
pub struct CurveOnSurface {
    pub pcurve_id: u32,     // 2D curve (pcurve) reference
    pub surface: Adaptor3dSurface,
    pub u_first: f64,
    pub u_last: f64,
    pub tolerance: f64,
}

impl CurveOnSurface {
    pub fn new(pcurve_id: u32, surface: Adaptor3dSurface) -> Self {
        let u0 = surface.u_first;
        let u1 = surface.u_last;
        Self { pcurve_id, surface, u_first: u0, u_last: u1, tolerance: 1e-7 }
    }

    pub fn value(&self, u: f64) -> [f64; 3] {
        // Stub: map u along pcurve to surface point
        let t = (u - self.u_first) / (self.u_last - self.u_first).max(1e-14);
        self.surface.value(t * (self.surface.u_last - self.surface.u_first), 0.5)
    }

    pub fn first_parameter(&self) -> f64 { self.u_first }
    pub fn last_parameter(&self) -> f64 { self.u_last }
    pub fn get_type(&self) -> CurveType { CurveType::BSplineCurve }
}

// occt-note: Adaptor2d_HCurve2d — 2D adaptor curve handle
#[derive(Clone, Debug)]
pub struct HCurve2d {
    pub curve_id: u32,
    pub curve_type: CurveType,
    pub u_first: f64,
    pub u_last: f64,
}

impl HCurve2d {
    pub fn new(curve_id: u32, u0: f64, u1: f64) -> Self {
        Self { curve_id, curve_type: CurveType::BSplineCurve, u_first: u0, u_last: u1 }
    }

    pub fn value(&self, u: f64) -> [f64; 2] {
        let t = (u - self.u_first) / (self.u_last - self.u_first).max(1e-14);
        [t, 0.0]
    }

    pub fn d1(&self, u: f64) -> ([f64; 2], [f64; 2]) {
        (self.value(u), [1.0, 0.0])
    }

    pub fn first_parameter(&self) -> f64 { self.u_first }
    pub fn last_parameter(&self) -> f64 { self.u_last }
    pub fn is_null(&self) -> bool { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adaptor_curve_value() {
        let c = Adaptor3dCurve::new(0, CurveType::Line, 0.0, 1.0);
        let pt = c.value(0.5);
        assert!((pt[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn hcurve_get_type() {
        let c = Adaptor3dCurve::new(0, CurveType::Circle, 0.0, 2.0 * std::f64::consts::PI);
        let hc = HCurve::new(c);
        assert_eq!(hc.get_type(), CurveType::Circle);
        assert!(hc.curve().is_periodic());
    }

    #[test]
    fn adaptor_surface_normal() {
        let s = Adaptor3dSurface::new(0, SurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
        let n = s.normal(0.5, 0.5);
        assert!((n[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn curve_on_surface() {
        let surface = Adaptor3dSurface::new(0, SurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
        let cos = CurveOnSurface::new(0, surface);
        let pt = cos.value(0.0);
        assert!(pt[0].abs() < 1e-10);
    }

    #[test]
    fn hcurve2d() {
        let c2d = HCurve2d::new(0, 0.0, 1.0);
        let uv = c2d.value(0.5);
        assert!((uv[0] - 0.5).abs() < 1e-10);
    }
}

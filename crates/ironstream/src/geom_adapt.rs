// FILE: geom_adapt.rs
// occt-ref: GeomAdaptor_Curve
// occt-ref: GeomAdaptor_Surface

//! Lightweight stub adaptors for 3-D curves and surfaces.
//!
//! Models the public interface of `GeomAdaptor_Curve` and
//! `GeomAdaptor_Surface` from Open CASCADE Technology, providing
//! a zero-dependency, pure-Rust façade for parametric evaluation.
//!
//! All evaluation methods return placeholder values derived purely
//! from the parameter; replace with real geometry when the backing
//! representation is wired up.

// ── CurveType ────────────────────────────────────────────────────────────────

/// Classification of the underlying 3-D curve geometry.
// occt-ref: GeomAdaptor_Curve // (mirrors GeomAbs_CurveType)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveType {
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
    BezierCurve,
    BSplineCurve,
    OtherCurve,
}

// ── SurfaceType ───────────────────────────────────────────────────────────────

/// Classification of the underlying 3-D surface geometry.
// occt-ref: GeomAdaptor_Surface // (mirrors GeomAbs_SurfaceType)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceType {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    BezierSurface,
    BSplineSurface,
    OtherSurface,
}

// ── AdaptorCurve ─────────────────────────────────────────────────────────────

/// Adapts a named 3-D parametric curve to a uniform evaluation interface.
///
/// The parametric domain is `[first, last]`.
// occt-ref: GeomAdaptor_Curve
#[derive(Clone, Debug)]
pub struct AdaptorCurve {
    /// Human-readable label for the underlying curve (e.g. its OCCT name).
    pub curve_name: String,
    /// Start of the parametric domain.
    pub first: f64,
    /// End of the parametric domain.
    pub last: f64,
    /// Geometric kind of the underlying curve.
    pub curve_type: CurveType,
}

impl AdaptorCurve {
    /// Construct a new adaptor with default domain `[0.0, 1.0]` and type
    /// [`CurveType::OtherCurve`].
    // occt-ref: GeomAdaptor_Curve // ::GeomAdaptor_Curve(const Handle(Geom_Curve)&)
    pub fn new(name: &str) -> Self {
        AdaptorCurve {
            curve_name: name.to_string(),
            first: 0.0,
            last: 1.0,
            curve_type: CurveType::OtherCurve,
        }
    }

    /// Evaluate the curve position at parameter `t`.
    ///
    /// Returns `[x, y, z]`. Stub: returns `[t, 0.0, 0.0]`.
    // occt-ref: GeomAdaptor_Curve // ::Value
    pub fn value(&self, t: f64) -> [f64; 3] {
        [t, 0.0, 0.0]
    }

    /// Evaluate the first derivative (tangent) at parameter `t`.
    ///
    /// Returns `[dx, dy, dz]`. Stub: returns `[1.0, 0.0, 0.0]`.
    // occt-ref: GeomAdaptor_Curve // ::D1
    pub fn d1(&self, t: f64) -> [f64; 3] {
        let _ = t;
        [1.0, 0.0, 0.0]
    }

    /// Evaluate the second derivative (curvature vector) at parameter `t`.
    ///
    /// Returns `[d2x, d2y, d2z]`. Stub: returns `[0.0, 0.0, 0.0]`.
    // occt-ref: GeomAdaptor_Curve // ::D2
    pub fn d2(&self, t: f64) -> [f64; 3] {
        let _ = t;
        [0.0, 0.0, 0.0]
    }

    /// Return the geometric continuity order (C^n) of the curve.
    ///
    /// Stub: returns `2` (C² continuity).
    // occt-ref: GeomAdaptor_Curve // ::Continuity
    pub fn continuity(&self) -> u32 {
        2
    }
}

// ── AdaptorSurface ────────────────────────────────────────────────────────────

/// Adapts a named 3-D parametric surface to a uniform evaluation interface.
// occt-ref: GeomAdaptor_Surface
#[derive(Clone, Debug)]
pub struct AdaptorSurface {
    /// Human-readable label for the underlying surface (e.g. its OCCT name).
    pub surface_name: String,
    /// Geometric kind of the underlying surface.
    pub surface_type: SurfaceType,
}

impl AdaptorSurface {
    /// Construct a new adaptor with type [`SurfaceType::OtherSurface`].
    // occt-ref: GeomAdaptor_Surface // ::GeomAdaptor_Surface(const Handle(Geom_Surface)&)
    pub fn new(name: &str) -> Self {
        AdaptorSurface {
            surface_name: name.to_string(),
            surface_type: SurfaceType::OtherSurface,
        }
    }

    /// Evaluate the surface position at parameters `(u, v)`.
    ///
    /// Returns `[x, y, z]`. Stub: returns `[u, v, 0.0]`.
    // occt-ref: GeomAdaptor_Surface // ::Value
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        [u, v, 0.0]
    }

    /// Evaluate the first partial derivative in the U direction at `(u, v)`.
    ///
    /// Returns `[dx/du, dy/du, dz/du]`. Stub: returns `[1.0, 0.0, 0.0]`.
    // occt-ref: GeomAdaptor_Surface // ::D1U
    pub fn d1u(&self, u: f64, v: f64) -> [f64; 3] {
        let _ = (u, v);
        [1.0, 0.0, 0.0]
    }

    /// Evaluate the first partial derivative in the V direction at `(u, v)`.
    ///
    /// Returns `[dx/dv, dy/dv, dz/dv]`. Stub: returns `[0.0, 1.0, 0.0]`.
    // occt-ref: GeomAdaptor_Surface // ::D1V
    pub fn d1v(&self, u: f64, v: f64) -> [f64; 3] {
        let _ = (u, v);
        [0.0, 1.0, 0.0]
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_new_defaults() {
        let c = AdaptorCurve::new("MyLine");
        assert_eq!(c.curve_name, "MyLine");
        assert_eq!(c.first, 0.0);
        assert_eq!(c.last, 1.0);
        assert_eq!(c.curve_type, CurveType::OtherCurve);
    }

    #[test]
    fn curve_value_returns_t_on_x_axis() {
        let c = AdaptorCurve::new("test");
        assert_eq!(c.value(3.5), [3.5, 0.0, 0.0]);
    }

    #[test]
    fn curve_d1_unit_x() {
        let c = AdaptorCurve::new("test");
        assert_eq!(c.d1(0.0), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn curve_d2_zero() {
        let c = AdaptorCurve::new("test");
        assert_eq!(c.d2(0.5), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn curve_continuity_is_2() {
        let c = AdaptorCurve::new("test");
        assert_eq!(c.continuity(), 2);
    }

    #[test]
    fn surface_new_defaults() {
        let s = AdaptorSurface::new("MySphere");
        assert_eq!(s.surface_name, "MySphere");
        assert_eq!(s.surface_type, SurfaceType::OtherSurface);
    }

    #[test]
    fn surface_value_uv_plane() {
        let s = AdaptorSurface::new("test");
        assert_eq!(s.value(1.0, 2.0), [1.0, 2.0, 0.0]);
    }

    #[test]
    fn surface_d1u_unit_x() {
        let s = AdaptorSurface::new("test");
        assert_eq!(s.d1u(0.0, 0.0), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn surface_d1v_unit_y() {
        let s = AdaptorSurface::new("test");
        assert_eq!(s.d1v(0.0, 0.0), [0.0, 1.0, 0.0]);
    }
}

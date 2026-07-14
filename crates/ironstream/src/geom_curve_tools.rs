// FILE: crates/ironstream/src/geom_curve_tools.rs

// occt-ref: GeomLib_Tool
// occt-ref: GeomLib

//! Utility routines for evaluating and interrogating 3-D curves and surfaces,
//! modelled after `GeomLib_Tool` and `GeomLib` from the OCCT utility library.
//!
//! All geometry is represented as opaque string tags in this stub layer; the
//! heavy lifting will be forwarded to the real kernel once it is wired up.
//! Every function therefore returns a plausible zero/default value so that
//! callers can be compiled and unit-tested against the stub today.

// ---------------------------------------------------------------------------
// Free-function surface of GeomLib_Tool
// ---------------------------------------------------------------------------

// occt-ref: GeomLib_Tool
/// Evaluate a 3-D curve at parameter `t`.
///
/// Mirrors `GeomLib_Tool::Value`.
///
/// Returns the Cartesian point `[x, y, z]`.  The stub returns `[t, 0.0, 0.0]`
/// so that the parameter is traceable in unit tests.
pub fn curve_value(curve: &str, t: f64) -> [f64; 3] {
    let _ = curve;
    [t, 0.0, 0.0]
}

// occt-ref: GeomLib_Tool
/// Evaluate a 3-D curve together with its first derivative at parameter `t`.
///
/// Mirrors `GeomLib_Tool::D1`.
///
/// Returns `(point, tangent)` where each is `[x, y, z]`.  The stub returns
/// `([t, 0.0, 0.0], [1.0, 0.0, 0.0])` — a point on the parameter axis with
/// a unit tangent along X.
pub fn curve_d1(curve: &str, t: f64) -> ([f64; 3], [f64; 3]) {
    let _ = curve;
    ([t, 0.0, 0.0], [1.0, 0.0, 0.0])
}

// occt-ref: GeomLib_Tool
/// Compute the outward unit normal to a surface at parameters `(u, v)`.
///
/// Mirrors `GeomLib_Tool::NormalToSurface`.
///
/// Returns `[nx, ny, nz]`.  The stub returns a constant `[0.0, 0.0, 1.0]`
/// (i.e. a flat XY-plane normal).
pub fn normal_to_surface(surface: &str, u: f64, v: f64) -> [f64; 3] {
    let _ = (surface, u, v);
    [0.0, 0.0, 1.0]
}

// ---------------------------------------------------------------------------
// Free-function surface of GeomLib
// ---------------------------------------------------------------------------

// occt-ref: GeomLib
/// Test whether a curve satisfies C0 continuity within the given tolerance.
///
/// Mirrors `GeomLib::IsClosed` / continuity-check logic from `GeomLib`.
///
/// The stub conservatively returns `true` for any non-empty curve name and a
/// positive tolerance, consistent with a simple line or B-spline that is
/// trivially C0.
pub fn is_c0_curve(curve: &str, tol: f64) -> bool {
    !curve.is_empty() && tol >= 0.0
}

// occt-ref: GeomLib
/// Approximate/build a 3-D curve from a 2-D pcurve lying on a surface.
///
/// Mirrors `GeomLib::BuildCurve3d`.
///
/// Returns a string tag identifying the resulting 3-D curve.  The stub
/// returns a synthesised tag of the form `"3d:<curve2d>@<surface>"` so that
/// callers can verify that both inputs were forwarded.
pub fn build_curve_3d(curve2d: &str, surface: &str, tol: f64) -> String {
    let _ = tol;
    format!("3d:{}@{}", curve2d, surface)
}

// ---------------------------------------------------------------------------
// Struct surface of GeomLib_Tool / GeomLib
// ---------------------------------------------------------------------------

// occt-ref: GeomLib_Tool
/// A stateful helper that bundles a tolerance value with the curve-evaluation
/// and interrogation operations from `GeomLib_Tool`.
#[derive(Debug, Clone)]
pub struct CurveTools {
    /// Geometric tolerance used by all interrogation methods.
    pub tolerance: f64,
}

impl CurveTools {
    // occt-ref: GeomLib_Tool
    /// Construct a new `CurveTools` with the given tolerance.
    pub fn new(tol: f64) -> Self {
        Self { tolerance: tol }
    }

    // occt-ref: GeomLib_Tool
    /// Return `true` if `curve` is considered valid (non-empty name and a
    /// non-negative tolerance).
    ///
    /// Mirrors the validity pre-checks performed inside `GeomLib_Tool`.
    pub fn check_curve(&self, curve: &str) -> bool {
        !curve.is_empty() && self.tolerance >= 0.0
    }

    // occt-ref: GeomLib_Tool
    /// Sample `n` evenly-spaced points along the parametric range `[0, 1]`
    /// of the curve.
    ///
    /// Mirrors the uniform-distribution sampling used by `GeomLib` utilities.
    ///
    /// When `n` is zero an empty `Vec` is returned.  When `n` is one the
    /// midpoint `t = 0.5` is used.
    pub fn sample_curve(&self, curve: &str, n: usize) -> Vec<[f64; 3]> {
        if n == 0 {
            return Vec::new();
        }
        let (t0, t1) = self.curve_range(curve);
        let mut pts = Vec::with_capacity(n);
        if n == 1 {
            let t = (t0 + t1) * 0.5;
            pts.push(curve_value(curve, t));
        } else {
            for i in 0..n {
                let s = i as f64 / (n - 1) as f64;
                let t = t0 + s * (t1 - t0);
                pts.push(curve_value(curve, t));
            }
        }
        pts
    }

    // occt-ref: GeomLib_Tool
    /// Return the parametric range `(t_first, t_last)` of the curve.
    ///
    /// Mirrors `GeomLib_Tool::Parameters`.  The stub returns `(0.0, 1.0)` as
    /// the canonical unit parameter domain used throughout the stub layer.
    pub fn curve_range(&self, curve: &str) -> (f64, f64) {
        let _ = curve;
        (0.0, 1.0)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_value_uses_t() {
        let p = curve_value("line", 0.5);
        assert!((p[0] - 0.5).abs() < f64::EPSILON);
        assert_eq!(p[1], 0.0);
        assert_eq!(p[2], 0.0);
    }

    #[test]
    fn test_curve_d1_tangent() {
        let (pt, tang) = curve_d1("bspline", 0.25);
        assert!((pt[0] - 0.25).abs() < f64::EPSILON);
        assert_eq!(tang, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_normal_to_surface_flat() {
        let n = normal_to_surface("plane", 0.0, 0.0);
        assert_eq!(n, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_is_c0_curve() {
        assert!(is_c0_curve("circle", 1e-7));
        assert!(!is_c0_curve("", 1e-7));
        assert!(!is_c0_curve("line", -1.0));
    }

    #[test]
    fn test_build_curve_3d_tag() {
        let tag = build_curve_3d("pcurve1", "surf1", 1e-6);
        assert_eq!(tag, "3d:pcurve1@surf1");
    }

    #[test]
    fn test_curve_tools_new() {
        let ct = CurveTools::new(1e-7);
        assert!((ct.tolerance - 1e-7).abs() < f64::EPSILON);
    }

    #[test]
    fn test_curve_tools_check_curve() {
        let ct = CurveTools::new(1e-6);
        assert!(ct.check_curve("my_curve"));
        assert!(!ct.check_curve(""));
    }

    #[test]
    fn test_curve_tools_check_curve_negative_tol() {
        let ct = CurveTools::new(-1.0);
        assert!(!ct.check_curve("my_curve"));
    }

    #[test]
    fn test_curve_tools_curve_range() {
        let ct = CurveTools::new(1e-6);
        assert_eq!(ct.curve_range("line"), (0.0, 1.0));
    }

    #[test]
    fn test_curve_tools_sample_zero() {
        let ct = CurveTools::new(1e-6);
        assert!(ct.sample_curve("line", 0).is_empty());
    }

    #[test]
    fn test_curve_tools_sample_one() {
        let ct = CurveTools::new(1e-6);
        let pts = ct.sample_curve("line", 1);
        assert_eq!(pts.len(), 1);
        assert!((pts[0][0] - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_curve_tools_sample_five() {
        let ct = CurveTools::new(1e-6);
        let pts = ct.sample_curve("line", 5);
        assert_eq!(pts.len(), 5);
        // first point should be at t=0.0
        assert!((pts[0][0] - 0.0).abs() < f64::EPSILON);
        // last point should be at t=1.0
        assert!((pts[4][0] - 1.0).abs() < f64::EPSILON);
    }
}

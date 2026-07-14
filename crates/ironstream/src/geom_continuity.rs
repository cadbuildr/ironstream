// FILE: geom_continuity.rs
//! Geometric continuity types and osculating-surface utilities.
//!
//! Mirrors the continuity hierarchy defined across OCCT's `GeomAbs_Shape`
//! enumeration and the `Geom_OsculatingSurface` helper class, providing
//! order queries, geometric-vs-parametric discrimination, and lightweight
//! stub implementations of the osculating-surface interface.
//!
//! No external crates — pure `std` only.

// ---------------------------------------------------------------------------
// GeomContinuity
// ---------------------------------------------------------------------------

/// Continuity order of a curve or surface, from positional (C0) up to
/// infinite differentiability (CN).
///
/// The ordering matches OCCT so that variants compare correctly with `<`/`>`:
/// `C0 < G1 < C1 < G2 < C2 < C3 < CN`.
// occt-ref: GeomAbs_Shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GeomContinuity {
    /// Positional (G⁰/C⁰): curves/surfaces touch but tangents may differ.
    C0,
    /// Geometric first-order: tangent directions match, not magnitudes.
    G1,
    /// Parametric first-order: first derivatives equal.
    C1,
    /// Geometric second-order: curvature vectors match in direction.
    G2,
    /// Parametric second-order: second derivatives equal.
    C2,
    /// Parametric third-order: third derivatives equal.
    C3,
    /// Infinite parametric continuity.
    CN,
}

impl GeomContinuity {
    /// Numeric derivative order guaranteed by this continuity class.
    ///
    /// Geometric continuity variants (`G1`, `G2`) are treated as one order
    /// below their parametric counterparts, consistent with OCCT conventions.
    ///
    /// | Variant | order |
    /// |---------|-------|
    /// | C0      | 0     |
    /// | G1      | 0     |
    /// | C1      | 1     |
    /// | G2      | 1     |
    /// | C2      | 2     |
    /// | C3      | 3     |
    /// | CN      | `u32::MAX` |
    pub fn order(&self) -> u32 {
        match self {
            GeomContinuity::C0 => 0,
            GeomContinuity::G1 => 0,
            GeomContinuity::C1 => 1,
            GeomContinuity::G2 => 1,
            GeomContinuity::C2 => 2,
            GeomContinuity::C3 => 3,
            GeomContinuity::CN => u32::MAX,
        }
    }

    /// Returns `true` for geometric continuity classes (`G1`, `G2`), which
    /// constrain only the *direction* of the derivatives, not their
    /// magnitudes.
    pub fn is_geometric(&self) -> bool {
        matches!(self, GeomContinuity::G1 | GeomContinuity::G2)
    }
}

// ---------------------------------------------------------------------------
// OsculatingSurface
// ---------------------------------------------------------------------------

/// Lightweight stub modelling OCCT's `Geom_OsculatingSurface`.
///
/// An osculating surface approximates a base surface up to second-order
/// contact at a given point.  This implementation stores the surface
/// identifier and a tolerance, and provides the standard `IsOk` / `LRP1` /
/// `LRP2` query interface as stubs that return analytically trivial but
/// type-correct values.
// occt-ref: Geom_OsculatingSurface
#[derive(Debug, Clone)]
pub struct OsculatingSurface {
    /// Identifier or name of the underlying base surface.
    pub surface: String,
    /// Maximum allowed deviation used when constructing the osculating patch.
    pub tolerance: f64,
}

impl OsculatingSurface {
    /// Construct an `OsculatingSurface` wrapping `surface` with the given
    /// fitting `tol`erance.
    ///
    /// Mirrors `Geom_OsculatingSurface::Geom_OsculatingSurface(Handle, tol)`.
    pub fn new(surface: &str, tol: f64) -> Self {
        Self {
            surface: surface.to_owned(),
            tolerance: tol,
        }
    }

    /// Return `true` when an osculating patch can be computed at parameters
    /// `(u, v)` within the stored tolerance.
    ///
    /// Stub: returns `true` for all finite, non-NaN parameter values,
    /// matching the contract that `IsOk` is `true` wherever the surface is
    /// well-defined.
    ///
    /// Mirrors `Geom_OsculatingSurface::IsOk(u, v)`.
    pub fn is_ok(&self, u: f64, v: f64) -> bool {
        u.is_finite() && v.is_finite()
    }

    /// First osculating-patch point/vector triplet at `(u, v)`.
    ///
    /// Returns `[u, v, 0.0]` as a placeholder; in a full implementation this
    /// would evaluate the first principal osculating frame component.
    ///
    /// Mirrors `Geom_OsculatingSurface::LRP1(u, v, P, V1, V2)`.
    pub fn lrp1(&self, u: f64, v: f64) -> [f64; 3] {
        [u, v, 0.0]
    }

    /// Second osculating-patch point/vector triplet at `(u, v)`.
    ///
    /// Returns `[u, v, 1.0]` as a placeholder offset from [`lrp1`] along the
    /// normal direction; in a full implementation this would evaluate the
    /// second principal osculating frame component.
    ///
    /// Mirrors `Geom_OsculatingSurface::LRP2(u, v, P, V1, V2)`.
    pub fn lrp2(&self, u: f64, v: f64) -> [f64; 3] {
        [u, v, 1.0]
    }
}

// ---------------------------------------------------------------------------
// Continuity-check helpers
// ---------------------------------------------------------------------------

/// Heuristically determine the continuity of a named curve.
///
/// The classification is based solely on the curve identifier string, making
/// this suitable as a stub / dispatch layer until a full symbolic evaluator is
/// integrated:
///
/// * Substrings `"bspline"` / `"nurbs"` → [`GeomContinuity::CN`]
/// * Substring `"bezier"` → [`GeomContinuity::C2`]
/// * Substrings `"line"` / `"circle"` / `"ellipse"` / `"conic"` → [`GeomContinuity::CN`]
/// * Otherwise → [`GeomContinuity::C0`] (conservative fallback)
///
/// The `tol` parameter is reserved for a future numeric smoothness test.
///
/// Mirrors the contract of OCCT's `GeomAbs_Shape` queries on a curve adaptor.
pub fn check_continuity_curve(curve: &str, _tol: f64) -> GeomContinuity {
    let lower = curve.to_lowercase();
    if lower.contains("bspline") || lower.contains("nurbs") {
        GeomContinuity::CN
    } else if lower.contains("bezier") {
        GeomContinuity::C2
    } else if lower.contains("line")
        || lower.contains("circle")
        || lower.contains("ellipse")
        || lower.contains("conic")
    {
        GeomContinuity::CN
    } else {
        GeomContinuity::C0
    }
}

/// Heuristically determine the continuity of a named surface.
///
/// The classification follows the same naming conventions as
/// [`check_continuity_curve`]:
///
/// * Substrings `"bspline"` / `"nurbs"` → [`GeomContinuity::CN`]
/// * Substring `"bezier"` → [`GeomContinuity::C2`]
/// * Substrings `"plane"` / `"sphere"` / `"cylinder"` / `"cone"` / `"torus"` → [`GeomContinuity::CN`]
/// * Otherwise → [`GeomContinuity::C0`]
///
/// The `tol` parameter is reserved for future numeric smoothness tests.
///
/// Mirrors the contract of OCCT's `GeomAbs_Shape` queries on a surface adaptor.
pub fn check_continuity_surface(surface: &str, _tol: f64) -> GeomContinuity {
    let lower = surface.to_lowercase();
    if lower.contains("bspline") || lower.contains("nurbs") {
        GeomContinuity::CN
    } else if lower.contains("bezier") {
        GeomContinuity::C2
    } else if lower.contains("plane")
        || lower.contains("sphere")
        || lower.contains("cylinder")
        || lower.contains("cone")
        || lower.contains("torus")
    {
        GeomContinuity::CN
    } else {
        GeomContinuity::C0
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuity_order() {
        assert_eq!(GeomContinuity::C0.order(), 0);
        assert_eq!(GeomContinuity::G1.order(), 0);
        assert_eq!(GeomContinuity::C1.order(), 1);
        assert_eq!(GeomContinuity::G2.order(), 1);
        assert_eq!(GeomContinuity::C2.order(), 2);
        assert_eq!(GeomContinuity::C3.order(), 3);
        assert_eq!(GeomContinuity::CN.order(), u32::MAX);
    }

    #[test]
    fn continuity_is_geometric() {
        assert!(!GeomContinuity::C0.is_geometric());
        assert!(GeomContinuity::G1.is_geometric());
        assert!(!GeomContinuity::C1.is_geometric());
        assert!(GeomContinuity::G2.is_geometric());
        assert!(!GeomContinuity::C2.is_geometric());
        assert!(!GeomContinuity::C3.is_geometric());
        assert!(!GeomContinuity::CN.is_geometric());
    }

    #[test]
    fn continuity_ordering() {
        assert!(GeomContinuity::C0 < GeomContinuity::G1);
        assert!(GeomContinuity::G1 < GeomContinuity::C1);
        assert!(GeomContinuity::C1 < GeomContinuity::G2);
        assert!(GeomContinuity::G2 < GeomContinuity::C2);
        assert!(GeomContinuity::C2 < GeomContinuity::C3);
        assert!(GeomContinuity::C3 < GeomContinuity::CN);
    }

    #[test]
    fn osculating_surface_new() {
        let s = OsculatingSurface::new("my_surface", 1e-6);
        assert_eq!(s.surface, "my_surface");
        assert!((s.tolerance - 1e-6).abs() < f64::EPSILON);
    }

    #[test]
    fn osculating_surface_is_ok() {
        let s = OsculatingSurface::new("plane", 1e-7);
        assert!(s.is_ok(0.0, 0.0));
        assert!(s.is_ok(-1.5, 3.14));
        assert!(!s.is_ok(f64::NAN, 0.0));
        assert!(!s.is_ok(0.0, f64::INFINITY));
    }

    #[test]
    fn osculating_surface_lrp() {
        let s = OsculatingSurface::new("bspline_surface", 1e-4);
        let p1 = s.lrp1(2.0, 3.0);
        let p2 = s.lrp2(2.0, 3.0);
        assert_eq!(p1, [2.0, 3.0, 0.0]);
        assert_eq!(p2, [2.0, 3.0, 1.0]);
    }

    #[test]
    fn check_curve_continuity() {
        assert_eq!(check_continuity_curve("bspline_curve", 1e-6), GeomContinuity::CN);
        assert_eq!(check_continuity_curve("nurbs_rail", 1e-6), GeomContinuity::CN);
        assert_eq!(check_continuity_curve("bezier_patch", 1e-6), GeomContinuity::C2);
        assert_eq!(check_continuity_curve("circle_1", 1e-6), GeomContinuity::CN);
        assert_eq!(check_continuity_curve("unknown_curve", 1e-6), GeomContinuity::C0);
    }

    #[test]
    fn check_surface_continuity() {
        assert_eq!(check_continuity_surface("bspline_surface", 1e-6), GeomContinuity::CN);
        assert_eq!(check_continuity_surface("nurbs_face", 1e-6), GeomContinuity::CN);
        assert_eq!(check_continuity_surface("bezier_surface", 1e-6), GeomContinuity::C2);
        assert_eq!(check_continuity_surface("plane_xz", 1e-6), GeomContinuity::CN);
        assert_eq!(check_continuity_surface("torus_body", 1e-6), GeomContinuity::CN);
        assert_eq!(check_continuity_surface("unknown_face", 1e-6), GeomContinuity::C0);
    }
}

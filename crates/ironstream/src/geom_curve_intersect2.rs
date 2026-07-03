// FILE: rust/ironstream/crates/ironstream/src/geom_curve_intersect2.rs
//! Surface–surface and curve–surface intersection stubs, mirroring
//! OpenCascade's `GeomAPI_IntSS` and `GeomAPI_IntCS` classes
//! (`src/ModelingAlgorithms/TKGeomAlgo/GeomAPI`).
//!
//! These are **stub** implementations: the constructors mark the computation
//! as done and leave the result sets empty so the full public API surface is
//! available for compilation and testing without requiring a numerical solver.
//!
//! Covered types:
//! - [`IntSS`]  — `GeomAPI_IntSS`: surface–surface intersection, yielding a
//!               collection of 3D intersection lines (polyline approximations).
//! - [`IntCS`]  — `GeomAPI_IntCS`: curve–surface intersection, yielding a
//!               collection of 3D points together with the curve parameter and
//!               the surface UV parameters at each intersection.
//!
//! Free functions:
//! - [`surface_surface_intersect`] — convenience wrapper returning the set of
//!   intersection polylines for two named surfaces.

// ─────────────────────────────────────────────────────────────────────────────
// IntSS — surface / surface intersection
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `GeomAPI_IntSS` — computes the intersection curves between two
/// parametric surfaces.
///
/// Each intersection result is represented as an ordered polyline: a
/// `Vec<[f64; 3]>` of 3D sample points along the intersection curve.  A real
/// implementation would return `Handle(Geom_Curve)` objects; the polyline
/// representation keeps this stub free of external dependencies.
///
/// # Example
///
/// ```
/// use ironstream::geom_curve_intersect2::IntSS;
///
/// let mut iss = IntSS::new("Plane1", "Sphere1", 1e-7);
/// assert!(iss.is_done());
/// assert_eq!(iss.nb_lines(), 0);
/// assert!(iss.line(0).is_none());
/// ```
// occt: GeomAPI_IntSS
#[derive(Clone, Debug)]
pub struct IntSS {
    /// Name / tag of the first surface (lightweight handle used in this stub).
    pub s1: String,
    /// Name / tag of the second surface.
    pub s2: String,
    /// Positional tolerance used by the intersection algorithm.
    pub tolerance: f64,
    /// Intersection lines, each stored as an ordered polyline of 3D points.
    pub lines: Vec<Vec<[f64; 3]>>,
    /// `true` once the intersection has been computed.
    pub done: bool,
}

impl IntSS {
    /// Construct and immediately perform the intersection between `s1` and `s2`
    /// within the given positional tolerance `tol`.
    ///
    /// In the stub the computation is instantaneous: `done` is set to `true`
    /// and `lines` is left empty (no intersection curves are synthesised).
    ///
    /// In a production implementation this would invoke the OCCT `IntSurf` /
    /// `IntPatch` machinery to trace intersection curves and sample them into
    /// polylines.
    ///
    /// # Parameters
    ///
    /// - `s1`  — name of the first surface.
    /// - `s2`  — name of the second surface.
    /// - `tol` — positional tolerance for the solver.
    // occt: GeomAPI_IntSS::GeomAPI_IntSS(const Handle(Geom_Surface)&,
    //                                     const Handle(Geom_Surface)&,
    //                                     const Standard_Real)
    pub fn new(s1: &str, s2: &str, tol: f64) -> Self {
        Self {
            s1: s1.to_owned(),
            s2: s2.to_owned(),
            tolerance: tol,
            lines: Vec::new(),
            done: true,
        }
    }

    /// `NbLines()` — number of intersection curves found.
    // occt: GeomAPI_IntSS::NbLines
    pub fn nb_lines(&self) -> usize {
        self.lines.len()
    }

    /// `Line(Index)` — return a reference to the `i`-th intersection polyline
    /// (0-based index).  Returns `None` if `i` is out of range.
    ///
    /// In OCCT this is 1-based and returns a `Handle(Geom_Curve)`; here it
    /// returns an optional reference to the stored polyline.
    // occt: GeomAPI_IntSS::Line(const Standard_Integer) — 1-based in OCCT
    pub fn line(&self, i: usize) -> Option<&Vec<[f64; 3]>> {
        self.lines.get(i)
    }

    /// `IsDone()` — `true` if the intersection has been computed successfully.
    // occt: GeomAPI_IntSS::IsDone
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// IntCS — curve / surface intersection
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `GeomAPI_IntCS` — computes the intersection points between a
/// parametric 3D curve and a parametric surface.
///
/// Each intersection result is a tuple `([f64; 3], f64, [f64; 2])` holding:
/// 1. the 3D intersection point,
/// 2. the curve parameter `t` at the intersection,
/// 3. the surface UV parameters `[u, v]` at the intersection.
///
/// # Example
///
/// ```
/// use ironstream::geom_curve_intersect2::IntCS;
///
/// let ics = IntCS::new("Line1", "Plane1", 1e-7);
/// assert!(ics.is_done());
/// assert_eq!(ics.nb_points(), 0);
/// assert!(ics.point(0).is_none());
/// ```
// occt: GeomAPI_IntCS
#[derive(Clone, Debug)]
pub struct IntCS {
    /// Name / tag of the intersecting curve.
    pub curve: String,
    /// Name / tag of the intersecting surface.
    pub surface: String,
    /// Positional tolerance used by the intersection algorithm.
    pub tolerance: f64,
    /// Intersection results: each entry is `(3D point, curve param t, surface UV)`.
    pub points: Vec<([f64; 3], f64, [f64; 2])>,
    /// `true` once the intersection has been computed.
    pub done: bool,
}

impl IntCS {
    /// Construct and immediately perform the intersection between `curve` and
    /// `surface` within the given positional tolerance `tol`.
    ///
    /// In the stub the computation is instantaneous: `done` is set to `true`
    /// and `points` is left empty (no intersection points are synthesised).
    ///
    /// In a production implementation this would invoke the OCCT `IntCurveSurface`
    /// machinery, walking the curve's parameter range and solving `C(t) ∈ S(u,v)`
    /// numerically.
    ///
    /// # Parameters
    ///
    /// - `curve`   — name of the curve.
    /// - `surface` — name of the surface.
    /// - `tol`     — positional tolerance for the solver.
    // occt: GeomAPI_IntCS::GeomAPI_IntCS(const Handle(Geom_Curve)&,
    //                                     const Handle(Geom_Surface)&,
    //                                     const Standard_Real)
    pub fn new(curve: &str, surface: &str, tol: f64) -> Self {
        Self {
            curve: curve.to_owned(),
            surface: surface.to_owned(),
            tolerance: tol,
            points: Vec::new(),
            done: true,
        }
    }

    /// `NbPoints()` — number of intersection points found.
    // occt: GeomAPI_IntCS::NbPoints
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// `Point(Index)` — return the 3D coordinates of the `i`-th intersection
    /// point (0-based index).  Returns `None` if `i` is out of range.
    ///
    /// In OCCT this is 1-based and returns a `gp_Pnt`; here it returns the
    /// `[f64; 3]` component of the stored tuple.
    // occt: GeomAPI_IntCS::Point(const Standard_Integer) — 1-based in OCCT
    pub fn point(&self, i: usize) -> Option<[f64; 3]> {
        self.points.get(i).map(|&(pt, _, _)| pt)
    }

    /// Return the curve parameter `t` at the `i`-th intersection (0-based).
    /// Returns `None` if `i` is out of range.
    // occt: GeomAPI_IntCS::Parameters — w component (curve param)
    pub fn curve_param(&self, i: usize) -> Option<f64> {
        self.points.get(i).map(|&(_, t, _)| t)
    }

    /// Return the surface UV parameters at the `i`-th intersection (0-based).
    /// Returns `None` if `i` is out of range.
    // occt: GeomAPI_IntCS::Parameters — u,v components (surface params)
    pub fn surface_params(&self, i: usize) -> Option<[f64; 2]> {
        self.points.get(i).map(|&(_, _, uv)| uv)
    }

    /// `IsDone()` — `true` if the intersection has been computed successfully.
    // occt: GeomAPI_IntCS::IsDone
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// surface_surface_intersect — convenience free function
// ─────────────────────────────────────────────────────────────────────────────

/// Convenience wrapper: compute the intersection polylines between two named
/// surfaces `s1` and `s2` at the given positional tolerance `tol`.
///
/// Constructs an [`IntSS`], and returns a clone of the `lines` field.  In the
/// stub this is always an empty `Vec` because no intersection curves are
/// synthesised.
///
/// In a production implementation the returned vectors would each hold an
/// ordered sequence of 3D sample points approximating one intersection curve.
///
/// # Parameters
///
/// - `s1`  — name of the first surface.
/// - `s2`  — name of the second surface.
/// - `tol` — positional tolerance for the solver.
// occt: GeomAPI_IntSS — single-result convenience accessor
pub fn surface_surface_intersect(s1: &str, s2: &str, tol: f64) -> Vec<Vec<[f64; 3]>> {
    let iss = IntSS::new(s1, s2, tol);
    iss.lines.clone()
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ─── IntSS ───────────────────────────────────────────────────────────────

    #[test]
    fn intss_is_done_on_construction() {
        let iss = IntSS::new("S1", "S2", 1e-7);
        assert!(iss.is_done());
    }

    #[test]
    fn intss_nb_lines_zero_for_stub() {
        let iss = IntSS::new("Plane", "Sphere", 1e-7);
        assert_eq!(iss.nb_lines(), 0);
    }

    #[test]
    fn intss_line_out_of_range_returns_none() {
        let iss = IntSS::new("S1", "S2", 1e-7);
        assert!(iss.line(0).is_none());
        assert!(iss.line(99).is_none());
    }

    #[test]
    fn intss_stores_surface_names_and_tolerance() {
        let iss = IntSS::new("CylinderA", "TorusB", 1e-6);
        assert_eq!(iss.s1, "CylinderA");
        assert_eq!(iss.s2, "TorusB");
        assert!((iss.tolerance - 1e-6).abs() < 1e-15);
    }

    #[test]
    fn intss_line_returns_correct_entry_when_populated() {
        let mut iss = IntSS::new("S1", "S2", 1e-7);
        // Manually inject a synthetic line to verify the accessor.
        let poly = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]];
        iss.lines.push(poly.clone());
        assert_eq!(iss.nb_lines(), 1);
        let retrieved = iss.line(0);
        assert!(retrieved.is_some());
        assert_eq!(*retrieved.unwrap(), poly);
    }

    #[test]
    fn intss_line_index_one_out_of_bounds_is_none_after_one_entry() {
        let mut iss = IntSS::new("S1", "S2", 1e-7);
        iss.lines.push(vec![[0.0, 0.0, 0.0]]);
        assert!(iss.line(1).is_none());
    }

    #[test]
    fn intss_multiple_lines_accessible_by_index() {
        let mut iss = IntSS::new("S1", "S2", 1e-7);
        iss.lines.push(vec![[0.0, 0.0, 0.0]]);
        iss.lines.push(vec![[1.0, 1.0, 1.0], [2.0, 2.0, 2.0]]);
        assert_eq!(iss.nb_lines(), 2);
        assert_eq!(iss.line(1).unwrap().len(), 2);
    }

    // ─── IntCS ───────────────────────────────────────────────────────────────

    #[test]
    fn intcs_is_done_on_construction() {
        let ics = IntCS::new("Line1", "Plane1", 1e-7);
        assert!(ics.is_done());
    }

    #[test]
    fn intcs_nb_points_zero_for_stub() {
        let ics = IntCS::new("Curve", "Surface", 1e-7);
        assert_eq!(ics.nb_points(), 0);
    }

    #[test]
    fn intcs_point_out_of_range_returns_none() {
        let ics = IntCS::new("C", "S", 1e-7);
        assert!(ics.point(0).is_none());
        assert!(ics.point(42).is_none());
    }

    #[test]
    fn intcs_stores_curve_surface_names_and_tolerance() {
        let ics = IntCS::new("BSplineCurve", "NurbsSurface", 1e-5);
        assert_eq!(ics.curve, "BSplineCurve");
        assert_eq!(ics.surface, "NurbsSurface");
        assert!((ics.tolerance - 1e-5).abs() < 1e-15);
    }

    #[test]
    fn intcs_point_returns_3d_coordinates_when_populated() {
        let mut ics = IntCS::new("C", "S", 1e-7);
        ics.points.push(([3.0, 4.0, 5.0], 0.5, [0.1, 0.2]));
        let pt = ics.point(0);
        assert!(pt.is_some());
        assert_eq!(pt.unwrap(), [3.0, 4.0, 5.0]);
    }

    #[test]
    fn intcs_curve_param_returned_correctly() {
        let mut ics = IntCS::new("C", "S", 1e-7);
        ics.points.push(([0.0, 0.0, 0.0], 0.75, [0.0, 0.0]));
        let t = ics.curve_param(0);
        assert!(t.is_some());
        assert!((t.unwrap() - 0.75).abs() < 1e-15);
    }

    #[test]
    fn intcs_surface_params_returned_correctly() {
        let mut ics = IntCS::new("C", "S", 1e-7);
        ics.points.push(([0.0, 0.0, 0.0], 0.0, [0.3, 0.8]));
        let uv = ics.surface_params(0);
        assert!(uv.is_some());
        assert_eq!(uv.unwrap(), [0.3, 0.8]);
    }

    #[test]
    fn intcs_curve_param_out_of_range_returns_none() {
        let ics = IntCS::new("C", "S", 1e-7);
        assert!(ics.curve_param(0).is_none());
    }

    #[test]
    fn intcs_surface_params_out_of_range_returns_none() {
        let ics = IntCS::new("C", "S", 1e-7);
        assert!(ics.surface_params(0).is_none());
    }

    #[test]
    fn intcs_multiple_points_accessible_by_index() {
        let mut ics = IntCS::new("C", "S", 1e-7);
        ics.points.push(([1.0, 0.0, 0.0], 0.0, [0.0, 0.0]));
        ics.points.push(([0.0, 1.0, 0.0], 1.0, [0.5, 0.5]));
        assert_eq!(ics.nb_points(), 2);
        assert_eq!(ics.point(1).unwrap(), [0.0, 1.0, 0.0]);
        assert!((ics.curve_param(1).unwrap() - 1.0).abs() < 1e-15);
    }

    // ─── surface_surface_intersect ────────────────────────────────────────────

    #[test]
    fn surface_surface_intersect_returns_empty_for_stub() {
        let lines = surface_surface_intersect("Plane", "Sphere", 1e-7);
        assert!(lines.is_empty());
    }

    #[test]
    fn surface_surface_intersect_accepts_any_names() {
        let lines = surface_surface_intersect("", "TorusXYZ", 1e-4);
        assert!(lines.is_empty());
    }

    #[test]
    fn surface_surface_intersect_zero_tolerance() {
        // Must not panic for edge-case tolerance values.
        let lines = surface_surface_intersect("S1", "S2", 0.0);
        assert!(lines.is_empty());
    }

    #[test]
    fn surface_surface_intersect_large_tolerance() {
        let lines = surface_surface_intersect("S1", "S2", f64::MAX);
        assert!(lines.is_empty());
    }
}

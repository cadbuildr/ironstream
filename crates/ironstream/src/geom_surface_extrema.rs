// FILE: rust/ironstream/crates/ironstream/src/geom_surface_extrema.rs
//! `Extrema_ExtPS` / `Extrema_ExtCS` — extrema (minimum-distance) algorithms
//! between a point or curve and a surface, mirroring OpenCascade's `Extrema`
//! package (`src/ModelingAlgorithms/TKGeomAlgo/Extrema`).
//!
//! These are **stub** implementations: the `perform` methods mark the
//! computation as done and return synthetic results (distance = 1.0, UV = 0.0)
//! so the full public API surface is available for compilation and testing
//! without requiring a numerical solver.
//!
//! Covered types:
//! - [`ExtremaPtSurface`]       — `Extrema_ExtPS`: extrema between a 3D point
//!                                and a parametric surface.
//! - [`ExtremaCurveSurface`]    — `Extrema_ExtCS`: extrema between a 3D curve
//!                                and a parametric surface.
//!
//! Free functions:
//! - [`nearest_point_on_surface`] — convenience wrapper returning the single
//!                                  closest (U, V) pair and distance.

// ─────────────────────────────────────────────────────────────────────────────
// ExtremaPtSurface
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `Extrema_ExtPS` — computes all extrema (minimum or maximum
/// distance foot-points) between a 3D point and a parametric surface.
///
/// Each solution is a pair `([u, v], distance)` where `[u, v]` are the surface
/// parameter values at the foot-point and `distance` is the Euclidean distance
/// from the input point to that foot-point.
///
/// The `perform` method is a stub that injects one synthetic solution
/// `([0.0, 0.0], 1.0)` and sets `done = true`.  In a production implementation
/// `perform` would run a multi-start Newton solver over the surface parameter
/// domain.
// occt: Extrema_ExtPS
#[derive(Clone, Debug)]
pub struct ExtremaPtSurface {
    /// The query point in 3D space.
    pub point: [f64; 3],
    /// Name / tag of the target surface (used as a lightweight handle in this
    /// stub; a real implementation would hold a `Handle(Geom_Surface)`).
    pub surface: String,
    /// All extrema solutions found: each entry is `([u, v], distance)`.
    pub solutions: Vec<([f64; 2], f64)>,
    /// `true` if `perform` has been called successfully.
    pub done: bool,
    /// Tolerance used by the solver (stored for informational purposes).
    _tol: f64,
}

impl ExtremaPtSurface {
    /// Construct a new, un-performed extrema computer.
    ///
    /// - `pt`      — the 3D query point.
    /// - `surface` — name / tag of the target surface.
    /// - `tol`     — solver tolerance (stored but unused in the stub).
    ///
    /// Call [`perform`](Self::perform) to run the computation.
    // occt: Extrema_ExtPS // ::Extrema_ExtPS(const gp_Pnt&, const Adaptor3d_Surface&, …)
    pub fn new(pt: [f64; 3], surface: &str, tol: f64) -> Self {
        Self {
            point: pt,
            surface: surface.to_owned(),
            solutions: Vec::new(),
            done: false,
            _tol: tol,
        }
    }

    /// Run the extrema computation (stub).
    ///
    /// Injects one synthetic solution `([0.0, 0.0], 1.0)` and marks the
    /// computation as done.  A real implementation would solve the system
    /// `∂/∂u ||S(u,v) - P||² = 0`, `∂/∂v ||S(u,v) - P||² = 0` numerically.
    // occt: Extrema_ExtPS // ::Perform
    pub fn perform(&mut self) {
        self.solutions.clear();
        self.solutions.push(([0.0, 0.0], 1.0));
        self.done = true;
    }

    /// `NbExt()` — total number of extrema solutions found.
    // occt: Extrema_ExtPS // ::NbExt
    pub fn nb_ext(&self) -> usize {
        self.solutions.len()
    }

    /// `Point(Index)` — return the surface parameter pair `[u, v]` for the
    /// `i`-th solution (0-based).  Returns `None` if out of range.
    // occt: Extrema_ExtPS // ::Point(Standard_Integer) — 1-based in OCCT
    pub fn point(&self, i: usize) -> Option<[f64; 2]> {
        self.solutions.get(i).map(|&(uv, _)| uv)
    }

    /// `Distance(Index)` — return the distance for the `i`-th solution
    /// (0-based).  Returns `None` if out of range.
    // occt: Extrema_ExtPS // ::SquareDistance(Standard_Integer) — OCCT returns d²
    pub fn distance(&self, i: usize) -> Option<f64> {
        self.solutions.get(i).map(|&(_, d)| d)
    }

    /// `IsDone()` — `true` if `perform` has been called successfully.
    // occt: Extrema_ExtPS // ::IsDone
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ExtremaCurveSurface
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `Extrema_ExtCS` — computes all extrema (minimum or maximum
/// distance pairs) between a 3D curve and a parametric surface.
///
/// Each solution is a pair `(t, [u, v])` where `t` is the curve parameter at
/// the foot-point and `[u, v]` are the surface parameters at the corresponding
/// foot-point.
///
/// The `perform` method is a stub that injects one synthetic solution
/// `(0.0, [0.0, 0.0])` and marks the computation as done.
// occt-ref: Extrema_ExtCS
#[derive(Clone, Debug)]
pub struct ExtremaCurveSurface {
    /// Name / tag of the source curve.
    pub curve: String,
    /// Name / tag of the target surface.
    pub surface: String,
    /// All extrema solutions found: each entry is `(curve_param, [u, v])`.
    pub solutions: Vec<(f64, [f64; 2])>,
    /// `true` if `perform` has been called successfully.
    done: bool,
    /// Tolerance used by the solver (stored for informational purposes).
    _tol: f64,
}

impl ExtremaCurveSurface {
    /// Construct a new, un-performed extrema computer.
    ///
    /// - `curve`   — name / tag of the source curve.
    /// - `surface` — name / tag of the target surface.
    /// - `tol`     — solver tolerance (stored but unused in the stub).
    ///
    /// Call [`perform`](Self::perform) to run the computation.
    // occt-ref: Extrema_ExtCS // ::Extrema_ExtCS(const Adaptor3d_Curve&, const Adaptor3d_Surface&, …)
    pub fn new(curve: &str, surface: &str, tol: f64) -> Self {
        Self {
            curve: curve.to_owned(),
            surface: surface.to_owned(),
            solutions: Vec::new(),
            done: false,
            _tol: tol,
        }
    }

    /// Run the extrema computation (stub).
    ///
    /// Injects one synthetic solution `(0.0, [0.0, 0.0])` and marks the
    /// computation as done.  A real implementation would enumerate candidate
    /// intervals, evaluate the Jacobian of the distance function, and apply
    /// Newton iterations in the `(t, u, v)` parameter space.
    // occt-ref: Extrema_ExtCS // ::Perform
    pub fn perform(&mut self) {
        self.solutions.clear();
        self.solutions.push((0.0, [0.0, 0.0]));
        self.done = true;
    }

    /// `NbExt()` — total number of extrema solutions found.
    // occt-ref: Extrema_ExtCS // ::NbExt
    pub fn nb_ext(&self) -> usize {
        self.solutions.len()
    }

    /// `IsDone()` — `true` if `perform` has been called successfully.
    // occt-ref: Extrema_ExtCS // ::IsDone
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// nearest_point_on_surface
// ─────────────────────────────────────────────────────────────────────────────

/// Convenience wrapper: find the nearest point on `surface` to `pt`.
///
/// Constructs an [`ExtremaPtSurface`], calls `perform`, and returns the first
/// solution's `([u, v], distance)` pair.  In the stub this is always
/// `([0.0, 0.0], 1.0)`.
///
/// In a real implementation the returned `[u, v]` pair would be the surface
/// parameters of the closest foot-point, and `distance` would be the true
/// Euclidean distance.
///
/// # Panics
///
/// Panics if `perform` did not produce any solutions (should not happen with
/// the stub implementation, but could happen if a real solver fails to
/// converge).
// occt-ref: GeomAPI_ProjectPointOnSurf // — single-result convenience accessor
pub fn nearest_point_on_surface(pt: [f64; 3], surface: &str) -> ([f64; 2], f64) {
    let mut ext = ExtremaPtSurface::new(pt, surface, 1.0e-7);
    ext.perform();
    // Return the solution with the smallest distance.
    ext.solutions
        .iter()
        .copied()
        .reduce(|a, b| if a.1 <= b.1 { a } else { b })
        .expect("perform must produce at least one solution")
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ─── ExtremaPtSurface ─────────────────────────────────────────────────────

    #[test]
    fn ext_ps_not_done_before_perform() {
        let ext = ExtremaPtSurface::new([1.0, 2.0, 3.0], "MyPlane", 1e-6);
        assert!(!ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn ext_ps_done_after_perform() {
        let mut ext = ExtremaPtSurface::new([0.0, 0.0, 5.0], "MySphere", 1e-6);
        ext.perform();
        assert!(ext.is_done());
        assert_eq!(ext.nb_ext(), 1);
    }

    #[test]
    fn ext_ps_point_returns_uv() {
        let mut ext = ExtremaPtSurface::new([1.0, 0.0, 0.0], "MyCylinder", 1e-6);
        ext.perform();
        let uv = ext.point(0);
        assert!(uv.is_some());
        let [u, v] = uv.unwrap();
        assert_eq!(u, 0.0);
        assert_eq!(v, 0.0);
    }

    #[test]
    fn ext_ps_point_out_of_range_returns_none() {
        let mut ext = ExtremaPtSurface::new([0.0, 0.0, 0.0], "S", 1e-6);
        ext.perform();
        assert!(ext.point(1).is_none());
    }

    #[test]
    fn ext_ps_distance_returns_value() {
        let mut ext = ExtremaPtSurface::new([0.0, 0.0, 1.0], "S", 1e-6);
        ext.perform();
        let d = ext.distance(0);
        assert!(d.is_some());
        assert_eq!(d.unwrap(), 1.0);
    }

    #[test]
    fn ext_ps_distance_out_of_range_returns_none() {
        let mut ext = ExtremaPtSurface::new([0.0, 0.0, 0.0], "S", 1e-6);
        ext.perform();
        assert!(ext.distance(5).is_none());
    }

    #[test]
    fn ext_ps_perform_idempotent() {
        let mut ext = ExtremaPtSurface::new([0.0, 0.0, 0.0], "S", 1e-6);
        ext.perform();
        ext.perform();
        // Solutions must be reset — still exactly one entry.
        assert_eq!(ext.nb_ext(), 1);
    }

    // ─── ExtremaCurveSurface ──────────────────────────────────────────────────

    #[test]
    fn ext_cs_not_done_before_perform() {
        let ext = ExtremaCurveSurface::new("MyCurve", "MySurface", 1e-6);
        assert!(!ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn ext_cs_done_after_perform() {
        let mut ext = ExtremaCurveSurface::new("Line", "Plane", 1e-6);
        ext.perform();
        assert!(ext.is_done());
        assert_eq!(ext.nb_ext(), 1);
    }

    #[test]
    fn ext_cs_solution_content() {
        let mut ext = ExtremaCurveSurface::new("C", "S", 1e-6);
        ext.perform();
        let (t, uv) = ext.solutions[0];
        assert_eq!(t, 0.0);
        assert_eq!(uv, [0.0, 0.0]);
    }

    #[test]
    fn ext_cs_perform_idempotent() {
        let mut ext = ExtremaCurveSurface::new("C", "S", 1e-6);
        ext.perform();
        ext.perform();
        assert_eq!(ext.nb_ext(), 1);
    }

    #[test]
    fn ext_cs_stores_names() {
        let ext = ExtremaCurveSurface::new("BSplineCurve", "TorusSurface", 1e-7);
        assert_eq!(ext.curve, "BSplineCurve");
        assert_eq!(ext.surface, "TorusSurface");
    }

    // ─── nearest_point_on_surface ─────────────────────────────────────────────

    #[test]
    fn nearest_point_returns_stub_result() {
        let (uv, dist) = nearest_point_on_surface([3.0, 4.0, 0.0], "Plane");
        assert_eq!(uv, [0.0, 0.0]);
        assert_eq!(dist, 1.0);
    }

    #[test]
    fn nearest_point_does_not_panic() {
        // Any input should work without panicking.
        let _ = nearest_point_on_surface([f64::MAX, -1.0, 0.0], "S");
    }
}

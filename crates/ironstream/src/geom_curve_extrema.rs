// FILE: rust/ironstream/crates/ironstream/src/geom_curve_extrema.rs
//! `Extrema_ExtPC` / `Extrema_ExtCC` — extrema (minimum-distance) algorithms
//! between a point or curve and a curve, mirroring OpenCascade's `Extrema`
//! package (`src/ModelingAlgorithms/TKGeomAlgo/Extrema`).
//!
//! These are **stub** implementations: the `perform` methods mark the
//! computation as done and inject synthetic results (distance = 1.0,
//! parameter = 0.0) so the full public API surface is available for
//! compilation and testing without requiring a numerical solver.
//!
//! Covered types:
//! - [`ExtremaPtCurve`]        — `Extrema_ExtPC`: extrema between a 3D point
//!                               and a parametric curve.
//! - [`ExtremaCurveCurve`]     — `Extrema_ExtCC`: extrema between two
//!                               parametric curves.
//!
//! Free functions:
//! - [`nearest_point_on_curve`] — convenience wrapper returning the single
//!                                closest parameter and 3D foot-point.

// ─────────────────────────────────────────────────────────────────────────────
// ExtremaPtCurve
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `Extrema_ExtPC` — computes all extrema (minimum or maximum
/// distance foot-points) between a 3D point and a parametric curve.
///
/// Each solution stored in `solutions` is a pair `(t, distance)` where `t` is
/// the curve parameter at the foot-point and `distance` is the Euclidean
/// distance from the input point to that foot-point.
///
/// The `perform` method is a stub that injects one synthetic solution
/// `(0.0, 1.0)` and sets `done = true`.  In a production implementation
/// `perform` would solve `d/dt ||C(t) - P||² = 0` numerically (e.g. via
/// Brent's method seeded by a uniform parameter sweep).
// occt: Extrema_ExtPC
#[derive(Clone, Debug)]
pub struct ExtremaPtCurve {
    /// The query point in 3D space.
    pub point: [f64; 3],
    /// Name / tag of the target curve (used as a lightweight handle in this
    /// stub; a real implementation would hold a `Handle(Geom_Curve)` or an
    /// `Adaptor3d_Curve`).
    pub curve: String,
    /// All extrema solutions found: each entry is `(curve_parameter, distance)`.
    pub solutions: Vec<(f64, f64)>,
    /// `true` if `perform` has been called successfully.
    pub done: bool,
    /// Tolerance used by the solver (stored for informational purposes).
    _tol: f64,
}

impl ExtremaPtCurve {
    /// Construct a new, un-performed extrema computer.
    ///
    /// - `pt`    — the 3D query point.
    /// - `curve` — name / tag of the target curve.
    /// - `tol`   — solver tolerance (stored but unused in the stub).
    ///
    /// Call [`perform`](Self::perform) to run the computation.
    // occt: Extrema_ExtPC::Extrema_ExtPC(const gp_Pnt&, const Adaptor3d_Curve&, …)
    pub fn new(pt: [f64; 3], curve: &str, tol: f64) -> Self {
        Self {
            point: pt,
            curve: curve.to_owned(),
            solutions: Vec::new(),
            done: false,
            _tol: tol,
        }
    }

    /// Run the extrema computation (stub).
    ///
    /// Clears any previous solutions, injects one synthetic solution
    /// `(0.0, 1.0)` (parameter = 0.0, distance = 1.0), and marks the
    /// computation as done.
    ///
    /// A real implementation would evaluate `C(t)` over a uniform parameter
    /// grid, identify local minima of `||C(t) - P||`, then refine each
    /// candidate with a one-dimensional golden-section or Brent search.
    // occt: Extrema_ExtPC::Perform
    pub fn perform(&mut self) {
        self.solutions.clear();
        self.solutions.push((0.0, 1.0));
        self.done = true;
    }

    /// `NbExt()` — total number of extrema solutions found.
    // occt: Extrema_ExtPC::NbExt
    pub fn nb_ext(&self) -> usize {
        self.solutions.len()
    }

    /// `Parameter(Index)` — return the curve parameter `t` for the `i`-th
    /// solution (0-based).  Returns `None` if `i` is out of range.
    // occt: Extrema_ExtPC::Point(Standard_Integer) — 1-based in OCCT
    pub fn parameter(&self, i: usize) -> Option<f64> {
        self.solutions.get(i).map(|&(t, _)| t)
    }

    /// `Distance(Index)` — return the Euclidean distance for the `i`-th
    /// solution (0-based).  Returns `None` if `i` is out of range.
    // occt: Extrema_ExtPC::SquareDistance(Standard_Integer) — OCCT returns d²
    pub fn distance(&self, i: usize) -> Option<f64> {
        self.solutions.get(i).map(|&(_, d)| d)
    }

    /// `Point(Index)` — return the 3D foot-point for the `i`-th solution
    /// (0-based).  In the stub this is always `[0.0, 0.0, 0.0]`.  Returns
    /// `None` if `i` is out of range.
    ///
    /// In a real implementation the returned point would be `C(t_i)`.
    // occt: Extrema_ExtPC::Point(Standard_Integer) — position component
    pub fn point(&self, i: usize) -> Option<[f64; 3]> {
        self.solutions.get(i).map(|_| [0.0, 0.0, 0.0])
    }

    /// `IsDone()` — `true` if `perform` has been called successfully.
    // occt: Extrema_ExtPC::IsDone
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ExtremaCurveCurve
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for `Extrema_ExtCC` — computes all extrema (minimum or maximum
/// distance point pairs) between two parametric 3D curves.
///
/// Each solution stored in `solutions` is a triple
/// `(distance, t1, t2)` where `t1` and `t2` are the parameter values on
/// `curve1` and `curve2` respectively at the closest/farthest point pair.
///
/// The `perform` method is a stub that injects one synthetic solution
/// `(1.0, 0.0, 0.0)` and marks the computation as done.  In a production
/// implementation `perform` would solve the system
/// `∂/∂t1 ||C1(t1) - C2(t2)||² = 0`, `∂/∂t2 ||C1(t1) - C2(t2)||² = 0`
/// numerically via a grid sweep followed by coordinate-descent refinement.
// occt: Extrema_ExtCC
#[derive(Clone, Debug)]
pub struct ExtremaCurveCurve {
    /// Name / tag of the first curve.
    pub curve1: String,
    /// Name / tag of the second curve.
    pub curve2: String,
    /// All extrema solutions found: each entry is `(distance, t1, t2)`.
    pub solutions: Vec<(f64, f64, f64)>,
    /// `true` if `perform` has been called successfully.
    done: bool,
    /// Tolerance used by the solver (stored for informational purposes).
    _tol: f64,
}

impl ExtremaCurveCurve {
    /// Construct a new, un-performed extrema computer.
    ///
    /// - `c1`  — name / tag of the first curve.
    /// - `c2`  — name / tag of the second curve.
    /// - `tol` — solver tolerance (stored but unused in the stub).
    ///
    /// Call [`perform`](Self::perform) to run the computation.
    // occt: Extrema_ExtCC::Extrema_ExtCC(const Adaptor3d_Curve&, const Adaptor3d_Curve&, …)
    pub fn new(c1: &str, c2: &str, tol: f64) -> Self {
        Self {
            curve1: c1.to_owned(),
            curve2: c2.to_owned(),
            solutions: Vec::new(),
            done: false,
            _tol: tol,
        }
    }

    /// Run the extrema computation (stub).
    ///
    /// Clears any previous solutions, injects one synthetic solution
    /// `(1.0, 0.0, 0.0)` (distance = 1.0, t1 = 0.0, t2 = 0.0), and marks
    /// the computation as done.
    ///
    /// A real implementation would sample both curve parameter ranges on a
    /// 2-D grid, locate all local-minimum cells, and refine each candidate
    /// with alternating golden-section searches along `t1` and `t2`.
    // occt: Extrema_ExtCC::Perform
    pub fn perform(&mut self) {
        self.solutions.clear();
        self.solutions.push((1.0, 0.0, 0.0));
        self.done = true;
    }

    /// `NbExt()` — total number of extrema solutions found.
    // occt: Extrema_ExtCC::NbExt
    pub fn nb_ext(&self) -> usize {
        self.solutions.len()
    }

    /// `IsDone()` — `true` if `perform` has been called successfully.
    // occt: Extrema_ExtCC::IsDone
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// nearest_point_on_curve
// ─────────────────────────────────────────────────────────────────────────────

/// Convenience wrapper: find the nearest point on `curve` to `pt`.
///
/// Constructs an [`ExtremaPtCurve`], calls `perform`, and returns the
/// parameter `t` and the 3D foot-point of the closest solution.  In the stub
/// this is always `(0.0, [0.0, 0.0, 0.0])`.
///
/// In a real implementation the returned `t` would be the curve parameter of
/// the closest foot-point and the `[f64; 3]` the corresponding `C(t)`.
///
/// # Panics
///
/// Panics if `perform` did not produce any solutions (should not happen with
/// the stub implementation, but could happen if a real solver fails to
/// converge).
// occt: GeomAPI_ProjectPointOnCurve — single-result convenience accessor
pub fn nearest_point_on_curve(pt: [f64; 3], curve: &str) -> (f64, [f64; 3]) {
    let mut ext = ExtremaPtCurve::new(pt, curve, 1.0e-7);
    ext.perform();
    // Select the solution with the smallest distance.
    let best_idx = ext
        .solutions
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.1.partial_cmp(&b.1).unwrap())
        .map(|(i, _)| i)
        .expect("perform must produce at least one solution");
    let t = ext.parameter(best_idx).unwrap();
    let foot = ext.point(best_idx).unwrap();
    (t, foot)
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ─── ExtremaPtCurve ───────────────────────────────────────────────────────

    #[test]
    fn ext_pc_not_done_before_perform() {
        let ext = ExtremaPtCurve::new([1.0, 2.0, 3.0], "MyLine", 1e-6);
        assert!(!ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn ext_pc_done_after_perform() {
        let mut ext = ExtremaPtCurve::new([0.0, 0.0, 5.0], "MyCircle", 1e-6);
        ext.perform();
        assert!(ext.is_done());
        assert_eq!(ext.nb_ext(), 1);
    }

    #[test]
    fn ext_pc_parameter_returns_value() {
        let mut ext = ExtremaPtCurve::new([1.0, 0.0, 0.0], "BSpline", 1e-6);
        ext.perform();
        let t = ext.parameter(0);
        assert!(t.is_some());
        assert_eq!(t.unwrap(), 0.0);
    }

    #[test]
    fn ext_pc_parameter_out_of_range_returns_none() {
        let mut ext = ExtremaPtCurve::new([0.0, 0.0, 0.0], "C", 1e-6);
        ext.perform();
        assert!(ext.parameter(1).is_none());
    }

    #[test]
    fn ext_pc_distance_returns_value() {
        let mut ext = ExtremaPtCurve::new([0.0, 0.0, 1.0], "C", 1e-6);
        ext.perform();
        let d = ext.distance(0);
        assert!(d.is_some());
        assert_eq!(d.unwrap(), 1.0);
    }

    #[test]
    fn ext_pc_distance_out_of_range_returns_none() {
        let mut ext = ExtremaPtCurve::new([0.0, 0.0, 0.0], "C", 1e-6);
        ext.perform();
        assert!(ext.distance(5).is_none());
    }

    #[test]
    fn ext_pc_point_returns_foot() {
        let mut ext = ExtremaPtCurve::new([3.0, 4.0, 0.0], "C", 1e-6);
        ext.perform();
        let foot = ext.point(0);
        assert!(foot.is_some());
        assert_eq!(foot.unwrap(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn ext_pc_point_out_of_range_returns_none() {
        let mut ext = ExtremaPtCurve::new([0.0, 0.0, 0.0], "C", 1e-6);
        ext.perform();
        assert!(ext.point(2).is_none());
    }

    #[test]
    fn ext_pc_perform_idempotent() {
        let mut ext = ExtremaPtCurve::new([0.0, 0.0, 0.0], "C", 1e-6);
        ext.perform();
        ext.perform();
        // Solutions must be reset each time — still exactly one entry.
        assert_eq!(ext.nb_ext(), 1);
    }

    #[test]
    fn ext_pc_stores_point_and_curve_name() {
        let ext = ExtremaPtCurve::new([1.0, 2.0, 3.0], "Helix", 1e-7);
        assert_eq!(ext.point, [1.0, 2.0, 3.0]);
        assert_eq!(ext.curve, "Helix");
    }

    // ─── ExtremaCurveCurve ────────────────────────────────────────────────────

    #[test]
    fn ext_cc_not_done_before_perform() {
        let ext = ExtremaCurveCurve::new("MyCurve1", "MyCurve2", 1e-6);
        assert!(!ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn ext_cc_done_after_perform() {
        let mut ext = ExtremaCurveCurve::new("Line", "Circle", 1e-6);
        ext.perform();
        assert!(ext.is_done());
        assert_eq!(ext.nb_ext(), 1);
    }

    #[test]
    fn ext_cc_solution_content() {
        let mut ext = ExtremaCurveCurve::new("C1", "C2", 1e-6);
        ext.perform();
        let (dist, t1, t2) = ext.solutions[0];
        assert_eq!(dist, 1.0);
        assert_eq!(t1, 0.0);
        assert_eq!(t2, 0.0);
    }

    #[test]
    fn ext_cc_perform_idempotent() {
        let mut ext = ExtremaCurveCurve::new("C1", "C2", 1e-6);
        ext.perform();
        ext.perform();
        assert_eq!(ext.nb_ext(), 1);
    }

    #[test]
    fn ext_cc_stores_curve_names() {
        let ext = ExtremaCurveCurve::new("BSpline1", "BSpline2", 1e-7);
        assert_eq!(ext.curve1, "BSpline1");
        assert_eq!(ext.curve2, "BSpline2");
    }

    // ─── nearest_point_on_curve ───────────────────────────────────────────────

    #[test]
    fn nearest_point_returns_stub_result() {
        let (t, foot) = nearest_point_on_curve([3.0, 4.0, 0.0], "Line");
        assert_eq!(t, 0.0);
        assert_eq!(foot, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn nearest_point_does_not_panic() {
        // Any input must work without panicking.
        let _ = nearest_point_on_curve([f64::MAX, -1.0, 0.0], "C");
    }

    #[test]
    fn nearest_point_on_curve_nan_safe() {
        // NaN coordinates should not cause an unwind from safe Rust.
        let result = std::panic::catch_unwind(|| {
            nearest_point_on_curve([f64::NAN, 0.0, 0.0], "C")
        });
        // The stub never inspects the point value, so this must succeed.
        assert!(result.is_ok());
    }
}

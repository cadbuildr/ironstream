// FILE: rust/ironstream/crates/ironstream/src/geom_pnt_proj.rs
//! Point-on-curve and point-on-surface projection stubs.
//!
//! Covered types:
//! - [`ProjectOnCurve`]   — project a 3D point onto a named curve
//! - [`ProjectOnSurface`] — project a 3D point onto a named surface
//!
//! Both types model the public API of the corresponding OCCT classes while
//! remaining zero-dependency pure-Rust stubs.

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Euclidean distance between two 3D points.
#[inline]
fn dist3(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// ─────────────────────────────────────────────────────────────────────────────
// ProjectOnCurve
// ─────────────────────────────────────────────────────────────────────────────

/// Project a 3D point onto a named parametric curve and collect all foot
/// points together with their curve parameters and distances.
///
/// Each solution is stored as `(parameter, distance)`.
///
/// This is a stub: [`new`](Self::new) immediately populates a single solution
/// at parameter `0.5` with a placeholder distance of `1.0` and marks the
/// computation as done.  The nearest-point foot is set equal to the query
/// point as a placeholder.
// occt: GeomAPI_ProjectPointOnCurve
#[derive(Clone, Debug)]
pub struct ProjectOnCurve {
    /// The query point to project.
    pub point: [f64; 3],
    /// An identifier for the curve being projected onto.
    pub curve: String,
    /// All solutions found, each stored as `(curve_parameter, distance)`.
    pub solutions: Vec<(f64, f64)>,
    /// Whether the projection computation succeeded.
    pub done: bool,
}

impl ProjectOnCurve {
    /// Construct a new projector and immediately run the stub algorithm.
    ///
    /// After construction [`is_done`](Self::is_done) returns `true` and
    /// [`nb_points`](Self::nb_points) returns `1`.
    pub fn new(pt: [f64; 3], curve: &str) -> Self {
        Self {
            point: pt,
            curve: curve.to_string(),
            // Stub: single solution at parameter 0.5, distance 1.0.
            solutions: vec![(0.5, 1.0)],
            done: true,
        }
    }

    /// Return the number of projection solutions found.
    pub fn nb_points(&self) -> usize {
        self.solutions.len()
    }

    /// Return the foot point of the nearest projection.
    ///
    /// Stub: returns the query point itself as the foot point.
    pub fn nearest_point(&self) -> [f64; 3] {
        self.point
    }

    /// Return the curve parameter of the solution with the smallest distance.
    ///
    /// Returns `0.0` when there are no solutions.
    pub fn lower_distance_parameter(&self) -> f64 {
        self.solutions
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|s| s.0)
            .unwrap_or(0.0)
    }

    /// Return the smallest distance over all solutions.
    ///
    /// Returns `f64::MAX` when there are no solutions.
    pub fn lower_distance(&self) -> f64 {
        self.solutions
            .iter()
            .map(|s| s.1)
            .fold(f64::MAX, f64::min)
    }

    /// Return `true` if the projection computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ProjectOnSurface
// ─────────────────────────────────────────────────────────────────────────────

/// Project a 3D point onto a named parametric surface and collect all foot
/// points together with their `(u, v)` surface parameters and distances.
///
/// Each solution is stored as `([u, v], distance)`.
///
/// This is a stub: [`new`](Self::new) immediately populates a single solution
/// at `(u, v) = (0.0, 0.0)` with a placeholder distance of `1.0` and marks
/// the computation as done.  The nearest-point foot is set equal to the query
/// point as a placeholder.
// occt: GeomAPI_ProjectPointOnSurf
#[derive(Clone, Debug)]
pub struct ProjectOnSurface {
    /// The query point to project.
    pub point: [f64; 3],
    /// An identifier for the surface being projected onto.
    pub surface: String,
    /// All solutions found, each stored as `([u, v], distance)`.
    pub solutions: Vec<([f64; 2], f64)>,
    /// Whether the projection computation succeeded.
    pub done: bool,
}

impl ProjectOnSurface {
    /// Construct a new projector and immediately run the stub algorithm.
    ///
    /// After construction [`is_done`](Self::is_done) returns `true` and
    /// [`nb_points`](Self::nb_points) returns `1`.
    pub fn new(pt: [f64; 3], surface: &str) -> Self {
        Self {
            point: pt,
            surface: surface.to_string(),
            // Stub: single solution at (u=0.0, v=0.0), distance 1.0.
            solutions: vec![([0.0, 0.0], 1.0)],
            done: true,
        }
    }

    /// Return the number of projection solutions found.
    pub fn nb_points(&self) -> usize {
        self.solutions.len()
    }

    /// Return the foot point of the nearest projection.
    ///
    /// Stub: returns the query point itself as the foot point.
    pub fn nearest_point(&self) -> [f64; 3] {
        self.point
    }

    /// Return the `[u, v]` parameters of the solution with the smallest
    /// distance.
    ///
    /// Returns `[0.0, 0.0]` when there are no solutions.
    pub fn lower_distance_parameter(&self) -> [f64; 2] {
        self.solutions
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|s| s.0)
            .unwrap_or([0.0, 0.0])
    }

    /// Return the smallest distance over all solutions.
    ///
    /// Returns `f64::MAX` when there are no solutions.
    pub fn lower_distance(&self) -> f64 {
        self.solutions
            .iter()
            .map(|s| s.1)
            .fold(f64::MAX, f64::min)
    }

    /// Return `true` if the projection computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free function
// ─────────────────────────────────────────────────────────────────────────────

/// Project `pt` onto `curve` and return the nearest curve parameter together
/// with the foot point.
///
/// This is a convenience wrapper around [`ProjectOnCurve`].  The foot point
/// is computed as `pt + (parameter - 0.5) * unit_vector` as a placeholder;
/// in a real implementation it would evaluate the curve at the returned
/// parameter.
///
/// Returns `(nearest_parameter, foot_point)`.
pub fn project_point_on_curve(pt: [f64; 3], curve: &str) -> (f64, [f64; 3]) {
    let proj = ProjectOnCurve::new(pt, curve);
    let param = proj.lower_distance_parameter();
    let foot = proj.nearest_point();
    (param, foot)
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── ProjectOnCurve ────────────────────────────────────────────────────────

    #[test]
    fn curve_new_is_done() {
        let proj = ProjectOnCurve::new([1.0, 2.0, 3.0], "line");
        assert!(proj.is_done());
    }

    #[test]
    fn curve_nb_points_is_one() {
        let proj = ProjectOnCurve::new([0.0, 0.0, 0.0], "circle");
        assert_eq!(proj.nb_points(), 1);
    }

    #[test]
    fn curve_nearest_point_equals_query() {
        let pt = [3.0, 4.0, 5.0];
        let proj = ProjectOnCurve::new(pt, "spline");
        assert_eq!(proj.nearest_point(), pt);
    }

    #[test]
    fn curve_lower_distance_parameter_stub() {
        let proj = ProjectOnCurve::new([0.0, 0.0, 0.0], "line");
        assert_eq!(proj.lower_distance_parameter(), 0.5);
    }

    #[test]
    fn curve_lower_distance_stub() {
        let proj = ProjectOnCurve::new([0.0, 0.0, 0.0], "line");
        assert_eq!(proj.lower_distance(), 1.0);
    }

    #[test]
    fn curve_no_solutions_lower_distance_is_max() {
        let mut proj = ProjectOnCurve::new([0.0, 0.0, 0.0], "line");
        proj.solutions.clear();
        assert_eq!(proj.lower_distance(), f64::MAX);
    }

    #[test]
    fn curve_no_solutions_lower_distance_parameter_is_zero() {
        let mut proj = ProjectOnCurve::new([0.0, 0.0, 0.0], "line");
        proj.solutions.clear();
        assert_eq!(proj.lower_distance_parameter(), 0.0);
    }

    #[test]
    fn curve_stores_curve_name() {
        let proj = ProjectOnCurve::new([0.0, 0.0, 0.0], "my_curve");
        assert_eq!(proj.curve, "my_curve");
    }

    // ── ProjectOnSurface ──────────────────────────────────────────────────────

    #[test]
    fn surf_new_is_done() {
        let proj = ProjectOnSurface::new([1.0, 2.0, 3.0], "plane");
        assert!(proj.is_done());
    }

    #[test]
    fn surf_nb_points_is_one() {
        let proj = ProjectOnSurface::new([0.0, 0.0, 0.0], "sphere");
        assert_eq!(proj.nb_points(), 1);
    }

    #[test]
    fn surf_nearest_point_equals_query() {
        let pt = [1.0, 2.0, 3.0];
        let proj = ProjectOnSurface::new(pt, "torus");
        assert_eq!(proj.nearest_point(), pt);
    }

    #[test]
    fn surf_lower_distance_parameter_stub() {
        let proj = ProjectOnSurface::new([0.0, 0.0, 0.0], "plane");
        assert_eq!(proj.lower_distance_parameter(), [0.0, 0.0]);
    }

    #[test]
    fn surf_lower_distance_stub() {
        let proj = ProjectOnSurface::new([0.0, 0.0, 0.0], "plane");
        assert_eq!(proj.lower_distance(), 1.0);
    }

    #[test]
    fn surf_no_solutions_lower_distance_is_max() {
        let mut proj = ProjectOnSurface::new([0.0, 0.0, 0.0], "plane");
        proj.solutions.clear();
        assert_eq!(proj.lower_distance(), f64::MAX);
    }

    #[test]
    fn surf_no_solutions_lower_distance_parameter_is_zero() {
        let mut proj = ProjectOnSurface::new([0.0, 0.0, 0.0], "plane");
        proj.solutions.clear();
        assert_eq!(proj.lower_distance_parameter(), [0.0, 0.0]);
    }

    #[test]
    fn surf_stores_surface_name() {
        let proj = ProjectOnSurface::new([0.0, 0.0, 0.0], "my_surface");
        assert_eq!(proj.surface, "my_surface");
    }

    // ── project_point_on_curve free function ──────────────────────────────────

    #[test]
    fn free_fn_returns_stub_parameter() {
        let (param, _foot) = project_point_on_curve([1.0, 2.0, 3.0], "line");
        assert_eq!(param, 0.5);
    }

    #[test]
    fn free_fn_returns_query_point_as_foot() {
        let pt = [7.0, 8.0, 9.0];
        let (_param, foot) = project_point_on_curve(pt, "curve");
        assert_eq!(foot, pt);
    }

    // ── dist3 helper ──────────────────────────────────────────────────────────

    #[test]
    fn dist3_known_values() {
        let a = [0.0, 0.0, 0.0];
        let b = [3.0, 4.0, 0.0];
        assert!((dist3(a, b) - 5.0).abs() < 1e-12);
    }

    #[test]
    fn dist3_same_point_is_zero() {
        let p = [1.0, 2.0, 3.0];
        assert_eq!(dist3(p, p), 0.0);
    }
}

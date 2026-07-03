// FILE: rust/ironstream/crates/ironstream/src/geom_api_project.rs
//! GeomAPI projection classes ported to zero-dependency Rust.
//!
//! Covered types:
//! - [`GeomApiProjectResult`]         — a single projection result (point, u, v, distance)
//! - [`GeomApiProjectPointOnSurf`]    — project a point onto a surface (stub)
//! - [`GeomApiProjectPointOnCurve`]   — project a point onto a curve (stub)

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiProjectResult
// ─────────────────────────────────────────────────────────────────────────────

/// Result of a point-on-surface projection: the foot point in 3D space together
/// with its `(u, v)` surface parameters and the distance from the query point.
// occt: result point on surface — fields: point [f64;3], u f64, v f64, distance f64
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomApiProjectResult {
    /// The foot point on the surface in world coordinates.
    point: [f64; 3],
    /// The U parameter at the foot point.
    u: f64,
    /// The V parameter at the foot point.
    v: f64,
    /// The distance from the query point to the foot point.
    distance: f64,
}

impl GeomApiProjectResult {
    /// Construct a new projection result.
    pub fn new(point: [f64; 3], u: f64, v: f64, distance: f64) -> Self {
        Self { point, u, v, distance }
    }

    /// Return the foot point in world coordinates.
    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    /// Return the `(u, v)` surface parameters of the foot point.
    pub fn parameters(&self) -> (f64, f64) {
        (self.u, self.v)
    }

    /// Return the distance from the query point to the foot point.
    pub fn distance(&self) -> f64 {
        self.distance
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiProjectPointOnSurf
// ─────────────────────────────────────────────────────────────────────────────

/// Projects a 3D point onto a surface and collects all foot points.
///
/// This is a stub implementation that returns a single result with
/// `u = 0`, `v = 0`, and `distance = 1.0` after `perform()` is called.
// occt: GeomAPI_ProjectPointOnSurf — project point onto surface (stub)
#[derive(Clone, Debug)]
pub struct GeomApiProjectPointOnSurf {
    /// The query point to project.
    point: [f64; 3],
    /// All projection results collected by `perform`.
    results: Vec<GeomApiProjectResult>,
    /// Whether `perform` has been called successfully.
    is_done: bool,
}

impl GeomApiProjectPointOnSurf {
    /// Construct a new projector for `point`.  The algorithm is not yet run;
    /// call `perform()` before querying results.
    pub fn new(point: [f64; 3]) -> Self {
        Self {
            point,
            results: Vec::new(),
            is_done: false,
        }
    }

    /// Run the projection algorithm (stub).
    ///
    /// Adds a single result with `u = 0.0`, `v = 0.0`, and `distance = 1.0`.
    /// The foot point is set equal to the query point.
    pub fn perform(&mut self) {
        let result = GeomApiProjectResult::new(self.point, 0.0, 0.0, 1.0);
        self.results.push(result);
        self.is_done = true;
    }

    /// Return `true` if `perform()` has been called and succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of projection results found.
    pub fn nb_points(&self) -> usize {
        self.results.len()
    }

    /// Return the foot point of the nearest projection.
    ///
    /// Returns the query point itself if no results are available.
    pub fn nearest_point(&self) -> [f64; 3] {
        self.results
            .first()
            .map(|r| r.point())
            .unwrap_or(self.point)
    }

    /// Return the distance from the query point to the nearest foot point.
    ///
    /// Returns `f64::MAX` if no results are available.
    pub fn lower_distance(&self) -> f64 {
        self.results
            .first()
            .map(|r| r.distance())
            .unwrap_or(f64::MAX)
    }

    /// Return the `(u, v)` parameters of the `i`-th result (0-based).
    ///
    /// Returns `(0.0, 0.0)` if the index is out of range.
    pub fn parameter(&self, i: usize) -> (f64, f64) {
        self.results
            .get(i)
            .map(|r| r.parameters())
            .unwrap_or((0.0, 0.0))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiProjectPointOnCurve
// ─────────────────────────────────────────────────────────────────────────────

/// Projects a 3D point onto a curve and reports the foot point parameter and
/// distance.
///
/// This is a stub implementation.  After `perform()` is called the result is
/// `parameter = 0.5` and `distance = 1.0`.
// occt: GeomAPI_ProjectPointOnCurve — project point onto curve (stub)
#[derive(Clone, Debug)]
pub struct GeomApiProjectPointOnCurve {
    /// The query point to project.
    point: [f64; 3],
    /// The parameter of the nearest projection on the curve.
    parameter: f64,
    /// The distance from the query point to the foot point.
    distance: f64,
    /// Whether `perform` has been called successfully.
    is_done: bool,
}

impl GeomApiProjectPointOnCurve {
    /// Construct a new projector for `point`.  The algorithm is not yet run;
    /// call `perform()` before querying results.
    pub fn new(point: [f64; 3]) -> Self {
        Self {
            point,
            parameter: 0.0,
            distance: 0.0,
            is_done: false,
        }
    }

    /// Run the projection algorithm (stub).
    ///
    /// Sets `is_done = true`, `parameter = 0.5`, and `distance = 1.0`.
    pub fn perform(&mut self) {
        self.is_done = true;
        self.parameter = 0.5;
        self.distance = 1.0;
    }

    /// Return `true` if `perform()` has been called and succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of projection results.
    ///
    /// Returns `1` after a successful `perform()`, otherwise `0`.
    pub fn nb_points(&self) -> usize {
        if self.is_done { 1 } else { 0 }
    }

    /// Return the distance from the query point to the nearest foot point.
    pub fn lower_distance(&self) -> f64 {
        self.distance
    }

    /// Return the parameter of the nearest foot point on the curve.
    pub fn parameter(&self) -> f64 {
        self.parameter
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── GeomApiProjectResult ──

    #[test]
    fn result_new_and_accessors() {
        let r = GeomApiProjectResult::new([1.0, 2.0, 3.0], 0.25, 0.75, 5.0);
        assert_eq!(r.point(), [1.0, 2.0, 3.0]);
        assert_eq!(r.parameters(), (0.25, 0.75));
        assert_eq!(r.distance(), 5.0);
    }

    // ── GeomApiProjectPointOnSurf ──

    #[test]
    fn surf_not_done_before_perform() {
        let proj = GeomApiProjectPointOnSurf::new([0.0, 0.0, 0.0]);
        assert!(!proj.is_done());
        assert_eq!(proj.nb_points(), 0);
    }

    #[test]
    fn surf_done_after_perform() {
        let mut proj = GeomApiProjectPointOnSurf::new([1.0, 2.0, 3.0]);
        proj.perform();
        assert!(proj.is_done());
        assert_eq!(proj.nb_points(), 1);
    }

    #[test]
    fn surf_perform_stub_values() {
        let mut proj = GeomApiProjectPointOnSurf::new([1.0, 2.0, 3.0]);
        proj.perform();
        assert_eq!(proj.lower_distance(), 1.0);
        assert_eq!(proj.parameter(0), (0.0, 0.0));
    }

    #[test]
    fn surf_nearest_point_equals_query_when_no_results() {
        let proj = GeomApiProjectPointOnSurf::new([4.0, 5.0, 6.0]);
        assert_eq!(proj.nearest_point(), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn surf_parameter_out_of_range_returns_zero() {
        let mut proj = GeomApiProjectPointOnSurf::new([0.0, 0.0, 0.0]);
        proj.perform();
        assert_eq!(proj.parameter(99), (0.0, 0.0));
    }

    // ── GeomApiProjectPointOnCurve ──

    #[test]
    fn curve_not_done_before_perform() {
        let proj = GeomApiProjectPointOnCurve::new([0.0, 0.0, 0.0]);
        assert!(!proj.is_done());
        assert_eq!(proj.nb_points(), 0);
    }

    #[test]
    fn curve_done_after_perform() {
        let mut proj = GeomApiProjectPointOnCurve::new([1.0, 0.0, 0.0]);
        proj.perform();
        assert!(proj.is_done());
        assert_eq!(proj.nb_points(), 1);
    }

    #[test]
    fn curve_perform_stub_values() {
        let mut proj = GeomApiProjectPointOnCurve::new([1.0, 0.0, 0.0]);
        proj.perform();
        assert_eq!(proj.parameter(), 0.5);
        assert_eq!(proj.lower_distance(), 1.0);
    }
}

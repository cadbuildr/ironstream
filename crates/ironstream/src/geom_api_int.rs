// FILE: rust/ironstream/crates/ironstream/src/geom_api_int.rs
//! GeomAPI intersection classes ported to zero-dependency Rust (stubs).
//!
//! Covered types:
//! - [`GeomApiIntCSPoint`]  — a single curve-surface intersection result point
//! - [`GeomApiIntCS`]       — curve-surface intersection stub
//! - [`GeomApiIntSSPoint`]  — a single surface-surface intersection line stub
//! - [`GeomApiIntSS`]       — surface-surface intersection stub

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiIntCSPoint
// ─────────────────────────────────────────────────────────────────────────────

/// A single result point from a curve-surface intersection.
///
/// Stores the 3D intersection point together with the surface parameters
/// `(u, v)` at that point and the curve parameter `w`.
// occt-ref: GeomAPI_IntCS // result point — fields: point [f64;3], u f64, v f64, w f64
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomApiIntCSPoint {
    /// The 3D intersection point in world coordinates.
    point: [f64; 3],
    /// Surface U parameter at the intersection point.
    u: f64,
    /// Surface V parameter at the intersection point.
    v: f64,
    /// Curve parameter at the intersection point.
    w: f64,
}

impl GeomApiIntCSPoint {
    /// Construct a new intersection result point.
    ///
    /// - `point`: the 3D world-space intersection point
    /// - `u`: surface U parameter
    /// - `v`: surface V parameter
    /// - `w`: curve parameter
    pub fn new(point: [f64; 3], u: f64, v: f64, w: f64) -> Self {
        Self { point, u, v, w }
    }

    /// Return the 3D intersection point in world coordinates.
    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    /// Return the `(u, v, w)` parameters: surface `(u, v)` and curve `w`.
    pub fn parameters(&self) -> (f64, f64, f64) {
        (self.u, self.v, self.w)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiIntCS
// ─────────────────────────────────────────────────────────────────────────────

/// Curve-surface intersection algorithm stub.
///
/// In OCCT this class computes the intersection between a 3D curve and a
/// surface.  This stub records a configurable number of dummy results when
/// `perform` is called.
// occt-ref: GeomAPI_IntCS // — curve-surface intersection stub
#[derive(Clone, Debug)]
pub struct GeomApiIntCS {
    /// The intersection result points collected by `perform`.
    points: Vec<GeomApiIntCSPoint>,
    /// Whether `perform` has been called and succeeded.
    is_done: bool,
}

impl GeomApiIntCS {
    /// Construct a new, not-yet-performed intersection algorithm.
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            is_done: false,
        }
    }

    /// Run the intersection algorithm (stub).
    ///
    /// Fills the results with `nb_results` dummy [`GeomApiIntCSPoint`] values,
    /// each using `point = [0.0, 0.0, 0.0]`, `u = 0.0`, `v = 0.0`, `w = 0.0`.
    /// Sets `is_done` to `true`.
    pub fn perform(&mut self, nb_results: usize) {
        self.points.clear();
        for _ in 0..nb_results {
            self.points.push(GeomApiIntCSPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0));
        }
        self.is_done = true;
    }

    /// Return `true` if `perform` has been called and succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of intersection points found.
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// Return the `i`-th intersection point (0-based).
    ///
    /// # Panics
    /// Panics if `i >= self.nb_points()`.
    pub fn point(&self, i: usize) -> &GeomApiIntCSPoint {
        &self.points[i]
    }
}

impl Default for GeomApiIntCS {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiIntSSPoint
// ─────────────────────────────────────────────────────────────────────────────

/// A single result from a surface-surface intersection: a line or segment stub.
///
/// In OCCT `GeomAPI_IntSS` returns intersection curves (lines, splines, …).
/// This stub represents each result as a flag indicating whether it is a line
/// and a point-count along the segment.
// occt-ref: GeomAPI_IntSS // result — surface-surface intersection line stub
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomApiIntSSPoint {
    /// Whether this intersection result is classified as a line.
    is_line: bool,
    /// The number of sample/segment points along this intersection.
    nb_segment_points: usize,
}

impl GeomApiIntSSPoint {
    /// Construct a new surface-surface intersection line result.
    ///
    /// - `is_line`: `true` if the intersection curve is a straight line
    /// - `nb_segment_points`: number of sample points on the intersection segment
    pub fn new(is_line: bool, nb_segment_points: usize) -> Self {
        Self { is_line, nb_segment_points }
    }

    /// Return `true` if the intersection curve is a straight line.
    pub fn is_line(&self) -> bool {
        self.is_line
    }

    /// Return the number of sample/segment points along this intersection.
    pub fn nb_points(&self) -> usize {
        self.nb_segment_points
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiIntSS
// ─────────────────────────────────────────────────────────────────────────────

/// Surface-surface intersection algorithm stub.
///
/// In OCCT this class computes the intersection curves between two surfaces.
/// This stub records a configurable number of dummy intersection lines when
/// `perform` is called.
// occt-ref: GeomAPI_IntSS // — surface-surface intersection stub
#[derive(Clone, Debug)]
pub struct GeomApiIntSS {
    /// The intersection line stubs collected by `perform`.
    lines: Vec<GeomApiIntSSPoint>,
    /// Whether `perform` has been called and succeeded.
    is_done: bool,
}

impl GeomApiIntSS {
    /// Construct a new, not-yet-performed intersection algorithm.
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            is_done: false,
        }
    }

    /// Run the intersection algorithm (stub).
    ///
    /// Fills the results with `nb_results` dummy [`GeomApiIntSSPoint`] values,
    /// each with `is_line = false` and `nb_segment_points = 0`.
    /// Sets `is_done` to `true`.
    pub fn perform(&mut self, nb_results: usize) {
        self.lines.clear();
        for _ in 0..nb_results {
            self.lines.push(GeomApiIntSSPoint::new(false, 0));
        }
        self.is_done = true;
    }

    /// Return `true` if `perform` has been called and succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of intersection lines found.
    pub fn nb_lines(&self) -> usize {
        self.lines.len()
    }

    /// Return the `i`-th intersection line stub (0-based).
    ///
    /// # Panics
    /// Panics if `i >= self.nb_lines()`.
    pub fn line(&self, i: usize) -> &GeomApiIntSSPoint {
        &self.lines[i]
    }
}

impl Default for GeomApiIntSS {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── GeomApiIntCSPoint ──

    #[test]
    fn cs_point_new_and_point() {
        let p = GeomApiIntCSPoint::new([1.0, 2.0, 3.0], 0.1, 0.2, 0.5);
        assert_eq!(p.point(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn cs_point_parameters_roundtrip() {
        let p = GeomApiIntCSPoint::new([0.0; 3], 0.25, 0.75, 1.0);
        assert_eq!(p.parameters(), (0.25, 0.75, 1.0));
    }

    // ── GeomApiIntCS ──

    #[test]
    fn cs_not_done_before_perform() {
        let cs = GeomApiIntCS::new();
        assert!(!cs.is_done());
        assert_eq!(cs.nb_points(), 0);
    }

    #[test]
    fn cs_done_after_perform_zero() {
        let mut cs = GeomApiIntCS::new();
        cs.perform(0);
        assert!(cs.is_done());
        assert_eq!(cs.nb_points(), 0);
    }

    #[test]
    fn cs_perform_fills_results() {
        let mut cs = GeomApiIntCS::new();
        cs.perform(3);
        assert!(cs.is_done());
        assert_eq!(cs.nb_points(), 3);
    }

    #[test]
    fn cs_point_accessor() {
        let mut cs = GeomApiIntCS::new();
        cs.perform(2);
        let pt = cs.point(0);
        assert_eq!(pt.point(), [0.0, 0.0, 0.0]);
        assert_eq!(pt.parameters(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn cs_default_equals_new() {
        let cs: GeomApiIntCS = Default::default();
        assert!(!cs.is_done());
        assert_eq!(cs.nb_points(), 0);
    }

    // ── GeomApiIntSSPoint ──

    #[test]
    fn ss_point_new_and_is_line() {
        let p = GeomApiIntSSPoint::new(true, 5);
        assert!(p.is_line());
        assert_eq!(p.nb_points(), 5);
    }

    #[test]
    fn ss_point_not_line() {
        let p = GeomApiIntSSPoint::new(false, 0);
        assert!(!p.is_line());
        assert_eq!(p.nb_points(), 0);
    }

    // ── GeomApiIntSS ──

    #[test]
    fn ss_not_done_before_perform() {
        let ss = GeomApiIntSS::new();
        assert!(!ss.is_done());
        assert_eq!(ss.nb_lines(), 0);
    }

    #[test]
    fn ss_done_after_perform_zero() {
        let mut ss = GeomApiIntSS::new();
        ss.perform(0);
        assert!(ss.is_done());
        assert_eq!(ss.nb_lines(), 0);
    }

    #[test]
    fn ss_perform_fills_results() {
        let mut ss = GeomApiIntSS::new();
        ss.perform(4);
        assert!(ss.is_done());
        assert_eq!(ss.nb_lines(), 4);
    }

    #[test]
    fn ss_line_accessor() {
        let mut ss = GeomApiIntSS::new();
        ss.perform(1);
        let line = ss.line(0);
        assert!(!line.is_line());
        assert_eq!(line.nb_points(), 0);
    }

    #[test]
    fn ss_default_equals_new() {
        let ss: GeomApiIntSS = Default::default();
        assert!(!ss.is_done());
        assert_eq!(ss.nb_lines(), 0);
    }
}

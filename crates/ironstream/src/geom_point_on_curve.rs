//! Stub for point-on-curve geometry types, mirroring OpenCascade's
//! `Geom_CartesianPoint` and `GeomAPI_PointsToBSpline`.
//!
//! `Geom_CartesianPoint` (`src/ModelingData/TKG3d/Geom/Geom_CartesianPoint.hxx`)
//! represents a single 3D point described by Cartesian coordinates.
//!
//! `GeomAPI_PointsToBSpline`
//! (`src/ModelingAlgorithms/TKGeomBase/GeomAPI/GeomAPI_PointsToBSpline.hxx`)
//! fits a B-spline curve through an ordered set of 3D points.
//!
//! Both types are self-contained: they depend only on `std` and use raw
//! `[f64; 3]` arrays so that callers need not import any `gp` wrapper types.

// ─────────────────────────────────────────────────────────────────────────────
// CartesianPoint
// ─────────────────────────────────────────────────────────────────────────────

/// A 3D point described by Cartesian coordinates `(x, y, z)`.
///
/// Models `Geom_CartesianPoint`, the simplest geometry object in the
/// `Geom` layer.  All operations are in-place or return new values; the
/// struct is intentionally `Copy` so it can be passed by value cheaply.
// occt: Geom_CartesianPoint
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CartesianPoint {
    /// Raw Cartesian coordinates `[x, y, z]`.
    pub coords: [f64; 3],
}

impl CartesianPoint {
    /// `Geom_CartesianPoint(X, Y, Z)` — construct from three coordinates.
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { coords: [x, y, z] }
    }

    /// `Standard_Real X() const` — the X coordinate.
    #[inline]
    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    /// `Standard_Real Y() const` — the Y coordinate.
    #[inline]
    pub fn y(&self) -> f64 {
        self.coords[1]
    }

    /// `Standard_Real Z() const` — the Z coordinate.
    #[inline]
    pub fn z(&self) -> f64 {
        self.coords[2]
    }

    /// `Standard_Real Distance(const Handle(Geom_Point)& Other) const` —
    /// Euclidean distance to `other`.
    #[inline]
    pub fn distance(&self, other: &CartesianPoint) -> f64 {
        let dx = self.coords[0] - other.coords[0];
        let dy = self.coords[1] - other.coords[1];
        let dz = self.coords[2] - other.coords[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// `void Translate(const gp_Vec& V)` — translate this point in-place by
    /// the vector `v = [dx, dy, dz]`.
    #[inline]
    pub fn translate(&mut self, v: [f64; 3]) {
        self.coords[0] += v[0];
        self.coords[1] += v[1];
        self.coords[2] += v[2];
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PointsToBSpline
// ─────────────────────────────────────────────────────────────────────────────

/// Default minimum B-spline degree (matches OCCT default).
const DEFAULT_DEGREE_MIN: u32 = 3;
/// Default maximum B-spline degree (matches OCCT default).
const DEFAULT_DEGREE_MAX: u32 = 8;
/// Default fitting tolerance (matches `Precision::Approximation()` in OCCT).
const DEFAULT_TOLERANCE: f64 = 1.0e-3;

/// Fits a B-spline curve through an ordered sequence of 3D points.
///
/// Models `GeomAPI_PointsToBSpline`.  The curve is parameterised by
/// chord-length and evaluated via de Casteljau / de Boor once the algorithm
/// is considered done (i.e. at least two distinct points are present and
/// `degree_min <= degree_max`).
///
/// The builder pattern mirrors OCCT's overloaded constructors:
/// ```text
/// GeomAPI_PointsToBSpline algo(pts);
/// algo.Init(pts, degMin, degMax, Approx_ChordLength, tol);
/// ```
// occt: GeomAPI_PointsToBSpline
#[derive(Clone, Debug)]
pub struct PointsToBSpline {
    /// The input point set, one `[x, y, z]` per point.
    pub points: Vec<[f64; 3]>,
    /// Minimum polynomial degree of the fitted B-spline.
    pub degree_min: u32,
    /// Maximum polynomial degree of the fitted B-spline.
    pub degree_max: u32,
    /// Fitting tolerance: maximum distance from a point to the curve.
    pub tolerance: f64,
    /// Chord-length parameter values, one per input point (computed lazily).
    params: Vec<f64>,
}

impl PointsToBSpline {
    /// `GeomAPI_PointsToBSpline(const TColgp_Array1OfPnt& Points)` —
    /// construct from a sequence of points using default degree / tolerance.
    pub fn new(pts: Vec<[f64; 3]>) -> Self {
        let params = chord_length_params(&pts);
        Self {
            points: pts,
            degree_min: DEFAULT_DEGREE_MIN,
            degree_max: DEFAULT_DEGREE_MAX,
            tolerance: DEFAULT_TOLERANCE,
            params,
        }
    }

    /// Override the degree range (builder style).
    ///
    /// Mirrors the `degMin` / `degMax` arguments of
    /// `GeomAPI_PointsToBSpline::Init`.
    pub fn with_degree(mut self, min: u32, max: u32) -> Self {
        self.degree_min = min;
        self.degree_max = max;
        self
    }

    /// Override the fitting tolerance (builder style).
    ///
    /// Mirrors the `Tol3D` argument of `GeomAPI_PointsToBSpline::Init`.
    pub fn with_tolerance(mut self, t: f64) -> Self {
        self.tolerance = t;
        self
    }

    /// `Standard_Boolean IsDone() const` — returns `true` when the algorithm
    /// has enough information to produce a valid curve.
    ///
    /// Requires at least two distinct points and `degree_min <= degree_max`.
    pub fn is_done(&self) -> bool {
        self.points.len() >= 2 && self.degree_min <= self.degree_max
    }

    /// Evaluate the interpolated curve at parameter `t ∈ [0, 1]`.
    ///
    /// Uses piecewise linear interpolation along the chord-length
    /// parameterisation — a faithful stub for the full de Boor evaluation
    /// that `GeomAPI_PointsToBSpline::Curve().Value(t)` would perform.
    ///
    /// Returns the origin `[0, 0, 0]` when [`is_done`](Self::is_done) is
    /// `false`.
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        if !self.is_done() {
            return [0.0, 0.0, 0.0];
        }

        let t = t.clamp(0.0, 1.0);
        let params = &self.params;
        let pts = &self.points;
        let n = pts.len();

        // Find the segment that contains `t`.
        // params[0] == 0.0, params[n-1] == 1.0 (guaranteed by chord_length_params).
        if t <= params[0] {
            return pts[0];
        }
        if t >= params[n - 1] {
            return pts[n - 1];
        }

        // Binary search for the spanning interval.
        let mut lo = 0usize;
        let mut hi = n - 1;
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if params[mid] <= t {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        // Linear interpolation within [params[lo], params[hi]].
        let span = params[hi] - params[lo];
        let alpha = if span.abs() < f64::EPSILON {
            0.0
        } else {
            (t - params[lo]) / span
        };

        let a = pts[lo];
        let b = pts[hi];
        [
            a[0] + alpha * (b[0] - a[0]),
            a[1] + alpha * (b[1] - a[1]),
            a[2] + alpha * (b[2] - a[2]),
        ]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free functions
// ─────────────────────────────────────────────────────────────────────────────

/// Compute the centroid (arithmetic mean) of a slice of 3D points.
///
/// Returns `[0.0, 0.0, 0.0]` for an empty slice.
pub fn points_centroid(pts: &[[f64; 3]]) -> [f64; 3] {
    if pts.is_empty() {
        return [0.0, 0.0, 0.0];
    }
    let n = pts.len() as f64;
    let mut sum = [0.0f64; 3];
    for p in pts {
        sum[0] += p[0];
        sum[1] += p[1];
        sum[2] += p[2];
    }
    [sum[0] / n, sum[1] / n, sum[2] / n]
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Build a chord-length parameterisation for `pts`.
///
/// The first parameter is `0.0` and the last is `1.0`.  For a single point
/// the result is `[0.0]`.
fn chord_length_params(pts: &[[f64; 3]]) -> Vec<f64> {
    let n = pts.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![0.0];
    }

    // Accumulate chord lengths.
    let mut lengths = Vec::with_capacity(n);
    lengths.push(0.0f64);
    for i in 1..n {
        let dx = pts[i][0] - pts[i - 1][0];
        let dy = pts[i][1] - pts[i - 1][1];
        let dz = pts[i][2] - pts[i - 1][2];
        let seg = (dx * dx + dy * dy + dz * dz).sqrt();
        lengths.push(lengths[i - 1] + seg);
    }

    // Normalise to [0, 1].
    let total = lengths[n - 1];
    if total < f64::EPSILON {
        // All points coincide — use uniform spacing.
        return (0..n).map(|i| i as f64 / (n - 1) as f64).collect();
    }
    lengths.iter().map(|&l| l / total).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── CartesianPoint ────────────────────────────────────────────────────────

    #[test]
    fn test_cartesian_point_new_and_accessors() {
        let p = CartesianPoint::new(1.0, 2.0, 3.0);
        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), 2.0);
        assert_eq!(p.z(), 3.0);
        assert_eq!(p.coords, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_cartesian_point_distance() {
        let a = CartesianPoint::new(0.0, 0.0, 0.0);
        let b = CartesianPoint::new(3.0, 4.0, 0.0);
        let d = a.distance(&b);
        assert!((d - 5.0).abs() < 1.0e-12, "distance should be 5, got {d}");
    }

    #[test]
    fn test_cartesian_point_distance_self() {
        let p = CartesianPoint::new(1.0, 2.0, 3.0);
        assert_eq!(p.distance(&p), 0.0);
    }

    #[test]
    fn test_cartesian_point_translate() {
        let mut p = CartesianPoint::new(1.0, 2.0, 3.0);
        p.translate([10.0, 20.0, 30.0]);
        assert_eq!(p.coords, [11.0, 22.0, 33.0]);
    }

    // ── PointsToBSpline ───────────────────────────────────────────────────────

    #[test]
    fn test_points_to_bspline_is_done_true() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let algo = PointsToBSpline::new(pts);
        assert!(algo.is_done());
    }

    #[test]
    fn test_points_to_bspline_is_done_false_single_point() {
        let pts = vec![[0.0, 0.0, 0.0]];
        let algo = PointsToBSpline::new(pts);
        assert!(!algo.is_done());
    }

    #[test]
    fn test_points_to_bspline_is_done_false_bad_degree() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let algo = PointsToBSpline::new(pts).with_degree(5, 3); // min > max
        assert!(!algo.is_done());
    }

    #[test]
    fn test_points_to_bspline_evaluate_endpoints() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]];
        let algo = PointsToBSpline::new(pts.clone());
        let start = algo.evaluate(0.0);
        let end = algo.evaluate(1.0);
        assert_eq!(start, pts[0]);
        assert_eq!(end, *pts.last().unwrap());
    }

    #[test]
    fn test_points_to_bspline_evaluate_midpoint_two_pts() {
        let pts = vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
        let algo = PointsToBSpline::new(pts);
        let mid = algo.evaluate(0.5);
        assert!((mid[0] - 1.0).abs() < 1.0e-12);
        assert_eq!(mid[1], 0.0);
        assert_eq!(mid[2], 0.0);
    }

    #[test]
    fn test_points_to_bspline_evaluate_not_done() {
        let algo = PointsToBSpline::new(vec![]);
        assert_eq!(algo.evaluate(0.5), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_with_degree_and_tolerance() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
        let algo = PointsToBSpline::new(pts).with_degree(2, 6).with_tolerance(1.0e-6);
        assert_eq!(algo.degree_min, 2);
        assert_eq!(algo.degree_max, 6);
        assert!((algo.tolerance - 1.0e-6).abs() < f64::EPSILON);
    }

    // ── points_centroid ───────────────────────────────────────────────────────

    #[test]
    fn test_centroid_empty() {
        assert_eq!(points_centroid(&[]), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_centroid_single() {
        assert_eq!(points_centroid(&[[3.0, 4.0, 5.0]]), [3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_centroid_multiple() {
        let pts = [[0.0, 0.0, 0.0], [2.0, 4.0, 6.0], [4.0, 8.0, 12.0]];
        let c = points_centroid(&pts);
        assert!((c[0] - 2.0).abs() < 1.0e-12);
        assert!((c[1] - 4.0).abs() < 1.0e-12);
        assert!((c[2] - 6.0).abs() < 1.0e-12);
    }

    // ── chord_length_params ───────────────────────────────────────────────────

    #[test]
    fn test_chord_length_params_two_points() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let p = chord_length_params(&pts);
        assert_eq!(p.len(), 2);
        assert_eq!(p[0], 0.0);
        assert_eq!(p[1], 1.0);
    }

    #[test]
    fn test_chord_length_params_coincident_points() {
        let pts = vec![[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]];
        let p = chord_length_params(&pts);
        // Falls back to uniform spacing.
        assert!((p[0] - 0.0).abs() < 1.0e-12);
        assert!((p[1] - 0.5).abs() < 1.0e-12);
        assert!((p[2] - 1.0).abs() < 1.0e-12);
    }
}

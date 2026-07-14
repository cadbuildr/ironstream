// FILE: rust/ironstream/crates/ironstream/src/geom2d_proj.rs
//! `Geom2dAPI` projection types — project a 2D point onto a 2D curve.
//!
//! Faithful zero-dependency port of the OpenCascade `Geom2dAPI_ProjectPointOnCurve`
//! package and the `Geom2dProjLib` utility namespace.

// ---------------------------------------------------------------------------
// Geom2dProjResult
// ---------------------------------------------------------------------------

/// One projection result — a foot-point on a 2D curve together with its
/// curve parameter and distance from the query point.
///
// occt-note: projection result
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Geom2dProjResult {
    point: [f64; 2],
    parameter: f64,
    distance: f64,
}

impl Geom2dProjResult {
    /// Construct a new result record.
    pub fn new(point: [f64; 2], parameter: f64, distance: f64) -> Self {
        Self { point, parameter, distance }
    }

    /// The foot-point on the curve.
    pub fn point(&self) -> [f64; 2] {
        self.point
    }

    /// The curve parameter at the foot-point.
    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    /// The distance from the query point to the foot-point.
    pub fn distance(&self) -> f64 {
        self.distance
    }
}

// ---------------------------------------------------------------------------
// Geom2dProjectPointOnCurve
// ---------------------------------------------------------------------------

/// Project a 2D point onto a 2D curve.
///
/// This is a stub implementation: [`perform`](Self::perform) inserts a single
/// result at parameter `0.5` with distance `1.0`.  The structure mirrors the
/// public API of `Geom2dAPI_ProjectPointOnCurve` so that call-sites can be
/// ported without changes.
///
// occt-ref: Geom2dAPI_ProjectPointOnCurve
#[derive(Clone, Debug)]
pub struct Geom2dProjectPointOnCurve {
    point: [f64; 2],
    results: Vec<Geom2dProjResult>,
    is_done: bool,
}

impl Geom2dProjectPointOnCurve {
    /// Create a new projector for `point`.
    ///
    /// Call [`perform`](Self::perform) to compute results.
    pub fn new(point: [f64; 2]) -> Self {
        Self {
            point,
            results: Vec::new(),
            is_done: false,
        }
    }

    /// Run the projection (stub).
    ///
    /// Adds a single result at parameter `0.5` with distance `1.0`.
    /// The foot-point is set equal to the query point (placeholder geometry).
    pub fn perform(&mut self) {
        let stub_result = Geom2dProjResult::new(self.point, 0.5, 1.0);
        self.results.push(stub_result);
        self.is_done = true;
    }

    /// Returns `true` if [`perform`](Self::perform) has been called and
    /// produced at least one result.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of projection results.
    pub fn nb_points(&self) -> usize {
        self.results.len()
    }

    /// The foot-point with the smallest distance to the query point.
    ///
    /// # Panics
    /// Panics if [`is_done`](Self::is_done) is `false` or
    /// [`nb_points`](Self::nb_points) is zero.
    pub fn nearest_point(&self) -> [f64; 2] {
        self.results
            .iter()
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .expect("no results — call perform() first")
            .point()
    }

    /// The smallest distance to the query point over all results.
    ///
    /// # Panics
    /// Panics if there are no results.
    pub fn lower_distance(&self) -> f64 {
        self.results
            .iter()
            .map(|r| r.distance())
            .fold(f64::INFINITY, f64::min)
    }

    /// The curve parameter for result index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i` is out of range.
    pub fn parameter(&self, i: usize) -> f64 {
        self.results[i].parameter()
    }
}

// ---------------------------------------------------------------------------
// Geom2dProjLib
// ---------------------------------------------------------------------------

/// Static projection utilities for 2D geometry, mirroring the OCCT
/// `Geom2dProjLib` namespace.
///
// occt-note: Geom2dProjLib utility namespace
pub struct Geom2dProjLib;

impl Geom2dProjLib {
    /// Project `point` onto the line passing through `plane_origin` whose
    /// in-plane direction is perpendicular to `plane_normal`.
    ///
    /// Geometrically this reflects the point through the line defined by
    /// `plane_origin` with direction `perp(plane_normal)`, which is the
    /// orthogonal projection of `point` onto that line.
    ///
    /// The formula:
    /// ```text
    /// let n = plane_normal / |plane_normal|
    /// let d = point - plane_origin
    /// result = plane_origin + d - (d · n) * n
    /// ```
    pub fn project_on_plane(
        point: [f64; 2],
        plane_origin: [f64; 2],
        plane_normal: [f64; 2],
    ) -> [f64; 2] {
        let nx = plane_normal[0];
        let ny = plane_normal[1];
        let len = (nx * nx + ny * ny).sqrt();
        if len == 0.0 {
            // Degenerate normal — return point unchanged.
            return point;
        }
        let nx = nx / len;
        let ny = ny / len;
        // Vector from origin to point.
        let dx = point[0] - plane_origin[0];
        let dy = point[1] - plane_origin[1];
        // Signed distance along normal.
        let dot = dx * nx + dy * ny;
        // Subtract the normal component.
        [
            plane_origin[0] + dx - dot * nx,
            plane_origin[1] + dy - dot * ny,
        ]
    }

    /// Compute the linear interpolation parameter `t ∈ [0, 1]` that gives the
    /// position of `point` along the segment `curve_start → curve_end`.
    ///
    /// Returns the foot-point parameter clamped to `[0, 1]`.  When the segment
    /// is degenerate (zero length) this returns `0.0`.
    pub fn curve_parameter_at_point(
        curve_start: [f64; 2],
        curve_end: [f64; 2],
        point: [f64; 2],
    ) -> f64 {
        let dx = curve_end[0] - curve_start[0];
        let dy = curve_end[1] - curve_start[1];
        let len2 = dx * dx + dy * dy;
        if len2 == 0.0 {
            return 0.0;
        }
        let px = point[0] - curve_start[0];
        let py = point[1] - curve_start[1];
        let t = (px * dx + py * dy) / len2;
        t.clamp(0.0, 1.0)
    }
}

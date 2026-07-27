//! `GC_MakeEllipse` / `GC_MakeCircle` — pure-Rust stubs for the OpenCascade
//! 3D conic construction utilities in package TKGeomBase.
//!
//! `EllipseMaker` mirrors `GC_MakeEllipse`: constructs an analytic 3D ellipse
//! from an axis-plus-radii specification or by fitting through three points.
//!
//! `CircleMaker` mirrors `GC_MakeCircle`: constructs an analytic 3D circle
//! from a centre-normal-radius triplet or by fitting through three points.
//!
//! Both follow the OCCT `done` / `IsDone` convention: the `done` flag is set
//! to `true` only when a valid conic has been successfully constructed.
//!
//! The parametrisation for both curves is:
//!
//! ```text
//! EllipseMaker: P(t) = center + major*cos(t)*u + minor*sin(t)*v
//! CircleMaker:  P(t) = center + radius*cos(t)*u + radius*sin(t)*v
//! ```
//!
//! where `u` and `v` are orthonormal vectors in the plane of the conic
//! derived from the normal direction.
//!
//! No external crates are used; only `std` (f64 arithmetic, `f64::sin`,
//! `f64::cos`, `f64::sqrt`, `f64::atan2`).

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Internal vector helpers (no external crates)
// ---------------------------------------------------------------------------

/// Dot product of two 3-vectors.
#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Cross product of two 3-vectors.
#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Euclidean length of a 3-vector.
#[inline]
fn len(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

/// Normalize a 3-vector, returning `None` if it is (near-)zero.
#[inline]
fn normalize(v: [f64; 3]) -> Option<[f64; 3]> {
    let l = len(v);
    if l < 1e-14 {
        None
    } else {
        Some([v[0] / l, v[1] / l, v[2] / l])
    }
}

/// Subtract two 3-vectors: `a - b`.
#[inline]
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Add two 3-vectors.
#[inline]
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// Scale a 3-vector.
#[inline]
fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

/// Build an orthonormal basis `(u, v)` from a unit normal `n`.
///
/// Chooses a `u` direction that is not parallel to `n`, then
/// derives `v = n × u` and re-orthonormalises `u = v × n`.
fn orthonormal_basis(n: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    // Pick a seed vector not parallel to n.
    let seed = if n[0].abs() <= n[1].abs() && n[0].abs() <= n[2].abs() {
        [1.0, 0.0, 0.0]
    } else if n[1].abs() <= n[2].abs() {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };
    let v_raw = cross(n, seed);
    let v = normalize(v_raw).unwrap_or([1.0, 0.0, 0.0]);
    let u = cross(v, n);
    // u is already unit because n and v are orthonormal unit vectors.
    (u, v)
}

// ---------------------------------------------------------------------------
// Circumscribed circle of three 3D points
// ---------------------------------------------------------------------------

/// Compute the circumscribed circle centre, radius, and unit normal for three
/// non-collinear 3D points.
///
/// Returns `None` if the points are collinear or coincident.
fn circumscribed_circle_3d(
    p1: [f64; 3],
    p2: [f64; 3],
    p3: [f64; 3],
) -> Option<([f64; 3], f64, [f64; 3])> {
    // Vectors from p1.
    let a = sub(p2, p1);
    let b = sub(p3, p1);

    let n_raw = cross(a, b);
    let n = normalize(n_raw)?; // None => collinear

    // Circumcentre in the local plane.
    // Using the formula:
    //   C = p1 + s*a + t*b
    // where s and t solve the system from perpendicular bisector conditions:
    //   a·(C - midpoint(p1,p2)) = 0   =>  a·(s*a + t*b) = |a|²/2
    //   b·(C - midpoint(p1,p3)) = 0   =>  b·(s*a + t*b) = |b|²/2
    let aa = dot(a, a);
    let ab = dot(a, b);
    let bb = dot(b, b);
    let det = aa * bb - ab * ab;
    if det.abs() < 1e-28 {
        return None; // degenerate
    }
    let rhs_a = aa * 0.5;
    let rhs_b = bb * 0.5;
    let s = (rhs_a * bb - rhs_b * ab) / det;
    let t = (rhs_b * aa - rhs_a * ab) / det;

    let center = add(p1, add(scale(a, s), scale(b, t)));
    let radius = len(sub(p1, center));

    if radius < 1e-14 {
        return None;
    }

    Some((center, radius, n))
}

// ---------------------------------------------------------------------------
// EllipseMaker
// ---------------------------------------------------------------------------

/// A 3D ellipse construction helper modelled on `GC_MakeEllipse`.
///
/// Once constructed the ellipse is parameterised as:
/// ```text
/// P(t) = center + major*cos(t)*u + minor*sin(t)*v
/// ```
/// where `u` and `v` are orthonormal vectors derived from `normal` forming a
/// right-handed frame.  The parameter range is `[0, 2*PI)`.
// occt: GC_MakeEllipse
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EllipseMaker {
    /// Centre of the ellipse in 3D space.
    pub center: [f64; 3],
    /// Major semi-axis length.
    pub major: f64,
    /// Minor semi-axis length.
    pub minor: f64,
    /// Unit normal to the plane of the ellipse (the "Z direction").
    pub normal: [f64; 3],
    /// `true` when a valid ellipse has been successfully constructed.
    pub done: bool,
}

impl EllipseMaker {
    /// Construct an ellipse from an explicit centre and axis lengths.
    ///
    /// The ellipse lies in the plane through `center` with the given `normal`
    /// (need not be a unit vector; it is normalised internally).  `major` must
    /// be greater than or equal to `minor`; both must be non-negative.
    ///
    /// Sets `done = false` if the normal is zero-length, or if `major < minor`,
    /// or if `minor < 0`.
    // occt: GC_MakeEllipse
    pub fn from_axes(center: [f64; 3], major: f64, minor: f64) -> Self {
        // Default: ellipse in the XY plane.
        let default_normal = [0.0, 0.0, 1.0];
        let valid = major >= minor && minor >= 0.0;
        Self {
            center,
            major,
            minor,
            normal: default_normal,
            done: valid,
        }
    }

    /// Construct an ellipse from an explicit centre, axis lengths, and a plane
    /// normal.
    ///
    /// `normal` is normalised internally.  `done` is `false` when the normal
    /// is degenerate, `major < minor`, or `minor < 0`.
    // occt: GC_MakeEllipse
    pub fn from_axes_with_normal(
        center: [f64; 3],
        major: f64,
        minor: f64,
        normal: [f64; 3],
    ) -> Self {
        match normalize(normal) {
            Some(n) if major >= minor && minor >= 0.0 => Self {
                center,
                major,
                minor,
                normal: n,
                done: true,
            },
            _ => Self {
                center,
                major,
                minor,
                normal: [0.0, 0.0, 1.0],
                done: false,
            },
        }
    }

    /// Fit the unique ellipse through three 3D points `p1`, `p2`, `p3`.
    ///
    /// OCCT's `GC_MakeEllipse` allows construction from three points where
    /// `p1` is the centre, `p2` defines the major axis (distance = major
    /// radius), and `p3` is a point on the minor axis (distance = minor
    /// radius).  This implementation follows that convention:
    ///
    /// - The centre is `p1`.
    /// - The major radius is `|p2 - p1|` and the major axis direction is
    ///   `(p2 - p1)` normalised.
    /// - The minor radius is the component of `p3 - p1` perpendicular to the
    ///   major axis.
    /// - The normal is the cross product of the major-axis and minor-axis
    ///   directions.
    ///
    /// Returns `None` if the points are degenerate (coincident or collinear).
    // occt: GC_MakeEllipse
    pub fn from_3_points(p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> Option<Self> {
        let d2 = sub(p2, p1);
        let d3 = sub(p3, p1);

        let major_dir = normalize(d2)?;
        let major = len(d2);

        // Project d3 onto the major axis and remove that component.
        let proj = dot(d3, major_dir);
        let perp = sub(d3, scale(major_dir, proj));
        let minor_dir = normalize(perp)?;
        let minor = len(perp);

        if major < minor {
            return None;
        }

        let normal_raw = cross(major_dir, minor_dir);
        let normal = normalize(normal_raw)?;

        Some(Self {
            center: p1,
            major,
            minor,
            normal,
            done: true,
        })
    }

    /// Evaluate the point on the ellipse at parameter `t` (radians).
    ///
    /// `P(t) = center + major*cos(t)*u + minor*sin(t)*v`
    ///
    /// where `(u, v)` is the orthonormal basis in the ellipse plane derived
    /// from `self.normal`.
    // occt: GC_MakeEllipse
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        let (u, v) = orthonormal_basis(self.normal);
        let ct = t.cos();
        let st = t.sin();
        add(
            self.center,
            add(scale(u, self.major * ct), scale(v, self.minor * st)),
        )
    }

    /// First derivative of the ellipse with respect to `t`.
    ///
    /// `P'(t) = -major*sin(t)*u + minor*cos(t)*v`
    // occt: GC_MakeEllipse
    pub fn d1(&self, t: f64) -> [f64; 3] {
        let (u, v) = orthonormal_basis(self.normal);
        let ct = t.cos();
        let st = t.sin();
        add(scale(u, -self.major * st), scale(v, self.minor * ct))
    }

    /// Returns `true` when the construction succeeded (`self.done`).
    // occt: GC_MakeEllipse
    #[inline]
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ---------------------------------------------------------------------------
// CircleMaker
// ---------------------------------------------------------------------------

/// A 3D circle construction helper modelled on `GC_MakeCircle`.
///
/// Once constructed the circle is parameterised as:
/// ```text
/// P(t) = center + radius*cos(t)*u + radius*sin(t)*v
/// ```
/// where `u` and `v` are orthonormal vectors derived from `normal` forming a
/// right-handed frame.  The parameter range is `[0, 2*PI)`.
// occt-ref: GC_MakeCircle
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircleMaker {
    /// Centre of the circle in 3D space.
    pub center: [f64; 3],
    /// Radius of the circle.
    pub radius: f64,
    /// Unit normal to the plane of the circle (the "Z direction").
    pub normal: [f64; 3],
    /// `true` when a valid circle has been successfully constructed.
    pub done: bool,
}

impl CircleMaker {
    /// Construct a circle from a centre point, radius, and plane normal.
    ///
    /// `normal` is normalised internally.  `done` is set to `false` when the
    /// normal is degenerate (near-zero length) or when `r < 0`.
    // occt-ref: GC_MakeCircle
    pub fn from_center_radius(center: [f64; 3], r: f64, normal: [f64; 3]) -> Self {
        match normalize(normal) {
            Some(n) if r >= 0.0 => Self {
                center,
                radius: r,
                normal: n,
                done: true,
            },
            _ => Self {
                center,
                radius: r,
                normal: [0.0, 0.0, 1.0],
                done: false,
            },
        }
    }

    /// Fit the unique circle through three 3D points `p1`, `p2`, `p3`.
    ///
    /// Finds the circumscribed circle of the triangle `(p1, p2, p3)`.
    /// Returns `None` when the points are collinear or coincident.
    // occt-ref: GC_MakeCircle
    pub fn from_3_points(p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> Option<Self> {
        let (center, radius, normal) = circumscribed_circle_3d(p1, p2, p3)?;
        Some(Self {
            center,
            radius,
            normal,
            done: true,
        })
    }

    /// Evaluate the point on the circle at parameter `t` (radians).
    ///
    /// `P(t) = center + radius*cos(t)*u + radius*sin(t)*v`
    ///
    /// where `(u, v)` is the orthonormal basis in the circle plane derived
    /// from `self.normal`.
    // occt-ref: GC_MakeCircle
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        let (u, v) = orthonormal_basis(self.normal);
        let ct = t.cos();
        let st = t.sin();
        add(
            self.center,
            add(scale(u, self.radius * ct), scale(v, self.radius * st)),
        )
    }

    /// Returns `true` when the construction succeeded (`self.done`).
    // occt-ref: GC_MakeCircle
    #[inline]
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn approx_eq3(a: [f64; 3], b: [f64; 3]) -> bool {
        (a[0] - b[0]).abs() < EPS && (a[1] - b[1]).abs() < EPS && (a[2] - b[2]).abs() < EPS
    }

    // ------------------------------------------------------------------
    // EllipseMaker – from_axes
    // ------------------------------------------------------------------

    #[test]
    fn ellipse_from_axes_is_done() {
        let e = EllipseMaker::from_axes([0.0, 0.0, 0.0], 3.0, 2.0);
        assert!(e.is_done());
        assert!(approx_eq(e.major, 3.0));
        assert!(approx_eq(e.minor, 2.0));
    }

    #[test]
    fn ellipse_from_axes_invalid_radii_not_done() {
        // minor > major: invalid
        let e = EllipseMaker::from_axes([0.0, 0.0, 0.0], 1.0, 2.0);
        assert!(!e.is_done());
        // negative minor: invalid
        let e2 = EllipseMaker::from_axes([0.0, 0.0, 0.0], 3.0, -1.0);
        assert!(!e2.is_done());
    }

    #[test]
    fn ellipse_evaluate_xy_plane_endpoints() {
        // Semi-axes: major=3, minor=2 in the XY plane (default normal).
        let e = EllipseMaker::from_axes([0.0, 0.0, 0.0], 3.0, 2.0);
        let p0 = e.evaluate(0.0);
        let p_half = e.evaluate(PI / 2.0);
        // At t=0: center + major*u (u should be along X for default normal Z).
        assert!(approx_eq(p0[2], 0.0), "z at t=0: {}", p0[2]);
        // At t=PI/2: center + minor*v.
        assert!(approx_eq(p_half[2], 0.0), "z at t=pi/2: {}", p_half[2]);
        // The distances from centre match the semi-axes.
        let r0 = len(sub(p0, e.center));
        let r_half = len(sub(p_half, e.center));
        assert!(approx_eq(r0, 3.0), "major dist: {r0}");
        assert!(approx_eq(r_half, 2.0), "minor dist: {r_half}");
    }

    #[test]
    fn ellipse_d1_perpendicular_to_radius() {
        // At t=0 the tangent P'(0)= -major*sin(0)*u + minor*cos(0)*v = minor*v
        // which is perpendicular to P(0) - center = major*u.
        let e = EllipseMaker::from_axes([0.0, 0.0, 0.0], 3.0, 2.0);
        let p0 = e.evaluate(0.0);
        let dp0 = e.d1(0.0);
        let radial = sub(p0, e.center);
        let dot_val = dot(radial, dp0);
        assert!(approx_eq(dot_val, 0.0), "tangent not perpendicular: {dot_val}");
    }

    // ------------------------------------------------------------------
    // EllipseMaker – from_3_points
    // ------------------------------------------------------------------

    #[test]
    fn ellipse_from_3pts_basic() {
        // p1 = center, p2 = center + (3,0,0), p3 = centre + (0,2,0)
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [3.0, 0.0, 0.0];
        let p3 = [0.0, 2.0, 0.0];
        let e = EllipseMaker::from_3_points(p1, p2, p3).expect("should succeed");
        assert!(e.is_done());
        assert!(approx_eq3(e.center, p1));
        assert!(approx_eq(e.major, 3.0), "major={}", e.major);
        assert!(approx_eq(e.minor, 2.0), "minor={}", e.minor);
    }

    #[test]
    fn ellipse_from_3pts_collinear_returns_none() {
        // p3 is on the same line as p1-p2: minor axis has zero length.
        let e = EllipseMaker::from_3_points([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        assert!(e.is_none(), "collinear should return None");
    }

    #[test]
    fn ellipse_from_3pts_coincident_returns_none() {
        let e = EllipseMaker::from_3_points([1.0, 1.0, 0.0], [1.0, 1.0, 0.0], [2.0, 3.0, 0.0]);
        assert!(e.is_none());
    }

    // ------------------------------------------------------------------
    // CircleMaker – from_center_radius
    // ------------------------------------------------------------------

    #[test]
    fn circle_from_center_radius_is_done() {
        let c = CircleMaker::from_center_radius([0.0, 0.0, 0.0], 5.0, [0.0, 0.0, 1.0]);
        assert!(c.is_done());
        assert!(approx_eq(c.radius, 5.0));
    }

    #[test]
    fn circle_from_center_radius_zero_normal_not_done() {
        let c = CircleMaker::from_center_radius([0.0, 0.0, 0.0], 5.0, [0.0, 0.0, 0.0]);
        assert!(!c.is_done());
    }

    #[test]
    fn circle_from_center_radius_negative_r_not_done() {
        let c = CircleMaker::from_center_radius([0.0, 0.0, 0.0], -1.0, [0.0, 0.0, 1.0]);
        assert!(!c.is_done());
    }

    #[test]
    fn circle_evaluate_unit_circle_xy() {
        let c = CircleMaker::from_center_radius([0.0, 0.0, 0.0], 1.0, [0.0, 0.0, 1.0]);
        // Each point should be at distance 1 from centre and z=0.
        for i in 0..8 {
            let t = (i as f64) * PI / 4.0;
            let p = c.evaluate(t);
            let r = len(sub(p, c.center));
            assert!(approx_eq(r, 1.0), "t={t:.3}, r={r}");
            assert!(approx_eq(p[2], 0.0), "z non-zero at t={t:.3}: {}", p[2]);
        }
    }

    // ------------------------------------------------------------------
    // CircleMaker – from_3_points
    // ------------------------------------------------------------------

    #[test]
    fn circle_from_3pts_unit_circle() {
        // Three points on the unit circle in the XY plane.
        let p1 = [1.0, 0.0, 0.0];
        let p2 = [0.0, 1.0, 0.0];
        let p3 = [-1.0, 0.0, 0.0];
        let c = CircleMaker::from_3_points(p1, p2, p3).expect("should succeed");
        assert!(c.is_done());
        assert!(approx_eq(len(c.center), 0.0), "center={:?}", c.center);
        assert!(approx_eq(c.radius, 1.0), "radius={}", c.radius);
    }

    #[test]
    fn circle_from_3pts_arbitrary_radius() {
        // Three points on a circle of radius 5 centred at (1,2,3) in XY plane.
        let cx = 1.0_f64;
        let cy = 2.0_f64;
        let cz = 3.0_f64;
        let r = 5.0_f64;
        let p1 = [cx + r, cy, cz];
        let p2 = [cx, cy + r, cz];
        let p3 = [cx - r, cy, cz];
        let c = CircleMaker::from_3_points(p1, p2, p3).expect("should succeed");
        assert!(approx_eq(c.center[0], cx), "cx={}", c.center[0]);
        assert!(approx_eq(c.center[1], cy), "cy={}", c.center[1]);
        assert!(approx_eq(c.center[2], cz), "cz={}", c.center[2]);
        assert!(approx_eq(c.radius, r), "r={}", c.radius);
    }

    #[test]
    fn circle_from_3pts_collinear_returns_none() {
        let c = CircleMaker::from_3_points([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        assert!(c.is_none(), "collinear should return None");
    }

    #[test]
    fn circle_from_3pts_points_lie_on_result() {
        // All three input points must lie on the resulting circle.
        let p1 = [1.0, 0.0, 0.0];
        let p2 = [0.0, 1.0, 0.0];
        let p3 = [-1.0, 0.0, 0.0];
        let c = CircleMaker::from_3_points(p1, p2, p3).unwrap();
        for p in [p1, p2, p3] {
            let d = len(sub(p, c.center));
            assert!(approx_eq(d, c.radius), "point {p:?} dist={d} != r={}", c.radius);
        }
    }

    #[test]
    fn circle_from_3pts_3d_plane() {
        // Three points on a circle in a tilted plane (XZ plane, y=0 eliminated).
        // Circle in the XZ plane at origin, radius 2.
        let r = 2.0_f64;
        let p1 = [r, 0.0, 0.0];
        let p2 = [0.0, 0.0, r];
        let p3 = [-r, 0.0, 0.0];
        let c = CircleMaker::from_3_points(p1, p2, p3).expect("should succeed");
        assert!(approx_eq(c.radius, r), "r={}", c.radius);
        // Normal should be along Y axis (±[0,1,0]).
        assert!(
            approx_eq(c.normal[1].abs(), 1.0),
            "normal not along Y: {:?}",
            c.normal
        );
    }
}

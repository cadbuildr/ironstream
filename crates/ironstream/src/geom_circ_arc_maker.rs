//! `GCE2d_MakeArcOfCircle` / `GCE2d_MakeSegment` — pure-Rust stubs for the
//! OpenCascade 2D arc-and-segment construction utilities.
//!
//! `ArcOfCircle2d` mirrors `GCE2d_MakeArcOfCircle`: it holds a circular arc
//! defined by its circumscribed circle (centre + radius) and a pair of angular
//! parameters `[start_param, end_param]` in radians (counter-clockwise).
//!
//! `Segment2d` mirrors `GCE2d_MakeSegment`: a bounded straight line between
//! two 2D points.
//!
//! The free function `arc_2d_through_3pts` is a convenience wrapper for
//! `ArcOfCircle2d::from_3_points`.
//!
//! No external crates are used; only `std` (f64 arithmetic, `f64::sin`,
//! `f64::cos`, `f64::sqrt`, `f64::atan2`).

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// ArcOfCircle2d
// ---------------------------------------------------------------------------

/// A 2D circular arc defined by the circumscribed circle and a parameter
/// range `[start_param, end_param]` in radians.
///
/// The parametrisation follows OCCT convention:
/// ```text
/// P(t) = center + radius * [cos(t), sin(t)]
/// ```
/// The arc runs counter-clockwise from `start_param` to `end_param`. If
/// `end_param < start_param` the arc wraps around through `2*PI`.
// occt-ref: GCE2d_MakeArcOfCircle
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcOfCircle2d {
    /// Centre of the circumscribed circle in the 2D plane.
    pub center: [f64; 2],
    /// Radius of the circumscribed circle (always `>= 0`).
    pub radius: f64,
    /// Angular parameter at the start of the arc (radians).
    pub start_param: f64,
    /// Angular parameter at the end of the arc (radians).
    pub end_param: f64,
}

impl ArcOfCircle2d {
    /// Construct an arc through three distinct 2D points `p1`, `p2`, `p3`.
    ///
    /// The circumscribed circle is computed by finding the circumcentre of the
    /// triangle formed by the three points (intersection of perpendicular
    /// bisectors). Returns `None` when the three points are collinear (no
    /// unique circle exists) or when two or more points coincide.
    ///
    /// The resulting arc runs counter-clockwise from `p1` to `p3` through
    /// `p2`. `start_param` is the angle of `p1`, `end_param` is the angle of
    /// `p3` adjusted so that the arc passes through `p2` in the
    /// counter-clockwise sense.
    // occt-ref: GCE2d_MakeArcOfCircle
    pub fn from_3_points(p1: [f64; 2], p2: [f64; 2], p3: [f64; 2]) -> Option<Self> {
        // Find circumcentre: intersection of the perpendicular bisectors of
        // segments p1–p2 and p2–p3.
        //
        // Mid-point of p1–p2: m1 = (p1 + p2) / 2.
        // Direction of p1–p2: d1 = p2 - p1.
        // Perpendicular bisector passes through m1 with direction perp(d1) = (-d1.y, d1.x).
        //
        // Likewise for p2–p3.
        //
        // Solve for t in:  m1 + t * perp(d1)  =  m2 + s * perp(d2)
        //
        // Cross the equation with perp(d2):
        //   t * (perp(d1) × perp(d2)) = (m2 - m1) × perp(d2)
        //
        // Note: perp(d) × perp(e) = d × e  (rotation by 90° preserves the
        // magnitude of the 2D cross product).

        let ax = p2[0] - p1[0];
        let ay = p2[1] - p1[1];
        let bx = p3[0] - p2[0];
        let by = p3[1] - p2[1];

        // 2D cross product of d1 = (ax, ay) and d2 = (bx, by).
        let denom = ax * by - ay * bx;

        if denom.abs() < 1e-14 {
            // Collinear or degenerate — no circumscribed circle.
            return None;
        }

        // Mid-points.
        let m1x = (p1[0] + p2[0]) * 0.5;
        let m1y = (p1[1] + p2[1]) * 0.5;
        let m2x = (p2[0] + p3[0]) * 0.5;
        let m2y = (p2[1] + p3[1]) * 0.5;

        // Perpendicular bisector of p1–p2 is m1 + t*(-ay, ax).
        // Perpendicular bisector of p2–p3 is m2 + s*(-by, bx).
        //
        // Cross of perp(d1) = (-ay, ax) and perp(d2) = (-by, bx):
        //   (-ay)*bx - ax*(-by) = -ay*bx + ax*by = denom  (same sign).
        //
        // (m2 - m1) × perp(d2) where × is the 2D "cross":
        //   dx = m2x - m1x,  dy = m2y - m1y
        //   (dx, dy) × (-by, bx) = dx*bx - dy*(-by) = dx*bx + dy*by
        //   Wait — 2D cross(u,v) = u.x*v.y - u.y*v.x.
        //   (dx, dy) × (-by, bx) = dx*bx - dy*(-by) = dx*bx + dy*by.

        let dx = m2x - m1x;
        let dy = m2y - m1y;
        let t = (dx * bx + dy * by) / denom;

        let cx = m1x + t * (-ay);
        let cy = m1y + t * ax;

        // Radius: distance from circumcentre to p1.
        let rx = p1[0] - cx;
        let ry = p1[1] - cy;
        let radius = (rx * rx + ry * ry).sqrt();

        if radius < 1e-14 {
            return None;
        }

        // Angular parameters.
        let theta1 = (p1[1] - cy).atan2(p1[0] - cx);
        let theta2 = (p2[1] - cy).atan2(p2[0] - cx);
        let theta3 = (p3[1] - cy).atan2(p3[0] - cx);

        // Ensure the arc goes counter-clockwise from theta1 through theta2 to
        // theta3. Normalise all angles relative to theta1.
        let two_pi = 2.0 * PI;
        let norm2 = normalize_angle(theta2 - theta1);
        let mut norm3 = normalize_angle(theta3 - theta1);

        // If theta2 (normalised) lies beyond theta3 (normalised), theta3 needs
        // to be interpreted as wrapping once more around (i.e. add 2*PI).
        if norm2 > norm3 {
            norm3 += two_pi;
        }

        let start_param = theta1;
        let end_param = theta1 + norm3;

        Some(Self {
            center: [cx, cy],
            radius,
            start_param,
            end_param,
        })
    }

    /// Evaluate the point on the arc at parameter `t` (radians).
    ///
    /// `P(t) = center + radius * [cos(t), sin(t)]`.
    ///
    /// `t` does not have to be clamped to `[start_param, end_param]`; the
    /// caller is responsible for staying within the arc range.
    // occt-ref: GCE2d_MakeArcOfCircle
    pub fn evaluate(&self, t: f64) -> [f64; 2] {
        [
            self.center[0] + self.radius * t.cos(),
            self.center[1] + self.radius * t.sin(),
        ]
    }

    /// First derivative of the arc at parameter `t` (radians).
    ///
    /// `P'(t) = radius * [-sin(t), cos(t)]`.
    // occt-ref: GCE2d_MakeArcOfCircle
    pub fn d1(&self, t: f64) -> [f64; 2] {
        [
            -self.radius * t.sin(),
            self.radius * t.cos(),
        ]
    }

    /// Arc length between `start_param` and `end_param`.
    ///
    /// For a circular arc the length is simply `radius * |delta_angle|`,
    /// where `delta_angle = end_param - start_param`.
    // occt-ref: GCE2d_MakeArcOfCircle
    pub fn length(&self) -> f64 {
        self.radius * (self.end_param - self.start_param).abs()
    }
}

// ---------------------------------------------------------------------------
// Segment2d
// ---------------------------------------------------------------------------

/// A bounded 2D line segment between two points.
///
/// The parametrisation uses `t` in `[0, 1]`:
/// ```text
/// P(t) = start + t * (end - start)
/// ```
// occt-ref: GCE2d_MakeSegment
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment2d {
    /// Start point of the segment.
    pub start: [f64; 2],
    /// End point of the segment.
    pub end: [f64; 2],
}

impl Segment2d {
    /// Construct a segment from two endpoints.
    // occt-ref: GCE2d_MakeSegment
    pub fn new(s: [f64; 2], e: [f64; 2]) -> Self {
        Self { start: s, end: e }
    }

    /// Evaluate the point at normalised parameter `t` in `[0, 1]`.
    ///
    /// `P(t) = start + t * (end - start)`.
    ///
    /// `t = 0` gives `start`, `t = 1` gives `end`.
    // occt-ref: GCE2d_MakeSegment
    pub fn evaluate(&self, t: f64) -> [f64; 2] {
        [
            self.start[0] + t * (self.end[0] - self.start[0]),
            self.start[1] + t * (self.end[1] - self.start[1]),
        ]
    }

    /// Constant first derivative vector `end - start`.
    ///
    /// This is not normalised; its magnitude equals the segment length.
    // occt-ref: GCE2d_MakeSegment
    pub fn d1(&self) -> [f64; 2] {
        [
            self.end[0] - self.start[0],
            self.end[1] - self.start[1],
        ]
    }

    /// Euclidean length of the segment.
    // occt-ref: GCE2d_MakeSegment
    pub fn length(&self) -> f64 {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        (dx * dx + dy * dy).sqrt()
    }

    /// Mid-point of the segment (`evaluate(0.5)`).
    // occt-ref: GCE2d_MakeSegment
    pub fn midpoint(&self) -> [f64; 2] {
        [
            (self.start[0] + self.end[0]) * 0.5,
            (self.start[1] + self.end[1]) * 0.5,
        ]
    }
}

// ---------------------------------------------------------------------------
// Free-function convenience wrapper
// ---------------------------------------------------------------------------

/// Convenience wrapper for `ArcOfCircle2d::from_3_points`.
///
/// Constructs the unique circular arc that passes through `p1`, `p2`, and
/// `p3` (in that order, counter-clockwise). Returns `None` when the points
/// are collinear or coincident.
// occt-ref: GCE2d_MakeArcOfCircle
pub fn arc_2d_through_3pts(
    p1: [f64; 2],
    p2: [f64; 2],
    p3: [f64; 2],
) -> Option<ArcOfCircle2d> {
    ArcOfCircle2d::from_3_points(p1, p2, p3)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Map an angle difference into `[0, 2*PI)`.
#[inline]
fn normalize_angle(mut a: f64) -> f64 {
    let two_pi = 2.0 * PI;
    while a < 0.0 {
        a += two_pi;
    }
    while a >= two_pi {
        a -= two_pi;
    }
    a
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn approx_eq2(a: [f64; 2], b: [f64; 2]) -> bool {
        (a[0] - b[0]).abs() < EPS && (a[1] - b[1]).abs() < EPS
    }

    // ------------------------------------------------------------------
    // ArcOfCircle2d
    // ------------------------------------------------------------------

    #[test]
    fn arc_from_3pts_on_unit_circle() {
        // Three points on the unit circle at 0, PI/2, PI.
        let p1 = [1.0_f64, 0.0];
        let p2 = [0.0_f64, 1.0];
        let p3 = [-1.0_f64, 0.0];
        let arc = ArcOfCircle2d::from_3_points(p1, p2, p3).expect("should succeed");
        assert!(approx_eq(arc.center[0], 0.0), "cx={}", arc.center[0]);
        assert!(approx_eq(arc.center[1], 0.0), "cy={}", arc.center[1]);
        assert!(approx_eq(arc.radius, 1.0), "r={}", arc.radius);
    }

    #[test]
    fn arc_collinear_returns_none() {
        let arc = ArcOfCircle2d::from_3_points([0.0, 0.0], [1.0, 0.0], [2.0, 0.0]);
        assert!(arc.is_none(), "collinear points must return None");
    }

    #[test]
    fn arc_evaluate_returns_point_on_circle() {
        let p1 = [1.0_f64, 0.0];
        let p2 = [0.0_f64, 1.0];
        let p3 = [-1.0_f64, 0.0];
        let arc = ArcOfCircle2d::from_3_points(p1, p2, p3).unwrap();
        let pt = arc.evaluate(arc.start_param);
        assert!(approx_eq2(pt, p1), "start point mismatch: {pt:?}");
    }

    #[test]
    fn arc_d1_is_tangent() {
        // For a unit circle, the tangent at angle t is (-sin t, cos t).
        let arc = ArcOfCircle2d {
            center: [0.0, 0.0],
            radius: 1.0,
            start_param: 0.0,
            end_param: PI,
        };
        let t = PI / 4.0;
        let d = arc.d1(t);
        assert!(approx_eq(d[0], -t.sin()), "dx={}", d[0]);
        assert!(approx_eq(d[1], t.cos()), "dy={}", d[1]);
    }

    #[test]
    fn arc_length_semicircle() {
        let arc = ArcOfCircle2d {
            center: [0.0, 0.0],
            radius: 2.0,
            start_param: 0.0,
            end_param: PI,
        };
        // Length of a semicircle with r=2 is PI*2.
        assert!(approx_eq(arc.length(), 2.0 * PI), "len={}", arc.length());
    }

    // ------------------------------------------------------------------
    // Segment2d
    // ------------------------------------------------------------------

    #[test]
    fn segment_evaluate_endpoints() {
        let s = Segment2d::new([1.0, 2.0], [5.0, 6.0]);
        assert!(approx_eq2(s.evaluate(0.0), [1.0, 2.0]));
        assert!(approx_eq2(s.evaluate(1.0), [5.0, 6.0]));
    }

    #[test]
    fn segment_d1_is_direction_vector() {
        let s = Segment2d::new([0.0, 0.0], [3.0, 4.0]);
        assert!(approx_eq2(s.d1(), [3.0, 4.0]));
    }

    #[test]
    fn segment_length_is_euclidean() {
        let s = Segment2d::new([0.0, 0.0], [3.0, 4.0]);
        assert!(approx_eq(s.length(), 5.0), "len={}", s.length());
    }

    #[test]
    fn segment_midpoint() {
        let s = Segment2d::new([0.0, 0.0], [4.0, 6.0]);
        assert!(approx_eq2(s.midpoint(), [2.0, 3.0]));
    }

    #[test]
    fn segment_zero_length() {
        let s = Segment2d::new([1.0, 1.0], [1.0, 1.0]);
        assert!(approx_eq(s.length(), 0.0));
        assert!(approx_eq2(s.midpoint(), [1.0, 1.0]));
    }

    // ------------------------------------------------------------------
    // Free function
    // ------------------------------------------------------------------

    #[test]
    fn arc_2d_through_3pts_matches_method() {
        let p1 = [1.0_f64, 0.0];
        let p2 = [0.0_f64, 1.0];
        let p3 = [-1.0_f64, 0.0];
        let via_fn = arc_2d_through_3pts(p1, p2, p3).unwrap();
        let via_method = ArcOfCircle2d::from_3_points(p1, p2, p3).unwrap();
        assert_eq!(via_fn, via_method);
    }
}

// FILE: crates/ironstream/src/geom_curve2d_line.rs
//
// Pure-Rust, zero-dependency stub modelled after OpenCascade's Geom2d_Line.
//
// No external crates are used; only `std` primitives (`f64::sqrt`) are
// required.

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Dot product of two 2-vectors.
#[inline]
fn dot2(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}

/// Euclidean norm of a 2-vector.
#[inline]
fn norm2(v: [f64; 2]) -> f64 {
    f64::sqrt(dot2(v, v))
}

/// Normalise a 2-vector. Returns the zero-vector when input is degenerate.
#[inline]
fn normalize2(v: [f64; 2]) -> [f64; 2] {
    let n = norm2(v);
    if n < f64::EPSILON {
        [0.0, 0.0]
    } else {
        [v[0] / n, v[1] / n]
    }
}

/// Subtract two 2-vectors.
#[inline]
fn sub2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

/// 2-D cross product (scalar).  Positive when `b` is to the left of `a`.
#[inline]
fn cross2(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}

// ---------------------------------------------------------------------------
// Line2d
// ---------------------------------------------------------------------------

// occt: Geom2d_Line
/// An infinite parameterised line in the 2-D plane.
///
/// The line is represented by an `origin` point and a unit `direction` vector.
/// The parametric equation is:
/// ```text
/// P(t) = origin + t * direction
/// ```
///
/// Modelled after `Geom2d_Line` from OpenCascade Technology.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2d {
    /// A point on the line (the point corresponding to parameter `t = 0`).
    pub origin: [f64; 2],
    /// Unit direction vector of the line.
    pub direction: [f64; 2],
}

impl Line2d {
    // occt: Geom2d_Line::Geom2d_Line(P, V)
    /// Construct a `Line2d` from an origin point and a direction vector.
    ///
    /// The supplied `dir` is normalised; if it is degenerate the stored
    /// direction is the zero vector (callers should ensure `dir` is non-zero).
    pub fn new(origin: [f64; 2], dir: [f64; 2]) -> Self {
        Line2d {
            origin,
            direction: normalize2(dir),
        }
    }

    // occt: Geom2d_Line — constructed through GCE2d_MakeLine(P1, P2)
    /// Construct a `Line2d` passing through two distinct points.
    ///
    /// The direction runs from `p1` to `p2`.  If the two points coincide the
    /// stored direction is the zero vector.
    pub fn from_2pts(p1: [f64; 2], p2: [f64; 2]) -> Self {
        Line2d::new(p1, sub2(p2, p1))
    }

    // occt: Geom2d_Line::D0 / Value
    /// Evaluate the line at parameter `t`.
    ///
    /// ```text
    /// P(t) = origin + t * direction
    /// ```
    pub fn point_at(&self, t: f64) -> [f64; 2] {
        [
            self.origin[0] + t * self.direction[0],
            self.origin[1] + t * self.direction[1],
        ]
    }

    // occt: Geom2d_Line::D1
    /// First derivative at parameter `_t`.
    ///
    /// For a line this is constant and equals `direction` regardless of `_t`.
    pub fn d1(&self, _t: f64) -> [f64; 2] {
        self.direction
    }

    // occt: Geom2d_Line::D2
    /// Second derivative at parameter `_t`.
    ///
    /// For a line this is always the zero vector (zero curvature).
    pub fn d2(&self, _t: f64) -> [f64; 2] {
        [0.0, 0.0]
    }

    // occt: Geom2d_Line — parameter of projection, analogous to
    //       GeomAPI_ProjectPointOnCurve in 2-D
    /// Return the parameter `t` of the foot of the perpendicular from `pt`
    /// onto the line.
    ///
    /// ```text
    /// t = dot(pt - origin, direction)
    /// ```
    pub fn project(&self, pt: [f64; 2]) -> f64 {
        dot2(sub2(pt, self.origin), self.direction)
    }

    // occt: Geom2d_Line::Distance
    /// Signed-free distance from point `pt` to the line.
    ///
    /// Computed as the magnitude of the component of `(pt − origin)`
    /// perpendicular to `direction`.
    pub fn distance_to(&self, pt: [f64; 2]) -> f64 {
        let v = sub2(pt, self.origin);
        let t = dot2(v, self.direction);
        let foot = [
            self.origin[0] + t * self.direction[0],
            self.origin[1] + t * self.direction[1],
        ];
        norm2(sub2(pt, foot))
    }

    // occt: Geom2d_Line::Reverse
    /// Reverse the orientation of the line in-place.
    ///
    /// After calling `reverse`, the parametric direction is negated:
    /// the origin is unchanged but `direction` becomes `-direction`.
    pub fn reverse(&mut self) {
        self.direction = [-self.direction[0], -self.direction[1]];
    }

    // occt: Geom2d_Line — signed side test (no direct OCCT counterpart, but
    //       commonly derived from Geom2d_Line geometry)
    /// Return a signed value indicating which side of the line `pt` lies on.
    ///
    /// The sign equals `cross(direction, pt − origin)`:
    /// - Positive: `pt` is to the left of the line (in the direction of travel).
    /// - Negative: `pt` is to the right.
    /// - Zero: `pt` is on the line.
    pub fn side(&self, pt: [f64; 2]) -> f64 {
        cross2(self.direction, sub2(pt, self.origin))
    }

    // occt: Geom2d_Line — line/line intersection analogous to
    //       Geom2dAPI_InterCurveCurve for two lines
    /// Compute the parameter `t` on `self` at which `self` and `other`
    /// intersect.
    ///
    /// Returns `None` when the lines are parallel or coincident (the
    /// cross product of their directions is below `f64::EPSILON`).
    ///
    /// The returned value satisfies `self.point_at(t) ≈ other.point_at(s)` for
    /// some `s`.
    pub fn intersect(&self, other: &Line2d) -> Option<f64> {
        // Solve: origin + t * direction = other.origin + s * other.direction
        //
        // Taking the 2-D cross product with other.direction on both sides:
        //   cross(direction, other.direction) * t =
        //       cross(other.origin - origin, other.direction)
        let denom = cross2(self.direction, other.direction);
        if denom.abs() < f64::EPSILON {
            return None;
        }
        let delta = sub2(other.origin, self.origin);
        let t = cross2(delta, other.direction) / denom;
        Some(t)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    fn vec2_approx_eq(a: [f64; 2], b: [f64; 2]) -> bool {
        approx_eq(a[0], b[0]) && approx_eq(a[1], b[1])
    }

    #[test]
    fn new_normalises_direction() {
        let l = Line2d::new([0.0, 0.0], [3.0, 4.0]);
        assert!(approx_eq(norm2(l.direction), 1.0), "direction should be unit");
        assert!(approx_eq(l.direction[0], 0.6));
        assert!(approx_eq(l.direction[1], 0.8));
    }

    #[test]
    fn from_2pts_direction() {
        let l = Line2d::from_2pts([0.0, 0.0], [1.0, 0.0]);
        assert!(vec2_approx_eq(l.direction, [1.0, 0.0]));
        assert!(vec2_approx_eq(l.origin, [0.0, 0.0]));
    }

    #[test]
    fn point_at_origin_and_unit() {
        let l = Line2d::new([1.0, 2.0], [1.0, 0.0]);
        assert!(vec2_approx_eq(l.point_at(0.0), [1.0, 2.0]));
        assert!(vec2_approx_eq(l.point_at(3.0), [4.0, 2.0]));
        assert!(vec2_approx_eq(l.point_at(-1.0), [0.0, 2.0]));
    }

    #[test]
    fn d1_is_direction() {
        let l = Line2d::new([0.0, 0.0], [0.0, 1.0]);
        assert!(vec2_approx_eq(l.d1(99.0), l.direction));
    }

    #[test]
    fn d2_is_zero() {
        let l = Line2d::new([0.0, 0.0], [1.0, 1.0]);
        assert!(vec2_approx_eq(l.d2(0.0), [0.0, 0.0]));
        assert!(vec2_approx_eq(l.d2(5.0), [0.0, 0.0]));
    }

    #[test]
    fn project_foot_of_perpendicular() {
        // Horizontal line y=0; project point (3, 4).
        let l = Line2d::new([0.0, 0.0], [1.0, 0.0]);
        assert!(approx_eq(l.project([3.0, 4.0]), 3.0));
        assert!(approx_eq(l.project([-2.0, 7.0]), -2.0));
    }

    #[test]
    fn distance_to_horizontal_line() {
        let l = Line2d::new([0.0, 0.0], [1.0, 0.0]);
        assert!(approx_eq(l.distance_to([3.0, 4.0]), 4.0));
        assert!(approx_eq(l.distance_to([0.0, -5.0]), 5.0));
        assert!(approx_eq(l.distance_to([1.0, 0.0]), 0.0));
    }

    #[test]
    fn distance_to_diagonal_line() {
        // Line through origin at 45°; distance from (1,0) should be 1/sqrt(2).
        let l = Line2d::new([0.0, 0.0], [1.0, 1.0]);
        let expected = 1.0 / f64::sqrt(2.0);
        let got = l.distance_to([1.0, 0.0]);
        assert!(
            approx_eq(got, expected),
            "expected {expected}, got {got}"
        );
    }

    #[test]
    fn reverse_negates_direction() {
        let mut l = Line2d::new([1.0, 2.0], [1.0, 0.0]);
        l.reverse();
        assert!(vec2_approx_eq(l.direction, [-1.0, 0.0]));
        // Origin unchanged.
        assert!(vec2_approx_eq(l.origin, [1.0, 2.0]));
    }

    #[test]
    fn side_left_right_on() {
        let l = Line2d::new([0.0, 0.0], [1.0, 0.0]);
        assert!(l.side([0.0, 1.0]) > 0.0, "above line should be positive");
        assert!(l.side([0.0, -1.0]) < 0.0, "below line should be negative");
        assert!(approx_eq(l.side([5.0, 0.0]), 0.0), "on line should be zero");
    }

    #[test]
    fn intersect_perpendicular_lines() {
        // x-axis and y-axis intersect at origin.
        let lx = Line2d::new([0.0, 0.0], [1.0, 0.0]);
        let ly = Line2d::new([0.0, 0.0], [0.0, 1.0]);
        let t = lx.intersect(&ly).expect("should intersect");
        assert!(approx_eq(t, 0.0));
        assert!(vec2_approx_eq(lx.point_at(t), [0.0, 0.0]));
    }

    #[test]
    fn intersect_offset_lines() {
        // l1: origin (0,0) going right; l2: origin (5,0) going up.
        // Intersection at (5, 0) → t = 5 on l1.
        let l1 = Line2d::new([0.0, 0.0], [1.0, 0.0]);
        let l2 = Line2d::new([5.0, 0.0], [0.0, 1.0]);
        let t = l1.intersect(&l2).expect("should intersect");
        assert!(approx_eq(t, 5.0), "t should be 5, got {t}");
        let pt = l1.point_at(t);
        assert!(vec2_approx_eq(pt, [5.0, 0.0]), "intersection point {pt:?}");
    }

    #[test]
    fn intersect_parallel_returns_none() {
        let l1 = Line2d::new([0.0, 0.0], [1.0, 0.0]);
        let l2 = Line2d::new([0.0, 1.0], [1.0, 0.0]);
        assert!(l1.intersect(&l2).is_none(), "parallel lines should not intersect");
    }

    #[test]
    fn intersect_diagonal() {
        // l1: y = x  (origin 0,0, dir (1,1)/sqrt2)
        // l2: y = -x + 2  i.e. origin (2,0), dir (-1,1)/sqrt2
        // Intersection: x = x → x = 1, y = 1  → t on l1 = sqrt(2)
        let l1 = Line2d::new([0.0, 0.0], [1.0, 1.0]);
        let l2 = Line2d::new([2.0, 0.0], [-1.0, 1.0]);
        let t = l1.intersect(&l2).expect("should intersect");
        let pt = l1.point_at(t);
        assert!(
            approx_eq(pt[0], 1.0) && approx_eq(pt[1], 1.0),
            "intersection should be (1,1), got {pt:?}"
        );
    }
}

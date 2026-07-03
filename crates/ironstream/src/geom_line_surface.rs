//! Geometry primitives: `GeomLine` and `GeomPlane`.
//!
//! Lightweight, zero-dependency stubs that mirror the essential behaviour of
//! OpenCascade's `Geom_Line` and `Geom_Plane`.  All positions and directions
//! are represented as raw `[f64; 3]` arrays so that this module can be used
//! without pulling in the rest of the crate's `gp` layer.
//!
//! Parameterization conventions follow OCCT:
//! - `GeomLine`: `P(t) = origin + t * direction`
//! - `GeomPlane`: `P(u, v) = origin + u * u_dir + v * v_dir`

// ─────────────────────────── helpers ────────────────────────────────────────

#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[inline]
fn norm(v: [f64; 3]) -> f64 {
    f64::sqrt(dot(v, v))
}

#[inline]
fn normalize(v: [f64; 3]) -> [f64; 3] {
    let n = norm(v);
    if n == 0.0 {
        v
    } else {
        scale(v, 1.0 / n)
    }
}

#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Build an arbitrary unit vector orthogonal to `v`.
fn any_orthogonal(v: [f64; 3]) -> [f64; 3] {
    // Pick the axis least aligned with v to avoid near-zero cross products.
    let abs = [v[0].abs(), v[1].abs(), v[2].abs()];
    let candidate = if abs[0] <= abs[1] && abs[0] <= abs[2] {
        [1.0, 0.0, 0.0]
    } else if abs[1] <= abs[2] {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };
    normalize(cross(v, candidate))
}

// ─────────────────────────── GeomLine ───────────────────────────────────────

/// An infinite parameterized 3D line defined by an origin and a unit direction.
///
/// Parameterization: `P(t) = origin + t * direction`.
///
/// Mirrors the essential interface of OpenCascade's `Geom_Line`.
// occt: Geom_Line
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomLine {
    /// The origin point of the line (the position at parameter `t = 0`).
    pub origin: [f64; 3],
    /// The unit direction vector of the line.
    pub direction: [f64; 3],
}

impl GeomLine {
    /// Construct a new `GeomLine` from an origin point and a direction vector.
    ///
    /// The direction is **normalized** on construction, matching the OCCT
    /// convention that `Geom_Line` always stores a unit-length axis.
    pub fn new(origin: [f64; 3], dir: [f64; 3]) -> Self {
        Self {
            origin,
            direction: normalize(dir),
        }
    }

    /// Evaluate the point on the line at parameter `t`.
    ///
    /// `P(t) = origin + t * direction` — mirrors `Geom_Line::D0`.
    pub fn point_at(&self, t: f64) -> [f64; 3] {
        add(self.origin, scale(self.direction, t))
    }

    /// First derivative at parameter `t`.
    ///
    /// For an infinite line the first derivative is constant and equals the
    /// unit direction vector — mirrors `Geom_Line::D1`.  The `t` parameter is
    /// accepted for API uniformity but is not used.
    pub fn d1(&self, _t: f64) -> [f64; 3] {
        self.direction
    }

    /// Return the unit direction of the line — mirrors `Geom_Line::Direction`.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    /// Reverse the orientation of the line in place by negating its direction.
    ///
    /// Mirrors `Geom_Line::Reverse`.
    pub fn reverse(&mut self) {
        self.direction = scale(self.direction, -1.0);
    }
}

// ─────────────────────────── GeomPlane ──────────────────────────────────────

/// An infinite parameterized 3D plane defined by an origin and a unit normal.
///
/// The plane is implicitly `{ p : dot(p - origin, normal) = 0 }` and is
/// parameterized as `P(u, v) = origin + u * u_dir + v * v_dir`, where
/// `u_dir` and `v_dir` are internally-computed unit vectors orthogonal to
/// `normal` and to each other.
///
/// Mirrors the essential interface of OpenCascade's `Geom_Plane`.
// occt: Geom_Plane
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomPlane {
    /// The origin (location) of the plane.
    pub origin: [f64; 3],
    /// The unit outward normal of the plane (`ZDir` in OCCT terms).
    pub normal: [f64; 3],
    /// The U-parametric axis (`XDir`), orthogonal to `normal`.
    u_dir: [f64; 3],
    /// The V-parametric axis (`YDir`), orthogonal to both `normal` and `u_dir`.
    v_dir: [f64; 3],
}

impl GeomPlane {
    /// Construct a `GeomPlane` from an origin point and a normal vector.
    ///
    /// The normal is normalized; two arbitrary orthonormal tangent vectors
    /// are derived automatically, matching the OCCT behaviour where
    /// `Geom_Plane(const gp_Pnt&, const gp_Dir&)` builds a full `Ax3`.
    pub fn new(origin: [f64; 3], normal: [f64; 3]) -> Self {
        let n = normalize(normal);
        let u = any_orthogonal(n);
        let v = normalize(cross(n, u));
        Self {
            origin,
            normal: n,
            u_dir: u,
            v_dir: v,
        }
    }

    /// Evaluate the point on the plane at parameters `(u, v)`.
    ///
    /// `P(u, v) = origin + u * u_dir + v * v_dir` — mirrors `Geom_Plane::D0`.
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        add(self.origin, add(scale(self.u_dir, u), scale(self.v_dir, v)))
    }

    /// Project a 3D point onto the plane and return its `(u, v)` parameters.
    ///
    /// The foot of the perpendicular from `p` to the plane satisfies
    /// `foot = p - dot(p - origin, normal) * normal`; the returned `(u, v)`
    /// are the coordinates of `foot` in the local frame.
    ///
    /// Mirrors `gp_Pln::Parameters`.
    pub fn project_point(&self, p: [f64; 3]) -> [f64; 2] {
        let d = sub(p, self.origin);
        let u = dot(d, self.u_dir);
        let v = dot(d, self.v_dir);
        [u, v]
    }

    /// Signed distance from point `p` to the plane.
    ///
    /// The sign is positive on the side of the outward normal.
    /// Mirrors `gp_Pln::Distance`.
    pub fn distance(&self, p: [f64; 3]) -> f64 {
        dot(sub(p, self.origin), self.normal)
    }

    /// Return the unit normal of the plane — mirrors `Geom_Plane::Normal`.
    pub fn normal(&self) -> [f64; 3] {
        self.normal
    }
}

// ────────────────────────── free functions ───────────────────────────────────

/// Compute the intersection of an infinite line with an infinite plane.
///
/// Returns `Some(point)` when the line is not parallel to the plane (i.e.
/// `|dot(direction, normal)| > 0`), or `None` when the line lies in or is
/// parallel to the plane.
///
/// The intersection parameter satisfies
/// `t = dot(plane.origin - line.origin, plane.normal) / dot(line.direction, plane.normal)`.
///
/// Mirrors the behaviour of `IntAna_IntConicQuad` for the line/plane case.
pub fn line_plane_intersect(line: &GeomLine, plane: &GeomPlane) -> Option<[f64; 3]> {
    let denom = dot(line.direction, plane.normal);
    if denom == 0.0 {
        return None;
    }
    let t = dot(sub(plane.origin, line.origin), plane.normal) / denom;
    Some(line.point_at(t))
}

/// Compute the shortest distance between two infinite lines in 3D.
///
/// Uses the standard formula:
/// `distance = |dot(l2.origin - l1.origin, cross(l1.direction, l2.direction))| / |cross|`.
///
/// When the lines are parallel (`|cross| ≈ 0`) the function falls back to the
/// point-to-line distance.
///
/// Mirrors the functionality of `Extrema_ExtCC` for two lines.
pub fn line_line_distance(l1: &GeomLine, l2: &GeomLine) -> f64 {
    let c = cross(l1.direction, l2.direction);
    let c_len = norm(c);
    let w = sub(l2.origin, l1.origin);
    if c_len == 0.0 {
        // Lines are parallel; return point-to-line distance.
        let cross_w = cross(w, l1.direction);
        return norm(cross_w); // |l1.direction| == 1 by construction
    }
    f64::abs(dot(w, c)) / c_len
}

// ─────────────────────────── tests ──────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn approx_eq3(a: [f64; 3], b: [f64; 3]) -> bool {
        approx_eq(a[0], b[0]) && approx_eq(a[1], b[1]) && approx_eq(a[2], b[2])
    }

    // ── GeomLine ─────────────────────────────────────────────────────────────

    #[test]
    fn geom_line_point_at() {
        let l = GeomLine::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(approx_eq3(l.point_at(3.0), [3.0, 0.0, 0.0]));
        assert!(approx_eq3(l.point_at(-2.0), [-2.0, 0.0, 0.0]));
    }

    #[test]
    fn geom_line_d1_is_direction() {
        let l = GeomLine::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);
        assert!(approx_eq3(l.d1(99.0), l.direction()));
    }

    #[test]
    fn geom_line_normalizes_direction() {
        let l = GeomLine::new([0.0; 3], [3.0, 0.0, 0.0]);
        assert!(approx_eq(norm(l.direction), 1.0));
    }

    #[test]
    fn geom_line_reverse() {
        let mut l = GeomLine::new([0.0; 3], [1.0, 0.0, 0.0]);
        l.reverse();
        assert!(approx_eq3(l.direction(), [-1.0, 0.0, 0.0]));
    }

    // ── GeomPlane ────────────────────────────────────────────────────────────

    #[test]
    fn geom_plane_evaluate_origin() {
        let p = GeomPlane::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);
        // P(0, 0) must equal the origin.
        assert!(approx_eq3(p.evaluate(0.0, 0.0), [1.0, 2.0, 3.0]));
    }

    #[test]
    fn geom_plane_evaluate_lies_on_plane() {
        let plane = GeomPlane::new([0.0; 3], [0.0, 1.0, 0.0]);
        let pt = plane.evaluate(3.0, -2.0);
        // Any evaluated point must satisfy the plane equation.
        assert!(approx_eq(plane.distance(pt), 0.0));
    }

    #[test]
    fn geom_plane_project_and_distance() {
        let plane = GeomPlane::new([0.0; 3], [0.0, 0.0, 1.0]);
        let p = [4.0, -3.0, 7.0];
        assert!(approx_eq(plane.distance(p), 7.0));
        let [u, v] = plane.project_point(p);
        // Projected point must evaluate back to the foot.
        let foot = plane.evaluate(u, v);
        assert!(approx_eq3(foot, [4.0, -3.0, 0.0]));
    }

    #[test]
    fn geom_plane_normal_is_unit() {
        let plane = GeomPlane::new([0.0; 3], [1.0, 2.0, 3.0]);
        assert!(approx_eq(norm(plane.normal()), 1.0));
    }

    // ── line_plane_intersect ─────────────────────────────────────────────────

    #[test]
    fn line_plane_intersect_basic() {
        let line = GeomLine::new([0.0, 0.0, -5.0], [0.0, 0.0, 1.0]);
        let plane = GeomPlane::new([0.0; 3], [0.0, 0.0, 1.0]);
        let pt = line_plane_intersect(&line, &plane).expect("should intersect");
        assert!(approx_eq3(pt, [0.0, 0.0, 0.0]));
    }

    #[test]
    fn line_plane_intersect_parallel_returns_none() {
        let line = GeomLine::new([0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
        let plane = GeomPlane::new([0.0; 3], [0.0, 0.0, 1.0]);
        assert!(line_plane_intersect(&line, &plane).is_none());
    }

    // ── line_line_distance ───────────────────────────────────────────────────

    #[test]
    fn line_line_distance_skew() {
        // Classic skew lines: l1 along X at z=0, l2 along Y at z=1.
        let l1 = GeomLine::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let l2 = GeomLine::new([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]);
        assert!(approx_eq(line_line_distance(&l1, &l2), 1.0));
    }

    #[test]
    fn line_line_distance_parallel() {
        let l1 = GeomLine::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let l2 = GeomLine::new([0.0, 3.0, 4.0], [1.0, 0.0, 0.0]);
        assert!(approx_eq(line_line_distance(&l1, &l2), 5.0));
    }

    #[test]
    fn line_line_distance_intersecting() {
        let l1 = GeomLine::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let l2 = GeomLine::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!(approx_eq(line_line_distance(&l1, &l2), 0.0));
    }
}

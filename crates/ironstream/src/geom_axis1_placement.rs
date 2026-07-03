// FILE: geom_axis1_placement.rs
//! Placement objects in 3D Geom — Axis1 and Axis2 placements.
//!
//! Ports Geom_Axis1Placement and Geom_Axis2Placement from OCCT.

// occt: Geom_Axis1Placement
/// A placement object defined by a location point and a direction vector.
/// Wraps gp_Ax1 semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct GeomAxis1Placement {
    location: [f64; 3],
    direction: [f64; 3],
}

// occt: Geom_Axis2Placement
/// A placement object defined by a location point, a main direction, and an X direction.
/// Wraps gp_Ax2 semantics (right-handed coordinate system).
#[derive(Debug, Clone, PartialEq)]
pub struct GeomAxis2Placement {
    location: [f64; 3],
    direction: [f64; 3],   // main axis (Z)
    x_direction: [f64; 3], // X axis
    y_direction: [f64; 3], // Y axis (derived: dir cross x_dir)
}

// ── helpers ─────────────────────────────────────────────────────────────────

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let mag = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if mag < 1e-300 {
        panic!("cannot normalize a zero-length vector");
    }
    [v[0] / mag, v[1] / mag, v[2] / mag]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn magnitude(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn scale_v(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

/// Rotate vector `v` around unit axis `k` by angle `angle` (radians), Rodrigues formula.
fn rotate_vec(v: [f64; 3], k: [f64; 3], angle: f64) -> [f64; 3] {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let kv = dot(k, v);
    let c = cross(k, v);
    [
        v[0] * cos_a + c[0] * sin_a + k[0] * kv * (1.0 - cos_a),
        v[1] * cos_a + c[1] * sin_a + k[1] * kv * (1.0 - cos_a),
        v[2] * cos_a + c[2] * sin_a + k[2] * kv * (1.0 - cos_a),
    ]
}

/// Mirror a point through a plane defined by point `p` and unit normal `n`.
fn mirror_point_through_plane(pt: [f64; 3], p: [f64; 3], n: [f64; 3]) -> [f64; 3] {
    let d = dot(sub(pt, p), n);
    sub(pt, scale_v(n, 2.0 * d))
}

/// Mirror a direction through a plane with unit normal `n`.
fn mirror_dir_through_plane(dir: [f64; 3], n: [f64; 3]) -> [f64; 3] {
    let d = dot(dir, n);
    sub(dir, scale_v(n, 2.0 * d))
}

/// Mirror a point through a line defined by point `p` and unit direction `d`.
fn mirror_point_through_line(pt: [f64; 3], p: [f64; 3], d: [f64; 3]) -> [f64; 3] {
    let v = sub(pt, p);
    let proj = dot(v, d);
    let foot = add(p, scale_v(d, proj));
    sub(scale_v(foot, 2.0), pt)
}

/// Mirror a direction through a line direction `d` (reflects transverse component).
fn mirror_dir_through_line(dir: [f64; 3], d: [f64; 3]) -> [f64; 3] {
    let proj = dot(dir, d);
    sub(scale_v(d, 2.0 * proj), dir)
}

/// Angle between two unit vectors, clamped to [0, pi].
fn angle_between_dirs(a: [f64; 3], b: [f64; 3]) -> f64 {
    let c = dot(a, b).clamp(-1.0, 1.0);
    c.acos()
}

// ── Ax1 / Ax2 simple tuple types ─────────────────────────────────────────────

/// Minimal gp_Ax1 equivalent returned by ax1().
pub struct Ax1 {
    pub location: [f64; 3],
    pub direction: [f64; 3],
}

/// Minimal gp_Ax2 equivalent returned by ax2().
pub struct Ax2 {
    pub location: [f64; 3],
    pub direction: [f64; 3],
    pub x_direction: [f64; 3],
    pub y_direction: [f64; 3],
}

// ── GeomAxis1Placement ────────────────────────────────────────────────────────

impl GeomAxis1Placement {
    /// Create a new axis placement from a location point and direction.
    /// The direction is normalised.
    pub fn new(location: [f64; 3], direction: [f64; 3]) -> Self {
        Self {
            location,
            direction: normalize(direction),
        }
    }

    /// Return the location point.
    pub fn location(&self) -> [f64; 3] {
        self.location
    }

    /// Return the unit direction.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    /// Return the underlying Ax1 value.
    pub fn ax1(&self) -> Ax1 {
        Ax1 {
            location: self.location,
            direction: self.direction,
        }
    }

    /// Set a new direction (normalised).
    pub fn set_direction(&mut self, direction: [f64; 3]) {
        self.direction = normalize(direction);
    }

    /// Set a new location point.
    pub fn set_location(&mut self, location: [f64; 3]) {
        self.location = location;
    }

    /// Reverse the direction of the axis (negate).
    pub fn reverse(&mut self) {
        self.direction = [-self.direction[0], -self.direction[1], -self.direction[2]];
    }

    /// Return a reversed copy.
    pub fn reversed(&self) -> Self {
        let mut c = self.clone();
        c.reverse();
        c
    }

    /// True if `self` and `other` are coaxial within tolerances.
    /// `ang_tol` in radians, `lin_tol` in length units.
    pub fn is_coaxial(&self, other: &Self, ang_tol: f64, lin_tol: f64) -> bool {
        if !self.is_parallel(other, ang_tol) {
            return false;
        }
        // Distance between the two lines.
        let diff = sub(other.location, self.location);
        let proj = dot(diff, self.direction);
        let perp = sub(diff, scale_v(self.direction, proj));
        magnitude(perp) <= lin_tol
    }

    /// True if `self` and `other` are normal (perpendicular) within `ang_tol`.
    pub fn is_normal(&self, other: &Self, ang_tol: f64) -> bool {
        let a = angle_between_dirs(self.direction, other.direction);
        (a - std::f64::consts::FRAC_PI_2).abs() <= ang_tol
    }

    /// True if `self` and `other` are anti-parallel within `ang_tol`.
    pub fn is_opposite(&self, other: &Self, ang_tol: f64) -> bool {
        let a = angle_between_dirs(self.direction, other.direction);
        (a - std::f64::consts::PI).abs() <= ang_tol
    }

    /// True if `self` and `other` are parallel (or anti-parallel) within `ang_tol`.
    pub fn is_parallel(&self, other: &Self, ang_tol: f64) -> bool {
        let a = angle_between_dirs(self.direction, other.direction);
        a <= ang_tol || (a - std::f64::consts::PI).abs() <= ang_tol
    }

    // ── transformations ───────────────────────────────────────────────────────

    /// Mirror through a point.
    pub fn mirror_through_point(&self, point: [f64; 3]) -> Self {
        let new_loc = [
            2.0 * point[0] - self.location[0],
            2.0 * point[1] - self.location[1],
            2.0 * point[2] - self.location[2],
        ];
        // Direction is reversed on point mirroring.
        let new_dir = [-self.direction[0], -self.direction[1], -self.direction[2]];
        Self {
            location: new_loc,
            direction: new_dir,
        }
    }

    /// Mirror through an axis (line).
    /// `axis_loc` is a point on the mirror axis, `axis_dir` is its unit direction.
    pub fn mirror_through_axis(&self, axis_loc: [f64; 3], axis_dir: [f64; 3]) -> Self {
        let axis_dir = normalize(axis_dir);
        let new_loc = mirror_point_through_line(self.location, axis_loc, axis_dir);
        let new_dir = mirror_dir_through_line(self.direction, axis_dir);
        Self {
            location: new_loc,
            direction: normalize(new_dir),
        }
    }

    /// Mirror through a plane.
    /// `plane_loc` is a point on the plane, `plane_normal` is the unit normal.
    pub fn mirror_through_plane(&self, plane_loc: [f64; 3], plane_normal: [f64; 3]) -> Self {
        let plane_normal = normalize(plane_normal);
        let new_loc = mirror_point_through_plane(self.location, plane_loc, plane_normal);
        let new_dir = mirror_dir_through_plane(self.direction, plane_normal);
        Self {
            location: new_loc,
            direction: normalize(new_dir),
        }
    }

    /// Rotate around an axis by `angle` radians.
    pub fn rotate(&self, axis_loc: [f64; 3], axis_dir: [f64; 3], angle: f64) -> Self {
        let axis_dir = normalize(axis_dir);
        let v = sub(self.location, axis_loc);
        let rotated_v = rotate_vec(v, axis_dir, angle);
        let new_loc = add(axis_loc, rotated_v);
        let new_dir = rotate_vec(self.direction, axis_dir, angle);
        Self {
            location: new_loc,
            direction: normalize(new_dir),
        }
    }

    /// Scale from a centre point by factor `factor`.
    pub fn scale(&self, centre: [f64; 3], factor: f64) -> Self {
        let new_loc = add(centre, scale_v(sub(self.location, centre), factor));
        // Scaling does not change direction (may reverse if factor < 0).
        let new_dir = if factor < 0.0 {
            [-self.direction[0], -self.direction[1], -self.direction[2]]
        } else {
            self.direction
        };
        Self {
            location: new_loc,
            direction: new_dir,
        }
    }

    /// Translate by a vector.
    pub fn translate(&self, vector: [f64; 3]) -> Self {
        Self {
            location: add(self.location, vector),
            direction: self.direction,
        }
    }

    /// Translate from point `p1` to point `p2`.
    pub fn translate_from_to(&self, p1: [f64; 3], p2: [f64; 3]) -> Self {
        self.translate(sub(p2, p1))
    }
}

// ── GeomAxis2Placement ────────────────────────────────────────────────────────

impl GeomAxis2Placement {
    /// Create from location, main direction (Z), and X direction.
    /// The X direction is orthogonalised against the main direction.
    /// Panics if xdir is parallel to direction.
    pub fn new(location: [f64; 3], direction: [f64; 3], xdir: [f64; 3]) -> Self {
        let direction = normalize(direction);
        // Gram-Schmidt: remove component of xdir along direction.
        let xdir_raw = xdir;
        let d_comp = dot(xdir_raw, direction);
        let xdir_ortho = sub(xdir_raw, scale_v(direction, d_comp));
        let x_direction = normalize(xdir_ortho);
        let y_direction = normalize(cross(direction, x_direction));
        Self {
            location,
            direction,
            x_direction,
            y_direction,
        }
    }

    /// Return the location point.
    pub fn location(&self) -> [f64; 3] {
        self.location
    }

    /// Return the main (Z) direction.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    /// Return the X direction.
    pub fn x_direction(&self) -> [f64; 3] {
        self.x_direction
    }

    /// Return the Y direction (derived as direction cross x_direction).
    pub fn y_direction(&self) -> [f64; 3] {
        self.y_direction
    }

    /// Return the underlying Ax2 value.
    pub fn ax2(&self) -> Ax2 {
        Ax2 {
            location: self.location,
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
        }
    }

    /// Set a new main direction, recomputing Y from existing X.
    pub fn set_direction(&mut self, direction: [f64; 3]) {
        let direction = normalize(direction);
        let d_comp = dot(self.x_direction, direction);
        let xdir_ortho = sub(self.x_direction, scale_v(direction, d_comp));
        self.x_direction = normalize(xdir_ortho);
        self.y_direction = normalize(cross(direction, self.x_direction));
        self.direction = direction;
    }

    /// Set a new location.
    pub fn set_location(&mut self, location: [f64; 3]) {
        self.location = location;
    }

    /// Set a new X direction (orthogonalised against main direction).
    pub fn set_x_direction(&mut self, xdir: [f64; 3]) {
        let d_comp = dot(xdir, self.direction);
        let xdir_ortho = sub(xdir, scale_v(self.direction, d_comp));
        self.x_direction = normalize(xdir_ortho);
        self.y_direction = normalize(cross(self.direction, self.x_direction));
    }

    // ── transformations ───────────────────────────────────────────────────────

    /// Rotate around an Axis1 (gp_Ax1-like pair) by `angle` radians.
    pub fn rotate_around_ax1(
        &self,
        axis_loc: [f64; 3],
        axis_dir: [f64; 3],
        angle: f64,
    ) -> Self {
        let axis_dir = normalize(axis_dir);
        let v = sub(self.location, axis_loc);
        let rotated_v = rotate_vec(v, axis_dir, angle);
        let new_loc = add(axis_loc, rotated_v);
        let new_dir = normalize(rotate_vec(self.direction, axis_dir, angle));
        let new_x = normalize(rotate_vec(self.x_direction, axis_dir, angle));
        let new_y = normalize(cross(new_dir, new_x));
        Self {
            location: new_loc,
            direction: new_dir,
            x_direction: new_x,
            y_direction: new_y,
        }
    }

    /// Mirror through a point.
    pub fn mirror_through_point(&self, point: [f64; 3]) -> Self {
        let new_loc = [
            2.0 * point[0] - self.location[0],
            2.0 * point[1] - self.location[1],
            2.0 * point[2] - self.location[2],
        ];
        let new_dir = [-self.direction[0], -self.direction[1], -self.direction[2]];
        let new_x = [-self.x_direction[0], -self.x_direction[1], -self.x_direction[2]];
        let new_y = normalize(cross(new_dir, new_x));
        Self {
            location: new_loc,
            direction: normalize(new_dir),
            x_direction: normalize(new_x),
            y_direction: new_y,
        }
    }

    /// Mirror through an axis (line).
    pub fn mirror_through_axis(&self, axis_loc: [f64; 3], axis_dir: [f64; 3]) -> Self {
        let axis_dir = normalize(axis_dir);
        let new_loc = mirror_point_through_line(self.location, axis_loc, axis_dir);
        let new_dir = normalize(mirror_dir_through_line(self.direction, axis_dir));
        let new_x = normalize(mirror_dir_through_line(self.x_direction, axis_dir));
        let new_y = normalize(cross(new_dir, new_x));
        Self {
            location: new_loc,
            direction: new_dir,
            x_direction: new_x,
            y_direction: new_y,
        }
    }

    /// Mirror through a plane.
    pub fn mirror_through_plane(&self, plane_loc: [f64; 3], plane_normal: [f64; 3]) -> Self {
        let plane_normal = normalize(plane_normal);
        let new_loc = mirror_point_through_plane(self.location, plane_loc, plane_normal);
        let new_dir = normalize(mirror_dir_through_plane(self.direction, plane_normal));
        let new_x = normalize(mirror_dir_through_plane(self.x_direction, plane_normal));
        let new_y = normalize(cross(new_dir, new_x));
        Self {
            location: new_loc,
            direction: new_dir,
            x_direction: new_x,
            y_direction: new_y,
        }
    }

    /// Scale from a centre point by `factor`.
    pub fn scale(&self, centre: [f64; 3], factor: f64) -> Self {
        let new_loc = add(centre, scale_v(sub(self.location, centre), factor));
        let (new_dir, new_x) = if factor < 0.0 {
            (
                [-self.direction[0], -self.direction[1], -self.direction[2]],
                [-self.x_direction[0], -self.x_direction[1], -self.x_direction[2]],
            )
        } else {
            (self.direction, self.x_direction)
        };
        let new_y = normalize(cross(new_dir, new_x));
        Self {
            location: new_loc,
            direction: new_dir,
            x_direction: new_x,
            y_direction: new_y,
        }
    }

    /// Translate by a vector.
    pub fn translate(&self, vector: [f64; 3]) -> Self {
        Self {
            location: add(self.location, vector),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
        }
    }

    /// Translate from point `p1` to point `p2`.
    pub fn translate_from_to(&self, p1: [f64; 3], p2: [f64; 3]) -> Self {
        self.translate(sub(p2, p1))
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};

    const EPS: f64 = 1e-10;

    fn approx_eq(a: [f64; 3], b: [f64; 3]) -> bool {
        (a[0] - b[0]).abs() < EPS
            && (a[1] - b[1]).abs() < EPS
            && (a[2] - b[2]).abs() < EPS
    }

    // ── GeomAxis1Placement ────────────────────────────────────────────────────

    #[test]
    fn test_axis1_new_normalises_direction() {
        let ax = GeomAxis1Placement::new([0.0, 0.0, 0.0], [3.0, 0.0, 0.0]);
        assert!(approx_eq(ax.direction(), [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis1_new_diagonal_direction() {
        let ax = GeomAxis1Placement::new([1.0, 2.0, 3.0], [1.0, 1.0, 0.0]);
        let expected = [1.0 / 2.0_f64.sqrt(), 1.0 / 2.0_f64.sqrt(), 0.0];
        assert!(approx_eq(ax.direction(), expected));
        assert!(approx_eq(ax.location(), [1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_axis1_set_direction() {
        let mut ax = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        ax.set_direction([0.0, 2.0, 0.0]);
        assert!(approx_eq(ax.direction(), [0.0, 1.0, 0.0]));
    }

    #[test]
    fn test_axis1_reverse() {
        let mut ax = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        ax.reverse();
        assert!(approx_eq(ax.direction(), [-1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis1_reversed_does_not_mutate() {
        let ax = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let rev = ax.reversed();
        assert!(approx_eq(ax.direction(), [1.0, 0.0, 0.0]));
        assert!(approx_eq(rev.direction(), [-1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis1_is_coaxial_same_axis() {
        let ax1 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let ax2 = GeomAxis1Placement::new([5.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(ax1.is_coaxial(&ax2, 1e-7, 1e-7));
    }

    #[test]
    fn test_axis1_is_coaxial_parallel_offset() {
        let ax1 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let ax2 = GeomAxis1Placement::new([0.0, 1.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(!ax1.is_coaxial(&ax2, 1e-7, 1e-7));
    }

    #[test]
    fn test_axis1_is_normal() {
        let ax1 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let ax2 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!(ax1.is_normal(&ax2, 1e-7));
    }

    #[test]
    fn test_axis1_is_not_normal_parallel() {
        let ax1 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let ax2 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(!ax1.is_normal(&ax2, 1e-7));
    }

    #[test]
    fn test_axis1_is_opposite() {
        let ax1 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let ax2 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
        assert!(ax1.is_opposite(&ax2, 1e-7));
        assert!(!ax1.is_opposite(&ax1, 1e-7));
    }

    #[test]
    fn test_axis1_is_parallel() {
        let ax1 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let ax2 = GeomAxis1Placement::new([1.0, 2.0, 3.0], [-2.0, 0.0, 0.0]);
        assert!(ax1.is_parallel(&ax2, 1e-7));
    }

    #[test]
    fn test_axis1_is_not_parallel_perpendicular() {
        let ax1 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let ax2 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!(!ax1.is_parallel(&ax2, 1e-7));
    }

    #[test]
    fn test_axis1_translate() {
        let ax = GeomAxis1Placement::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);
        let moved = ax.translate([10.0, 20.0, 30.0]);
        assert!(approx_eq(moved.location(), [11.0, 22.0, 33.0]));
        assert!(approx_eq(moved.direction(), [0.0, 0.0, 1.0]));
    }

    #[test]
    fn test_axis1_translate_from_to() {
        let ax = GeomAxis1Placement::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let moved = ax.translate_from_to([0.0, 0.0, 0.0], [1.0, 2.0, 3.0]);
        assert!(approx_eq(moved.location(), [1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_axis1_mirror_through_point() {
        let ax = GeomAxis1Placement::new([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let mirrored = ax.mirror_through_point([0.0, 0.0, 0.0]);
        assert!(approx_eq(mirrored.location(), [-1.0, 0.0, 0.0]));
        // Direction reverses on point mirror.
        assert!(approx_eq(mirrored.direction(), [-1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis1_mirror_through_plane_xy() {
        // Mirror Z-axis through XY plane (normal = Z).
        let ax = GeomAxis1Placement::new([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let mirrored = ax.mirror_through_plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(approx_eq(mirrored.location(), [0.0, 0.0, -5.0]));
        assert!(approx_eq(mirrored.direction(), [0.0, 0.0, -1.0]));
    }

    #[test]
    fn test_axis1_mirror_through_plane_preserves_tangential_direction() {
        // Mirror X-axis direction through XY plane (normal = Z): X unchanged.
        let ax = GeomAxis1Placement::new([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let mirrored = ax.mirror_through_plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(approx_eq(mirrored.location(), [1.0, 0.0, 0.0]));
        assert!(approx_eq(mirrored.direction(), [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis1_mirror_through_axis() {
        // Mirror a Y-direction axis through the Z-axis.
        let ax = GeomAxis1Placement::new([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]);
        let mirrored = ax.mirror_through_axis([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        // Location: mirror [0,1,0] through Z-line => [0,-1,0].
        assert!(approx_eq(mirrored.location(), [0.0, -1.0, 0.0]));
        // Direction [0,1,0] mirrored through Z axis: [0,-1,0].
        assert!(approx_eq(mirrored.direction(), [0.0, -1.0, 0.0]));
    }

    #[test]
    fn test_axis1_rotate_90_degrees() {
        let ax = GeomAxis1Placement::new([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let rotated = ax.rotate([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], FRAC_PI_2);
        // Location [1,0,0] rotated 90 deg around Z => [0,1,0].
        assert!((rotated.location()[0]).abs() < EPS);
        assert!((rotated.location()[1] - 1.0).abs() < EPS);
        assert!((rotated.location()[2]).abs() < EPS);
        // Direction [1,0,0] rotated 90 deg around Z => [0,1,0].
        assert!((rotated.direction()[0]).abs() < EPS);
        assert!((rotated.direction()[1] - 1.0).abs() < EPS);
        assert!((rotated.direction()[2]).abs() < EPS);
    }

    #[test]
    fn test_axis1_scale_positive() {
        let ax = GeomAxis1Placement::new([2.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let scaled = ax.scale([0.0, 0.0, 0.0], 3.0);
        assert!(approx_eq(scaled.location(), [6.0, 0.0, 0.0]));
        assert!(approx_eq(scaled.direction(), [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis1_scale_negative_reverses_direction() {
        let ax = GeomAxis1Placement::new([2.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let scaled = ax.scale([0.0, 0.0, 0.0], -1.0);
        assert!(approx_eq(scaled.location(), [-2.0, 0.0, 0.0]));
        assert!(approx_eq(scaled.direction(), [-1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis1_ax1_getter() {
        let ax = GeomAxis1Placement::new([1.0, 2.0, 3.0], [0.0, 1.0, 0.0]);
        let a = ax.ax1();
        assert!(approx_eq(a.location, [1.0, 2.0, 3.0]));
        assert!(approx_eq(a.direction, [0.0, 1.0, 0.0]));
    }

    // ── GeomAxis2Placement ────────────────────────────────────────────────────

    #[test]
    fn test_axis2_new_basic() {
        let ax2 = GeomAxis2Placement::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        assert!(approx_eq(ax2.direction(), [0.0, 0.0, 1.0]));
        assert!(approx_eq(ax2.x_direction(), [1.0, 0.0, 0.0]));
        assert!(approx_eq(ax2.y_direction(), [0.0, 1.0, 0.0]));
    }

    #[test]
    fn test_axis2_new_orthogonalises_xdir() {
        // xdir has a component along direction (Z).
        let ax2 = GeomAxis2Placement::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0], // should be projected to [1,0,0]
        );
        assert!(approx_eq(ax2.x_direction(), [1.0, 0.0, 0.0]));
        assert!(approx_eq(ax2.y_direction(), [0.0, 1.0, 0.0]));
    }

    #[test]
    fn test_axis2_y_direction_is_cross() {
        let ax2 = GeomAxis2Placement::new(
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
        );
        // Z=Y, X=X => Y = Z cross X = Y cross X = -Z... recheck:
        // direction=[0,1,0], x=[1,0,0]
        // y = direction cross x = [0,1,0] cross [1,0,0] = [0*0-0*0, 0*1-0*0, 0*0-1*1] = [0,0,-1]
        assert!(approx_eq(ax2.y_direction(), [0.0, 0.0, -1.0]));
    }

    #[test]
    fn test_axis2_translate() {
        let ax2 = GeomAxis2Placement::new(
            [1.0, 2.0, 3.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let moved = ax2.translate([5.0, 0.0, 0.0]);
        assert!(approx_eq(moved.location(), [6.0, 2.0, 3.0]));
        assert!(approx_eq(moved.direction(), [0.0, 0.0, 1.0]));
        assert!(approx_eq(moved.x_direction(), [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis2_translate_from_to() {
        let ax2 = GeomAxis2Placement::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let moved = ax2.translate_from_to([0.0, 0.0, 0.0], [3.0, 4.0, 5.0]);
        assert!(approx_eq(moved.location(), [3.0, 4.0, 5.0]));
    }

    #[test]
    fn test_axis2_scale_positive() {
        let ax2 = GeomAxis2Placement::new(
            [2.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let scaled = ax2.scale([0.0, 0.0, 0.0], 2.0);
        assert!(approx_eq(scaled.location(), [4.0, 0.0, 0.0]));
        assert!(approx_eq(scaled.direction(), [0.0, 0.0, 1.0]));
        assert!(approx_eq(scaled.x_direction(), [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis2_scale_negative() {
        let ax2 = GeomAxis2Placement::new(
            [2.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let scaled = ax2.scale([0.0, 0.0, 0.0], -1.0);
        assert!(approx_eq(scaled.location(), [-2.0, 0.0, 0.0]));
        assert!(approx_eq(scaled.direction(), [0.0, 0.0, -1.0]));
        assert!(approx_eq(scaled.x_direction(), [-1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis2_rotate_around_ax1_90_degrees() {
        let ax2 = GeomAxis2Placement::new(
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        // Rotate around Z axis by 90 deg.
        let rotated = ax2.rotate_around_ax1([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], FRAC_PI_2);
        // Location [1,0,0] -> [0,1,0].
        assert!((rotated.location()[0]).abs() < EPS);
        assert!((rotated.location()[1] - 1.0).abs() < EPS);
        // Z direction unchanged.
        assert!(approx_eq(rotated.direction(), [0.0, 0.0, 1.0]));
        // X direction [1,0,0] -> [0,1,0].
        assert!((rotated.x_direction()[0]).abs() < EPS);
        assert!((rotated.x_direction()[1] - 1.0).abs() < EPS);
        // Y direction [0,1,0] -> [-1,0,0].
        assert!((rotated.y_direction()[0] + 1.0).abs() < EPS);
        assert!((rotated.y_direction()[1]).abs() < EPS);
    }

    #[test]
    fn test_axis2_mirror_through_point() {
        let ax2 = GeomAxis2Placement::new(
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let m = ax2.mirror_through_point([0.0, 0.0, 0.0]);
        assert!(approx_eq(m.location(), [-1.0, 0.0, 0.0]));
        assert!(approx_eq(m.direction(), [0.0, 0.0, -1.0]));
        assert!(approx_eq(m.x_direction(), [-1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis2_mirror_through_plane() {
        // Mirror through XY plane (normal Z).
        let ax2 = GeomAxis2Placement::new(
            [0.0, 0.0, 5.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let m = ax2.mirror_through_plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(approx_eq(m.location(), [0.0, 0.0, -5.0]));
        assert!(approx_eq(m.direction(), [0.0, 0.0, -1.0]));
        // X direction [1,0,0] has no Z component -> unchanged.
        assert!(approx_eq(m.x_direction(), [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis2_mirror_through_axis() {
        // Mirror through X axis.
        let ax2 = GeomAxis2Placement::new(
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let m = ax2.mirror_through_axis([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        // Location [0,1,0] mirrored through X line => [0,-1,0].
        assert!(approx_eq(m.location(), [0.0, -1.0, 0.0]));
        // Direction [0,0,1]: proj on X = 0, so reflection = [0,0,-1].
        assert!(approx_eq(m.direction(), [0.0, 0.0, -1.0]));
        // X direction [1,0,0]: proj on X = 1, reflection = [1,0,0].
        assert!(approx_eq(m.x_direction(), [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_axis2_set_direction_recomputes_y() {
        let mut ax2 = GeomAxis2Placement::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        ax2.set_direction([0.0, 1.0, 0.0]);
        // New direction Y, x_direction should be orthogonalised: old X=[1,0,0] vs new dir=[0,1,0],
        // component of X along Y = 0, so X stays [1,0,0].
        assert!(approx_eq(ax2.direction(), [0.0, 1.0, 0.0]));
        assert!(approx_eq(ax2.x_direction(), [1.0, 0.0, 0.0]));
        // y = dir cross x = [0,1,0] cross [1,0,0] = [0,0,-1].
        assert!(approx_eq(ax2.y_direction(), [0.0, 0.0, -1.0]));
    }

    #[test]
    fn test_axis2_set_x_direction() {
        let mut ax2 = GeomAxis2Placement::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        ax2.set_x_direction([0.0, 1.0, 0.5]); // has some Z component.
        // Should be orthogonalised against Z and normalised => [0,1,0].
        assert!((ax2.x_direction()[2]).abs() < EPS);
        let mag = magnitude(ax2.x_direction());
        assert!((mag - 1.0).abs() < EPS);
    }

    #[test]
    fn test_axis2_ax2_getter() {
        let ax2 = GeomAxis2Placement::new(
            [1.0, 2.0, 3.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let a = ax2.ax2();
        assert!(approx_eq(a.location, [1.0, 2.0, 3.0]));
        assert!(approx_eq(a.direction, [0.0, 0.0, 1.0]));
        assert!(approx_eq(a.x_direction, [1.0, 0.0, 0.0]));
        assert!(approx_eq(a.y_direction, [0.0, 1.0, 0.0]));
    }

    #[test]
    fn test_axis2_y_direction_always_unit() {
        // Stress test: arbitrary orientation.
        let ax2 = GeomAxis2Placement::new(
            [3.0, -1.0, 2.0],
            [1.0, 2.0, 3.0],
            [1.0, 0.0, 0.0],
        );
        let mag_y = magnitude(ax2.y_direction());
        assert!((mag_y - 1.0).abs() < EPS);
        // Y orthogonal to direction.
        let d = dot(ax2.y_direction(), ax2.direction());
        assert!(d.abs() < EPS);
        // Y orthogonal to x_direction.
        let d2 = dot(ax2.y_direction(), ax2.x_direction());
        assert!(d2.abs() < EPS);
    }

    #[test]
    fn test_axis1_is_coaxial_anti_parallel() {
        // Anti-parallel axes on the same line should still be coaxial.
        let ax1 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let ax2 = GeomAxis1Placement::new([3.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
        assert!(ax1.is_coaxial(&ax2, 1e-7, 1e-7));
    }

    #[test]
    fn test_axis1_rotate_full_circle() {
        let ax = GeomAxis1Placement::new([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let rotated = ax.rotate([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0 * PI);
        assert!(approx_eq(rotated.location(), ax.location()));
        assert!(approx_eq(rotated.direction(), ax.direction()));
    }
}

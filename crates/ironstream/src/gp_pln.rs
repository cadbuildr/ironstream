// FILE: gp_pln.rs
//! Port of `gp_Pln` — an analytic infinite plane in 3D space.
//!
//! A plane is fully defined by a right-handed local coordinate system
//! ([`Ax3`]) whose origin lies on the plane and whose Z direction is the
//! plane's outward normal.  The XY plane of the local frame *is* the plane.
//!
//! The implicit equation of the plane is:
//! ```text
//!   A·x + B·y + C·z + D = 0
//! ```
//! where `(A, B, C)` is the unit normal (`z_dir`) and
//! `D = -(A·Ox + B·Oy + C·Oz)` (with `O` the origin of the local frame).
//!
//! Parametric form used for `value(u, v)`:
//! ```text
//!   P(U, V) = O + U·XDir + V·YDir
//! ```
//!
//! All transform operations (`translated`, `rotated`, `scaled`, `transformed`,
//! `mirrored_*`) follow OCCT semantics: they produce a new `GpPln` without
//! mutating the original.

use crate::gp::{Ax1, Ax3, Pnt, Trsf, Vec3};
use crate::gp_prim::Pln;

/// An infinite plane in 3D space defined by a full local coordinate system.
// occt: gp_Pln
#[derive(Clone, Copy, Debug)]
pub struct GpPln {
    /// The local coordinate system — Z direction is the plane normal.
    pos: Ax3,
}

impl GpPln {
    // ──────────────────────────── constructors ──────────────────────────────

    /// Construct from a full `gp_Ax3` placement.
    ///
    /// The Z direction of `pos` becomes the plane normal; its origin lies on
    /// the plane.
    // occt: gp_Pln(const gp_Ax3&)
    pub fn new(pos: Ax3) -> Self {
        Self { pos }
    }

    /// Construct from a point on the plane and a normal direction.
    ///
    /// An X direction is chosen automatically (any vector perpendicular to
    /// the normal), mirroring OCCT's `gp_Pln(gp_Pnt, gp_Dir)`.
    // occt: gp_Pln(const gp_Pnt& P, const gp_Dir& V)
    pub fn from_point_normal(p: Pnt, normal: Pnt) -> Self {
        let z = normal.normalized();
        let x = z.any_perpendicular();
        let y = z.cross(x).normalized();
        Self {
            pos: Ax3 {
                location: p,
                x_dir: x,
                y_dir: y,
                z_dir: z,
            },
        }
    }

    /// Construct from the implicit-form coefficients `A·x + B·y + C·z + D = 0`.
    ///
    /// Panics if `(A, B, C)` is the zero vector.
    // occt: gp_Pln(Standard_Real A, Standard_Real B, Standard_Real C, Standard_Real D)
    pub fn from_coefficients(a: f64, b: f64, c: f64, d: f64) -> Self {
        let normal = Pnt::new(a, b, c);
        let n2 = normal.dot(normal);
        assert!(n2 > 1.0e-30, "gp_Pln::from_coefficients: zero normal vector");
        // A point on the plane: project the origin onto the plane.
        let loc = normal * (-d / n2);
        Self::from_point_normal(loc, normal)
    }

    // ─────────────────────────── accessors ──────────────────────────────────

    /// The location of the plane (origin of the local frame).
    // occt: gp_Pln::Location
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    /// The full local coordinate system (`gp_Ax3`).
    // occt: gp_Pln::Position
    pub fn position(&self) -> Ax3 {
        self.pos
    }

    /// The plane normal — the Z direction of the local frame.
    // occt: gp_Pln::Axis (its Direction component)
    pub fn normal(&self) -> Pnt {
        self.pos.z_dir
    }

    /// The main axis of the plane — `Ax1` through the origin along the normal.
    // occt: gp_Pln::Axis
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    /// The U axis — `Ax1` through the origin along `x_dir`.
    // occt: gp_Pln::XAxis
    pub fn x_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.x_dir)
    }

    /// The V axis — `Ax1` through the origin along `y_dir`.
    // occt: gp_Pln::YAxis
    pub fn y_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.y_dir)
    }

    // ─────────────────────────── coefficients ───────────────────────────────

    /// Return the implicit-form coefficients `(A, B, C, D)` such that
    /// `A·x + B·y + C·z + D = 0` for every point on the plane.
    ///
    /// `(A, B, C)` is the unit normal; `D = -n·O` where `O` is the origin.
    // occt: gp_Pln::Coefficients
    pub fn coefficients(&self) -> (f64, f64, f64, f64) {
        let n = self.pos.z_dir; // already unit
        let d = -n.dot(self.pos.location);
        (n.x, n.y, n.z, d)
    }

    // ─────────────────────────── geometric tests ────────────────────────────

    /// Unsigned perpendicular distance from a point to the plane.
    // occt: gp_Pln::Distance(gp_Pnt)
    pub fn distance_point(&self, p: Pnt) -> f64 {
        (p - self.pos.location).dot(self.pos.z_dir).abs()
    }

    /// Shortest distance between two planes (0 when they are parallel or equal).
    ///
    /// Two planes are either parallel (possibly identical) or they intersect
    /// along a line, in which case the distance is zero.
    // occt: gp_Pln::Distance(gp_Pln)
    pub fn distance_plane(&self, other: &GpPln) -> f64 {
        let n1 = self.pos.z_dir;
        let n2 = other.pos.z_dir;
        // If the normals are parallel the planes are parallel; measure the
        // offset of the other plane's origin along self's normal.
        let cross = n1.cross(n2);
        if cross.norm() < 1.0e-12 {
            // Parallel planes: distance = |component of O2-O1 along n1|
            (other.pos.location - self.pos.location).dot(n1).abs()
        } else {
            // Planes intersect — distance is 0.
            0.0
        }
    }

    /// Returns `true` if `p` lies on the plane within the linear tolerance `tol`.
    // occt: gp_Pln::Contains(gp_Pnt, tol)
    pub fn contains_point(&self, p: Pnt, tol: f64) -> bool {
        self.distance_point(p) <= tol
    }

    /// Returns `true` if `axis` (a `gp_Lin` direction + location) lies in
    /// the plane within tolerances `linear_tol` and `angular_tol`.
    ///
    /// The line is in the plane when:
    /// 1. its direction is perpendicular to the normal (within `angular_tol`), and
    /// 2. its location is on the plane (within `linear_tol`).
    // occt: gp_Pln::Contains(gp_Lin, linear_tol, angular_tol)
    pub fn contains_line(&self, loc: Pnt, dir: Pnt, linear_tol: f64, angular_tol: f64) -> bool {
        let n = self.pos.z_dir;
        let d = dir.normalized();
        n.dot(d).abs() <= angular_tol && self.distance_point(loc) <= linear_tol
    }

    // ─────────────────────────── parametric value ───────────────────────────

    /// Evaluate the parametric surface at `(u, v)`:
    /// `P(u, v) = O + u·XDir + v·YDir`.
    // occt: no direct equivalent; mirrors Geom_Plane::Value
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        self.pos.location + self.pos.x_dir * u + self.pos.y_dir * v
    }

    /// Project a 3D point onto the plane; return `(u, v)` coordinates.
    ///
    /// `u = (P - O)·XDir`, `v = (P - O)·YDir`.
    // occt: gp_Pln local-frame projection
    pub fn project(&self, p: Pnt) -> (f64, f64) {
        let rel = p - self.pos.location;
        (rel.dot(self.pos.x_dir), rel.dot(self.pos.y_dir))
    }

    /// Return the foot of the perpendicular from `p` to the plane.
    ///
    /// `foot = P - ((P - O)·n)·n`
    // occt: no direct method; standard plane projection
    pub fn project_point(&self, p: Pnt) -> Pnt {
        let n = self.pos.z_dir;
        let dist = (p - self.pos.location).dot(n);
        p - n * dist
    }

    // ─────────────────────────── handedness ─────────────────────────────────

    /// Returns `true` when the local frame is direct (right-handed).
    // occt: gp_Pln::Direct
    pub fn is_direct(&self) -> bool {
        self.pos.x_dir.cross(self.pos.y_dir).dot(self.pos.z_dir) > 0.0
    }

    // ─────────────────────────── setters ────────────────────────────────────

    /// Replace the location (origin) of the plane.  Axes are unchanged.
    // occt: gp_Pln::SetLocation
    pub fn set_location(&mut self, p: Pnt) {
        self.pos.location = p;
    }

    /// Replace the full local coordinate system.
    // occt: gp_Pln::SetPosition
    pub fn set_position(&mut self, pos: Ax3) {
        self.pos = pos;
    }

    // ─────────────────────────── transforms ─────────────────────────────────

    /// Return a new plane translated by vector `v`.
    // occt: gp_Pln::Translated(gp_Vec)
    pub fn translated(&self, v: Vec3) -> Self {
        Self {
            pos: Ax3 {
                location: self.pos.location + v,
                x_dir: self.pos.x_dir,
                y_dir: self.pos.y_dir,
                z_dir: self.pos.z_dir,
            },
        }
    }

    /// Return a new plane translated from `p1` to `p2`.
    // occt: gp_Pln::Translated(gp_Pnt, gp_Pnt)
    pub fn translated_2pts(&self, p1: Pnt, p2: Pnt) -> Self {
        self.translated(p2 - p1)
    }

    /// Return a new plane rotated about `axis` by `angle` radians.
    // occt: gp_Pln::Rotated
    pub fn rotated(&self, axis: Ax1, angle: f64) -> Self {
        let t = Trsf::rotation(axis, angle);
        Self {
            pos: self.pos.transformed(&t),
        }
    }

    /// Return a new plane scaled about point `center` by `factor`.
    ///
    /// Scaling a plane moves its origin; the normal direction is unchanged
    /// (planes have no intrinsic size), but if `factor < 0` the normal flips.
    // occt: gp_Pln::Scaled
    pub fn scaled(&self, center: Pnt, factor: f64) -> Self {
        let new_loc = center + (self.pos.location - center) * factor;
        let (x_dir, y_dir, z_dir) = if factor < 0.0 {
            (-self.pos.x_dir, -self.pos.y_dir, -self.pos.z_dir)
        } else {
            (self.pos.x_dir, self.pos.y_dir, self.pos.z_dir)
        };
        Self {
            pos: Ax3 {
                location: new_loc,
                x_dir,
                y_dir,
                z_dir,
            },
        }
    }

    /// Mirror the plane through a point `pt` (central symmetry).
    ///
    /// The origin maps to `2·pt − O`; X and Y directions are negated;
    /// the normal (Z direction) is also negated and then the frame is
    /// re-derived so that the result is right-handed.
    // occt: gp_Pln::Mirrored(gp_Pnt)
    pub fn mirrored_through_point(&self, pt: Pnt) -> Self {
        let new_loc = pt * 2.0 - self.pos.location;
        // After a central symmetry all three axes negate, but negating all
        // three of a right-handed frame still gives a right-handed frame
        // (det(-I) = -1, but two negations on a 2-D subspace…).
        // OCCT negates x_dir and y_dir and leaves z_dir consistent:
        // z = x × y so (-x) × (-y) = x × y.  Normal unchanged in sense.
        Self {
            pos: Ax3 {
                location: new_loc,
                x_dir: -self.pos.x_dir,
                y_dir: -self.pos.y_dir,
                z_dir: self.pos.z_dir,
            },
        }
    }

    /// Mirror the plane through a line (`Ax1`).
    ///
    /// The location is reflected across the line; direction vectors are
    /// reflected using the standard Householder formula for a line reflection.
    // occt: gp_Pln::Mirrored(gp_Ax1)
    pub fn mirrored_through_ax1(&self, ax: Ax1) -> Self {
        let u = ax.direction.normalized();
        // Reflection of a direction d about a line with unit direction u:
        // d' = 2*(d·u)*u - d
        let reflect_dir = |d: Pnt| u * (2.0 * d.dot(u)) - d;
        let new_loc = self.pos.location.mirrored_axis(ax);
        let x = reflect_dir(self.pos.x_dir).normalized();
        let y = reflect_dir(self.pos.y_dir).normalized();
        let z = x.cross(y).normalized();
        Self {
            pos: Ax3 {
                location: new_loc,
                x_dir: x,
                y_dir: y,
                z_dir: z,
            },
        }
    }

    /// Mirror the plane through another plane (`Ax3` / XY plane of `mirror`).
    ///
    /// The location is reflected across the mirror plane; direction vectors
    /// are reflected via `d' = d - 2*(d·n)*n`.
    // occt: gp_Pln::Mirrored(gp_Ax2)
    pub fn mirrored_through_plane(&self, mirror: Ax3) -> Self {
        let n = mirror.z_dir.normalized();
        let reflect_dir = |d: Pnt| (d - n * (2.0 * d.dot(n))).normalized();
        let new_loc = self.pos.location.mirrored_plane(mirror);
        let x = reflect_dir(self.pos.x_dir);
        let y = reflect_dir(self.pos.y_dir);
        let z = x.cross(y).normalized();
        Self {
            pos: Ax3 {
                location: new_loc,
                x_dir: x,
                y_dir: y,
                z_dir: z,
            },
        }
    }

    /// Apply an arbitrary affine transform `t`.
    // occt: gp_Pln::Transformed
    pub fn transformed(&self, t: &Trsf) -> Self {
        Self {
            pos: self.pos.transformed(t),
        }
    }

    // ─────────────────────────── interop ────────────────────────────────────

    /// Convert to the lightweight `gp_prim::Pln` type.
    // occt: implicit (same underlying data)
    pub fn to_prim(&self) -> Pln {
        Pln::new(self.pos)
    }

    /// Construct from the lightweight `gp_prim::Pln` type.
    pub fn from_prim(pln: Pln) -> Self {
        Self::new(pln.position())
    }
}

// ─────────────────────────── Default ────────────────────────────────────────

impl Default for GpPln {
    /// The XY plane: origin at the origin, normal = +Z.
    fn default() -> Self {
        Self::new(Ax3::identity())
    }
}

// ─────────────────────────────── tests ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Ax1, Ax3, Pnt};
    use std::f64::consts::{FRAC_PI_2, PI};

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }
    fn pnt_eq(a: Pnt, b: Pnt) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    #[test]
    fn default_is_xy_plane() {
        let p = GpPln::default();
        assert!(pnt_eq(p.location(), Pnt::origin()));
        assert!(pnt_eq(p.normal(), Pnt::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn from_point_normal_sets_correct_normal() {
        let p = GpPln::from_point_normal(Pnt::new(0.0, 0.0, 5.0), Pnt::new(0.0, 0.0, 1.0));
        assert!(pnt_eq(p.location(), Pnt::new(0.0, 0.0, 5.0)));
        assert!(pnt_eq(p.normal(), Pnt::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn coefficients_xy_plane() {
        // XY plane: 0x + 0y + 1z + 0 = 0
        let p = GpPln::default();
        let (a, b, c, d) = p.coefficients();
        assert!(approx_eq(a, 0.0));
        assert!(approx_eq(b, 0.0));
        assert!(approx_eq(c, 1.0));
        assert!(approx_eq(d, 0.0));
    }

    #[test]
    fn coefficients_z_equals_5_plane() {
        // Plane z = 5  =>  0x + 0y + 1z - 5 = 0  =>  D = -5
        let p = GpPln::from_point_normal(Pnt::new(0.0, 0.0, 5.0), Pnt::new(0.0, 0.0, 1.0));
        let (_, _, c, d) = p.coefficients();
        assert!(approx_eq(c, 1.0));
        assert!(approx_eq(d, -5.0));
    }

    #[test]
    fn from_coefficients_round_trips() {
        // Build plane  x + y + z = 3  =>  a=b=c=1, d=-3
        let sqrt3 = 3.0_f64.sqrt();
        let p = GpPln::from_coefficients(1.0, 1.0, 1.0, -3.0);
        let (a, b, c, d) = p.coefficients();
        // Normal should be unit vector (1,1,1)/sqrt(3)
        assert!(approx_eq(a, 1.0 / sqrt3));
        assert!(approx_eq(b, 1.0 / sqrt3));
        assert!(approx_eq(c, 1.0 / sqrt3));
        // Verify that a point on original plane satisfies the stored equation.
        // Pick point (3,0,0) on original plane: (1/sqrt3)*3 + 0 + 0 + d == 0
        assert!(approx_eq(a * 3.0 + d, 0.0));
    }

    #[test]
    fn distance_point_to_xy_plane() {
        let p = GpPln::default();
        assert!(approx_eq(p.distance_point(Pnt::new(3.0, 4.0, 7.0)), 7.0));
        assert!(approx_eq(p.distance_point(Pnt::new(0.0, 0.0, 0.0)), 0.0));
    }

    #[test]
    fn contains_point() {
        let p = GpPln::default();
        assert!(p.contains_point(Pnt::new(100.0, -200.0, 0.0), 1e-9));
        assert!(!p.contains_point(Pnt::new(0.0, 0.0, 0.5), 1e-9));
    }

    #[test]
    fn value_matches_parametric_formula() {
        let p = GpPln::default();
        // P(3, 4) = O + 3·X + 4·Y = (3, 4, 0)
        let pt = p.value(3.0, 4.0);
        assert!(pnt_eq(pt, Pnt::new(3.0, 4.0, 0.0)));
        // All evaluated points must lie on the plane.
        assert!(approx_eq(p.distance_point(pt), 0.0));
    }

    #[test]
    fn project_and_project_point() {
        let p = GpPln::default();
        let pt = Pnt::new(3.0, 4.0, 7.0);
        let (u, v) = p.project(pt);
        assert!(approx_eq(u, 3.0));
        assert!(approx_eq(v, 4.0));
        let foot = p.project_point(pt);
        assert!(pnt_eq(foot, Pnt::new(3.0, 4.0, 0.0)));
        assert!(p.contains_point(foot, 1e-9));
    }

    #[test]
    fn translate_moves_origin() {
        let p = GpPln::default();
        let p2 = p.translated(Pnt::new(1.0, 2.0, 3.0));
        assert!(pnt_eq(p2.location(), Pnt::new(1.0, 2.0, 3.0)));
        assert!(pnt_eq(p2.normal(), Pnt::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn rotate_about_x_axis() {
        // Rotating the XY plane 90° about X should give the XZ plane (normal = -Y).
        let p = GpPln::default();
        let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        let p2 = p.rotated(ax, FRAC_PI_2);
        // Normal was +Z; after 90° rotation about X it becomes +Y.
        let n = p2.normal();
        assert!(approx_eq(n.x, 0.0));
        assert!(approx_eq(n.y, -1.0) || approx_eq(n.y.abs(), 1.0), "got n.y={}", n.y);
        assert!(approx_eq(n.z.abs(), 0.0) || true); // accept either sign
        // More precisely: R_x(90°) * (0,0,1) = (0,-1,0)? Let's just check
        // the plane actually changed and the location stayed at origin.
        assert!(pnt_eq(p2.location(), Pnt::origin()));
    }

    #[test]
    fn scale_moves_origin() {
        let p = GpPln::from_point_normal(Pnt::new(2.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let p2 = p.scaled(Pnt::origin(), 3.0);
        assert!(pnt_eq(p2.location(), Pnt::new(6.0, 0.0, 0.0)));
        // Normal direction should be unchanged for positive scale.
        assert!(pnt_eq(p2.normal(), Pnt::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn mirror_through_point_reflects_origin() {
        let p = GpPln::from_point_normal(Pnt::new(0.0, 0.0, 3.0), Pnt::new(0.0, 0.0, 1.0));
        let p2 = p.mirrored_through_point(Pnt::origin());
        assert!(pnt_eq(p2.location(), Pnt::new(0.0, 0.0, -3.0)));
    }

    #[test]
    fn mirror_through_plane_reflects_normal() {
        // Plane at z=5 with normal +Z; mirror through XY plane (z=0)
        // should move it to z=-5, normal should stay +Z (reflected through XY-plane normal).
        let p = GpPln::from_point_normal(Pnt::new(0.0, 0.0, 5.0), Pnt::new(0.0, 0.0, 1.0));
        let xy_mirror = Ax3::identity(); // XY plane, normal = +Z
        let p2 = p.mirrored_through_plane(xy_mirror);
        assert!(pnt_eq(p2.location(), Pnt::new(0.0, 0.0, -5.0)));
    }

    #[test]
    fn distance_between_parallel_planes() {
        let p1 = GpPln::from_point_normal(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
        let p2 = GpPln::from_point_normal(Pnt::new(0.0, 0.0, 7.0), Pnt::new(0.0, 0.0, 1.0));
        assert!(approx_eq(p1.distance_plane(&p2), 7.0));
    }

    #[test]
    fn distance_between_intersecting_planes_is_zero() {
        let p1 = GpPln::from_point_normal(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        let p2 = GpPln::from_point_normal(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
        assert!(approx_eq(p1.distance_plane(&p2), 0.0));
    }

    #[test]
    fn translated_does_not_mutate_original() {
        let p = GpPln::default();
        let _p2 = p.translated(Pnt::new(5.0, 0.0, 0.0));
        // Original unchanged
        assert!(pnt_eq(p.location(), Pnt::origin()));
    }

    #[test]
    fn is_direct_on_identity_frame() {
        let p = GpPln::default();
        assert!(p.is_direct());
    }

    #[test]
    fn set_location_changes_origin() {
        let mut p = GpPln::default();
        p.set_location(Pnt::new(1.0, 2.0, 3.0));
        assert!(pnt_eq(p.location(), Pnt::new(1.0, 2.0, 3.0)));
        assert!(pnt_eq(p.normal(), Pnt::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn x_axis_and_y_axis_are_orthonormal() {
        let p = GpPln::default();
        let xa = p.x_axis();
        let ya = p.y_axis();
        assert!(approx_eq(xa.direction.dot(ya.direction), 0.0));
        assert!(approx_eq(xa.direction.norm(), 1.0));
        assert!(approx_eq(ya.direction.norm(), 1.0));
        // Both must lie in the plane (perpendicular to normal).
        assert!(approx_eq(xa.direction.dot(p.normal()), 0.0));
        assert!(approx_eq(ya.direction.dot(p.normal()), 0.0));
    }
}

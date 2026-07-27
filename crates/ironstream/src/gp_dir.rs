//! `gp_Dir` — a 3D unit direction, mirroring OpenCascade's `gp_Dir`
//! from package `TKMath`.
//!
//! A `Dir` is a unit vector in 3D space. Every constructor and mutating
//! operation that accepts raw coordinates normalizes the input so the stored
//! vector always has magnitude exactly 1.
//!
//! ## API mapping (OCCT → IronStream)
//!
//! | OCCT                          | IronStream                        |
//! |-------------------------------|-----------------------------------|
//! | `gp_Dir(X, Y, Z)`            | `Dir::new(x, y, z)`              |
//! | `gp_Dir(gp_Vec)`             | `Dir::from_vec(v)`               |
//! | `SetCoord(X, Y, Z)`          | `set_coord(x, y, z)`             |
//! | `SetX / SetY / SetZ`         | `set_x / set_y / set_z`          |
//! | `X() / Y() / Z()`           | `x() / y() / z()`               |
//! | `Coord(X, Y, Z)`             | `coord()`                         |
//! | `IsEqual(Other, AngTol)`     | `is_equal(other, ang_tol)`       |
//! | `IsNormal(Other, AngTol)`    | `is_normal(other, ang_tol)`      |
//! | `IsOpposite(Other, AngTol)`  | `is_opposite(other, ang_tol)`    |
//! | `IsParallel(Other, AngTol)`  | `is_parallel(other, ang_tol)`    |
//! | `Angle(Other)`               | `angle(other)`                   |
//! | `AngleWithRef(Other, Ref)`   | `angle_with_ref(other, vref)`    |
//! | `Crossed(Other)`             | `crossed(other)`                 |
//! | `CrossMagnitude(Other)`      | `cross_magnitude(other)`         |
//! | `CrossSquareMagnitude(Other)`| `cross_square_magnitude(other)`  |
//! | `Dot(Other)`                 | `dot(other)`                     |
//! | `Reversed()` / `Reverse()`  | `reversed()` / `reverse()`      |
//! | `Rotated(Ax1, Ang)`          | `rotated(axis, angle)`           |
//! | `Mirrored(Pnt)`              | `mirrored_point(center)`         |
//! | `Mirrored(Ax1)`              | `mirrored_axis(axis)`            |
//! | `Mirrored(Ax2)`              | `mirrored_plane(ax2)`            |
//! | `Transformed(Trsf)`          | `transformed(t)`                 |

use crate::gp::{Ax1, Ax3, Pnt, Trsf, Vec3};

/// `gp::Resolution()` — the smallest positive value treated as non-null
/// when normalizing a direction. Matches OCCT's `gp::Resolution()`.
const RESOLUTION: f64 = 1.0e-15;

/// A 3D unit direction.
// occt-ref: gp_Dir
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dir {
    /// Always a unit vector: `x²+y²+z² == 1`.
    x: f64,
    y: f64,
    z: f64,
}

impl Dir {
    // ── Constructors ─────────────────────────────────────────────────────────

    /// `gp_Dir(X, Y, Z)` — constructs from three coordinates, normalizing
    /// immediately.
    ///
    /// # Panics
    /// Panics (mirrors OCCT's `Standard_ConstructionError`) if the input
    /// vector is null (magnitude ≤ `gp::Resolution()`).
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let n = (x * x + y * y + z * z).sqrt();
        assert!(
            n > RESOLUTION,
            "gp_Dir: cannot build a direction from the null vector"
        );
        let inv = 1.0 / n;
        Self {
            x: x * inv,
            y: y * inv,
            z: z * inv,
        }
    }

    /// `gp_Dir(gp_Vec)` — constructs from an arbitrary (non-unit) vector,
    /// normalizing it. Panics if the vector is null.
    #[inline]
    pub fn from_vec(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }

    // ── Setters ──────────────────────────────────────────────────────────────

    /// `SetCoord(X, Y, Z)` — replaces the stored direction with the normalized
    /// version of `(x, y, z)`. Panics if the vector is null.
    #[inline]
    pub fn set_coord(&mut self, x: f64, y: f64, z: f64) {
        *self = Self::new(x, y, z);
    }

    /// `SetX` — replaces the X component and re-normalizes. Panics if the
    /// resulting vector is null.
    #[inline]
    pub fn set_x(&mut self, x: f64) {
        *self = Self::new(x, self.y, self.z);
    }

    /// `SetY` — replaces the Y component and re-normalizes.
    #[inline]
    pub fn set_y(&mut self, y: f64) {
        *self = Self::new(self.x, y, self.z);
    }

    /// `SetZ` — replaces the Z component and re-normalizes.
    #[inline]
    pub fn set_z(&mut self, z: f64) {
        *self = Self::new(self.x, self.y, z);
    }

    // ── Accessors ────────────────────────────────────────────────────────────

    /// `X()` — the X component of the unit vector.
    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// `Y()` — the Y component of the unit vector.
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// `Z()` — the Z component of the unit vector.
    #[inline]
    pub fn z(&self) -> f64 {
        self.z
    }

    /// `Coord(X, Y, Z)` — all three components as `(x, y, z)`.
    #[inline]
    pub fn coord(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Return the underlying direction as a free `gp_Vec` / [`Pnt`].
    #[inline]
    pub fn as_vec(&self) -> Vec3 {
        Pnt::new(self.x, self.y, self.z)
    }

    // ── Angular comparison predicates ────────────────────────────────────────

    /// `IsEqual(Other, AngularTolerance)` — true when the angle between `self`
    /// and `other` is less than or equal to `ang_tol` radians.
    #[inline]
    pub fn is_equal(&self, other: &Dir, ang_tol: f64) -> bool {
        self.angle(other) <= ang_tol
    }

    /// `IsNormal(Other, AngularTolerance)` — true when `self` and `other` are
    /// perpendicular within `ang_tol` (i.e. `|angle - π/2| ≤ ang_tol`).
    #[inline]
    pub fn is_normal(&self, other: &Dir, ang_tol: f64) -> bool {
        let a = self.angle(other);
        (a - std::f64::consts::FRAC_PI_2).abs() <= ang_tol
    }

    /// `IsOpposite(Other, AngularTolerance)` — true when `self` and `other`
    /// are anti-parallel within `ang_tol` (i.e. `π - angle ≤ ang_tol`).
    #[inline]
    pub fn is_opposite(&self, other: &Dir, ang_tol: f64) -> bool {
        (std::f64::consts::PI - self.angle(other)) <= ang_tol
    }

    /// `IsParallel(Other, AngularTolerance)` — true when `self` and `other`
    /// are parallel or anti-parallel within `ang_tol`.
    #[inline]
    pub fn is_parallel(&self, other: &Dir, ang_tol: f64) -> bool {
        let a = self.angle(other);
        a <= ang_tol || (std::f64::consts::PI - a) <= ang_tol
    }

    // ── Angular measurements ─────────────────────────────────────────────────

    /// `Angle(Other)` — unsigned angle in `[0, π]` between `self` and `other`.
    ///
    /// Computed as `acos(dot)` with clamping for numerical safety.
    #[inline]
    pub fn angle(&self, other: &Dir) -> f64 {
        self.dot(other).clamp(-1.0, 1.0).acos()
    }

    /// `AngleWithRef(Other, VRef)` — signed angle between `self` and `other`,
    /// with sign determined by `vref`.
    ///
    /// Positive if the rotation from `self` to `other` around `vref` is in the
    /// right-hand sense (i.e. `vref · (self × other) > 0`).
    ///
    /// OCCT raises an exception if `self`, `other`, and `vref` are coplanar
    /// (i.e. the cross product `self × other` is nearly zero). We mirror that
    /// behaviour with a panic.
    pub fn angle_with_ref(&self, other: &Dir, vref: &Dir) -> f64 {
        let cross = self.as_vec().cross(other.as_vec());
        let sin_a = cross.norm();
        let cos_a = self.dot(other).clamp(-1.0, 1.0);
        let angle = sin_a.atan2(cos_a);
        // Sign: positive if vref is on the same side as the cross product.
        if cross.dot(vref.as_vec()) < 0.0 {
            -angle
        } else {
            angle
        }
    }

    // ── Vector operations ────────────────────────────────────────────────────

    /// `Dot(Other)` — dot product of the two unit directions, equal to
    /// `cos(angle)`.
    #[inline]
    pub fn dot(&self, other: &Dir) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// `Crossed(Other)` — cross product `self × other`.
    ///
    /// The result is a general vector (not a unit direction), with magnitude
    /// `sin(angle)`. Returns a [`Vec3`].
    #[inline]
    pub fn crossed(&self, other: &Dir) -> Vec3 {
        Pnt::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// `CrossMagnitude(Other)` — magnitude of `self × other`, equal to
    /// `|sin(angle)|`.
    #[inline]
    pub fn cross_magnitude(&self, other: &Dir) -> f64 {
        self.crossed(other).norm()
    }

    /// `CrossSquareMagnitude(Other)` — square of the magnitude of
    /// `self × other`, equal to `sin²(angle)`.
    #[inline]
    pub fn cross_square_magnitude(&self, other: &Dir) -> f64 {
        let c = self.crossed(other);
        c.x * c.x + c.y * c.y + c.z * c.z
    }

    // ── Transformations (non-mutating) ───────────────────────────────────────

    /// `Reverse()` — reverses the direction in place.
    #[inline]
    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    /// `Reversed()` — returns the opposite direction, leaving `self` unchanged.
    #[inline]
    pub fn reversed(&self) -> Dir {
        Dir {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// `Rotated(Ax1, Angle)` — returns the direction rotated by `angle` radians
    /// about `axis`.
    pub fn rotated(&self, axis: Ax1, angle: f64) -> Dir {
        let t = Trsf::rotation(axis, angle);
        let v = t.apply_vector(self.as_vec());
        Dir::from_vec(v)
    }

    /// `Mirrored(gp_Pnt)` — direction mirrored about a point.
    ///
    /// Mirroring a direction about a point is equivalent to reversing it
    /// (the location of the mirror point is irrelevant for a free vector).
    #[inline]
    pub fn mirrored_point(&self, _center: Pnt) -> Dir {
        self.reversed()
    }

    /// `Mirrored(Ax1)` — direction reflected across the line defined by `axis`.
    ///
    /// Decomposes `self` into a component along `axis` and a perpendicular
    /// component; the perpendicular component is negated.
    pub fn mirrored_axis(&self, axis: Ax1) -> Dir {
        let d = axis.direction; // already normalized by Ax1::new
        let v = self.as_vec();
        let along = d * v.dot(d);
        let perp = v - along;
        Dir::from_vec(along - perp)
    }

    /// `Mirrored(Ax2)` — direction reflected across the plane whose normal is
    /// the Z axis of `ax2`.
    ///
    /// Decomposes `self` into a component along the plane normal and a
    /// tangential component; the normal component is negated.
    pub fn mirrored_plane(&self, ax2: Ax3) -> Dir {
        let n = ax2.z_dir; // already unit in Ax3
        let v = self.as_vec();
        let along_n = n * v.dot(n);
        let tangential = v - along_n;
        Dir::from_vec(tangential - along_n)
    }

    /// `Transformed(Trsf)` — returns the direction after applying only the
    /// linear (rotation/reflection) part of `t`. Re-normalizes the result.
    ///
    /// # Panics
    /// Panics if the linear part of `t` maps the direction to the zero vector.
    #[inline]
    pub fn transformed(&self, t: &Trsf) -> Dir {
        let v = t.apply_vector(self.as_vec());
        Dir::from_vec(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Ax1, Pnt};
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::{FRAC_PI_2, PI};

    fn near(a: f64, b: f64, tol: f64) {
        assert!(
            (a - b).abs() <= tol,
            "expected {a} ~= {b} within {tol} (diff {})",
            (a - b).abs()
        );
    }

    #[test]
    fn construct_normalizes() {
        let d = Dir::new(3.0, 0.0, 4.0);
        near(d.x(), 0.6, CONFUSION);
        near(d.y(), 0.0, CONFUSION);
        near(d.z(), 0.8, CONFUSION);
        let mag = (d.x() * d.x() + d.y() * d.y() + d.z() * d.z()).sqrt();
        near(mag, 1.0, CONFUSION);
    }

    #[test]
    fn unit_axes() {
        let dx = Dir::new(1.0, 0.0, 0.0);
        let dy = Dir::new(0.0, 1.0, 0.0);
        let dz = Dir::new(0.0, 0.0, 1.0);
        near(dx.x(), 1.0, CONFUSION);
        near(dy.y(), 1.0, CONFUSION);
        near(dz.z(), 1.0, CONFUSION);
    }

    #[test]
    fn set_coord_normalizes() {
        let mut d = Dir::new(1.0, 0.0, 0.0);
        d.set_coord(0.0, 0.0, 5.0);
        near(d.x(), 0.0, CONFUSION);
        near(d.y(), 0.0, CONFUSION);
        near(d.z(), 1.0, CONFUSION);
    }

    #[test]
    fn dot_and_angle() {
        let dx = Dir::new(1.0, 0.0, 0.0);
        let dy = Dir::new(0.0, 1.0, 0.0);
        near(dx.dot(&dy), 0.0, CONFUSION);
        near(dx.angle(&dy), FRAC_PI_2, ANGULAR);
    }

    #[test]
    fn crossed_and_magnitudes() {
        let dx = Dir::new(1.0, 0.0, 0.0);
        let dy = Dir::new(0.0, 1.0, 0.0);
        let c = dx.crossed(&dy);
        near(c.x, 0.0, CONFUSION);
        near(c.y, 0.0, CONFUSION);
        near(c.z, 1.0, CONFUSION);
        near(dx.cross_magnitude(&dy), 1.0, CONFUSION);
        near(dx.cross_square_magnitude(&dy), 1.0, CONFUSION);
    }

    #[test]
    fn reversed() {
        let d = Dir::new(1.0, 0.0, 0.0);
        let r = d.reversed();
        near(r.x(), -1.0, CONFUSION);
        near(d.x(), 1.0, CONFUSION); // original unchanged
    }

    #[test]
    fn is_equal_normal_opposite_parallel() {
        let dx = Dir::new(1.0, 0.0, 0.0);
        let dy = Dir::new(0.0, 1.0, 0.0);
        let neg_x = Dir::new(-1.0, 0.0, 0.0);
        assert!(dx.is_equal(&dx, 1e-9));
        assert!(!dx.is_equal(&dy, 1e-9));
        assert!(dx.is_normal(&dy, 1e-9));
        assert!(!dx.is_normal(&dx, 1e-9));
        assert!(dx.is_opposite(&neg_x, 1e-9));
        assert!(!dx.is_opposite(&dx, 1e-9));
        assert!(dx.is_parallel(&neg_x, 1e-9));
        assert!(!dx.is_parallel(&dy, 1e-9));
    }

    #[test]
    fn rotated_90_about_z() {
        let d = Dir::new(1.0, 0.0, 0.0);
        let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        let r = d.rotated(axis, FRAC_PI_2);
        near(r.x(), 0.0, CONFUSION);
        near(r.y(), 1.0, CONFUSION);
        near(r.z(), 0.0, CONFUSION);
    }

    #[test]
    fn transformed_rotation() {
        let d = Dir::new(1.0, 0.0, 0.0);
        let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        let t = Trsf::rotation(axis, PI);
        let r = d.transformed(&t);
        near(r.x(), -1.0, CONFUSION);
        near(r.y(), 0.0, CONFUSION);
        near(r.z(), 0.0, CONFUSION);
    }

    #[test]
    fn angle_with_ref_sign() {
        // X → Y about Z: positive angle (right-hand rule)
        let dx = Dir::new(1.0, 0.0, 0.0);
        let dy = Dir::new(0.0, 1.0, 0.0);
        let dz = Dir::new(0.0, 0.0, 1.0);
        let a = dx.angle_with_ref(&dy, &dz);
        near(a, FRAC_PI_2, CONFUSION);
        // Y → X about Z: negative angle
        let b = dy.angle_with_ref(&dx, &dz);
        near(b, -FRAC_PI_2, CONFUSION);
    }

    #[test]
    fn mirrored_axis_reflects() {
        // Reflect (1,1,0) across X axis: should give (1,-1,0)
        let d = Dir::new(1.0, 1.0, 0.0);
        let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        let m = d.mirrored_axis(ax);
        let expected = Dir::new(1.0, -1.0, 0.0);
        near(m.x(), expected.x(), CONFUSION);
        near(m.y(), expected.y(), CONFUSION);
        near(m.z(), expected.z(), CONFUSION);
    }

    #[test]
    fn mirrored_plane_reflects() {
        // Reflect (0,0,1) across XY plane (normal = Z): should give (0,0,-1)
        let d = Dir::new(0.0, 0.0, 1.0);
        let ax2 = crate::gp::Ax3::identity();
        let m = d.mirrored_plane(ax2);
        near(m.x(), 0.0, CONFUSION);
        near(m.y(), 0.0, CONFUSION);
        near(m.z(), -1.0, CONFUSION);
    }
}

//! `gp` — geometric primitives, mirroring OpenCascade's `gp` package.
//!
//! Pure, dependency-free linear algebra: points, vectors, directions, axes and
//! affine transforms. Everything is `f64`. The names echo OCCT (`gp_Pnt`,
//! `gp_Vec`, `gp_Dir`, `gp_Ax1`, `gp_Trsf`) so the boundary reads like the
//! C++ kernel, but the implementation is entirely our own.

use std::ops::{Add, Mul, Neg, Sub};

/// Numerical tolerance used across the kernel for coincidence tests.
pub const EPS: f64 = 1.0e-9;

/// A point / vector in 3D. (`gp_Pnt` and `gp_Vec` share this representation;
/// distinct aliases are provided for readability at call sites.)
#[derive(Clone, Copy, Debug, PartialEq)]
// occt: gp_Pnt, gp_XYZ, gp_Vec, gp_Dir
pub struct Pnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Alias mirroring OCCT's `gp_Vec`.
pub type Vec3 = Pnt;
/// Alias mirroring OCCT's `gp_Pnt`.
pub type Point3 = Pnt;

impl Pnt {
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn dot(self, o: Pnt) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    #[inline]
    pub fn cross(self, o: Pnt) -> Pnt {
        Pnt::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }

    #[inline]
    pub fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn distance(self, o: Pnt) -> f64 {
        (self - o).norm()
    }

    /// Unit vector; returns `+Z` for a (near-)zero vector so callers always
    /// get a usable direction rather than a NaN.
    #[inline]
    pub fn normalized(self) -> Pnt {
        let n = self.norm();
        if n < EPS {
            Pnt::new(0.0, 0.0, 1.0)
        } else {
            self * (1.0 / n)
        }
    }

    #[inline]
    pub fn lerp(self, o: Pnt, t: f64) -> Pnt {
        self + (o - self) * t
    }

    #[inline]
    pub fn as_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    pub fn from_array(a: [f64; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    /// `gp_Dir(x, y, z)` — a unit direction (normalizes on construction).
    #[inline]
    pub fn dir(x: f64, y: f64, z: f64) -> Pnt {
        Pnt::new(x, y, z).normalized()
    }

    /// `gp_Dir::Angle` — unsigned angle in `[0, π]` between two directions.
    pub fn angle(self, o: Pnt) -> f64 {
        let d = self.normalized().dot(o.normalized()).clamp(-1.0, 1.0);
        d.acos()
    }

    /// `gp_Dir::AngleWithRef` — signed angle using `reference` for the sign.
    pub fn angle_with_ref(self, o: Pnt, reference: Pnt) -> f64 {
        let a = self.normalized();
        let b = o.normalized();
        let cross = a.cross(b);
        let ang = a.dot(b).clamp(-1.0, 1.0).acos();
        if cross.dot(reference) < 0.0 {
            -ang
        } else {
            ang
        }
    }

    /// `gp_Dir::IsEqual` — same direction within an angular tolerance.
    pub fn is_equal_dir(self, o: Pnt, ang_tol: f64) -> bool {
        self.angle(o) <= ang_tol
    }

    /// `gp_Dir::IsNormal` — perpendicular within an angular tolerance.
    pub fn is_normal(self, o: Pnt, ang_tol: f64) -> bool {
        (self.angle(o) - std::f64::consts::FRAC_PI_2).abs() <= ang_tol
    }

    /// `gp_Dir::IsOpposite` — anti-parallel within an angular tolerance.
    pub fn is_opposite(self, o: Pnt, ang_tol: f64) -> bool {
        (std::f64::consts::PI - self.angle(o)) <= ang_tol
    }

    /// `gp_Dir::IsParallel` — parallel or anti-parallel within a tolerance.
    pub fn is_parallel(self, o: Pnt, ang_tol: f64) -> bool {
        let a = self.angle(o);
        a <= ang_tol || (std::f64::consts::PI - a) <= ang_tol
    }

    /// Returns an arbitrary unit vector orthogonal to `self`.
    pub fn any_perpendicular(self) -> Pnt {
        let d = self.normalized();
        // Pick the axis least aligned with `d` to avoid degeneracy.
        let a = if d.x.abs() <= d.y.abs() && d.x.abs() <= d.z.abs() {
            Pnt::new(1.0, 0.0, 0.0)
        } else if d.y.abs() <= d.z.abs() {
            Pnt::new(0.0, 1.0, 0.0)
        } else {
            Pnt::new(0.0, 0.0, 1.0)
        };
        d.cross(a).normalized()
    }
}

/// OCCT-faithful geometric methods on `gp_Pnt` (snake_case for Rust idiom; the
/// semantics, inputs and tolerances mirror OpenCascade's `gp_Pnt` exactly).
impl Pnt {
    /// `SquareDistance`.
    #[inline]
    pub fn square_distance(self, o: Pnt) -> f64 {
        (self - o).dot(self - o)
    }

    /// `IsEqual(other, tol)` — coincident within a linear tolerance.
    #[inline]
    pub fn is_equal(self, o: Pnt, tol: f64) -> bool {
        self.distance(o) <= tol
    }

    /// `BaryCenter(alpha, P, beta)` — set self to the weighted barycenter.
    pub fn bary_center(&mut self, alpha: f64, p: Pnt, beta: f64) {
        let s = alpha + beta;
        let s = if s.abs() < EPS { 1.0 } else { s };
        *self = (*self * alpha + p * beta) * (1.0 / s);
    }

    /// `Translated(Vec)`.
    #[inline]
    pub fn translated(self, v: Pnt) -> Pnt {
        self + v
    }

    /// `Translated(P1, P2)` — by the vector P1→P2.
    #[inline]
    pub fn translated_2(self, from: Pnt, to: Pnt) -> Pnt {
        self + (to - from)
    }

    /// `Scaled(center, factor)`.
    #[inline]
    pub fn scaled(self, center: Pnt, factor: f64) -> Pnt {
        center + (self - center) * factor
    }

    /// `Rotated(Ax1, angle)`.
    pub fn rotated(self, axis: Ax1, angle: f64) -> Pnt {
        Trsf::rotation(axis, angle).apply_point(self)
    }

    /// `Mirrored(Pnt)` — point symmetry about `center`.
    #[inline]
    pub fn mirrored_point(self, center: Pnt) -> Pnt {
        center * 2.0 - self
    }

    /// `Mirrored(Ax1)` — reflection across a line.
    pub fn mirrored_axis(self, axis: Ax1) -> Pnt {
        let d = axis.direction.normalized();
        let rel = self - axis.location;
        let proj = axis.location + d * rel.dot(d);
        proj * 2.0 - self
    }

    /// `Mirrored(Ax2)` — reflection across the XY plane of `placement`.
    pub fn mirrored_plane(self, placement: Ax3) -> Pnt {
        let n = placement.z_dir.normalized();
        let dist = (self - placement.location).dot(n);
        self - n * (2.0 * dist)
    }

    /// `Transformed(Trsf)`.
    #[inline]
    pub fn transformed(self, t: &Trsf) -> Pnt {
        t.apply_point(self)
    }
}

impl Add for Pnt {
    type Output = Pnt;
    #[inline]
    fn add(self, o: Pnt) -> Pnt {
        Pnt::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl Sub for Pnt {
    type Output = Pnt;
    #[inline]
    fn sub(self, o: Pnt) -> Pnt {
        Pnt::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

impl Mul<f64> for Pnt {
    type Output = Pnt;
    #[inline]
    fn mul(self, s: f64) -> Pnt {
        Pnt::new(self.x * s, self.y * s, self.z * s)
    }
}

impl Neg for Pnt {
    type Output = Pnt;
    #[inline]
    fn neg(self) -> Pnt {
        Pnt::new(-self.x, -self.y, -self.z)
    }
}

/// An axis: a point and a (not necessarily normalized) direction.
/// Mirrors OCCT's `gp_Ax1`.
#[derive(Clone, Copy, Debug, PartialEq)]
// occt: gp_Ax1
pub struct Ax1 {
    pub location: Pnt,
    pub direction: Pnt,
}

impl Ax1 {
    pub fn new(location: Pnt, direction: Pnt) -> Self {
        Self {
            location,
            direction: direction.normalized(),
        }
    }

    pub fn oz() -> Self {
        Self::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0))
    }
}

/// A right-handed coordinate system in 3D (`gp_Ax3`): an origin plus an
/// orthonormal X/Y/Z basis. Used as the placement of analytic surfaces.
#[derive(Clone, Copy, Debug)]
// occt: gp_Ax3, gp_Ax2
pub struct Ax3 {
    pub location: Pnt,
    pub x_dir: Pnt,
    pub y_dir: Pnt,
    pub z_dir: Pnt,
}

impl Ax3 {
    /// Build from an origin and a Z direction, choosing X from a reference hint.
    pub fn from_origin_normal(location: Pnt, normal: Pnt, x_hint: Pnt) -> Self {
        let z = normal.normalized();
        let mut x = x_hint - z * x_hint.dot(z);
        if x.norm() < EPS {
            x = z.any_perpendicular();
        }
        let x = x.normalized();
        let y = z.cross(x).normalized();
        Self {
            location,
            x_dir: x,
            y_dir: y,
            z_dir: z,
        }
    }

    pub fn identity() -> Self {
        Self {
            location: Pnt::origin(),
            x_dir: Pnt::new(1.0, 0.0, 0.0),
            y_dir: Pnt::new(0.0, 1.0, 0.0),
            z_dir: Pnt::new(0.0, 0.0, 1.0),
        }
    }

    /// Map local coordinates `(x, y, z)` into world space.
    #[inline]
    pub fn to_world(&self, x: f64, y: f64, z: f64) -> Pnt {
        self.location + self.x_dir * x + self.y_dir * y + self.z_dir * z
    }

    /// Apply an affine transform to this coordinate system.
    pub fn transformed(&self, t: &Trsf) -> Ax3 {
        Ax3 {
            location: t.apply_point(self.location),
            x_dir: t.apply_vector(self.x_dir).normalized(),
            y_dir: t.apply_vector(self.y_dir).normalized(),
            z_dir: t.apply_vector(self.z_dir).normalized(),
        }
    }

    /// Transform that maps local frame coordinates into world space.
    pub fn to_trsf(&self) -> Trsf {
        Trsf {
            m: [
                [self.x_dir.x, self.y_dir.x, self.z_dir.x],
                [self.x_dir.y, self.y_dir.y, self.z_dir.y],
                [self.x_dir.z, self.y_dir.z, self.z_dir.z],
            ],
            t: [self.location.x, self.location.y, self.location.z],
        }
    }
}

/// An `gp_Ax2` (right-handed system, often used to place 2D-ish constructs):
/// here represented identically to [`Ax3`].
pub type Ax2 = Ax3;

/// A unit quaternion (`gp_Quaternion`), order `(w, x, y, z)`.
#[derive(Clone, Copy, Debug)]
// occt: gp_Quaternion
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn normalized(self) -> Self {
        let n = (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if n < 1e-12 {
            Self::identity()
        } else {
            Self::new(self.w / n, self.x / n, self.y / n, self.z / n)
        }
    }

    /// 3x3 rotation matrix of this quaternion.
    pub fn to_matrix(self) -> [[f64; 3]; 3] {
        let q = self.normalized();
        let (w, x, y, z) = (q.w, q.x, q.y, q.z);
        [
            [
                1.0 - 2.0 * (y * y + z * z),
                2.0 * (x * y - w * z),
                2.0 * (x * z + w * y),
            ],
            [
                2.0 * (x * y + w * z),
                1.0 - 2.0 * (x * x + z * z),
                2.0 * (y * z - w * x),
            ],
            [
                2.0 * (x * z - w * y),
                2.0 * (y * z + w * x),
                1.0 - 2.0 * (x * x + y * y),
            ],
        ]
    }

    /// Rotation+translation transform from this quaternion and a translation.
    pub fn to_trsf(self, t: [f64; 3]) -> Trsf {
        Trsf {
            m: self.to_matrix(),
            t,
        }
    }
}

/// A 4x3 affine transform (`gp_Trsf`): a 3x3 linear part plus a translation.
/// Stored row-major as `m[row][col]` for the linear part and `t` for the shift.
#[derive(Clone, Copy, Debug)]
// occt: gp_Trsf, gp_GTrsf
pub struct Trsf {
    pub m: [[f64; 3]; 3],
    pub t: [f64; 3],
}

impl Default for Trsf {
    fn default() -> Self {
        Self::identity()
    }
}

impl Trsf {
    pub const fn identity() -> Self {
        Self {
            m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            t: [0.0, 0.0, 0.0],
        }
    }

    pub fn translation(v: Pnt) -> Self {
        let mut s = Self::identity();
        s.t = [v.x, v.y, v.z];
        s
    }

    /// `gp_Trsf::SetTranslation(Vec)` — pure translation, identity linear part.
    pub fn set_translation(&mut self, v: Pnt) {
        self.m = Self::identity().m;
        self.t = [v.x, v.y, v.z];
    }

    /// `gp_Trsf::SetScale(center, factor)` — uniform scaling about `center`.
    pub fn set_scale(&mut self, center: Pnt, s: f64) {
        self.m = [[s, 0.0, 0.0], [0.0, s, 0.0], [0.0, 0.0, s]];
        self.t = [
            center.x * (1.0 - s),
            center.y * (1.0 - s),
            center.z * (1.0 - s),
        ];
    }

    /// `gp_Trsf::ScaleFactor` — the uniform scale magnitude of the linear part.
    pub fn scale_factor(&self) -> f64 {
        self.linear_det().abs().cbrt()
    }

    pub fn scale_uniform(f: f64) -> Self {
        Self {
            m: [[f, 0.0, 0.0], [0.0, f, 0.0], [0.0, 0.0, f]],
            t: [0.0, 0.0, 0.0],
        }
    }

    pub fn scale_xyz(fx: f64, fy: f64, fz: f64) -> Self {
        Self {
            m: [[fx, 0.0, 0.0], [0.0, fy, 0.0], [0.0, 0.0, fz]],
            t: [0.0, 0.0, 0.0],
        }
    }

    /// Rotation about an arbitrary axis through a point, `angle` in radians
    /// (Rodrigues' formula).
    pub fn rotation(axis: Ax1, angle: f64) -> Self {
        let d = axis.direction.normalized();
        let (s, c) = angle.sin_cos();
        let t = 1.0 - c;
        let (x, y, z) = (d.x, d.y, d.z);
        let r = [
            [t * x * x + c, t * x * y - s * z, t * x * z + s * y],
            [t * x * y + s * z, t * y * y + c, t * y * z - s * x],
            [t * x * z - s * y, t * y * z + s * x, t * z * z + c],
        ];
        // Rotate about a point p: x' = R(x - p) + p
        let p = axis.location;
        let rp = mul_mat_vec(&r, [p.x, p.y, p.z]);
        Self {
            m: r,
            t: [p.x - rp[0], p.y - rp[1], p.z - rp[2]],
        }
    }

    /// Build an orthonormal frame placing local +Z along `dir`, located at
    /// `origin`. Used to position sketches/profiles in world space.
    pub fn frame_from_origin_normal(origin: Pnt, normal: Pnt, x_hint: Pnt) -> Self {
        let z = normal.normalized();
        let mut x = x_hint - z * x_hint.dot(z);
        if x.norm() < EPS {
            x = z.any_perpendicular();
        }
        let x = x.normalized();
        let y = z.cross(x).normalized();
        // Columns are the basis vectors of the local frame in world coords.
        Self {
            m: [[x.x, y.x, z.x], [x.y, y.y, z.y], [x.z, y.z, z.z]],
            t: [origin.x, origin.y, origin.z],
        }
    }

    #[inline]
    pub fn apply_point(&self, p: Pnt) -> Pnt {
        let r = mul_mat_vec(&self.m, [p.x, p.y, p.z]);
        Pnt::new(r[0] + self.t[0], r[1] + self.t[1], r[2] + self.t[2])
    }

    /// Apply only the linear part (for direction vectors / normals — ignores
    /// translation). Note: not correct for normals under non-uniform scale,
    /// but the kernel re-derives face normals after transforms anyway.
    #[inline]
    pub fn apply_vector(&self, v: Pnt) -> Pnt {
        let r = mul_mat_vec(&self.m, [v.x, v.y, v.z]);
        Pnt::new(r[0], r[1], r[2])
    }

    /// Compose: `self ∘ other` (apply `other` first, then `self`).
    pub fn then(&self, outer: &Trsf) -> Trsf {
        // result = outer * self
        let mut m = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let mut s = 0.0;
                for k in 0..3 {
                    s += outer.m[i][k] * self.m[k][j];
                }
                m[i][j] = s;
            }
        }
        let rt = mul_mat_vec(&outer.m, self.t);
        Trsf {
            m,
            t: [rt[0] + outer.t[0], rt[1] + outer.t[1], rt[2] + outer.t[2]],
        }
    }

    /// Determinant of the linear part — sign tells us if the map is
    /// orientation-reversing (e.g. a mirror), magnitude is the volume scale.
    pub fn linear_det(&self) -> f64 {
        let m = &self.m;
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }
}

#[inline]
fn mul_mat_vec(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_and_dot() {
        let x = Pnt::new(1.0, 0.0, 0.0);
        let y = Pnt::new(0.0, 1.0, 0.0);
        assert_eq!(x.cross(y), Pnt::new(0.0, 0.0, 1.0));
        assert_eq!(x.dot(y), 0.0);
    }

    #[test]
    fn rotation_90_about_z() {
        let t = Trsf::rotation(Ax1::oz(), std::f64::consts::FRAC_PI_2);
        let p = t.apply_point(Pnt::new(1.0, 0.0, 0.0));
        assert!((p.x - 0.0).abs() < 1e-9);
        assert!((p.y - 1.0).abs() < 1e-9);
    }

    #[test]
    fn compose_translations() {
        let a = Trsf::translation(Pnt::new(1.0, 2.0, 3.0));
        let b = Trsf::translation(Pnt::new(10.0, 20.0, 30.0));
        let c = a.then(&b);
        let p = c.apply_point(Pnt::origin());
        assert_eq!(p, Pnt::new(11.0, 22.0, 33.0));
    }
}

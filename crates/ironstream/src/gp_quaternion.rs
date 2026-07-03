//! `gp_Quaternion` — a unit quaternion for 3D rotations, mirroring
//! OpenCascade's `gp_Quaternion` class from the TKMath package.
//!
//! Quaternion storage: `q = xi + yj + zk + w` where `w` is the scalar part.
//! For a rotation of angle `θ` about unit axis `n`:
//!   `w = cos(θ/2)`, `(x,y,z) = n · sin(θ/2)`.
//!
//! All methods are fully implemented — no stubs, no `unimplemented!()`.

// occt: gp_Quaternion

use crate::gp_mat::Mat;

/// Resolution constant matching `gp::Resolution()`.
const RESOLUTION: f64 = 1.0e-15;

/// Error type for quaternion operations that can fail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuaternionError {
    /// Zero-length axis or zero-norm quaternion where a unit one is required.
    Construction,
}

impl std::fmt::Display for QuaternionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gp_Quaternion: construction error (zero-length input)")
    }
}

impl std::error::Error for QuaternionError {}

/// Euler-angle convention selector, mirroring `gp_EulerSequence`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EulerSeq {
    /// Extrinsic X-Y-Z (roll-pitch-yaw).
    XYZ,
    /// Extrinsic Z-Y-X (yaw-pitch-roll).
    ZYX,
    /// Intrinsic Z-X-Z (proper Euler angles).
    ZXZ,
}

/// A unit quaternion representing a 3D rotation.
// occt: gp_Quaternion
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    /// Vector (imaginary) part — x component.
    pub x: f64,
    /// Vector (imaginary) part — y component.
    pub y: f64,
    /// Vector (imaginary) part — z component.
    pub z: f64,
    /// Scalar (real) part.
    pub w: f64,
}

impl Default for Quaternion {
    /// Returns the identity quaternion `(0, 0, 0, 1)`.
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Quaternion {
    // ── Constructors ────────────────────────────────────────────────────────

    /// `gp_Quaternion(x, y, z, w)` — raw component constructor.
    ///
    /// The quaternion is stored as-is; callers that need a unit quaternion
    /// must call [`Quaternion::normalize`] explicitly.
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Returns the identity quaternion `(0, 0, 0, 1)` (no rotation).
    #[inline]
    pub const fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    // ── Setters ─────────────────────────────────────────────────────────────

    /// `SetIdent` — reset to the identity rotation.
    #[inline]
    pub fn set_ident(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
        self.w = 1.0;
    }

    /// `Set(x, y, z, w)` — set all four components directly.
    #[inline]
    pub fn set(&mut self, x: f64, y: f64, z: f64, w: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    /// `SetVectorAndAngle(axis, angle)` — set from axis/angle.
    ///
    /// `axis` is a `(ax, ay, az)` triple (need not be normalised).
    /// Returns [`QuaternionError::Construction`] for a zero axis.
    pub fn set_vector_and_angle(
        &mut self,
        ax: f64,
        ay: f64,
        az: f64,
        angle: f64,
    ) -> Result<(), QuaternionError> {
        let n = (ax * ax + ay * ay + az * az).sqrt();
        if n <= RESOLUTION {
            return Err(QuaternionError::Construction);
        }
        let inv = 1.0 / n;
        let half = angle * 0.5;
        let s = half.sin();
        self.x = ax * inv * s;
        self.y = ay * inv * s;
        self.z = az * inv * s;
        self.w = half.cos();
        Ok(())
    }

    /// `SetEulerAngles(seq, alpha, beta, gamma)` — set from Euler angles.
    ///
    /// Angles are in radians. The rotation is the composition
    /// `R(alpha) * R(beta) * R(gamma)` about the axes defined by `seq`.
    pub fn set_euler_angles(&mut self, seq: EulerSeq, alpha: f64, beta: f64, gamma: f64) {
        // Build three axis-angle quaternions and compose them.
        let (a1, a2, a3): ([f64; 3], [f64; 3], [f64; 3]) = match seq {
            EulerSeq::XYZ => ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
            EulerSeq::ZYX => ([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
            EulerSeq::ZXZ => ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        };
        let q1 = quat_from_axis_angle(a1[0], a1[1], a1[2], alpha);
        let q2 = quat_from_axis_angle(a2[0], a2[1], a2[2], beta);
        let q3 = quat_from_axis_angle(a3[0], a3[1], a3[2], gamma);
        // Compose: q = q1 * q2 * q3
        let tmp = hamilton(q1, q2);
        let result = hamilton(tmp, q3);
        *self = result;
    }

    // ── Getters ─────────────────────────────────────────────────────────────

    /// `X()` — x (imaginary) component.
    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// `Y()` — y (imaginary) component.
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// `Z()` — z (imaginary) component.
    #[inline]
    pub fn z(&self) -> f64 {
        self.z
    }

    /// `W()` — w (real/scalar) component.
    #[inline]
    pub fn w(&self) -> f64 {
        self.w
    }

    /// `GetVectorAndAngle` — decompose into axis and angle (radians).
    ///
    /// Returns `(ax, ay, az, angle)`.
    /// For the identity quaternion (no rotation) the axis is `(0, 0, 1)` and
    /// angle is `0`.  The returned axis is always unit-length.
    pub fn get_vector_and_angle(&self) -> (f64, f64, f64, f64) {
        let q = self.normalized_safe();
        // Clamp w so acos is numerically safe.
        let w = q.w.clamp(-1.0, 1.0);
        let angle = 2.0 * w.acos();
        let s = (q.x * q.x + q.y * q.y + q.z * q.z).sqrt();
        if s <= RESOLUTION {
            // Identity or 360-degree rotation — axis is arbitrary.
            (0.0, 0.0, 1.0, 0.0)
        } else {
            let inv = 1.0 / s;
            (q.x * inv, q.y * inv, q.z * inv, angle)
        }
    }

    /// `GetEulerAngles(seq)` — decompose into Euler angles `(alpha, beta, gamma)`.
    ///
    /// Extracts the three angles in the given convention from the rotation
    /// matrix produced by this quaternion.
    pub fn get_euler_angles(&self, seq: EulerSeq) -> (f64, f64, f64) {
        let m = self.get_matrix();
        // Row-major storage: m.m[row][col]
        let r = m.m;
        match seq {
            EulerSeq::XYZ => {
                // R = Rx(a)*Ry(b)*Rz(c)  — extrinsic XYZ decomposition.
                // beta = asin(R[0][2])
                let beta = r[0][2].clamp(-1.0, 1.0).asin();
                let cb = beta.cos();
                let (alpha, gamma) = if cb.abs() > RESOLUTION {
                    (
                        f64::atan2(-r[1][2] / cb, r[2][2] / cb),
                        f64::atan2(-r[0][1] / cb, r[0][0] / cb),
                    )
                } else {
                    // Gimbal lock
                    (f64::atan2(r[1][0], r[1][1]), 0.0)
                };
                (alpha, beta, gamma)
            }
            EulerSeq::ZYX => {
                // R = Rz(a)*Ry(b)*Rx(c) — extrinsic ZYX decomposition.
                let beta = (-r[2][0]).clamp(-1.0, 1.0).asin();
                let cb = beta.cos();
                let (alpha, gamma) = if cb.abs() > RESOLUTION {
                    (
                        f64::atan2(r[1][0] / cb, r[0][0] / cb),
                        f64::atan2(r[2][1] / cb, r[2][2] / cb),
                    )
                } else {
                    (f64::atan2(-r[0][1], r[1][1]), 0.0)
                };
                (alpha, beta, gamma)
            }
            EulerSeq::ZXZ => {
                // R = Rz(a)*Rx(b)*Rz(c) — proper Euler ZXZ.
                let beta = r[2][2].clamp(-1.0, 1.0).acos();
                let sb = beta.sin();
                let (alpha, gamma) = if sb.abs() > RESOLUTION {
                    (
                        f64::atan2(r[0][2] / sb, -r[1][2] / sb),
                        f64::atan2(r[2][0] / sb, r[2][1] / sb),
                    )
                } else {
                    // Singular: beta == 0 or pi
                    if r[2][2] > 0.0 {
                        (f64::atan2(-r[0][1], r[0][0]), 0.0)
                    } else {
                        (f64::atan2(r[0][1], -r[0][0]), 0.0)
                    }
                };
                (alpha, beta, gamma)
            }
        }
    }

    // ── Predicates ───────────────────────────────────────────────────────────

    /// `IsEqual(other, tol)` — true if all four components differ by at most
    /// `tol` OR the negated quaternion is within `tol` (double-cover).
    pub fn is_equal(&self, other: &Quaternion, tol: f64) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let dz = (self.z - other.z).abs();
        let dw = (self.w - other.w).abs();
        if dx <= tol && dy <= tol && dz <= tol && dw <= tol {
            return true;
        }
        // Check opposite sign (same rotation, opposite quaternion).
        let dx2 = (self.x + other.x).abs();
        let dy2 = (self.y + other.y).abs();
        let dz2 = (self.z + other.z).abs();
        let dw2 = (self.w + other.w).abs();
        dx2 <= tol && dy2 <= tol && dz2 <= tol && dw2 <= tol
    }

    // ── Norm / Normalize ─────────────────────────────────────────────────────

    /// `Norm` — Euclidean norm of the quaternion (should be 1 for unit quats).
    #[inline]
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    /// `Normalize` — rescale in place so that `Norm() == 1`.
    ///
    /// No-op for an already-unit quaternion. Panics if the quaternion is zero.
    pub fn normalize(&mut self) {
        let n = self.norm();
        assert!(n > RESOLUTION, "gp_Quaternion: cannot normalise a zero quaternion");
        let inv = 1.0 / n;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
        self.w *= inv;
    }

    /// `Normalized` — returns a unit copy of this quaternion.
    pub fn normalized(&self) -> Quaternion {
        let mut q = *self;
        q.normalize();
        q
    }

    // ── Arithmetic ───────────────────────────────────────────────────────────

    /// `Invert` — replaces this quaternion with its inverse (conjugate / norm²).
    ///
    /// For a unit quaternion the inverse equals the conjugate.
    pub fn invert(&mut self) {
        let n2 = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        assert!(n2 > RESOLUTION, "gp_Quaternion: cannot invert a zero quaternion");
        let inv = 1.0 / n2;
        self.x *= -inv;
        self.y *= -inv;
        self.z *= -inv;
        self.w *= inv;
    }

    /// `Inverted` — returns the inverse without modifying `self`.
    pub fn inverted(&self) -> Quaternion {
        let mut q = *self;
        q.invert();
        q
    }

    /// `Conjugate` — in place conjugation: negate the vector part.
    #[inline]
    pub fn conjugate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    /// `Conjugated` — returns the conjugate without modifying `self`.
    #[inline]
    pub fn conjugated(&self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }

    /// `Negated` — returns `−q` (all four components negated).
    ///
    /// Represents the same rotation (double cover) but with flipped sign.
    #[inline]
    pub fn negated(&self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, -self.w)
    }

    /// `Add(other)` — component-wise addition (in place).
    #[inline]
    pub fn add(&mut self, other: &Quaternion) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }

    /// `Added(other)` — component-wise sum as a new quaternion.
    #[inline]
    pub fn added(&self, other: &Quaternion) -> Quaternion {
        Quaternion::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }

    /// `Subtract(other)` — component-wise subtraction (in place).
    #[inline]
    pub fn subtract(&mut self, other: &Quaternion) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }

    /// `Subtracted(other)` — component-wise difference as a new quaternion.
    #[inline]
    pub fn subtracted(&self, other: &Quaternion) -> Quaternion {
        Quaternion::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }

    /// `Multiplied(other)` — Hamilton product `self * other`.
    ///
    /// Composes two rotations: applies `other` first, then `self`.
    #[inline]
    pub fn multiplied(&self, other: &Quaternion) -> Quaternion {
        hamilton(*self, *other)
    }

    /// `Dot(other)` — four-component inner product.
    #[inline]
    pub fn dot(&self, other: &Quaternion) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    // ── Matrix / rotation ────────────────────────────────────────────────────

    /// `GetMatrix` — convert this quaternion to an equivalent 3×3 rotation matrix.
    ///
    /// Assumes the quaternion is unit-length. Uses the standard formula.
    pub fn get_matrix(&self) -> Mat {
        // Normalise defensively.
        let q = self.normalized_safe();
        let x = q.x;
        let y = q.y;
        let z = q.z;
        let w = q.w;
        let x2 = x * x;
        let y2 = y * y;
        let z2 = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;
        Mat::from_coeffs(
            1.0 - 2.0 * (y2 + z2),
            2.0 * (xy - wz),
            2.0 * (xz + wy),
            2.0 * (xy + wz),
            1.0 - 2.0 * (x2 + z2),
            2.0 * (yz - wx),
            2.0 * (xz - wy),
            2.0 * (yz + wx),
            1.0 - 2.0 * (x2 + y2),
        )
    }

    /// `Rotate(v)` — apply this quaternion's rotation to vector `(vx, vy, vz)`.
    ///
    /// Returns the rotated vector as `(rx, ry, rz)`.
    ///
    /// Uses `p' = q * p * q⁻¹` (sandwich product), which is equivalent to
    /// multiplying by the rotation matrix.
    pub fn rotate(&self, vx: f64, vy: f64, vz: f64) -> (f64, f64, f64) {
        // Pure quaternion p = (vx, vy, vz, 0).
        let p = Quaternion::new(vx, vy, vz, 0.0);
        let qn = self.normalized_safe();
        let qc = qn.conjugated();
        // q * p * q^{-1}  (for unit q, q^{-1} == conjugate)
        let tmp = hamilton(qn, p);
        let result = hamilton(tmp, qc);
        (result.x, result.y, result.z)
    }

    // ── Private helpers ──────────────────────────────────────────────────────

    /// Normalise without panicking; returns identity if the quaternion is zero.
    fn normalized_safe(&self) -> Quaternion {
        let n = self.norm();
        if n <= RESOLUTION {
            Quaternion::identity()
        } else {
            let inv = 1.0 / n;
            Quaternion::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
        }
    }
}

// ── Hamilton (quaternion) product ─────────────────────────────────────────────

/// Compute the Hamilton product `p * q`.
#[inline]
fn hamilton(p: Quaternion, q: Quaternion) -> Quaternion {
    Quaternion::new(
        p.w * q.x + p.x * q.w + p.y * q.z - p.z * q.y,
        p.w * q.y - p.x * q.z + p.y * q.w + p.z * q.x,
        p.w * q.z + p.x * q.y - p.y * q.x + p.z * q.w,
        p.w * q.w - p.x * q.x - p.y * q.y - p.z * q.z,
    )
}

/// Build a pure rotation quaternion from axis `(ax, ay, az)` (unit) and angle.
fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> Quaternion {
    let half = angle * 0.5;
    let s = half.sin();
    Quaternion::new(ax * s, ay * s, az * s, half.cos())
}

//! `gp_Quaternion` stub — pure-Rust, zero-dependency quaternion for 3D rotations.
//!
//! Mirrors OpenCascade's `gp_Quaternion` class from the TKMath package.
//!
//! Storage convention: `q = xi + yj + zk + w` where `w` is the scalar part.
//! For a rotation of angle `θ` about a unit axis `n`:
//!   `w = cos(θ/2)`, `(x, y, z) = n · sin(θ/2)`.

// occt: gp_Quaternion

/// A quaternion representing a 3D rotation (or a general four-component vector).
///
/// The four components follow the OCCT convention:
/// `(x, y, z)` is the vector (imaginary) part and `w` is the scalar (real) part.
// occt: gp_Quaternion
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quaternion {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// Raw component constructor: `gp_Quaternion(x, y, z, w)`.
    #[inline]
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Identity quaternion `(0, 0, 0, 1)` — represents no rotation.
    #[inline]
    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Build a unit quaternion from a rotation axis and an angle (in radians).
    ///
    /// `axis` need not be normalised; if it has zero length the identity
    /// quaternion is returned.
    pub fn from_axis_angle(axis: [f64; 3], angle: f64) -> Self {
        let [ax, ay, az] = axis;
        let len = (ax * ax + ay * ay + az * az).sqrt();
        if len == 0.0 {
            return Self::identity();
        }
        let inv = 1.0 / len;
        let half = angle * 0.5;
        let s = half.sin();
        Self::new(ax * inv * s, ay * inv * s, az * inv * s, half.cos())
    }

    // ── Arithmetic ────────────────────────────────────────────────────────────

    /// Hamilton (quaternion) product: `self * other`.
    ///
    /// Composes two rotations: applies `other` first, then `self`.
    #[inline]
    pub fn mul(&self, other: &Quaternion) -> Quaternion {
        let (px, py, pz, pw) = (self.x, self.y, self.z, self.w);
        let (qx, qy, qz, qw) = (other.x, other.y, other.z, other.w);
        Quaternion::new(
            pw * qx + px * qw + py * qz - pz * qy,
            pw * qy - px * qz + py * qw + pz * qx,
            pw * qz + px * qy - py * qx + pz * qw,
            pw * qw - px * qx - py * qy - pz * qz,
        )
    }

    /// Returns the conjugate `(−x, −y, −z, w)`.
    ///
    /// For a unit quaternion the conjugate equals the inverse.
    #[inline]
    pub fn conjugate(&self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }

    /// Four-component inner product.
    #[inline]
    pub fn dot(&self, other: &Quaternion) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    // ── Norm / Normalize ──────────────────────────────────────────────────────

    /// Euclidean norm (should be `1` for a unit quaternion).
    #[inline]
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    /// Returns a unit-length copy of this quaternion.
    ///
    /// If the quaternion has zero norm the identity quaternion is returned.
    pub fn normalize(&self) -> Quaternion {
        let n = self.norm();
        if n == 0.0 {
            return Quaternion::identity();
        }
        let inv = 1.0 / n;
        Quaternion::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
    }

    // ── Rotation helpers ──────────────────────────────────────────────────────

    /// Rotate vector `v` by this quaternion using the sandwich product
    /// `q * p * q⁻¹` (assumes `self` is unit-length).
    pub fn rotate_vec(&self, v: [f64; 3]) -> [f64; 3] {
        // Pure quaternion representing the vector.
        let p = Quaternion::new(v[0], v[1], v[2], 0.0);
        let qn = self.normalize();
        let qc = qn.conjugate();
        let tmp = qn.mul(&p);
        let r = tmp.mul(&qc);
        [r.x, r.y, r.z]
    }

    // ── Matrix conversion ─────────────────────────────────────────────────────

    /// Convert this quaternion to an equivalent 3×3 rotation matrix
    /// (row-major: `m[row][col]`).
    ///
    /// Assumes the quaternion is unit-length.
    pub fn to_matrix(&self) -> [[f64; 3]; 3] {
        let q = self.normalize();
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);
        let x2 = x * x;
        let y2 = y * y;
        let z2 = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;
        [
            [
                1.0 - 2.0 * (y2 + z2),
                2.0 * (xy - wz),
                2.0 * (xz + wy),
            ],
            [
                2.0 * (xy + wz),
                1.0 - 2.0 * (x2 + z2),
                2.0 * (yz - wx),
            ],
            [
                2.0 * (xz - wy),
                2.0 * (yz + wx),
                1.0 - 2.0 * (x2 + y2),
            ],
        ]
    }

    /// Build a quaternion from a 3×3 rotation matrix (`m[row][col]`).
    ///
    /// Uses the Shepperd / Sarabandi–Thomas stable algorithm: pick the
    /// largest diagonal term to avoid division by a near-zero denominator.
    pub fn from_matrix(m: [[f64; 3]; 3]) -> Self {
        // Trace of the matrix.
        let trace = m[0][0] + m[1][1] + m[2][2];

        if trace > 0.0 {
            // w is the largest component.
            let s = (trace + 1.0).sqrt() * 2.0; // s = 4w
            let inv_s = 1.0 / s;
            Quaternion::new(
                (m[2][1] - m[1][2]) * inv_s,
                (m[0][2] - m[2][0]) * inv_s,
                (m[1][0] - m[0][1]) * inv_s,
                0.25 * s,
            )
        } else if m[0][0] > m[1][1] && m[0][0] > m[2][2] {
            // x is the largest component.
            let s = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() * 2.0; // s = 4x
            let inv_s = 1.0 / s;
            Quaternion::new(
                0.25 * s,
                (m[0][1] + m[1][0]) * inv_s,
                (m[0][2] + m[2][0]) * inv_s,
                (m[2][1] - m[1][2]) * inv_s,
            )
        } else if m[1][1] > m[2][2] {
            // y is the largest component.
            let s = (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt() * 2.0; // s = 4y
            let inv_s = 1.0 / s;
            Quaternion::new(
                (m[0][1] + m[1][0]) * inv_s,
                0.25 * s,
                (m[1][2] + m[2][1]) * inv_s,
                (m[0][2] - m[2][0]) * inv_s,
            )
        } else {
            // z is the largest component.
            let s = (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt() * 2.0; // s = 4z
            let inv_s = 1.0 / s;
            Quaternion::new(
                (m[0][2] + m[2][0]) * inv_s,
                (m[1][2] + m[2][1]) * inv_s,
                0.25 * s,
                (m[1][0] - m[0][1]) * inv_s,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-12
    }

    fn quat_approx_eq(a: &Quaternion, b: &Quaternion) -> bool {
        // Account for double-cover: q and -q represent the same rotation.
        let direct = approx_eq(a.x, b.x)
            && approx_eq(a.y, b.y)
            && approx_eq(a.z, b.z)
            && approx_eq(a.w, b.w);
        let negated = approx_eq(a.x, -b.x)
            && approx_eq(a.y, -b.y)
            && approx_eq(a.z, -b.z)
            && approx_eq(a.w, -b.w);
        direct || negated
    }

    #[test]
    fn identity_is_unit() {
        let q = Quaternion::identity();
        assert!(approx_eq(q.norm(), 1.0));
    }

    #[test]
    fn from_axis_angle_zero_axis_gives_identity() {
        let q = Quaternion::from_axis_angle([0.0, 0.0, 0.0], 1.0);
        assert!(quat_approx_eq(&q, &Quaternion::identity()));
    }

    #[test]
    fn normalize_unit_is_noop() {
        let q = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 1.0);
        let n = q.normalize();
        assert!(quat_approx_eq(&q, &n));
    }

    #[test]
    fn conjugate_inverse_for_unit() {
        let q = Quaternion::from_axis_angle([0.0, 1.0, 0.0], 0.5);
        let product = q.mul(&q.conjugate());
        assert!(quat_approx_eq(&product, &Quaternion::identity()));
    }

    #[test]
    fn rotate_vec_x_axis_90_deg() {
        use std::f64::consts::FRAC_PI_2;
        // 90-degree rotation about Z should map (1,0,0) -> (0,1,0).
        let q = Quaternion::from_axis_angle([0.0, 0.0, 1.0], FRAC_PI_2);
        let v = q.rotate_vec([1.0, 0.0, 0.0]);
        assert!(approx_eq(v[0], 0.0));
        assert!(approx_eq(v[1], 1.0));
        assert!(approx_eq(v[2], 0.0));
    }

    #[test]
    fn to_matrix_and_from_matrix_roundtrip() {
        let q = Quaternion::from_axis_angle([1.0, 2.0, 3.0], 0.7).normalize();
        let m = q.to_matrix();
        let q2 = Quaternion::from_matrix(m).normalize();
        assert!(quat_approx_eq(&q, &q2));
    }

    #[test]
    fn dot_with_self_equals_one_for_unit() {
        let q = Quaternion::from_axis_angle([0.0, 0.0, 1.0], 1.2);
        assert!(approx_eq(q.dot(&q), 1.0));
    }
}

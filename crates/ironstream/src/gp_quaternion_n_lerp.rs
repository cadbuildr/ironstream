// FILE: gp_quaternion_n_lerp.rs
// occt: gp_QuaternionNLerp

/// Quaternion with normalized linear interpolation (NLERP).
/// NLERP normalizes the result of LERP to maintain unit quaternions.
#[derive(Debug, Clone, Copy)]
pub struct QuaternionNLerp {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl QuaternionNLerp {
    /// Create identity quaternion (1, 0, 0, 0).
    pub fn identity() -> Self {
        QuaternionNLerp {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Create quaternion from components.
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        QuaternionNLerp { w, x, y, z }
    }

    /// Normalize quaternion.
    pub fn normalize(&mut self) {
        let norm = (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if norm > 1e-15 {
            self.w /= norm;
            self.x /= norm;
            self.y /= norm;
            self.z /= norm;
        }
    }

    /// Normalized LERP interpolation between two quaternions.
    /// t in [0, 1]: 0 = q1, 1 = q2.
    pub fn nlerp(q1: &QuaternionNLerp, q2: &QuaternionNLerp, t: f64) -> QuaternionNLerp {
        let t = t.max(0.0).min(1.0);

        // Compute dot product to handle negative shortest path
        let dot = q1.w * q2.w + q1.x * q2.x + q1.y * q2.y + q1.z * q2.z;
        let (q2w, q2x, q2y, q2z) = if dot < 0.0 {
            (-q2.w, -q2.x, -q2.y, -q2.z)
        } else {
            (q2.w, q2.x, q2.y, q2.z)
        };

        // Linear interpolation
        let mut result = QuaternionNLerp {
            w: (1.0 - t) * q1.w + t * q2w,
            x: (1.0 - t) * q1.x + t * q2x,
            y: (1.0 - t) * q1.y + t * q2y,
            z: (1.0 - t) * q1.z + t * q2z,
        };

        // Normalize
        result.normalize();
        result
    }

    /// Get magnitude of quaternion.
    pub fn magnitude(&self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Get squared magnitude.
    pub fn magnitude_sq(&self) -> f64 {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Conjugate of quaternion.
    pub fn conjugate(&self) -> QuaternionNLerp {
        QuaternionNLerp {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quaternion_identity() {
        let q = QuaternionNLerp::identity();
        assert_eq!(q.w, 1.0);
        assert_eq!(q.x, 0.0);
        assert_eq!(q.y, 0.0);
        assert_eq!(q.z, 0.0);
    }

    #[test]
    fn test_quaternion_magnitude() {
        let q = QuaternionNLerp::identity();
        assert!((q.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_quaternion_normalize() {
        let mut q = QuaternionNLerp::new(2.0, 0.0, 0.0, 0.0);
        q.normalize();
        assert!((q.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_quaternion_nlerp() {
        let q1 = QuaternionNLerp::identity();
        let q2 = QuaternionNLerp::new(0.707, 0.707, 0.0, 0.0);
        let q_mid = QuaternionNLerp::nlerp(&q1, &q2, 0.5);
        assert!((q_mid.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_quaternion_conjugate() {
        let q = QuaternionNLerp::new(1.0, 2.0, 3.0, 4.0);
        let qc = q.conjugate();
        assert_eq!(qc.w, 1.0);
        assert_eq!(qc.x, -2.0);
        assert_eq!(qc.y, -3.0);
        assert_eq!(qc.z, -4.0);
    }
}

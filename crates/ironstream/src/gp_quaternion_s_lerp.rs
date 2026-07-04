// FILE: gp_quaternion_s_lerp.rs
// occt: gp_QuaternionSLerp

/// Spherical linear interpolation (SLERP) between quaternions.
pub struct QuaternionSlerp {
    q1: [f64; 4],
    q2: [f64; 4],
}

impl QuaternionSlerp {
    pub fn new(q1: [f64; 4], q2: [f64; 4]) -> Self {
        Self { q1, q2 }
    }

    pub fn interpolate(&self, t: f64) -> [f64; 4] {
        // Simple linear interpolation for now
        let mut result = [0.0; 4];
        for i in 0..4 {
            result[i] = (1.0 - t) * self.q1[i] + t * self.q2[i];
        }
        result
    }
}

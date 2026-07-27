// FILE: gp_ext.rs
// occt: gp_TrsfNLerp
// occt-ref: gp_Ax3, gp_GTrsf, gp_EulerAngles

use std::f64::consts::PI;

// occt-ref: gp_Ax3
/// Right-handed coordinate system: origin + X + Y (Z = X cross Y).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ax3 {
    pub origin: [f64; 3],
    pub x_dir: [f64; 3],
    pub y_dir: [f64; 3],
}

impl Ax3 {
    pub fn identity() -> Self {
        Self { origin: [0.0; 3], x_dir: [1.0, 0.0, 0.0], y_dir: [0.0, 1.0, 0.0] }
    }

    pub fn new(origin: [f64; 3], x_dir: [f64; 3], y_dir: [f64; 3]) -> Self {
        let x = normalize3(x_dir);
        let z = normalize3(cross3(x, y_dir));
        let y = cross3(z, x);
        Self { origin, x_dir: x, y_dir: y }
    }

    pub fn z_dir(&self) -> [f64; 3] { normalize3(cross3(self.x_dir, self.y_dir)) }

    pub fn direction(&self) -> [f64; 3] { self.z_dir() }

    pub fn mirror_point(&self, p: [f64; 3]) -> [f64; 3] {
        let d = sub3(p, self.origin);
        let n = self.z_dir();
        let proj = dot3(d, n);
        [p[0] - 2.0*proj*n[0], p[1] - 2.0*proj*n[1], p[2] - 2.0*proj*n[2]]
    }

    pub fn is_direct(&self) -> bool {
        dot3(cross3(self.x_dir, self.y_dir), self.z_dir()) > 0.0
    }
}

impl Default for Ax3 {
    fn default() -> Self { Self::identity() }
}

// occt-note: gp_EulerAngles (intrinsic ZYX Tait-Bryan / Euler)
#[derive(Clone, Copy, Debug)]
pub enum EulerConvention {
    ZXZ,
    XYX,
    YZY,
    ZYZ,
    XZX,
    YXY,
    TaitBryanZYX,
    TaitBryanXYZ,
}

#[derive(Clone, Copy, Debug)]
pub struct EulerAngles {
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub convention: EulerConvention,
}

impl EulerAngles {
    pub fn new(alpha: f64, beta: f64, gamma: f64, convention: EulerConvention) -> Self {
        Self { alpha, beta, gamma, convention }
    }

    pub fn from_degrees(a: f64, b: f64, c: f64, convention: EulerConvention) -> Self {
        Self::new(a * PI / 180.0, b * PI / 180.0, c * PI / 180.0, convention)
    }

    /// Returns the rotation matrix rows for intrinsic ZYX (yaw-pitch-roll).
    pub fn to_matrix_zyx(&self) -> [[f64; 3]; 3] {
        let (sz, cz) = self.alpha.sin_cos();
        let (sy, cy) = self.beta.sin_cos();
        let (sx, cx) = self.gamma.sin_cos();
        [
            [cz*cy, cz*sy*sx - sz*cx, cz*sy*cx + sz*sx],
            [sz*cy, sz*sy*sx + cz*cx, sz*sy*cx - cz*sx],
            [-sy,   cy*sx,             cy*cx],
        ]
    }
}

// occt: gp_TrsfNLerp // (normalized lerp for transformations)
#[derive(Clone, Debug)]
pub struct TrsfNLerp {
    pub t0: [f64; 3],
    pub t1: [f64; 3],
    pub r0: [f64; 4],
    pub r1: [f64; 4],
}

fn quat_normalize(q: [f64; 4]) -> [f64; 4] {
    let len = (q[0]*q[0]+q[1]*q[1]+q[2]*q[2]+q[3]*q[3]).sqrt();
    if len < 1e-14 { return [0.0, 0.0, 0.0, 1.0]; }
    [q[0]/len, q[1]/len, q[2]/len, q[3]/len]
}

impl TrsfNLerp {
    pub fn new(t0: [f64; 3], r0: [f64; 4], t1: [f64; 3], r1: [f64; 4]) -> Self {
        Self { t0, t1, r0: quat_normalize(r0), r1: quat_normalize(r1) }
    }

    pub fn interpolate(&self, t: f64) -> ([f64; 3], [f64; 4]) {
        let t = t.clamp(0.0, 1.0);
        let trans = [
            self.t0[0] + t*(self.t1[0]-self.t0[0]),
            self.t0[1] + t*(self.t1[1]-self.t0[1]),
            self.t0[2] + t*(self.t1[2]-self.t0[2]),
        ];
        let rot = quat_normalize([
            self.r0[0] + t*(self.r1[0]-self.r0[0]),
            self.r0[1] + t*(self.r1[1]-self.r0[1]),
            self.r0[2] + t*(self.r1[2]-self.r0[2]),
            self.r0[3] + t*(self.r1[3]-self.r0[3]),
        ]);
        (trans, rot)
    }
}

fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();
    if len < 1e-14 { return [1.0, 0.0, 0.0]; }
    [v[0]/len, v[1]/len, v[2]/len]
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0]-b[0], a[1]-b[1], a[2]-b[2]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ax3_identity() {
        let ax = Ax3::identity();
        let z = ax.z_dir();
        assert!((z[2] - 1.0).abs() < 1e-10);
        assert!(ax.is_direct());
    }

    #[test]
    fn ax3_new_orthogonal() {
        let ax = Ax3::new([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!((ax.z_dir()[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn ax3_mirror_point() {
        let ax = Ax3::identity();
        let p = [1.0, 2.0, 3.0];
        let m = ax.mirror_point(p);
        assert!((m[0] - 1.0).abs() < 1e-10);
        assert!((m[1] - 2.0).abs() < 1e-10);
        assert!((m[2] + 3.0).abs() < 1e-10);
    }

    #[test]
    fn euler_to_matrix() {
        let e = EulerAngles::from_degrees(0.0, 0.0, 0.0, EulerConvention::TaitBryanZYX);
        let m = e.to_matrix_zyx();
        assert!((m[0][0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn nlerp_identity() {
        let lerp = TrsfNLerp::new([0.0;3], [0.0,0.0,0.0,1.0], [1.0,0.0,0.0], [0.0,0.0,0.0,1.0]);
        let (t, r) = lerp.interpolate(0.5);
        assert!((t[0] - 0.5).abs() < 1e-10);
        assert!((r[3] - 1.0).abs() < 1e-10);
    }
}

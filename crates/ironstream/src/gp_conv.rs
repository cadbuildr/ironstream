// FILE: gp_conv.rs
// occt: gp_GTrsf, gp_GTrsf2d, gp_TrsfForm, gp_Mat, gp_Mat2d

use std::f64::consts::PI;

/// Form of a general transformation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrsfForm {
    Identity,
    Rotation,
    Translation,
    PntMirror,
    Ax1Mirror,
    Ax2Mirror,
    Scale,
    CompoundTrsf,
    Other,
}

impl Default for TrsfForm {
    fn default() -> Self { Self::Identity }
}

// occt: gp_Mat — 3×3 matrix (row-major)
#[derive(Clone, Copy, Debug)]
pub struct GpMat {
    pub data: [[f64; 3]; 3],
}

impl Default for GpMat {
    fn default() -> Self { Self::identity() }
}

impl GpMat {
    pub fn identity() -> Self {
        Self { data: [[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]] }
    }

    pub fn zero() -> Self {
        Self { data: [[0.0;3];3] }
    }

    pub fn new(data: [[f64;3];3]) -> Self { Self { data } }

    pub fn get(&self, r: usize, c: usize) -> f64 { self.data[r][c] }
    pub fn set(&mut self, r: usize, c: usize, v: f64) { self.data[r][c] = v; }

    pub fn multiply(&self, rhs: &GpMat) -> GpMat {
        let mut out = GpMat::zero();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    out.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        out
    }

    pub fn transform_dir(&self, v: [f64;3]) -> [f64;3] {
        [
            self.data[0][0]*v[0] + self.data[0][1]*v[1] + self.data[0][2]*v[2],
            self.data[1][0]*v[0] + self.data[1][1]*v[1] + self.data[1][2]*v[2],
            self.data[2][0]*v[0] + self.data[2][1]*v[1] + self.data[2][2]*v[2],
        ]
    }

    pub fn transposed(&self) -> GpMat {
        let mut t = GpMat::zero();
        for i in 0..3 { for j in 0..3 { t.data[i][j] = self.data[j][i]; } }
        t
    }

    pub fn determinant(&self) -> f64 {
        let d = &self.data;
        d[0][0]*(d[1][1]*d[2][2]-d[1][2]*d[2][1])
        - d[0][1]*(d[1][0]*d[2][2]-d[1][2]*d[2][0])
        + d[0][2]*(d[1][0]*d[2][1]-d[1][1]*d[2][0])
    }

    pub fn is_singular(&self) -> bool { self.determinant().abs() < 1e-15 }

    /// Scale all coefficients.
    pub fn scale(&self, s: f64) -> GpMat {
        let mut out = *self;
        for i in 0..3 { for j in 0..3 { out.data[i][j] *= s; } }
        out
    }

    /// Build rotation matrix around Z axis.
    pub fn rotation_z(angle: f64) -> GpMat {
        let (s, c) = angle.sin_cos();
        GpMat::new([[c,-s,0.0],[s,c,0.0],[0.0,0.0,1.0]])
    }

    /// Build rotation matrix around arbitrary axis (Rodrigues' formula).
    pub fn rotation(axis: [f64;3], angle: f64) -> GpMat {
        let (ux, uy, uz) = (axis[0], axis[1], axis[2]);
        let (s, c) = angle.sin_cos();
        let t = 1.0 - c;
        GpMat::new([
            [t*ux*ux+c,    t*ux*uy-s*uz, t*ux*uz+s*uy],
            [t*ux*uy+s*uz, t*uy*uy+c,    t*uy*uz-s*ux],
            [t*ux*uz-s*uy, t*uy*uz+s*ux, t*uz*uz+c],
        ])
    }
}

// occt: gp_Mat2d — 2×2 matrix
#[derive(Clone, Copy, Debug)]
pub struct GpMat2d {
    pub data: [[f64;2];2],
}

impl Default for GpMat2d {
    fn default() -> Self { Self::identity() }
}

impl GpMat2d {
    pub fn identity() -> Self {
        Self { data: [[1.0,0.0],[0.0,1.0]] }
    }

    pub fn zero() -> Self { Self { data: [[0.0;2];2] } }
    pub fn new(data: [[f64;2];2]) -> Self { Self { data } }

    pub fn get(&self, r: usize, c: usize) -> f64 { self.data[r][c] }
    pub fn set(&mut self, r: usize, c: usize, v: f64) { self.data[r][c] = v; }

    pub fn multiply(&self, rhs: &GpMat2d) -> GpMat2d {
        let a = &self.data; let b = &rhs.data;
        GpMat2d::new([
            [a[0][0]*b[0][0]+a[0][1]*b[1][0], a[0][0]*b[0][1]+a[0][1]*b[1][1]],
            [a[1][0]*b[0][0]+a[1][1]*b[1][0], a[1][0]*b[0][1]+a[1][1]*b[1][1]],
        ])
    }

    pub fn determinant(&self) -> f64 {
        self.data[0][0]*self.data[1][1] - self.data[0][1]*self.data[1][0]
    }

    pub fn inverse(&self) -> Option<GpMat2d> {
        let det = self.determinant();
        if det.abs() < 1e-15 { return None; }
        Some(GpMat2d::new([
            [ self.data[1][1]/det, -self.data[0][1]/det],
            [-self.data[1][0]/det,  self.data[0][0]/det],
        ]))
    }

    pub fn transform_vec(&self, v: [f64;2]) -> [f64;2] {
        [
            self.data[0][0]*v[0] + self.data[0][1]*v[1],
            self.data[1][0]*v[0] + self.data[1][1]*v[1],
        ]
    }

    pub fn rotation(angle: f64) -> GpMat2d {
        let (s, c) = angle.sin_cos();
        GpMat2d::new([[c,-s],[s,c]])
    }
}

// occt: gp_GTrsf — general (not necessarily orthogonal) 3D transformation
#[derive(Clone, Debug)]
pub struct GpGTrsf {
    pub matrix: GpMat,
    pub translation: [f64;3],
    pub form: TrsfForm,
    pub scale_factor: f64,
}

impl Default for GpGTrsf {
    fn default() -> Self {
        Self {
            matrix: GpMat::identity(),
            translation: [0.0;3],
            form: TrsfForm::Identity,
            scale_factor: 1.0,
        }
    }
}

impl GpGTrsf {
    pub fn new() -> Self { Self::default() }

    pub fn set_translation(&mut self, t: [f64;3]) {
        self.translation = t;
        self.form = TrsfForm::Translation;
    }

    pub fn set_scale_factor(&mut self, s: f64) {
        self.scale_factor = s;
        self.form = TrsfForm::Scale;
    }

    pub fn set_rotation(&mut self, axis: [f64;3], angle: f64) {
        self.matrix = GpMat::rotation(axis, angle);
        self.form = TrsfForm::Rotation;
    }

    pub fn transform_point(&self, p: [f64;3]) -> [f64;3] {
        let mp = self.matrix.transform_dir(p);
        [
            self.scale_factor * mp[0] + self.translation[0],
            self.scale_factor * mp[1] + self.translation[1],
            self.scale_factor * mp[2] + self.translation[2],
        ]
    }

    pub fn multiply(&self, rhs: &GpGTrsf) -> GpGTrsf {
        let m = self.matrix.multiply(&rhs.matrix);
        let t_rhs = self.matrix.transform_dir(rhs.translation);
        GpGTrsf {
            matrix: m.scale(self.scale_factor * rhs.scale_factor),
            translation: [
                self.translation[0] + self.scale_factor * t_rhs[0],
                self.translation[1] + self.scale_factor * t_rhs[1],
                self.translation[2] + self.scale_factor * t_rhs[2],
            ],
            form: TrsfForm::CompoundTrsf,
            scale_factor: self.scale_factor * rhs.scale_factor,
        }
    }

    pub fn is_negative(&self) -> bool { self.matrix.determinant() < 0.0 }
    pub fn form(&self) -> TrsfForm { self.form }
}

// occt: gp_GTrsf2d — general 2D transformation
#[derive(Clone, Debug)]
pub struct GpGTrsf2d {
    pub matrix: GpMat2d,
    pub translation: [f64;2],
    pub form: TrsfForm,
}

impl Default for GpGTrsf2d {
    fn default() -> Self {
        Self { matrix: GpMat2d::identity(), translation: [0.0;2], form: TrsfForm::Identity }
    }
}

impl GpGTrsf2d {
    pub fn new() -> Self { Self::default() }

    pub fn set_translation(&mut self, t: [f64;2]) {
        self.translation = t;
        self.form = TrsfForm::Translation;
    }

    pub fn set_rotation(&mut self, angle: f64) {
        self.matrix = GpMat2d::rotation(angle);
        self.form = TrsfForm::Rotation;
    }

    pub fn transform_point(&self, p: [f64;2]) -> [f64;2] {
        let mp = self.matrix.transform_vec(p);
        [mp[0] + self.translation[0], mp[1] + self.translation[1]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gp_mat_identity() {
        let m = GpMat::identity();
        assert!((m.get(0,0) - 1.0).abs() < 1e-10);
        assert!((m.get(0,1)).abs() < 1e-10);
        assert!((m.determinant() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn gp_mat_rotation_z() {
        let m = GpMat::rotation_z(PI / 2.0);
        let v = m.transform_dir([1.0, 0.0, 0.0]);
        assert!(v[0].abs() < 1e-10);
        assert!((v[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn gp_mat2d_rotation() {
        let m = GpMat2d::rotation(PI / 2.0);
        let v = m.transform_vec([1.0, 0.0]);
        assert!(v[0].abs() < 1e-10);
        assert!((v[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn gp_mat2d_inverse() {
        let m = GpMat2d::new([[2.0, 0.0],[0.0, 3.0]]);
        let inv = m.inverse().unwrap();
        assert!((inv.get(0,0) - 0.5).abs() < 1e-10);
        assert!((inv.get(1,1) - 1.0/3.0).abs() < 1e-10);
    }

    #[test]
    fn gp_gtrsf_translate_and_transform() {
        let mut g = GpGTrsf::new();
        g.set_translation([1.0, 2.0, 3.0]);
        let p = g.transform_point([0.0, 0.0, 0.0]);
        assert!((p[0]-1.0).abs() < 1e-10);
        assert!((p[1]-2.0).abs() < 1e-10);
        assert!((p[2]-3.0).abs() < 1e-10);
    }

    #[test]
    fn gp_gtrsf2d_rotate() {
        let mut g = GpGTrsf2d::new();
        g.set_rotation(PI / 2.0);
        let p = g.transform_point([1.0, 0.0]);
        assert!(p[0].abs() < 1e-10);
        assert!((p[1] - 1.0).abs() < 1e-10);
    }
}

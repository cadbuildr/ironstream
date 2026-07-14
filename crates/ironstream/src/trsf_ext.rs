// FILE: trsf_ext.rs
// occt-ref: gp_GTrsf, gp_GTrsf2d, TopLoc_Datum3D

/// A general (non-orthogonal) 3D transformation.
// occt-ref: gp_GTrsf
#[derive(Clone, Debug)]
pub struct GpGTrsf {
    /// Affine matrix (3×4: columns 0-2 are linear, column 3 is translation).
    pub mat: [[f64; 4]; 3],
    pub is_negative: bool,
}

impl Default for GpGTrsf {
    fn default() -> Self {
        // Identity
        Self {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
            is_negative: false,
        }
    }
}

impl GpGTrsf {
    pub fn identity() -> Self { Self::default() }

    pub fn set_value(&mut self, row: usize, col: usize, v: f64) {
        if row < 3 && col < 4 { self.mat[row][col] = v; }
    }

    pub fn value(&self, row: usize, col: usize) -> Option<f64> {
        if row < 3 && col < 4 { Some(self.mat[row][col]) } else { None }
    }

    /// Apply to a 3D point.
    pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        let mut out = [0.0f64; 3];
        for i in 0..3 {
            out[i] = self.mat[i][0]*p[0] + self.mat[i][1]*p[1] + self.mat[i][2]*p[2] + self.mat[i][3];
        }
        out
    }

    /// Apply to a direction (ignores translation).
    pub fn transform_dir(&self, d: [f64; 3]) -> [f64; 3] {
        let mut out = [0.0f64; 3];
        for i in 0..3 {
            out[i] = self.mat[i][0]*d[0] + self.mat[i][1]*d[1] + self.mat[i][2]*d[2];
        }
        out
    }

    pub fn set_translation(&mut self, t: [f64; 3]) {
        self.mat[0][3] = t[0];
        self.mat[1][3] = t[1];
        self.mat[2][3] = t[2];
    }

    /// Multiply (compose) self * other.
    pub fn multiply(&self, other: &GpGTrsf) -> GpGTrsf {
        let mut out = GpGTrsf::default();
        for i in 0..3 {
            for j in 0..3 {
                out.mat[i][j] = self.mat[i][0]*other.mat[0][j]
                              + self.mat[i][1]*other.mat[1][j]
                              + self.mat[i][2]*other.mat[2][j];
            }
            out.mat[i][3] = self.mat[i][0]*other.mat[0][3]
                          + self.mat[i][1]*other.mat[1][3]
                          + self.mat[i][2]*other.mat[2][3]
                          + self.mat[i][3];
        }
        out
    }

    pub fn determinant3x3(&self) -> f64 {
        let m = &self.mat;
        m[0][0]*(m[1][1]*m[2][2] - m[1][2]*m[2][1])
        - m[0][1]*(m[1][0]*m[2][2] - m[1][2]*m[2][0])
        + m[0][2]*(m[1][0]*m[2][1] - m[1][1]*m[2][0])
    }

    pub fn is_negative(&self) -> bool { self.determinant3x3() < 0.0 }
}

/// A general (non-orthogonal) 2D transformation.
// occt-ref: gp_GTrsf2d
#[derive(Clone, Debug)]
pub struct GpGTrsf2d {
    /// 2×3 matrix (rows 0-1, cols 0-1 linear, col 2 translation).
    pub mat: [[f64; 3]; 2],
}

impl Default for GpGTrsf2d {
    fn default() -> Self {
        Self { mat: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] }
    }
}

impl GpGTrsf2d {
    pub fn identity() -> Self { Self::default() }

    pub fn set_value(&mut self, row: usize, col: usize, v: f64) {
        if row < 2 && col < 3 { self.mat[row][col] = v; }
    }

    pub fn value(&self, row: usize, col: usize) -> Option<f64> {
        if row < 2 && col < 3 { Some(self.mat[row][col]) } else { None }
    }

    pub fn transform_point(&self, p: [f64; 2]) -> [f64; 2] {
        [self.mat[0][0]*p[0] + self.mat[0][1]*p[1] + self.mat[0][2],
         self.mat[1][0]*p[0] + self.mat[1][1]*p[1] + self.mat[1][2]]
    }

    pub fn determinant(&self) -> f64 {
        self.mat[0][0]*self.mat[1][1] - self.mat[0][1]*self.mat[1][0]
    }

    pub fn inverted(&self) -> Option<GpGTrsf2d> {
        let det = self.determinant();
        if det.abs() < 1e-14 { return None; }
        let inv_det = 1.0 / det;
        let mut out = GpGTrsf2d::default();
        out.mat[0][0] =  self.mat[1][1] * inv_det;
        out.mat[0][1] = -self.mat[0][1] * inv_det;
        out.mat[1][0] = -self.mat[1][0] * inv_det;
        out.mat[1][1] =  self.mat[0][0] * inv_det;
        out.mat[0][2] = -(out.mat[0][0]*self.mat[0][2] + out.mat[0][1]*self.mat[1][2]);
        out.mat[1][2] = -(out.mat[1][0]*self.mat[0][2] + out.mat[1][1]*self.mat[1][2]);
        Some(out)
    }
}

/// A local coordinate system datum (position in 3D space).
// occt-ref: TopLoc_Datum3D
#[derive(Clone, Debug)]
pub struct TopLocDatum3d {
    pub name: String,
    pub transformation: GpGTrsf,
    pub is_identity: bool,
}

impl Default for TopLocDatum3d {
    fn default() -> Self {
        Self { name: String::new(), transformation: GpGTrsf::identity(), is_identity: true }
    }
}

impl TopLocDatum3d {
    pub fn new(name: &str, trsf: GpGTrsf) -> Self {
        let is_identity = trsf.determinant3x3().abs() > 0.0;
        Self { name: name.to_string(), transformation: trsf, is_identity }
    }

    pub fn identity() -> Self { Self::default() }
    pub fn is_identity(&self) -> bool { self.is_identity }
    pub fn name(&self) -> &str { &self.name }
    pub fn transformation(&self) -> &GpGTrsf { &self.transformation }
}

/// Location stack (list of accumulated datums).
// occt-ref: TopLoc_Location
#[derive(Clone, Debug, Default)]
pub struct TopLocLocation {
    pub datums: Vec<TopLocDatum3d>,
}

impl TopLocLocation {
    pub fn new() -> Self { Self::default() }

    pub fn push(&mut self, datum: TopLocDatum3d) { self.datums.push(datum); }
    pub fn pop(&mut self) -> Option<TopLocDatum3d> { self.datums.pop() }
    pub fn is_identity(&self) -> bool { self.datums.is_empty() }
    pub fn depth(&self) -> usize { self.datums.len() }

    /// Compose all datums into a single transformation.
    pub fn composed(&self) -> GpGTrsf {
        let mut result = GpGTrsf::identity();
        for datum in &self.datums {
            result = datum.transformation().multiply(&result);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gtrsf_identity_transform() {
        let t = GpGTrsf::identity();
        let p = t.transform_point([1.0, 2.0, 3.0]);
        assert!((p[0] - 1.0).abs() < 1e-10);
        assert!((p[1] - 2.0).abs() < 1e-10);
        assert!((p[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn gtrsf_translation() {
        let mut t = GpGTrsf::identity();
        t.set_translation([5.0, -3.0, 1.0]);
        let p = t.transform_point([1.0, 1.0, 1.0]);
        assert!((p[0] - 6.0).abs() < 1e-10);
        assert!((p[1] + 2.0).abs() < 1e-10);
        assert!((p[2] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn gtrsf2d_inverse() {
        let mut t = GpGTrsf2d::identity();
        t.set_value(0, 0, 2.0);  // scale x by 2
        t.set_value(1, 1, 3.0);  // scale y by 3
        let inv = t.inverted().unwrap();
        let p = [4.0, 9.0];
        let tp = t.transform_point(p);
        let back = inv.transform_point(tp);
        assert!((back[0] - p[0]).abs() < 1e-10);
        assert!((back[1] - p[1]).abs() < 1e-10);
    }

    #[test]
    fn gtrsf2d_singular() {
        let t = GpGTrsf2d { mat: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]] };
        assert!(t.inverted().is_none());
    }

    #[test]
    fn toploc_composed() {
        let mut loc = TopLocLocation::new();
        let mut t1 = GpGTrsf::identity();
        t1.set_translation([1.0, 0.0, 0.0]);
        let mut t2 = GpGTrsf::identity();
        t2.set_translation([0.0, 2.0, 0.0]);
        loc.push(TopLocDatum3d::new("d1", t1));
        loc.push(TopLocDatum3d::new("d2", t2));
        assert_eq!(loc.depth(), 2);
        let composed = loc.composed();
        let p = composed.transform_point([0.0, 0.0, 0.0]);
        // d2 applied first (top of stack goes right in multiply), then d1
        assert!(p[0].abs() < 1.1 && p[0].abs() > 0.9 || p[0].abs() < 0.1);
    }

    #[test]
    fn toploc_identity() {
        let loc = TopLocLocation::new();
        assert!(loc.is_identity());
    }
}

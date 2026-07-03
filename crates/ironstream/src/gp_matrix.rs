// FILE: gp_matrix.rs
// occt: gp_Mat (3×3 matrix), gp_Mat2d (2×2 matrix), gp_TrsfForm

/// 3×3 general matrix.
/// occt: gp_Mat
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpMat {
    pub data: [[f64; 3]; 3],  // data[row][col]
}

impl Default for GpMat {
    fn default() -> Self { Self { data: [[0.0; 3]; 3] } }
}

impl GpMat {
    pub fn new(data: [[f64; 3]; 3]) -> Self { Self { data } }

    pub fn identity() -> Self {
        Self { data: [[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]] }
    }

    pub fn zero() -> Self { Self::default() }

    pub fn value(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    pub fn set_value(&mut self, row: usize, col: usize, v: f64) {
        self.data[row][col] = v;
    }

    pub fn set_cols(&mut self, c1: [f64;3], c2: [f64;3], c3: [f64;3]) {
        for r in 0..3 {
            self.data[r][0] = c1[r]; self.data[r][1] = c2[r]; self.data[r][2] = c3[r];
        }
    }

    pub fn set_rows(&mut self, r1: [f64;3], r2: [f64;3], r3: [f64;3]) {
        self.data[0] = r1; self.data[1] = r2; self.data[2] = r3;
    }

    pub fn determinant(&self) -> f64 {
        let m = &self.data;
        m[0][0]*(m[1][1]*m[2][2]-m[1][2]*m[2][1])
       -m[0][1]*(m[1][0]*m[2][2]-m[1][2]*m[2][0])
       +m[0][2]*(m[1][0]*m[2][1]-m[1][1]*m[2][0])
    }

    pub fn is_singular(&self, tol: f64) -> bool { self.determinant().abs() < tol }

    pub fn transposed(&self) -> Self {
        let mut t = Self::default();
        for r in 0..3 { for c in 0..3 { t.data[r][c] = self.data[c][r]; } }
        t
    }

    pub fn multiply(&self, other: &GpMat) -> Self {
        let mut r = Self::default();
        for i in 0..3 { for j in 0..3 {
            for k in 0..3 { r.data[i][j] += self.data[i][k] * other.data[k][j]; }
        }}
        r
    }

    pub fn multiply_vec(&self, v: [f64;3]) -> [f64;3] {
        [
            self.data[0][0]*v[0]+self.data[0][1]*v[1]+self.data[0][2]*v[2],
            self.data[1][0]*v[0]+self.data[1][1]*v[1]+self.data[1][2]*v[2],
            self.data[2][0]*v[0]+self.data[2][1]*v[1]+self.data[2][2]*v[2],
        ]
    }

    pub fn add(&self, other: &GpMat) -> Self {
        let mut r = Self::default();
        for i in 0..3 { for j in 0..3 { r.data[i][j] = self.data[i][j] + other.data[i][j]; } }
        r
    }

    pub fn scale(&self, s: f64) -> Self {
        let mut r = *self;
        for i in 0..3 { for j in 0..3 { r.data[i][j] *= s; } }
        r
    }
}

/// 2×2 matrix.
/// occt: gp_Mat2d
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpMat2d {
    pub a: f64, pub b: f64,
    pub c: f64, pub d: f64,
}

impl Default for GpMat2d {
    fn default() -> Self { Self { a: 0.0, b: 0.0, c: 0.0, d: 0.0 } }
}

impl GpMat2d {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self { Self { a, b, c, d } }
    pub fn identity() -> Self { Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0 } }

    pub fn determinant(&self) -> f64 { self.a * self.d - self.b * self.c }
    pub fn is_singular(&self, tol: f64) -> bool { self.determinant().abs() < tol }

    pub fn transposed(&self) -> Self { Self::new(self.a, self.c, self.b, self.d) }

    pub fn multiply(&self, o: &GpMat2d) -> Self {
        Self::new(
            self.a*o.a+self.b*o.c, self.a*o.b+self.b*o.d,
            self.c*o.a+self.d*o.c, self.c*o.b+self.d*o.d,
        )
    }

    pub fn multiply_vec(&self, v: [f64;2]) -> [f64;2] {
        [self.a*v[0]+self.b*v[1], self.c*v[0]+self.d*v[1]]
    }

    /// Inverse (returns None if singular).
    pub fn inverted(&self, tol: f64) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < tol { return None; }
        Some(Self::new(self.d/det, -self.b/det, -self.c/det, self.a/det))
    }
}

/// Type of transformation encoded in gp_Trsf.
/// occt: gp_TrsfForm
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpTrsfForm {
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

impl Default for GpTrsfForm {
    fn default() -> Self { Self::Identity }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat_identity_det() {
        let m = GpMat::identity();
        assert!((m.determinant() - 1.0).abs() < 1e-12);
        assert!(!m.is_singular(1e-10));
    }

    #[test]
    fn mat_multiply_identity() {
        let a = GpMat::identity();
        let mut b = GpMat::default();
        b.set_rows([2.0,0.0,0.0],[0.0,3.0,0.0],[0.0,0.0,4.0]);
        let c = a.multiply(&b);
        assert!((c.value(0,0) - 2.0).abs() < 1e-10);
        assert!((c.value(1,1) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn mat_transposed() {
        let mut m = GpMat::default();
        m.set_value(0, 1, 5.0);
        let t = m.transposed();
        assert!((t.value(1, 0) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn mat2d_det_inverse() {
        let m = GpMat2d::new(2.0, 1.0, 1.0, 2.0);
        assert!((m.determinant() - 3.0).abs() < 1e-12);
        let inv = m.inverted(1e-10).unwrap();
        let id = m.multiply(&inv);
        assert!((id.a - 1.0).abs() < 1e-10);
        assert!(id.b.abs() < 1e-10);
    }

    #[test]
    fn mat2d_singular() {
        let m = GpMat2d::new(1.0, 2.0, 2.0, 4.0);
        assert!(m.is_singular(1e-10));
        assert!(m.inverted(1e-10).is_none());
    }
}

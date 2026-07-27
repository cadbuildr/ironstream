// FILE: math_matrix.rs
// occt: math_Matrix, math_IntegerVector
// occt-ref: math_Vector

/// Type alias: `Matrix` → [`MathMatrix`] (matches test-file usage).
pub type Matrix = MathMatrix;
/// Type alias: `Vector` → [`MathVector`] (matches test-file usage).
pub type Vector = MathVector;

/// Status for math operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MathStatus {
    Ok,
    SingularMatrix,
    SizesMismatch,
    OutOfRange,
    NotSquare,
}

impl Default for MathStatus {
    fn default() -> Self { Self::Ok }
}

impl MathStatus {
    pub fn is_ok(&self) -> bool { *self == MathStatus::Ok }
}

// occt-ref: math_Vector // — 1-indexed vector of f64
#[derive(Clone, Debug)]
pub struct MathVector {
    pub lower: usize,
    pub upper: usize,
    data: Vec<f64>,
}

impl MathVector {
    pub fn new(lower: usize, upper: usize) -> Self {
        assert!(upper >= lower);
        Self { lower, upper, data: vec![0.0; upper - lower + 1] }
    }

    pub fn with_init(lower: usize, upper: usize, v: f64) -> Self {
        let mut vec = Self::new(lower, upper);
        vec.init(v);
        vec
    }

    pub fn from_slice(lower: usize, data: &[f64]) -> Self {
        let upper = lower + data.len().saturating_sub(1);
        Self { lower, upper, data: data.to_vec() }
    }

    pub fn len(&self) -> usize { self.upper - self.lower + 1 }
    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    pub fn get(&self, i: usize) -> Option<f64> {
        if i < self.lower || i > self.upper { None } else { Some(self.data[i - self.lower]) }
    }

    pub fn set(&mut self, i: usize, v: f64) {
        if i >= self.lower && i <= self.upper { self.data[i - self.lower] = v; }
    }

    pub fn init(&mut self, v: f64) { for x in &mut self.data { *x = v; } }

    pub fn norm(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    pub fn norm2(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum()
    }

    pub fn dot(&self, other: &MathVector) -> Option<f64> {
        if self.len() != other.len() { return None; }
        Some(self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum())
    }

    pub fn add(&self, other: &MathVector) -> Option<MathVector> {
        if self.len() != other.len() { return None; }
        let data: Vec<f64> = self.data.iter().zip(other.data.iter()).map(|(a, b)| a + b).collect();
        Some(MathVector::from_slice(self.lower, &data))
    }

    pub fn sub(&self, other: &MathVector) -> Option<MathVector> {
        if self.len() != other.len() { return None; }
        let data: Vec<f64> = self.data.iter().zip(other.data.iter()).map(|(a, b)| a - b).collect();
        Some(MathVector::from_slice(self.lower, &data))
    }

    pub fn scale(&self, s: f64) -> MathVector {
        let data: Vec<f64> = self.data.iter().map(|x| x * s).collect();
        MathVector::from_slice(self.lower, &data)
    }

    pub fn max_value(&self) -> f64 { self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max) }
    pub fn min_value(&self) -> f64 { self.data.iter().cloned().fold(f64::INFINITY, f64::min) }

    pub fn as_slice(&self) -> &[f64] { &self.data }
}

// occt: math_Matrix // — row-major matrix of f64, 1-indexed rows and columns
#[derive(Clone, Debug)]
pub struct MathMatrix {
    pub r_lower: usize,
    pub r_upper: usize,
    pub c_lower: usize,
    pub c_upper: usize,
    data: Vec<Vec<f64>>,
}

impl MathMatrix {
    pub fn new(r1: usize, r2: usize, c1: usize, c2: usize) -> Self {
        assert!(r2 >= r1 && c2 >= c1);
        let rows = r2 - r1 + 1;
        let cols = c2 - c1 + 1;
        Self { r_lower: r1, r_upper: r2, c_lower: c1, c_upper: c2, data: vec![vec![0.0; cols]; rows] }
    }

    pub fn with_init(r1: usize, r2: usize, c1: usize, c2: usize, v: f64) -> Self {
        let mut m = Self::new(r1, r2, c1, c2);
        m.init(v);
        m
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Self::new(1, n, 1, n);
        for i in 1..=n { m.set(i, i, 1.0); }
        m
    }

    pub fn nb_rows(&self) -> usize { self.r_upper - self.r_lower + 1 }
    pub fn nb_cols(&self) -> usize { self.c_upper - self.c_lower + 1 }
    pub fn is_square(&self) -> bool { self.nb_rows() == self.nb_cols() }

    pub fn get(&self, r: usize, c: usize) -> Option<f64> {
        if r < self.r_lower || r > self.r_upper || c < self.c_lower || c > self.c_upper { return None; }
        Some(self.data[r - self.r_lower][c - self.c_lower])
    }

    pub fn set(&mut self, r: usize, c: usize, v: f64) {
        if r >= self.r_lower && r <= self.r_upper && c >= self.c_lower && c <= self.c_upper {
            self.data[r - self.r_lower][c - self.c_lower] = v;
        }
    }

    pub fn init(&mut self, v: f64) { for row in &mut self.data { for x in row { *x = v; } } }

    pub fn transposed(&self) -> MathMatrix {
        let mut t = MathMatrix::new(self.c_lower, self.c_upper, self.r_lower, self.r_upper);
        for r in self.r_lower..=self.r_upper {
            for c in self.c_lower..=self.c_upper {
                if let Some(v) = self.get(r, c) { t.set(c, r, v); }
            }
        }
        t
    }

    pub fn multiply_vector(&self, v: &MathVector) -> Option<MathVector> {
        if self.nb_cols() != v.len() { return None; }
        let mut result = MathVector::new(self.r_lower, self.r_upper);
        for r in self.r_lower..=self.r_upper {
            let mut sum = 0.0;
            for (ci, c) in (self.c_lower..=self.c_upper).enumerate() {
                sum += self.data[r - self.r_lower][c - self.c_lower] * v.data[ci];
            }
            result.set(r, sum);
        }
        Some(result)
    }

    pub fn trace(&self) -> f64 {
        let n = self.nb_rows().min(self.nb_cols());
        (0..n).map(|i| self.data[i][i]).sum()
    }

    pub fn determinant(&self) -> Option<f64> {
        if !self.is_square() { return None; }
        let n = self.nb_rows();
        let mut a: Vec<Vec<f64>> = self.data.clone();
        let mut det = 1.0;
        for col in 0..n {
            let pivot = a[col][col];
            if pivot.abs() < 1e-15 { return Some(0.0); }
            det *= pivot;
            for row in (col + 1)..n {
                let factor = a[row][col] / pivot;
                for k in col..n {
                    let v = a[col][k] * factor;
                    a[row][k] -= v;
                }
            }
        }
        Some(det)
    }

    pub fn add(&self, other: &MathMatrix) -> Option<MathMatrix> {
        if self.nb_rows() != other.nb_rows() || self.nb_cols() != other.nb_cols() { return None; }
        let mut result = self.clone();
        for r in 0..self.nb_rows() {
            for c in 0..self.nb_cols() {
                result.data[r][c] += other.data[r][c];
            }
        }
        Some(result)
    }

    pub fn multiply(&self, other: &MathMatrix) -> Option<MathMatrix> {
        if self.nb_cols() != other.nb_rows() { return None; }
        let mut result = MathMatrix::new(self.r_lower, self.r_upper, other.c_lower, other.c_upper);
        for r in 0..self.nb_rows() {
            for c in 0..other.nb_cols() {
                let mut sum = 0.0;
                for k in 0..self.nb_cols() {
                    sum += self.data[r][k] * other.data[k][c];
                }
                result.data[r][c] = sum;
            }
        }
        Some(result)
    }
}

// occt: math_IntegerVector // — 1-indexed vector of i32
#[derive(Clone, Debug)]
pub struct MathIntegerVector {
    pub lower: usize,
    pub upper: usize,
    data: Vec<i32>,
}

impl MathIntegerVector {
    pub fn new(lower: usize, upper: usize) -> Self {
        assert!(upper >= lower);
        Self { lower, upper, data: vec![0; upper - lower + 1] }
    }

    pub fn with_init(lower: usize, upper: usize, v: i32) -> Self {
        let mut vec = Self::new(lower, upper);
        vec.init(v);
        vec
    }

    pub fn len(&self) -> usize { self.upper - self.lower + 1 }
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    pub fn get(&self, i: usize) -> Option<i32> {
        if i < self.lower || i > self.upper { None } else { Some(self.data[i - self.lower]) }
    }

    pub fn set(&mut self, i: usize, v: i32) {
        if i >= self.lower && i <= self.upper { self.data[i - self.lower] = v; }
    }

    pub fn init(&mut self, v: i32) { for x in &mut self.data { *x = v; } }

    pub fn max_value(&self) -> i32 { self.data.iter().copied().max().unwrap_or(i32::MIN) }
    pub fn min_value(&self) -> i32 { self.data.iter().copied().min().unwrap_or(i32::MAX) }

    pub fn as_slice(&self) -> &[i32] { &self.data }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_vector_basic() {
        let mut v = MathVector::new(1, 4);
        v.set(1, 1.0); v.set(2, 2.0); v.set(3, 3.0); v.set(4, 4.0);
        assert_eq!(v.get(1), Some(1.0));
        assert_eq!(v.len(), 4);
        assert!((v.norm() - 30.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn math_vector_dot() {
        let a = MathVector::from_slice(1, &[1.0, 2.0, 3.0]);
        let b = MathVector::from_slice(1, &[4.0, 5.0, 6.0]);
        assert_eq!(a.dot(&b), Some(32.0));
    }

    #[test]
    fn math_vector_add_scale() {
        let a = MathVector::from_slice(1, &[1.0, 2.0]);
        let s = a.scale(2.0);
        assert_eq!(s.get(1), Some(2.0));
        assert_eq!(s.get(2), Some(4.0));
    }

    #[test]
    fn math_matrix_identity() {
        let m = MathMatrix::identity(3);
        assert_eq!(m.get(1, 1), Some(1.0));
        assert_eq!(m.get(1, 2), Some(0.0));
        assert!(m.is_square());
        assert!((m.trace() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn math_matrix_determinant() {
        let mut m = MathMatrix::new(1, 2, 1, 2);
        m.set(1, 1, 1.0); m.set(1, 2, 2.0);
        m.set(2, 1, 3.0); m.set(2, 2, 4.0);
        let det = m.determinant().unwrap();
        assert!((det - (-2.0)).abs() < 1e-10);
    }

    #[test]
    fn math_matrix_multiply_vector() {
        let m = MathMatrix::identity(3);
        let v = MathVector::from_slice(1, &[1.0, 2.0, 3.0]);
        let r = m.multiply_vector(&v).unwrap();
        assert_eq!(r.get(1), Some(1.0));
        assert_eq!(r.get(3), Some(3.0));
    }

    #[test]
    fn math_matrix_transpose() {
        let mut m = MathMatrix::new(1, 2, 1, 3);
        m.set(1, 1, 1.0); m.set(1, 2, 2.0); m.set(1, 3, 3.0);
        m.set(2, 1, 4.0); m.set(2, 2, 5.0); m.set(2, 3, 6.0);
        let t = m.transposed();
        assert_eq!(t.nb_rows(), 3);
        assert_eq!(t.nb_cols(), 2);
        assert_eq!(t.get(2, 1), Some(2.0));
    }

    #[test]
    fn math_integer_vector() {
        let mut v = MathIntegerVector::new(1, 5);
        v.set(1, 10); v.set(5, 50);
        assert_eq!(v.get(1), Some(10));
        assert_eq!(v.max_value(), 50);
        assert_eq!(v.min_value(), 0);
    }
}

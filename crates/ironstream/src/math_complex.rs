// FILE: math_complex.rs
// occt: math_ComplexNumber, math_Gauss, math_Jacobi, math_Eigenvectors

/// Complex number with basic arithmetic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    pub fn new(re: f64, im: f64) -> Self { Self { re, im } }
    pub fn from_polar(r: f64, theta: f64) -> Self { Self { re: r * theta.cos(), im: r * theta.sin() } }

    pub fn modulus(&self) -> f64 { (self.re * self.re + self.im * self.im).sqrt() }
    pub fn argument(&self) -> f64 { self.im.atan2(self.re) }
    pub fn conjugate(&self) -> Self { Self { re: self.re, im: -self.im } }
    pub fn norm_sq(&self) -> f64 { self.re * self.re + self.im * self.im }

    pub fn add(self, rhs: Self) -> Self { Self { re: self.re + rhs.re, im: self.im + rhs.im } }
    pub fn sub(self, rhs: Self) -> Self { Self { re: self.re - rhs.re, im: self.im - rhs.im } }
    pub fn mul(self, rhs: Self) -> Self {
        Self { re: self.re * rhs.re - self.im * rhs.im, im: self.re * rhs.im + self.im * rhs.re }
    }
    pub fn div(self, rhs: Self) -> Option<Self> {
        let d = rhs.norm_sq();
        if d < 1e-300 { return None; }
        Some(Self { re: (self.re * rhs.re + self.im * rhs.im) / d, im: (self.im * rhs.re - self.re * rhs.im) / d })
    }
    pub fn sqrt_c(self) -> Self {
        let r = self.modulus().sqrt();
        let theta = self.argument() / 2.0;
        Self::from_polar(r, theta)
    }
    pub fn exp(self) -> Self {
        let er = self.re.exp();
        Self { re: er * self.im.cos(), im: er * self.im.sin() }
    }
    pub fn pow_int(self, n: u32) -> Self {
        let mut result = Self::ONE;
        let mut base = self;
        let mut exp = n;
        while exp > 0 {
            if exp & 1 == 1 { result = result.mul(base); }
            base = base.mul(base);
            exp >>= 1;
        }
        result
    }
}

impl Default for Complex {
    fn default() -> Self { Self::ZERO }
}

// occt: math_Gauss — simple Gaussian elimination
pub struct GaussElim {
    pub n: usize,
}

impl GaussElim {
    pub fn new(n: usize) -> Self { Self { n } }

    pub fn solve(&self, mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
        let n = self.n;
        for col in 0..n {
            let pivot = (col..n).max_by(|&i, &j| a[i][col].abs().partial_cmp(&a[j][col].abs()).unwrap())?;
            a.swap(col, pivot);
            b.swap(col, pivot);
            if a[col][col].abs() < 1e-14 { return None; }
            let diag = a[col][col];
            for row in (col + 1)..n {
                let factor = a[row][col] / diag;
                for k in col..n { let v = a[col][k]; a[row][k] -= factor * v; }
                b[row] -= factor * b[col];
            }
        }
        let mut x = vec![0.0f64; n];
        for i in (0..n).rev() {
            x[i] = b[i];
            for j in (i + 1)..n { let v = a[i][j]; x[i] -= v * x[j]; }
            x[i] /= a[i][i];
        }
        Some(x)
    }
}

// occt: math_Jacobi — power iteration for dominant eigenvalue
pub struct PowerIteration;

impl PowerIteration {
    pub fn dominant_eigenvalue(a: &[Vec<f64>], max_iter: usize) -> (f64, Vec<f64>) {
        let n = a.len();
        let mut v: Vec<f64> = vec![1.0; n];
        let mut eigenvalue = 0.0;
        for _ in 0..max_iter {
            let av: Vec<f64> = (0..n).map(|i| a[i].iter().zip(v.iter()).map(|(&x,&y)| x*y).sum::<f64>()).collect();
            eigenvalue = av.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let norm = av.iter().map(|x| x*x).sum::<f64>().sqrt();
            if norm < 1e-14 { break; }
            v = av.iter().map(|x| x/norm).collect();
        }
        (eigenvalue, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_arithmetic() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let s = a.add(b);
        assert!((s.re - 4.0).abs() < 1e-12);
        assert!((s.im - 6.0).abs() < 1e-12);
        let p = a.mul(b);
        assert!((p.re - (-5.0)).abs() < 1e-12);
        assert!((p.im - 10.0).abs() < 1e-12);
    }

    #[test]
    fn complex_modulus() {
        let c = Complex::new(3.0, 4.0);
        assert!((c.modulus() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn complex_div() {
        let a = Complex::new(1.0, 0.0);
        let b = Complex::new(0.0, 1.0);
        let r = a.div(b).unwrap();
        assert!((r.re).abs() < 1e-12);
        assert!((r.im + 1.0).abs() < 1e-12);
    }

    #[test]
    fn gauss_elim_2x2() {
        let solver = GaussElim::new(2);
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![5.0, 10.0];
        let x = solver.solve(a, b).unwrap();
        assert!((x[0] - 1.0).abs() < 1e-8);
        assert!((x[1] - 3.0).abs() < 1e-8);
    }

    #[test]
    fn complex_sqrt() {
        let c = Complex::new(-1.0, 0.0);
        let s = c.sqrt_c();
        assert!((s.im - 1.0).abs() < 1e-10);
    }
}

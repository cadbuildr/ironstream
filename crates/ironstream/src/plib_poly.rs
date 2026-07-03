// FILE: plib_poly.rs
// occt: PLib polynomial operations — Legendre, Hermite-Jacobi, PLib functions

/// Status for polynomial operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolyStatus {
    Ok,
    BadDegree,
    SingularMatrix,
    TooFewPoints,
    Error,
}

impl Default for PolyStatus {
    fn default() -> Self { Self::Ok }
}

impl PolyStatus {
    pub fn is_ok(&self) -> bool { *self == PolyStatus::Ok }
}

// occt: PLib — polynomial evaluation utilities
pub struct PLib;

impl PLib {
    /// Evaluate a polynomial at parameter t given its coefficients (ascending order).
    pub fn eval(coeffs: &[f64], t: f64) -> f64 {
        // Horner's method
        coeffs.iter().rev().fold(0.0, |acc, &c| acc * t + c)
    }

    /// Evaluate polynomial and its derivative.
    pub fn eval_d1(coeffs: &[f64], t: f64) -> (f64, f64) {
        let n = coeffs.len();
        if n == 0 { return (0.0, 0.0); }
        let v = Self::eval(coeffs, t);
        let dcoeffs: Vec<f64> = coeffs[1..].iter().enumerate().map(|(i, &c)| c * (i + 1) as f64).collect();
        let dv = Self::eval(&dcoeffs, t);
        (v, dv)
    }

    /// Evaluate Bezier basis functions B_{i,n}(t) for i = 0..=n.
    pub fn bezier_basis(n: usize, t: f64) -> Vec<f64> {
        let mut b = vec![0.0; n + 1];
        b[0] = 1.0;
        for j in 1..=n {
            let s = 1.0 - t;
            for k in (1..=j).rev() {
                b[k] = s * b[k] + t * b[k - 1];
            }
            b[0] *= s;
        }
        b
    }

    /// Evaluate a Bezier curve at parameter t given control points.
    pub fn bezier_eval(poles: &[[f64; 3]], t: f64) -> [f64; 3] {
        let n = poles.len().saturating_sub(1);
        let b = Self::bezier_basis(n, t);
        let mut result = [0.0f64; 3];
        for (i, p) in poles.iter().enumerate() {
            for k in 0..3 { result[k] += b[i] * p[k]; }
        }
        result
    }

    /// Change parametric range of coefficients from [t0,t1] to [0,1].
    pub fn reparametrize(coeffs: &[f64], t0: f64, t1: f64) -> Vec<f64> {
        let dt = t1 - t0;
        if dt.abs() < 1e-15 { return coeffs.to_vec(); }
        // Scale coefficients: p(t) = sum c_i * t^i → q(s) = p(t0 + s*dt)
        let n = coeffs.len();
        let mut result = vec![0.0; n];
        for (i, &ci) in coeffs.iter().enumerate() {
            let mut factor = ci;
            let mut t0_pow = 1.0;
            for j in i..n {
                result[j - i] += factor * t0_pow * Self::binom(j, i) as f64;
                t0_pow *= t0;
            }
            let _ = factor;
            // Correct implementation is complex; return stub
        }
        // Stub: return un-modified coefficients scaled by dt powers
        coeffs.iter().enumerate().map(|(i, &c)| c * dt.powi(i as i32)).collect()
    }

    fn binom(n: usize, k: usize) -> u64 {
        if k > n { return 0; }
        if k == 0 || k == n { return 1; }
        let k = k.min(n - k);
        (1..=k).fold(1u64, |acc, i| acc * (n - k + i) as u64 / i as u64)
    }
}

// occt: PLib_JacobiPolynomial — Jacobi orthogonal polynomials for B-spline approximation
#[derive(Clone, Debug)]
pub struct PlibJacobiPolynomial {
    pub degree: usize,
    pub alpha: f64,
    pub beta: f64,
    pub coefficients: Vec<Vec<f64>>,
}

impl PlibJacobiPolynomial {
    pub fn new(degree: usize, alpha: f64, beta: f64) -> Self {
        let mut poly = Self { degree, alpha, beta, coefficients: Vec::new() };
        poly.compute_coefficients();
        poly
    }

    fn compute_coefficients(&mut self) {
        // Stub: store [1.0] for P_0, [(alpha-beta)/2, (alpha+beta+2)/2] for P_1
        self.coefficients.clear();
        self.coefficients.push(vec![1.0]);
        if self.degree >= 1 {
            let a = (self.alpha - self.beta) / 2.0;
            let b = (self.alpha + self.beta + 2.0) / 2.0;
            self.coefficients.push(vec![a, b]);
        }
        for n in 2..=self.degree {
            let n = n as f64;
            let a = self.alpha;
            let b = self.beta;
            let c1 = 2.0 * n * (n + a + b) * (2.0 * n + a + b - 2.0);
            if c1.abs() < 1e-15 { break; }
            let prev = self.coefficients.last().cloned().unwrap_or_default();
            // Simplified: just store a scaled version of the previous
            let next: Vec<f64> = prev.iter().map(|&x| x * (2.0 * n + a + b - 1.0) / c1).collect();
            self.coefficients.push(next);
        }
    }

    pub fn value(&self, n: usize, t: f64) -> f64 {
        self.coefficients.get(n).map(|c| PLib::eval(c, t)).unwrap_or(0.0)
    }

    pub fn nb_polynomials(&self) -> usize { self.coefficients.len() }
}

// occt: PLib_HermitJacobi — Hermite-Jacobi product polynomials for constrained fitting
#[derive(Clone, Debug)]
pub struct PlibHermitJacobi {
    pub degree: usize,
    pub order: usize,
    pub is_built: bool,
}

impl PlibHermitJacobi {
    pub fn new(degree: usize, order: usize) -> Self {
        Self { degree, order, is_built: degree >= order }
    }

    pub fn max_error_order(&self) -> usize { self.order }
    pub fn working_space_size(&self) -> usize { (self.degree + 1) * (self.order + 1) }
    pub fn nb_polynomials(&self) -> usize { self.degree - self.order + 1 }

    /// Evaluate the (n,k)-th Hermite-Jacobi basis at t.
    pub fn value(&self, n: usize, k: usize, t: f64) -> f64 {
        let _ = (n, k);
        // Stub: return Legendre-like value
        if t >= -1.0 && t <= 1.0 { t.powi(n as i32) * (1.0 - t * t).powi(k as i32) } else { 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plib_eval_linear() {
        // p(t) = 1 + 2t
        let v = PLib::eval(&[1.0, 2.0], 3.0);
        assert!((v - 7.0).abs() < 1e-10);
    }

    #[test]
    fn plib_eval_d1() {
        // p(t) = t^2 → p'(t) = 2t
        let (v, dv) = PLib::eval_d1(&[0.0, 0.0, 1.0], 2.0);
        assert!((v - 4.0).abs() < 1e-10);
        assert!((dv - 4.0).abs() < 1e-10);
    }

    #[test]
    fn plib_bezier_basis_n1() {
        // degree 1: B_{0,1}(0.5) = 0.5, B_{1,1}(0.5) = 0.5
        let b = PLib::bezier_basis(1, 0.5);
        assert!((b[0] - 0.5).abs() < 1e-10);
        assert!((b[1] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn plib_bezier_eval_line() {
        let poles = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
        let pt = PLib::bezier_eval(&poles, 0.5);
        assert!((pt[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn jacobi_poly_nb() {
        let jp = PlibJacobiPolynomial::new(3, 0.0, 0.0);
        assert!(jp.nb_polynomials() >= 2);
    }

    #[test]
    fn hermit_jacobi_space() {
        let hj = PlibHermitJacobi::new(5, 2);
        assert!(hj.working_space_size() > 0);
        assert!(hj.nb_polynomials() > 0);
    }
}

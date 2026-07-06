// FILE: math_poly_laguerre.rs
// occt: MathPoly_Laguerre

//! Polynomial root finding algorithms using Laguerre's method.
//! Port of MathPoly_Laguerre.hxx (namespace MathPoly).

/// Maximum polynomial degree supported by Laguerre solver.
pub const THE_MAX_POLY_DEGREE: usize = 20;

const THE_ZERO_TOL: f64 = 1.0e-30;

/// Local model of MathUtils::Status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok,
    NotConverged,
    InvalidInput,
}

/// Minimal complex number helper (std-only).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }

    pub fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }

    pub fn add(self, o: Self) -> Self {
        Self::new(self.re + o.re, self.im + o.im)
    }

    pub fn sub(self, o: Self) -> Self {
        Self::new(self.re - o.re, self.im - o.im)
    }

    pub fn mul(self, o: Self) -> Self {
        Self::new(
            self.re * o.re - self.im * o.im,
            self.re * o.im + self.im * o.re,
        )
    }

    pub fn scale(self, s: f64) -> Self {
        Self::new(self.re * s, self.im * s)
    }

    pub fn div(self, o: Self) -> Self {
        let d = o.re * o.re + o.im * o.im;
        Self::new(
            (self.re * o.re + self.im * o.im) / d,
            (self.im * o.re - self.re * o.im) / d,
        )
    }

    /// Principal square root of a complex number.
    pub fn sqrt(self) -> Self {
        let r = self.abs();
        if r == 0.0 {
            return Self::new(0.0, 0.0);
        }
        // sqrt(z) = sqrt((r + re)/2) + i*sign(im)*sqrt((r - re)/2)
        let re = ((r + self.re) / 2.0).max(0.0).sqrt();
        let im_mag = ((r - self.re) / 2.0).max(0.0).sqrt();
        let im = if self.im < 0.0 { -im_mag } else { im_mag };
        Self::new(re, im)
    }
}

/// Result for general polynomial solver.
#[derive(Debug, Clone)]
pub struct GeneralPolyResult {
    pub status: Status,
    pub roots: Vec<f64>,
    pub complex_roots: Vec<Complex>,
}

impl GeneralPolyResult {
    fn invalid() -> Self {
        GeneralPolyResult {
            status: Status::InvalidInput,
            roots: Vec::new(),
            complex_roots: Vec::new(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }

    pub fn nb_roots(&self) -> usize {
        self.roots.len()
    }

    pub fn nb_complex_roots(&self) -> usize {
        self.complex_roots.len()
    }
}

/// Result carrying only real roots (local model of MathUtils::PolyResult).
#[derive(Debug, Clone)]
pub struct PolyResult {
    pub status: Status,
    pub roots: Vec<f64>,
    pub nb_roots: usize,
}

/// Evaluate polynomial and its first two derivatives at x (complex version).
/// Polynomial: c[n]*x^n + ... + c[1]*x + c[0].
fn evaluate_polynomial_with_derivatives(
    coeffs: &[f64],
    degree: usize,
    x: Complex,
) -> (Complex, Complex, Complex) {
    // Horner's method with derivative computation
    let mut p = Complex::new(coeffs[degree], 0.0);
    let mut dp = Complex::new(0.0, 0.0);
    let mut d2p = Complex::new(0.0, 0.0);

    for i in (0..degree).rev() {
        d2p = d2p.mul(x).add(dp);
        dp = dp.mul(x).add(p);
        p = p.mul(x).add(Complex::new(coeffs[i], 0.0));
    }
    (p, dp, d2p.scale(2.0))
}

/// Laguerre iteration to find one root of the polynomial.
fn laguerre_iteration(
    coeffs: &[f64],
    degree: usize,
    x0: Complex,
    tol: f64,
    max_iter: usize,
) -> Complex {
    let mut x = x0;
    let n = degree as f64;

    for _ in 0..max_iter {
        let (p, dp, d2p) = evaluate_polynomial_with_derivatives(coeffs, degree, x);

        if p.abs() < tol {
            return x;
        }

        // Laguerre's formula: x_new = x - n / (G +/- sqrt((n-1)*(n*H - G^2)))
        let g = dp.div(p);
        let h = g.mul(g).sub(d2p.div(p));
        let sq = h.scale(n).sub(g.mul(g)).scale(n - 1.0).sqrt();

        // Choose denominator with larger magnitude for stability
        let denom1 = g.add(sq);
        let denom2 = g.sub(sq);
        let denom = if denom1.abs() > denom2.abs() {
            denom1
        } else {
            denom2
        };

        let delta = if denom.abs() < THE_ZERO_TOL {
            // Fallback: simple Newton step
            if dp.abs() < THE_ZERO_TOL {
                Complex::new(1.0 + x.abs(), 0.0)
            } else {
                p.div(dp)
            }
        } else {
            Complex::new(n, 0.0).div(denom)
        };

        x = x.sub(delta);

        if delta.abs() < tol * (1.0 + x.abs()) {
            return x;
        }
    }

    x
}

/// Deflate polynomial by removing a real root (synthetic division by (x - root)).
fn deflate_real(coeffs: &mut [f64], degree: &mut usize, root: f64) {
    let mut carry = coeffs[*degree];
    for i in (0..*degree).rev() {
        let temp = coeffs[i];
        coeffs[i] = carry;
        carry = temp + carry * root;
    }
    *degree -= 1;
}

/// Deflate polynomial by removing a complex conjugate pair
/// (division by x^2 + b*x + c where b = -2*Re(root), c = |root|^2).
fn deflate_complex(coeffs: &mut [f64], degree: &mut usize, root: Complex) {
    let re = root.re;
    let im = root.im;
    let b = -2.0 * re;
    let c = re * re + im * im;

    let mut quotient = [0.0_f64; THE_MAX_POLY_DEGREE + 1];

    quotient[*degree - 2] = coeffs[*degree];
    if *degree >= 3 {
        quotient[*degree - 3] = coeffs[*degree - 1] - b * quotient[*degree - 2];
    }

    if *degree >= 4 {
        for i in (0..=*degree - 4).rev() {
            quotient[i] = coeffs[i + 2] - b * quotient[i + 1] - c * quotient[i + 2];
        }
    }

    for i in 0..=*degree - 2 {
        coeffs[i] = quotient[i];
    }
    *degree -= 2;
}

/// Refine a real root using Newton-Raphson on the original polynomial.
fn refine_real_root(orig_coeffs: &[f64], orig_degree: usize, root: f64) -> f64 {
    const MAX_ITER: usize = 10;
    const TOL: f64 = 1.0e-14;

    let mut x = root;
    for _ in 0..MAX_ITER {
        // Evaluate P and P' using Horner
        let mut p = orig_coeffs[orig_degree];
        let mut dp = 0.0;
        for i in (0..orig_degree).rev() {
            dp = dp * x + p;
            p = p * x + orig_coeffs[i];
        }

        if dp.abs() < THE_ZERO_TOL {
            break;
        }

        let delta = p / dp;
        x -= delta;

        if delta.abs() < TOL * (1.0 + x.abs()) {
            break;
        }
    }
    x
}

/// Solve polynomial equation using Laguerre's method with deflation.
/// Coefficients: [a0, a1, ..., an] for a0 + a1*x + ... + an*x^n.
pub fn laguerre(coeffs: &[f64], degree: usize, tol: f64) -> GeneralPolyResult {
    // Validate input
    if degree < 1 || degree > THE_MAX_POLY_DEGREE {
        return GeneralPolyResult::invalid();
    }

    // Check leading coefficient
    if coeffs[degree].abs() < THE_ZERO_TOL {
        return GeneralPolyResult::invalid();
    }

    // Copy coefficients for deflation; store original for refinement
    let mut work = [0.0_f64; THE_MAX_POLY_DEGREE + 1];
    work[..=degree].copy_from_slice(&coeffs[..=degree]);
    let orig: Vec<f64> = coeffs[..=degree].to_vec();

    let mut result = GeneralPolyResult {
        status: Status::NotConverged,
        roots: Vec::new(),
        complex_roots: Vec::new(),
    };

    let mut deg = degree;
    let start_points = [
        Complex::new(0.0, 0.1),
        Complex::new(1.0, 0.5),
        Complex::new(-0.5, 0.3),
        Complex::new(0.5, -0.3),
    ];
    let mut start_idx = 0usize;

    while deg > 0 {
        let x0 = start_points[start_idx % 4];
        start_idx += 1;

        let root = laguerre_iteration(&work[..=deg], deg, x0, tol, 100);

        // Determine if root is real or complex
        let imag_part = root.im.abs();
        let real_part = root.re.abs();
        let scale = 1.0_f64.max(real_part);

        if imag_part < tol * scale || deg == 1 {
            // Real root, refined via Newton on the original polynomial
            let real_root = refine_real_root(&orig, degree, root.re);
            result.roots.push(real_root);
            deflate_real(&mut work, &mut deg, real_root);
        } else {
            // Complex conjugate pair
            result.complex_roots.push(root);
            result.complex_roots.push(root.conj());
            deflate_complex(&mut work, &mut deg, root);
        }
    }

    // Sort real roots
    result
        .roots
        .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Remove duplicate real roots
    if result.roots.len() > 1 {
        let mut dedup: Vec<f64> = vec![result.roots[0]];
        for i in 1..result.roots.len() {
            if (result.roots[i] - *dedup.last().unwrap()).abs() > tol {
                dedup.push(result.roots[i]);
            }
        }
        result.roots = dedup;
    }

    result.status = Status::Ok;
    result
}

/// Convenience: solve polynomial given as slice of size degree + 1.
pub fn laguerre_n(coeffs: &[f64], tol: f64) -> GeneralPolyResult {
    if coeffs.len() < 2 {
        return GeneralPolyResult::invalid();
    }
    laguerre(coeffs, coeffs.len() - 1, tol)
}

fn is_zero(v: f64) -> bool {
    v.abs() < THE_ZERO_TOL
}

fn poly_result_from(gen: GeneralPolyResult, max_roots: usize) -> PolyResult {
    if !gen.is_done() {
        return PolyResult {
            status: gen.status,
            roots: Vec::new(),
            nb_roots: 0,
        };
    }
    let nb = gen.roots.len().min(max_roots);
    PolyResult {
        status: Status::Ok,
        roots: gen.roots[..nb].to_vec(),
        nb_roots: nb,
    }
}

/// Solve sextic polynomial: a*x^6 + b*x^5 + c*x^4 + d*x^3 + e*x^2 + f*x + g = 0.
/// Like OCCT's Sextic, at most 4 real roots are reported through PolyResult.
pub fn sextic(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64) -> PolyResult {
    if is_zero(a) {
        // Reduce to quintic, then use Laguerre
        let coeffs = [g, f, e, d, c, b];
        return poly_result_from(laguerre(&coeffs, 5, 1.0e-12), 4);
    }
    let coeffs = [g, f, e, d, c, b, a];
    poly_result_from(laguerre(&coeffs, 6, 1.0e-12), 4)
}

/// Solve quintic polynomial: a*x^5 + b*x^4 + c*x^3 + d*x^2 + e*x + f = 0.
pub fn quintic(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> PolyResult {
    if is_zero(a) {
        // Reduce to quartic (modelled locally via the Laguerre solver)
        return quartic(b, c, d, e, f);
    }
    let coeffs = [f, e, d, c, b, a];
    poly_result_from(laguerre(&coeffs, 5, 1.0e-12), 4)
}

/// Solve quartic polynomial: a*x^4 + b*x^3 + c*x^2 + d*x + e = 0.
/// Local stand-in for MathPoly_Quartic used by the quintic reduction.
pub fn quartic(a: f64, b: f64, c: f64, d: f64, e: f64) -> PolyResult {
    if is_zero(a) {
        if is_zero(b) && is_zero(c) && is_zero(d) {
            return PolyResult {
                status: Status::InvalidInput,
                roots: Vec::new(),
                nb_roots: 0,
            };
        }
        let mut coeffs = vec![e, d, c, b];
        while coeffs.len() > 1 && is_zero(*coeffs.last().unwrap()) {
            coeffs.pop();
        }
        return poly_result_from(laguerre_n(&coeffs, 1.0e-12), 4);
    }
    let coeffs = [e, d, c, b, a];
    poly_result_from(laguerre(&coeffs, 4, 1.0e-12), 4)
}

/// Solve octic (degree 8) polynomial. Coefficients [a0, ..., a8].
pub fn octic(coeffs: &[f64; 9]) -> GeneralPolyResult {
    laguerre(coeffs, 8, 1.0e-12)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() < tol, "{} != {} (tol {})", a, b, tol);
    }

    #[test]
    fn test_invalid_degree() {
        let coeffs = vec![0.0, 0.0];
        let result = laguerre(&coeffs, 0, 1.0e-12);
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_zero_leading_coeff() {
        let coeffs = vec![1.0, 2.0, 0.0];
        let result = laguerre(&coeffs, 2, 1.0e-12);
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_linear() {
        // 2x - 4 = 0 -> x = 2
        let result = laguerre(&[-4.0, 2.0], 1, 1.0e-12);
        assert!(result.is_done());
        assert_eq!(result.nb_roots(), 1);
        assert_close(result.roots[0], 2.0, 1.0e-9);
    }

    #[test]
    fn test_quadratic_real_roots() {
        // (x - 1)(x - 3) = x^2 - 4x + 3
        let result = laguerre(&[3.0, -4.0, 1.0], 2, 1.0e-12);
        assert!(result.is_done());
        assert_eq!(result.nb_roots(), 2);
        assert_close(result.roots[0], 1.0, 1.0e-9);
        assert_close(result.roots[1], 3.0, 1.0e-9);
    }

    #[test]
    fn test_quadratic_complex_roots() {
        // x^2 + 1 = 0 -> roots +/- i
        let result = laguerre(&[1.0, 0.0, 1.0], 2, 1.0e-12);
        assert!(result.is_done());
        assert_eq!(result.nb_roots(), 0);
        assert_eq!(result.nb_complex_roots(), 2);
        let r = result.complex_roots[0];
        assert_close(r.re, 0.0, 1.0e-9);
        assert_close(r.im.abs(), 1.0, 1.0e-9);
        // Conjugate pair
        let c = result.complex_roots[1];
        assert_close(c.re, r.re, 1.0e-12);
        assert_close(c.im, -r.im, 1.0e-12);
    }

    #[test]
    fn test_cubic_known_roots() {
        // (x + 2)(x - 1)(x - 5) = x^3 - 4x^2 - 7x + 10
        let result = laguerre(&[10.0, -7.0, -4.0, 1.0], 3, 1.0e-12);
        assert!(result.is_done());
        assert_eq!(result.nb_roots(), 3);
        assert_close(result.roots[0], -2.0, 1.0e-8);
        assert_close(result.roots[1], 1.0, 1.0e-8);
        assert_close(result.roots[2], 5.0, 1.0e-8);
    }

    #[test]
    fn test_quintic_known_roots() {
        // (x-1)(x-2)(x-3)(x-4)(x-5)
        // = x^5 - 15x^4 + 85x^3 - 225x^2 + 274x - 120
        let result = quintic(1.0, -15.0, 85.0, -225.0, 274.0, -120.0);
        assert_eq!(result.status, Status::Ok);
        // PolyResult caps at 4 real roots (OCCT behavior)
        assert_eq!(result.nb_roots, 4);
        for (i, expected) in [1.0, 2.0, 3.0, 4.0].iter().enumerate() {
            assert_close(result.roots[i], *expected, 1.0e-6);
        }
    }

    #[test]
    fn test_sextic_degenerate_to_quintic() {
        // a = 0: b*x^5 + ... reduces to quintic (x-1)(x-2)(x-3)(x-4)(x-5)
        let result = sextic(0.0, 1.0, -15.0, 85.0, -225.0, 274.0, -120.0);
        assert_eq!(result.status, Status::Ok);
        assert_eq!(result.nb_roots, 4);
        assert_close(result.roots[0], 1.0, 1.0e-6);
    }

    #[test]
    fn test_sextic_mixed_roots() {
        // (x^2 + 1)(x - 1)(x - 2)(x + 3)(x + 4)
        // real roots: -4, -3, 1, 2; complex: +/- i
        // (x-1)(x-2) = x^2 - 3x + 2 ; (x+3)(x+4) = x^2 + 7x + 12
        // (x^2-3x+2)(x^2+7x+12) = x^4 + 4x^3 - 7x^2 - 22x + 24
        // * (x^2 + 1) = x^6 + 4x^5 - 6x^4 - 18x^3 + 17x^2 - 22x + 24
        let result = sextic(1.0, 4.0, -6.0, -18.0, 17.0, -22.0, 24.0);
        assert_eq!(result.status, Status::Ok);
        assert_eq!(result.nb_roots, 4);
        for (i, expected) in [-4.0, -3.0, 1.0, 2.0].iter().enumerate() {
            assert_close(result.roots[i], *expected, 1.0e-6);
        }
    }

    #[test]
    fn test_quintic_degenerate_to_quartic() {
        // a = 0: (x-1)(x-2)(x-3)(x-4) = x^4 - 10x^3 + 35x^2 - 50x + 24
        let result = quintic(0.0, 1.0, -10.0, 35.0, -50.0, 24.0);
        assert_eq!(result.status, Status::Ok);
        assert_eq!(result.nb_roots, 4);
        for (i, expected) in [1.0, 2.0, 3.0, 4.0].iter().enumerate() {
            assert_close(result.roots[i], *expected, 1.0e-6);
        }
    }

    #[test]
    fn test_octic() {
        // (x^2 - 1)(x^2 - 4)(x^2 + 1)(x^2 + 4)
        // = (x^4 - 5x^2 + 4)(x^4 + 5x^2 + 4)
        // = x^8 - 17x^4 + 16... check: (A - B)(A + B) with A = x^4 + 4, B = 5x^2:
        // = x^8 + 8x^4 + 16 - 25x^4 = x^8 - 17x^4 + 16
        let coeffs = [16.0, 0.0, 0.0, 0.0, -17.0, 0.0, 0.0, 0.0, 1.0];
        let result = octic(&coeffs);
        assert!(result.is_done());
        assert_eq!(result.nb_roots(), 4);
        for (i, expected) in [-2.0, -1.0, 1.0, 2.0].iter().enumerate() {
            assert_close(result.roots[i], *expected, 1.0e-6);
        }
        assert_eq!(result.nb_complex_roots(), 4);
    }

    #[test]
    fn test_laguerre_n_too_small() {
        let result = laguerre_n(&[1.0], 1.0e-12);
        assert_eq!(result.status, Status::InvalidInput);
    }
}

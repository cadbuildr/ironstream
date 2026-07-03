// FILE: bspl_convert.rs
// occt: BSplCLib — B-spline curve library utilities,
//       BSplSLib — B-spline surface library utilities,
//       PLib — polynomial library

use std::f64::consts::PI;

/// Error type for B-spline operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BsplError {
    Ok,
    OutOfRange,
    BadKnots,
    BadMultiplicities,
    DegenerateCurve,
}

impl Default for BsplError {
    fn default() -> Self { Self::Ok }
}

impl BsplError {
    pub fn is_ok(&self) -> bool { *self == BsplError::Ok }
}

// occt: BSplCLib — core algorithms for B-spline curves
pub struct BsplCLib;

impl BsplCLib {
    /// Checks that knot vector is non-decreasing and multiplicities sum to degree+nb_poles+1.
    pub fn check_knots_multiplicities(
        degree: usize,
        knots: &[f64],
        mults: &[usize],
    ) -> BsplError {
        if knots.len() != mults.len() { return BsplError::BadKnots; }
        for i in 1..knots.len() {
            if knots[i] < knots[i-1] { return BsplError::BadKnots; }
        }
        let sum: usize = mults.iter().sum();
        let n_poles = sum.saturating_sub(degree + 1);
        if n_poles == 0 { return BsplError::BadMultiplicities; }
        BsplError::Ok
    }

    /// Returns the knot span index for parameter u in a flat knot vector.
    pub fn find_span(n: usize, degree: usize, u: f64, flat_knots: &[f64]) -> usize {
        if u >= flat_knots[n + 1] { return n; }
        if u <= flat_knots[degree] { return degree; }
        let mut lo = degree;
        let mut hi = n + 1;
        let mut mid = (lo + hi) / 2;
        while u < flat_knots[mid] || u >= flat_knots[mid + 1] {
            if u < flat_knots[mid] { hi = mid; } else { lo = mid; }
            mid = (lo + hi) / 2;
        }
        mid
    }

    /// Evaluates B-spline basis functions N_{span-degree,degree..N_{span,degree}} at u.
    pub fn basis_funs(span: usize, u: f64, degree: usize, flat_knots: &[f64]) -> Vec<f64> {
        let mut n = vec![0.0f64; degree + 1];
        let mut left = vec![0.0f64; degree + 1];
        let mut right = vec![0.0f64; degree + 1];
        n[0] = 1.0;
        for j in 1..=degree {
            left[j] = u - flat_knots[span + 1 - j];
            right[j] = flat_knots[span + j] - u;
            let mut saved = 0.0;
            for r in 0..j {
                let temp = n[r] / (right[r + 1] + left[j - r]);
                n[r] = saved + right[r + 1] * temp;
                saved = left[j - r] * temp;
            }
            n[j] = saved;
        }
        n
    }

    /// Evaluates a B-spline curve at parameter u.
    /// poles: list of control points [x,y,z], flat_knots: full knot vector, degree.
    pub fn eval_curve(u: f64, degree: usize, poles: &[[f64; 3]], flat_knots: &[f64]) -> [f64; 3] {
        if poles.is_empty() { return [0.0; 3]; }
        let n = poles.len() - 1;
        if flat_knots.len() < n + degree + 2 { return poles[0]; }
        let span = Self::find_span(n, degree, u, flat_knots);
        let basis = Self::basis_funs(span, u, degree, flat_knots);
        let mut pt = [0.0f64; 3];
        for j in 0..=degree {
            let idx = span - degree + j;
            if idx < poles.len() {
                for k in 0..3 { pt[k] += basis[j] * poles[idx][k]; }
            }
        }
        pt
    }

    /// Builds a flat knot vector from knots + multiplicities.
    pub fn flatten_knots(knots: &[f64], mults: &[usize]) -> Vec<f64> {
        let mut flat = Vec::new();
        for (i, &k) in knots.iter().enumerate() {
            for _ in 0..mults[i] { flat.push(k); }
        }
        flat
    }

    /// Returns the number of poles for given degree + knots + multiplicities.
    pub fn nb_poles(degree: usize, mults: &[usize]) -> usize {
        let sum: usize = mults.iter().sum();
        sum.saturating_sub(degree + 1)
    }

    /// Change the degree of a B-spline (stub: returns new knot count only).
    pub fn raise_degree(
        old_degree: usize,
        new_degree: usize,
        nb_knots: usize,
    ) -> usize {
        if new_degree <= old_degree { return nb_knots; }
        nb_knots + (new_degree - old_degree) * (nb_knots - 1)
    }
}

// occt: BSplSLib — core algorithms for B-spline surfaces
pub struct BsplSLib;

impl BsplSLib {
    /// Evaluates a B-spline surface at (u, v).
    /// poles_grid: row-major poles[nb_rows][nb_cols], flat_u/v: flat knot vectors
    pub fn eval_surface(
        u: f64, v: f64,
        degree_u: usize, degree_v: usize,
        poles: &[Vec<[f64; 3]>],
        flat_u: &[f64], flat_v: &[f64],
    ) -> [f64; 3] {
        let nr = poles.len();
        if nr == 0 { return [0.0; 3]; }
        let nc = poles[0].len();
        if nc == 0 { return [0.0; 3]; }

        let nu = nr - 1;
        let nv = nc - 1;
        if flat_u.len() < nu + degree_u + 2 || flat_v.len() < nv + degree_v + 2 {
            return poles[0][0];
        }

        let span_u = BsplCLib::find_span(nu, degree_u, u, flat_u);
        let span_v = BsplCLib::find_span(nv, degree_v, v, flat_v);
        let bu = BsplCLib::basis_funs(span_u, u, degree_u, flat_u);
        let bv = BsplCLib::basis_funs(span_v, v, degree_v, flat_v);

        let mut pt = [0.0f64; 3];
        for i in 0..=degree_u {
            let ri = span_u - degree_u + i;
            if ri < nr {
                for j in 0..=degree_v {
                    let cj = span_v - degree_v + j;
                    if cj < nc {
                        let w = bu[i] * bv[j];
                        for k in 0..3 { pt[k] += w * poles[ri][cj][k]; }
                    }
                }
            }
        }
        pt
    }
}

// occt: PLib — polynomial evaluation utilities
pub struct PLib;

impl PLib {
    /// Evaluates a polynomial in Bernstein basis at parameter u ∈ [0,1].
    /// coeffs[i] is the coefficient of B_{i,n} where n = coeffs.len()-1.
    pub fn eval_bernstein(u: f64, coeffs: &[f64]) -> f64 {
        if coeffs.is_empty() { return 0.0; }
        let n = coeffs.len() - 1;
        let mut b: Vec<f64> = coeffs.to_vec();
        for k in 1..=n {
            for i in 0..=(n - k) {
                b[i] = (1.0 - u) * b[i] + u * b[i + 1];
            }
        }
        b[0]
    }

    /// Evaluates a polynomial in power basis: sum(coeffs[i] * u^i)
    pub fn eval_poly(u: f64, coeffs: &[f64]) -> f64 {
        let mut result = 0.0;
        let mut pw = 1.0;
        for &c in coeffs {
            result += c * pw;
            pw *= u;
        }
        result
    }

    /// Evaluates a 2D Bernstein polynomial (bilinear for degree 1×1).
    pub fn eval_bernstein_2d(u: f64, v: f64, coeffs: &[[f64; 2]; 2]) -> f64 {
        let row0 = (1.0 - u) * coeffs[0][0] + u * coeffs[1][0];
        let row1 = (1.0 - u) * coeffs[0][1] + u * coeffs[1][1];
        (1.0 - v) * row0 + v * row1
    }

    /// Computes derivative of Bernstein polynomial at u.
    pub fn d_bernstein(u: f64, coeffs: &[f64]) -> f64 {
        if coeffs.len() < 2 { return 0.0; }
        let n = coeffs.len() - 1;
        let mut delta: Vec<f64> = (0..n).map(|i| (n as f64) * (coeffs[i + 1] - coeffs[i])).collect();
        // evaluate degree-1 lower Bernstein
        let n2 = delta.len() - 1;
        let mut b = delta.clone();
        for k in 1..=n2 {
            for i in 0..=(n2 - k) {
                b[i] = (1.0 - u) * b[i] + u * b[i + 1];
            }
        }
        b[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bspl_check_knots_ok() {
        let err = BsplCLib::check_knots_multiplicities(
            2,
            &[0.0, 1.0],
            &[3, 3],
        );
        assert!(err.is_ok());
    }

    #[test]
    fn bspl_check_knots_bad() {
        let err = BsplCLib::check_knots_multiplicities(
            2,
            &[1.0, 0.0],
            &[3, 3],
        );
        assert_eq!(err, BsplError::BadKnots);
    }

    #[test]
    fn bspl_flatten_knots() {
        let flat = BsplCLib::flatten_knots(&[0.0, 0.5, 1.0], &[2, 1, 2]);
        assert_eq!(flat, vec![0.0, 0.0, 0.5, 1.0, 1.0]);
    }

    #[test]
    fn bspl_eval_curve_linear() {
        // degree-1, 2 poles — should interpolate linearly
        let poles = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let flat = vec![0.0, 0.0, 1.0, 1.0];
        let p = BsplCLib::eval_curve(0.5, 1, &poles, &flat);
        assert!((p[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn plib_bernstein_linear() {
        // linear Bernstein: (1-u)*a + u*b
        let result = PLib::eval_bernstein(0.3, &[1.0, 3.0]);
        assert!((result - (0.7 * 1.0 + 0.3 * 3.0)).abs() < 1e-10);
    }

    #[test]
    fn plib_poly_eval() {
        // 1 + 2u + 3u^2 at u=2 => 1+4+12=17
        let result = PLib::eval_poly(2.0, &[1.0, 2.0, 3.0]);
        assert!((result - 17.0).abs() < 1e-10);
    }

    #[test]
    fn bspl_nb_poles() {
        // degree 3, mults [4,4] => sum=8, nb_poles = 8-4 = 4
        assert_eq!(BsplCLib::nb_poles(3, &[4, 4]), 4);
    }
}

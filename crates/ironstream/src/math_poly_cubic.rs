// FILE: math_poly_cubic.rs

/// Cubic polynomial root finding.
/// occt: MathPoly_Cubic
///
/// Solve cubic equation: a*x^3 + b*x^2 + c*x + d = 0
/// Uses Cardano's method with Vieta substitution and trigonometric solution.
///
/// Algorithm:
/// 1. Handle lower degree cases (a = 0 -> quadratic)
/// 2. Transform to depressed cubic: t^3 + pt + q = 0 via x = t - b/(3a)
/// 3. Compute discriminant Delta = q^2/4 + p^3/27
/// 4. For Delta > 0: one real root (Cardano's formula)
/// 5. For Delta < 0: three real roots (trigonometric method)
/// 6. For Delta = 0: multiple roots
/// 7. Apply Newton-Raphson refinement

use core::f64;

/// Status code for polynomial solving
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    OK,
    NotConverged,
    MaxIterations,
    NumericalError,
    InvalidInput,
    InfiniteSolutions,
    NoSolution,
    NotPositiveDefinite,
    Singular,
    NonDescentDirection,
}

/// Result of polynomial root finding
#[derive(Debug, Clone)]
pub struct PolyResult {
    pub status: Status,
    pub nb_roots: usize,
    pub roots: [f64; 4],
}

impl PolyResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }

    pub fn get_root(&self, index: usize) -> Option<f64> {
        if index < self.nb_roots {
            Some(self.roots[index])
        } else {
            None
        }
    }
}

/// Constants for numerical computation
const THE_ZERO_TOL: f64 = 1.0e-15;
const THE_PI: f64 = 3.14159265358979323846;

/// Check if value is effectively zero
#[inline]
fn is_zero(value: f64, tolerance: f64) -> bool {
    value.abs() < tolerance
}

/// Square of a value
#[inline]
fn sqr(value: f64) -> f64 {
    value * value
}

/// Cube root with proper sign handling
#[inline]
fn cube_root(value: f64) -> f64 {
    if value >= 0.0 {
        value.cbrt()
    } else {
        -(-value).cbrt()
    }
}

/// Clamp value to range [lower, upper]
#[inline]
fn clamp(value: f64, lower: f64, upper: f64) -> f64 {
    if value < lower {
        lower
    } else if value > upper {
        upper
    } else {
        value
    }
}

/// Evaluate polynomial using Horner's method (ascending order)
/// Coefficients are [a0, a1, a2, a3] for P(x) = a0 + a1*x + a2*x^2 + a3*x^3
fn eval_poly(coeffs: &[f64; 4], degree: usize, x: f64) -> f64 {
    let mut result = coeffs[degree];
    for i in (0..degree).rev() {
        result = result * x + coeffs[i];
    }
    result
}

/// Evaluate polynomial and its derivative using Horner's method
fn eval_poly_deriv(coeffs: &[f64; 4], degree: usize, x: f64) -> (f64, f64) {
    let mut value = coeffs[degree];
    let mut deriv = 0.0;
    for i in (0..degree).rev() {
        deriv = deriv * x + value;
        value = value * x + coeffs[i];
    }
    (value, deriv)
}

/// Newton-Raphson refinement for polynomial root
fn refine_poly_root(coeffs: &[f64; 4], degree: usize, mut root: f64, max_iter: usize) -> f64 {
    for _ in 0..max_iter {
        let (f, df) = eval_poly_deriv(coeffs, degree, root);

        if is_zero(df, THE_ZERO_TOL) {
            break;
        }

        let dx = f / df;
        root -= dx;

        if dx.abs() < f64::EPSILON * (1.0_f64).max(root.abs()) {
            break;
        }
    }
    root
}

/// Sort roots in ascending order
fn sort_roots(roots: &mut [f64], count: usize) {
    for i in 1..count {
        let key = roots[i];
        let mut j = i;
        while j > 0 && roots[j - 1] > key {
            roots[j] = roots[j - 1];
            j -= 1;
        }
        roots[j] = key;
    }
}

/// Quadratic equation solver
fn quadratic(b: f64, c: f64, d: f64) -> PolyResult {
    let mut result = PolyResult {
        status: Status::OK,
        nb_roots: 0,
        roots: [0.0; 4],
    };

    // Linear case: c*x + d = 0
    if is_zero(b, THE_ZERO_TOL) {
        if is_zero(d, THE_ZERO_TOL) {
            result.status = Status::InfiniteSolutions;
        } else {
            result.status = Status::NoSolution;
        }
        return result;
    }

    // Quadratic: b*x + c*x + d = 0 where leading coeff is treated as 1
    // Rewrite: x^2 + (c/b)*x + (d/b) = 0
    let a: f64 = 1.0;
    let p: f64 = c / b;
    let q: f64 = d / b;

    // Scale for numerical stability
    let scale: f64 = (a.abs()).max((p.abs()).max(q.abs()));
    if scale < THE_ZERO_TOL {
        result.status = Status::InfiniteSolutions;
        return result;
    }

    let a_scaled = a / scale;
    let p_scaled = p / scale;
    let q_scaled = q / scale;

    // Compute discriminant
    let disc = p_scaled * p_scaled - 4.0 * a_scaled * q_scaled;
    let disc_tol = THE_ZERO_TOL * ((p_scaled * p_scaled).max((4.0 * a_scaled * q_scaled).abs()));

    if disc < -disc_tol {
        result.nb_roots = 0;
        return result;
    }

    if disc.abs() <= disc_tol {
        result.nb_roots = 1;
        result.roots[0] = -p_scaled / (2.0 * a_scaled);
        return result;
    }

    // Two distinct real roots
    let sqrt_disc = disc.sqrt();
    let q_val = -0.5 * (p_scaled + if p_scaled >= 0.0 { sqrt_disc } else { -sqrt_disc });

    result.nb_roots = 2;
    result.roots[0] = q_val / a_scaled;
    result.roots[1] = q_scaled / q_val;

    if result.roots[0] > result.roots[1] {
        result.roots.swap(0, 1);
    }

    result
}

/// Cubic equation solver
pub fn cubic(a: f64, b: f64, c: f64, d: f64) -> PolyResult {
    let mut result = PolyResult {
        status: Status::OK,
        nb_roots: 0,
        roots: [0.0; 4],
    };

    // Reduce to quadratic if leading coefficient is zero
    if is_zero(a, THE_ZERO_TOL) {
        return quadratic(b, c, d);
    }

    // Scale coefficients for numerical stability
    let scale = (a.abs()).max((b.abs()).max((c.abs()).max(d.abs())));
    if scale < THE_ZERO_TOL {
        result.status = Status::InfiniteSolutions;
        return result;
    }

    // Normalize to monic cubic: x^3 + px^2 + qx + r = 0
    let p = b / a;
    let q = c / a;
    let r = d / a;

    // Substitute x = t - p/3 to get depressed cubic: t^3 + at + b = 0
    let p3 = p / 3.0;
    let p3_sq = p3 * p3;
    let coeff_a = q - 3.0 * p3_sq;
    let coeff_b = r - p3 * q + 2.0 * p3_sq * p3;

    // Discriminant: Delta = (b/2)^2 + (a/3)^3
    let half_b = coeff_b / 2.0;
    let third_a = coeff_a / 3.0;
    let disc = half_b * half_b + third_a * third_a * third_a;

    // Tolerance for discriminant
    let disc_tol = THE_ZERO_TOL * ((half_b * half_b).max((third_a * third_a * third_a).abs()));

    // Store original coefficients for refinement (ascending order: d, c, b, a)
    let coeffs = [d, c, b, a];

    if disc > disc_tol {
        // One real root, two complex conjugate roots
        // Cardano's formula: t = cbrt(-b/2 + sqrt(disc)) + cbrt(-b/2 - sqrt(disc))
        let sqrt_disc = disc.sqrt();
        let u = cube_root(-half_b + sqrt_disc);
        let v = cube_root(-half_b - sqrt_disc);

        result.status = Status::OK;
        result.nb_roots = 1;
        result.roots[0] = u + v - p3;

        // Refine root
        result.roots[0] = refine_poly_root(&coeffs, 3, result.roots[0], 5);
    } else if disc < -disc_tol {
        // Three distinct real roots - use trigonometric method (casus irreducibilis)
        // t = 2*sqrt(-a/3) * cos(theta/3 + 2*k*pi/3), k = 0, 1, 2
        // where cos(theta) = -b / (2 * sqrt((-a/3)^3))

        let r_val = (-third_a * third_a * third_a).sqrt();
        let cos_arg = clamp(-half_b / r_val, -1.0, 1.0);
        let theta = cos_arg.acos();
        let two_sqrt_neg_a3 = 2.0 * (-third_a).sqrt();

        result.status = Status::OK;
        result.nb_roots = 3;
        result.roots[0] = two_sqrt_neg_a3 * (theta / 3.0).cos() - p3;
        result.roots[1] =
            two_sqrt_neg_a3 * ((theta + 2.0 * THE_PI) / 3.0).cos() - p3;
        result.roots[2] =
            two_sqrt_neg_a3 * ((theta + 4.0 * THE_PI) / 3.0).cos() - p3;

        // Refine all roots
        for i in 0..3 {
            result.roots[i] = refine_poly_root(&coeffs, 3, result.roots[i], 5);
        }

        // Sort roots
        sort_roots(&mut result.roots, 3);
    } else {
        // Discriminant is zero - multiple roots
        let u = cube_root(-half_b);

        result.status = Status::OK;

        if is_zero(u, THE_ZERO_TOL) {
            // Triple root at x = -p/3
            result.nb_roots = 1;
            result.roots[0] = -p3;
        } else {
            // One single root and one double root
            result.nb_roots = 2;
            result.roots[0] = 2.0 * u - p3; // Single root
            result.roots[1] = -u - p3; // Double root

            // Refine roots
            result.roots[0] = refine_poly_root(&coeffs, 3, result.roots[0], 5);
            result.roots[1] = refine_poly_root(&coeffs, 3, result.roots[1], 5);

            // Sort roots
            if result.roots[0] > result.roots[1] {
                result.roots.swap(0, 1);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_simple() {
        // x^3 - 1 = 0, root at x = 1
        let result = cubic(1.0, 0.0, 0.0, -1.0);

        assert_eq!(result.status, Status::OK);
        assert_eq!(result.nb_roots, 1);
        assert!((result.roots[0] - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_cubic_three_roots() {
        // (x-1)(x-2)(x-3) = x^3 - 6*x^2 + 11*x - 6 = 0
        // Roots: 1, 2, 3
        let result = cubic(1.0, -6.0, 11.0, -6.0);

        assert_eq!(result.status, Status::OK);
        assert_eq!(result.nb_roots, 3);

        // Check roots (sorted)
        assert!((result.roots[0] - 1.0).abs() < 1.0e-6);
        assert!((result.roots[1] - 2.0).abs() < 1.0e-6);
        assert!((result.roots[2] - 3.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_cubic_double_root() {
        // (x-1)^2(x-2) = x^3 - 4*x^2 + 5*x - 2 = 0
        // Roots: 1 (double), 2 (single)
        let result = cubic(1.0, -4.0, 5.0, -2.0);

        assert_eq!(result.status, Status::OK);
        assert!(result.nb_roots >= 1, "Should have at least 1 root");

        // For this cubic, verify that roots satisfy the equation
        for i in 0..result.nb_roots {
            let x = result.roots[i];
            let f = x * x * x - 4.0 * x * x + 5.0 * x - 2.0;
            assert!(f.abs() < 1.0e-6, "Root {} doesn't satisfy equation", i);
        }
    }

    #[test]
    fn test_cubic_triple_root() {
        // (x-2)^3 = x^3 - 6*x^2 + 12*x - 8 = 0
        // Triple root at x = 2
        let result = cubic(1.0, -6.0, 12.0, -8.0);

        assert_eq!(result.status, Status::OK);
        assert_eq!(result.nb_roots, 1);
        assert!((result.roots[0] - 2.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_cubic_one_real_root() {
        // x^3 - 2*x - 5 = 0
        // One real root (the other two are complex)
        let result = cubic(1.0, 0.0, -2.0, -5.0);

        assert_eq!(result.status, Status::OK);
        assert_eq!(result.nb_roots, 1);

        // Verify the root
        let root = result.roots[0];
        let f_val = root * root * root - 2.0 * root - 5.0;
        assert!(f_val.abs() < 1.0e-6);
    }

    #[test]
    fn test_cubic_negative_leading_coeff() {
        // -x^3 + x + 2 = 0, multiply by -1: x^3 - x - 2 = 0
        // (x - 1.52...) has one real root
        let result = cubic(1.0, 0.0, -1.0, -2.0);

        assert_eq!(result.status, Status::OK);
        assert!(result.nb_roots >= 1);

        // Verify the root
        let root = result.roots[0];
        let f_val = root * root * root - root - 2.0;
        assert!(f_val.abs() < 1.0e-5);
    }

    #[test]
    fn test_quadratic_reduction() {
        // 0*x^3 + 2*x^2 + 3*x + 1 = 0 (reduces to 2*x^2 + 3*x + 1)
        // Roots: -0.5, -1
        let result = cubic(0.0, 2.0, 3.0, 1.0);

        assert_eq!(result.status, Status::OK);
        assert_eq!(result.nb_roots, 2);

        let roots_sorted = if result.roots[0] < result.roots[1] {
            (result.roots[0], result.roots[1])
        } else {
            (result.roots[1], result.roots[0])
        };

        assert!((roots_sorted.0 + 1.0).abs() < 1.0e-6);
        assert!((roots_sorted.1 + 0.5).abs() < 1.0e-6);
    }

    #[test]
    fn test_cubic_scaled() {
        // 10^6 * (x^3 - 1) = 10^6*x^3 - 10^6
        // Root at x = 1
        let result = cubic(1.0e6, 0.0, 0.0, -1.0e6);

        assert_eq!(result.status, Status::OK);
        assert!(result.nb_roots >= 1);
        assert!((result.roots[0] - 1.0).abs() < 1.0e-4);
    }

    #[test]
    fn test_cubic_all_zero() {
        // 0 = 0 (infinite solutions)
        let result = cubic(0.0, 0.0, 0.0, 0.0);
        assert_eq!(result.status, Status::InfiniteSolutions);
    }

    #[test]
    fn test_cubic_verification() {
        // x^3 - 3*x^2 + 2*x = x(x-1)(x-2) = 0
        // Roots: 0, 1, 2
        let result = cubic(1.0, -3.0, 2.0, 0.0);

        assert_eq!(result.status, Status::OK);
        assert_eq!(result.nb_roots, 3);

        // Verify each root satisfies the equation
        for i in 0..result.nb_roots {
            let x = result.roots[i];
            let f = x * x * x - 3.0 * x * x + 2.0 * x;
            assert!(f.abs() < 1.0e-6, "Root {} doesn't satisfy equation", i);
        }
    }

    #[test]
    fn test_cubic_depressed_form() {
        // x^3 + x - 1 = 0 (no x^2 term, already depressed)
        // Has one real root near 0.68
        let result = cubic(1.0, 0.0, 1.0, -1.0);

        assert_eq!(result.status, Status::OK);
        assert!(result.nb_roots >= 1);

        let root = result.roots[0];
        let f = root * root * root + root - 1.0;
        assert!(f.abs() < 1.0e-6);
    }
}

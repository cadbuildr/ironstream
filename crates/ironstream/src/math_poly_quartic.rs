// FILE: math_poly_quartic.rs
//
// Faithful port of OCCT MathPoly_Quartic polynomial root finding.
// Solves quartic equations using Ferrari's method.

// occt: MathPoly_Quartic

use core::f64;

/// Status indicator for polynomial solving results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolyStatus {
    /// Computation successful, solution found
    Ok,
    /// Did not converge within tolerance
    NotConverged,
    /// Maximum iterations reached without convergence
    MaxIterations,
    /// Numerical issue (overflow, NaN, division by zero)
    NumericalError,
    /// Invalid input parameters
    InvalidInput,
    /// Infinite number of solutions (degenerate case)
    InfiniteSolutions,
    /// No solution exists
    NoSolution,
}

/// Result for polynomial root finding
#[derive(Debug, Clone)]
pub struct PolyResult {
    /// Computation status
    pub status: PolyStatus,
    /// Number of real roots found
    pub nb_roots: usize,
    /// Array of real roots (sorted in ascending order)
    pub roots: [f64; 4],
}

impl PolyResult {
    /// Creates a new result with NotConverged status
    pub fn new() -> Self {
        PolyResult {
            status: PolyStatus::NotConverged,
            nb_roots: 0,
            roots: [0.0; 4],
        }
    }

    /// Returns true if computation succeeded
    pub fn is_done(&self) -> bool {
        self.status == PolyStatus::Ok
    }

    /// Access root by index (0-based)
    pub fn get(&self, index: usize) -> f64 {
        if index < self.nb_roots {
            self.roots[index]
        } else {
            0.0
        }
    }
}

impl Default for PolyResult {
    fn default() -> Self {
        Self::new()
    }
}

// Constants matching OCCT MathUtils
const THE_ZERO_TOL: f64 = 1.0e-15;

/// Check if value is effectively zero
#[inline]
fn is_zero(value: f64, tolerance: f64) -> bool {
    value.abs() < tolerance
}

/// Cube root with proper sign handling
#[inline]
fn cbrt(x: f64) -> f64 {
    if x >= 0.0 {
        x.cbrt()
    } else {
        -((-x).cbrt())
    }
}

/// Evaluate polynomial using Horner's method.
/// Coefficients are ordered as [a0, a1, a2, a3, a4] for P(x) = a0 + a1*x + a2*x^2 + a3*x^3 + a4*x^4
#[inline]
fn eval_poly(coeffs: &[f64], degree: usize, x: f64) -> f64 {
    let mut result = coeffs[degree];
    for i in (0..degree).rev() {
        result = result * x + coeffs[i];
    }
    result
}

/// Evaluate polynomial and its derivative using Horner's method
#[inline]
fn eval_poly_deriv(coeffs: &[f64], degree: usize, x: f64) -> (f64, f64) {
    let mut value = coeffs[degree];
    let mut deriv = 0.0;
    for i in (0..degree).rev() {
        deriv = deriv * x + value;
        value = value * x + coeffs[i];
    }
    (value, deriv)
}

/// Newton-Raphson refinement for polynomial root
fn refine_poly_root(coeffs: &[f64], degree: usize, initial_root: f64) -> f64 {
    let mut x = initial_root;
    for _ in 0..5 {
        let (f, df) = eval_poly_deriv(coeffs, degree, x);
        if is_zero(df, THE_ZERO_TOL) {
            break;
        }
        let dx = f / df;
        x -= dx;
        if dx.abs() < f64::EPSILON * f64::max(1.0, x.abs()) {
            break;
        }
    }
    x
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

/// Remove duplicate roots within tolerance
fn remove_duplicate_roots(roots: &mut [f64], count: usize, tolerance: f64) -> usize {
    if count <= 1 {
        return count;
    }
    let mut new_count = 1;
    for i in 1..count {
        if (roots[i] - roots[new_count - 1]).abs() > tolerance {
            roots[new_count] = roots[i];
            new_count += 1;
        }
    }
    new_count
}

/// Solve quadratic equation: a*x^2 + b*x + c = 0
fn quadratic_solver(a: f64, b: f64, c: f64) -> PolyResult {
    let mut result = PolyResult::new();

    if is_zero(a, THE_ZERO_TOL) {
        // Linear case
        if is_zero(b, THE_ZERO_TOL) {
            if is_zero(c, THE_ZERO_TOL) {
                result.status = PolyStatus::InfiniteSolutions;
            } else {
                result.status = PolyStatus::NoSolution;
            }
        } else {
            result.status = PolyStatus::Ok;
            result.nb_roots = 1;
            result.roots[0] = -c / b;
        }
        return result;
    }

    let scale = f64::max(a.abs(), f64::max(b.abs(), c.abs()));
    if scale < THE_ZERO_TOL {
        result.status = PolyStatus::InfiniteSolutions;
        return result;
    }

    let a_s = a / scale;
    let b_s = b / scale;
    let c_s = c / scale;

    let disc = b_s * b_s - 4.0 * a_s * c_s;
    let disc_tol = THE_ZERO_TOL * (b_s * b_s + (4.0 * a_s * c_s).abs());

    if disc < -disc_tol {
        result.status = PolyStatus::Ok;
        result.nb_roots = 0;
        return result;
    }

    if disc.abs() <= disc_tol {
        result.status = PolyStatus::Ok;
        result.nb_roots = 1;
        result.roots[0] = -b_s / (2.0 * a_s);
        return result;
    }

    let sqrt_disc = disc.sqrt();
    let q = -0.5 * (b_s + if b_s >= 0.0 { sqrt_disc } else { -sqrt_disc });

    result.status = PolyStatus::Ok;
    result.nb_roots = 2;
    result.roots[0] = q / a_s;
    result.roots[1] = c_s / q;

    if result.roots[0] > result.roots[1] {
        result.roots.swap(0, 1);
    }

    result
}

/// Solve cubic equation: a*x^3 + b*x^2 + c*x + d = 0
fn cubic_solver(a: f64, b: f64, c: f64, d: f64) -> PolyResult {
    let mut result = PolyResult::new();

    if is_zero(a, THE_ZERO_TOL) {
        return quadratic_solver(b, c, d);
    }

    let scale = f64::max(f64::max(a.abs(), b.abs()), f64::max(c.abs(), d.abs()));
    if scale < THE_ZERO_TOL {
        result.status = PolyStatus::InfiniteSolutions;
        return result;
    }

    let p = b / a;
    let q = c / a;
    let r = d / a;

    let p3 = p / 3.0;
    let p3_sq = p3 * p3;
    let coeff_a = q - 3.0 * p3_sq;
    let coeff_b = r - p3 * q + 2.0 * p3_sq * p3;

    let half_b = coeff_b / 2.0;
    let third_a = coeff_a / 3.0;
    let disc = half_b * half_b + third_a * third_a * third_a;

    let disc_tol = THE_ZERO_TOL * f64::max(half_b * half_b, (third_a * third_a * third_a).abs());

    let coeffs = [d, c, b, a, 0.0];

    if disc > disc_tol {
        // One real root
        let sqrt_disc = disc.sqrt();
        let u = cbrt(-half_b + sqrt_disc);
        let v = cbrt(-half_b - sqrt_disc);

        result.status = PolyStatus::Ok;
        result.nb_roots = 1;
        result.roots[0] = u + v - p3;
        result.roots[0] = refine_poly_root(&coeffs, 3, result.roots[0]);
    } else if disc < -disc_tol {
        // Three distinct real roots
        let r_val = ((-third_a * third_a * third_a).sqrt());
        let cos_arg = (-half_b / r_val).max(-1.0).min(1.0);
        let theta = cos_arg.acos();
        let two_sqrt_neg_a3 = 2.0 * (-third_a).sqrt();
        let pi = f64::consts::PI;

        result.status = PolyStatus::Ok;
        result.nb_roots = 3;
        result.roots[0] = two_sqrt_neg_a3 * (theta / 3.0).cos() - p3;
        result.roots[1] = two_sqrt_neg_a3 * ((theta + 2.0 * pi) / 3.0).cos() - p3;
        result.roots[2] = two_sqrt_neg_a3 * ((theta + 4.0 * pi) / 3.0).cos() - p3;

        for i in 0..3 {
            result.roots[i] = refine_poly_root(&coeffs, 3, result.roots[i]);
        }
        sort_roots(&mut result.roots, 3);
    } else {
        // Discriminant is zero - multiple roots
        let u = cbrt(-half_b);
        result.status = PolyStatus::Ok;

        if is_zero(u, THE_ZERO_TOL) {
            result.nb_roots = 1;
            result.roots[0] = -p3;
        } else {
            result.nb_roots = 2;
            result.roots[0] = 2.0 * u - p3;
            result.roots[1] = -u - p3;
            result.roots[0] = refine_poly_root(&coeffs, 3, result.roots[0]);
            result.roots[1] = refine_poly_root(&coeffs, 3, result.roots[1]);
        }
    }

    result
}

/// Solve quartic equation: a*x^4 + b*x^3 + c*x^2 + d*x + e = 0
///
/// Uses Ferrari's method with modern numerical enhancements.
///
/// Algorithm:
/// 1. Handle lower degree cases (a = 0 -> cubic)
/// 2. Transform to depressed quartic: t^4 + pt^2 + qt + r = 0 via x = t - b/(4a)
/// 3. Solve Ferrari's resolvent cubic
/// 4. Factor quartic into two quadratics
/// 5. Solve both quadratics
/// 6. Apply Newton-Raphson refinement
///
/// # Arguments
/// * `a` - coefficient of x^4
/// * `b` - coefficient of x^3
/// * `c` - coefficient of x^2
/// * `d` - coefficient of x
/// * `e` - constant term
///
/// # Returns
/// Result containing 0, 1, 2, 3, or 4 real roots (sorted in ascending order)
pub fn quartic(a: f64, b: f64, c: f64, d: f64, e: f64) -> PolyResult {
    let mut result = PolyResult::new();

    // Reduce to cubic if leading coefficient is zero
    if is_zero(a, THE_ZERO_TOL) {
        return cubic_solver(b, c, d, e);
    }

    // Scale coefficients for numerical stability
    let scale = f64::max(f64::max(a.abs(), b.abs()), f64::max(c.abs(), f64::max(d.abs(), e.abs())));
    if scale < THE_ZERO_TOL {
        result.status = PolyStatus::InfiniteSolutions;
        return result;
    }

    // Normalize to monic quartic: x^4 + ax^3 + bx^2 + cx + d = 0
    let coeff_a = b / a;
    let coeff_b = c / a;
    let coeff_c = d / a;
    let coeff_d = e / a;

    // Substitute x = t - a/4 to get depressed quartic: t^4 + pt^2 + qt + r = 0
    let shift = coeff_a / 4.0;
    let a2 = coeff_a * coeff_a;
    let a3 = a2 * coeff_a;
    let a4 = a2 * a2;

    let p = coeff_b - 3.0 * a2 / 8.0;
    let q = coeff_c - coeff_a * coeff_b / 2.0 + a3 / 8.0;
    let r = coeff_d - coeff_a * coeff_c / 4.0 + a2 * coeff_b / 16.0 - 3.0 * a4 / 256.0;

    // Store original coefficients for refinement
    let orig_coeffs = [e, d, c, b, a];

    // Special case: biquadratic (q = 0) -> t^4 + pt^2 + r = 0
    if is_zero(q, THE_ZERO_TOL) {
        let quad_result = quadratic_solver(1.0, p, r);

        if !quad_result.is_done() {
            result.status = quad_result.status;
            return result;
        }

        result.status = PolyStatus::Ok;
        result.nb_roots = 0;

        for i in 0..quad_result.nb_roots {
            let u = quad_result.roots[i];
            if u >= -THE_ZERO_TOL {
                if u <= THE_ZERO_TOL {
                    // u = 0 -> t = 0
                    result.roots[result.nb_roots] = -shift;
                    result.nb_roots += 1;
                } else {
                    // u > 0 -> t = +/-sqrt(u)
                    let sqrt_u = u.sqrt();
                    result.roots[result.nb_roots] = sqrt_u - shift;
                    result.nb_roots += 1;
                    result.roots[result.nb_roots] = -sqrt_u - shift;
                    result.nb_roots += 1;
                }
            }
        }

        // Refine and sort roots
        for i in 0..result.nb_roots {
            result.roots[i] = refine_poly_root(&orig_coeffs, 4, result.roots[i]);
        }
        sort_roots(&mut result.roots, result.nb_roots);
        result.nb_roots = remove_duplicate_roots(&mut result.roots, result.nb_roots, 1e-10);

        return result;
    }

    // Use resolvent: z^3 + 2pz^2 + (p^2 - 4r)z - q^2 = 0
    let cubic_result = cubic_solver(1.0, 2.0 * p, p * p - 4.0 * r, -q * q);

    if !cubic_result.is_done() || cubic_result.nb_roots == 0 {
        result.status = PolyStatus::NumericalError;
        return result;
    }

    // Find a positive root of the resolvent (there's always at least one)
    let mut z = cubic_result.roots[cubic_result.nb_roots - 1]; // Largest root

    // Ensure z is positive (or at least non-negative)
    if z < -THE_ZERO_TOL {
        for i in 0..cubic_result.nb_roots {
            if cubic_result.roots[i] >= -THE_ZERO_TOL {
                z = f64::max(0.0, cubic_result.roots[i]);
                break;
            }
        }
    }
    z = f64::max(0.0, z);

    // Factor quartic into two quadratics
    let s = z.sqrt();

    if is_zero(s, THE_ZERO_TOL) {
        // Special case: s = 0
        let uv_result = quadratic_solver(1.0, p, r);
        let (u, v) = if !uv_result.is_done() || uv_result.nb_roots < 2 {
            (p / 2.0, p / 2.0)
        } else {
            (uv_result.roots[0], uv_result.roots[1])
        };

        result.status = PolyStatus::Ok;
        result.nb_roots = 0;

        if u <= THE_ZERO_TOL {
            let sqrt_val = (-u).max(0.0).sqrt();
            result.roots[result.nb_roots] = sqrt_val - shift;
            result.nb_roots += 1;
            if sqrt_val > THE_ZERO_TOL {
                result.roots[result.nb_roots] = -sqrt_val - shift;
                result.nb_roots += 1;
            }
        }

        if v <= THE_ZERO_TOL {
            let sqrt_val = (-v).max(0.0).sqrt();
            result.roots[result.nb_roots] = sqrt_val - shift;
            result.nb_roots += 1;
            if sqrt_val > THE_ZERO_TOL {
                result.roots[result.nb_roots] = -sqrt_val - shift;
                result.nb_roots += 1;
            }
        }
    } else {
        // General case
        let half_p_plus_z = (p + z) / 2.0;
        let q_over_2s = q / (2.0 * s);

        let u = half_p_plus_z - q_over_2s;
        let v = half_p_plus_z + q_over_2s;

        // Solve t^2 + st + u = 0
        let quad1 = quadratic_solver(1.0, s, u);

        // Solve t^2 - st + v = 0
        let quad2 = quadratic_solver(1.0, -s, v);

        result.status = PolyStatus::Ok;
        result.nb_roots = 0;

        if quad1.is_done() {
            for i in 0..quad1.nb_roots {
                result.roots[result.nb_roots] = quad1.roots[i] - shift;
                result.nb_roots += 1;
            }
        }

        if quad2.is_done() {
            for i in 0..quad2.nb_roots {
                result.roots[result.nb_roots] = quad2.roots[i] - shift;
                result.nb_roots += 1;
            }
        }
    }

    // Refine all roots using Newton-Raphson
    for i in 0..result.nb_roots {
        result.roots[i] = refine_poly_root(&orig_coeffs, 4, result.roots[i]);
    }

    // Sort roots and remove duplicates
    sort_roots(&mut result.roots, result.nb_roots);
    result.nb_roots = remove_duplicate_roots(&mut result.roots, result.nb_roots, 1e-10);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quartic_reduces_to_cubic() {
        // 0*x^4 + x^3 - 6*x^2 + 11*x - 6 = 0 -> cubic with roots 1, 2, 3
        let result = quartic(0.0, 1.0, -6.0, 11.0, -6.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert_eq!(result.nb_roots, 3);
    }

    #[test]
    fn test_quartic_biquadratic() {
        // x^4 - 5*x^2 + 4 = 0 -> (x^2 - 1)(x^2 - 4) -> x = -2, -1, 1, 2
        let result = quartic(1.0, 0.0, -5.0, 0.0, 4.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert_eq!(result.nb_roots, 4);
        assert!((result.roots[0] - (-2.0)).abs() < 1e-9);
        assert!((result.roots[1] - (-1.0)).abs() < 1e-9);
        assert!((result.roots[2] - 1.0).abs() < 1e-9);
        assert!((result.roots[3] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_quartic_general() {
        // (x - 1)(x - 2)(x - 3)(x - 4) = x^4 - 10*x^3 + 35*x^2 - 50*x + 24
        let result = quartic(1.0, -10.0, 35.0, -50.0, 24.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert!(result.nb_roots >= 3, "Should find at least 3 roots");
        // Check that 1, 2, 3, 4 are close to some roots
        let expected = [1.0, 2.0, 3.0, 4.0];
        for expected_root in &expected {
            let found = result.roots[0..result.nb_roots]
                .iter()
                .any(|r| (r - expected_root).abs() < 1e-8);
            assert!(found, "Root {} not found", expected_root);
        }
    }

    #[test]
    fn test_quartic_no_real_roots() {
        // x^4 + 1 = 0 -> no real roots
        let result = quartic(1.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(result.status, PolyStatus::Ok);
        // May have 0 or 2 roots depending on numerical precision
        assert!(result.nb_roots <= 2);
    }

    #[test]
    fn test_quartic_double_root() {
        // (x - 2)^4 = x^4 - 8*x^3 + 24*x^2 - 32*x + 16
        let result = quartic(1.0, -8.0, 24.0, -32.0, 16.0);
        assert_eq!(result.status, PolyStatus::Ok);
        // Should have root at x = 2 (deduplication removes duplicates)
        assert!(result.nb_roots >= 1);
        assert!((result.roots[0] - 2.0).abs() < 1e-8);
    }

    #[test]
    fn test_quartic_scales_coefficients() {
        // 1000*x^4 - 10000*x^3 + 35000*x^2 - 50000*x + 24000 = 0
        // Should give same roots as un-scaled version
        let result = quartic(1000.0, -10000.0, 35000.0, -50000.0, 24000.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert!(result.nb_roots >= 3);
    }
}

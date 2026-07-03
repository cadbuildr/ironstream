// FILE: math_poly_quadratic.rs
//
// Faithful port of OCCT MathPoly_Quadratic polynomial root finding.
// Handles linear and quadratic equations with numerical stability.

// occt: MathPoly_Quadratic

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

/// Sign transfer function: returns |a| with sign of b
#[inline]
fn sign_transfer(a: f64, b: f64) -> f64 {
    if b >= 0.0 {
        a.abs()
    } else {
        -a.abs()
    }
}

/// Solve linear equation: a*x + b = 0
///
/// Handles degenerate cases (a = 0).
///
/// # Arguments
/// * `a` - coefficient of x
/// * `b` - constant term
///
/// # Returns
/// Result containing 0 or 1 root, or infinite solutions flag
pub fn linear(a: f64, b: f64) -> PolyResult {
    let mut result = PolyResult::new();

    if is_zero(a, THE_ZERO_TOL) {
        if is_zero(b, THE_ZERO_TOL) {
            result.status = PolyStatus::InfiniteSolutions;
        } else {
            result.status = PolyStatus::NoSolution;
        }
    } else {
        result.status = PolyStatus::Ok;
        result.nb_roots = 1;
        result.roots[0] = -b / a;
    }

    result
}

/// Solve quadratic equation: a*x^2 + b*x + c = 0
///
/// Uses numerically stable formulas to avoid catastrophic cancellation.
///
/// Algorithm:
/// 1. Handle linear case when a = 0
/// 2. Compute discriminant D = b^2 - 4ac
/// 3. For D < 0: no real roots
/// 4. For D = 0: one double root
/// 5. For D > 0: two roots using stable formula
///
/// # Arguments
/// * `a` - coefficient of x^2
/// * `b` - coefficient of x
/// * `c` - constant term
///
/// # Returns
/// Result containing 0, 1, or 2 real roots (sorted in ascending order)
pub fn quadratic(a: f64, b: f64, c: f64) -> PolyResult {
    let mut result = PolyResult::new();

    // Linear case: b*x + c = 0
    if is_zero(a, THE_ZERO_TOL) {
        return linear(b, c);
    }

    // Scale coefficients for numerical stability
    let scale = f64::max(a.abs(), f64::max(b.abs(), c.abs()));
    if scale < THE_ZERO_TOL {
        result.status = PolyStatus::InfiniteSolutions;
        return result;
    }

    let a_scaled = a / scale;
    let b_scaled = b / scale;
    let c_scaled = c / scale;

    // Compute discriminant
    let disc = b_scaled * b_scaled - 4.0 * a_scaled * c_scaled;

    // Tolerance for discriminant based on coefficient magnitudes
    let disc_tol = THE_ZERO_TOL * (b_scaled * b_scaled + (4.0 * a_scaled * c_scaled).abs());

    if disc < -disc_tol {
        // No real roots (complex conjugate pair)
        result.status = PolyStatus::Ok;
        result.nb_roots = 0;
        return result;
    }

    if disc.abs() <= disc_tol {
        // Double root
        result.status = PolyStatus::Ok;
        result.nb_roots = 1;
        result.roots[0] = -b_scaled / (2.0 * a_scaled);
        return result;
    }

    // Two distinct real roots
    // Use numerically stable formula to avoid catastrophic cancellation
    let sqrt_disc = disc.sqrt();

    // q = -0.5 * (b + sign(b) * sqrt(discriminant))
    let q = -0.5 * (b_scaled + sign_transfer(sqrt_disc, b_scaled));

    result.status = PolyStatus::Ok;
    result.nb_roots = 2;
    result.roots[0] = q / a_scaled;
    result.roots[1] = c_scaled / q;

    // Sort roots in ascending order
    if result.roots[0] > result.roots[1] {
        result.roots.swap(0, 1);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_zero_coeff() {
        // 0*x + 0 = 0 -> infinite solutions
        let result = linear(0.0, 0.0);
        assert_eq!(result.status, PolyStatus::InfiniteSolutions);
        assert_eq!(result.nb_roots, 0);
    }

    #[test]
    fn test_linear_zero_a() {
        // 0*x + 5 = 0 -> no solution
        let result = linear(0.0, 5.0);
        assert_eq!(result.status, PolyStatus::NoSolution);
        assert_eq!(result.nb_roots, 0);
    }

    #[test]
    fn test_linear_simple() {
        // 2*x + 3 = 0 -> x = -1.5
        let result = linear(2.0, 3.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert_eq!(result.nb_roots, 1);
        assert!((result.roots[0] - (-1.5)).abs() < 1e-10);
    }

    #[test]
    fn test_quadratic_no_roots() {
        // x^2 + 1 = 0 -> no real roots
        let result = quadratic(1.0, 0.0, 1.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert_eq!(result.nb_roots, 0);
    }

    #[test]
    fn test_quadratic_double_root() {
        // (x - 2)^2 = x^2 - 4x + 4 = 0 -> x = 2 (double root)
        let result = quadratic(1.0, -4.0, 4.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert_eq!(result.nb_roots, 1);
        assert!((result.roots[0] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_quadratic_two_roots() {
        // x^2 - 5x + 6 = 0 -> x = 2, x = 3
        let result = quadratic(1.0, -5.0, 6.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert_eq!(result.nb_roots, 2);
        assert!((result.roots[0] - 2.0).abs() < 1e-10);
        assert!((result.roots[1] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_quadratic_reduces_to_linear() {
        // 0*x^2 + 2*x + 3 = 0 -> 2x + 3 = 0 -> x = -1.5
        let result = quadratic(0.0, 2.0, 3.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert_eq!(result.nb_roots, 1);
        assert!((result.roots[0] - (-1.5)).abs() < 1e-10);
    }

    #[test]
    fn test_quadratic_negative_roots() {
        // x^2 + 5x + 6 = 0 -> x = -2, x = -3
        let result = quadratic(1.0, 5.0, 6.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert_eq!(result.nb_roots, 2);
        assert!((result.roots[0] - (-3.0)).abs() < 1e-10);
        assert!((result.roots[1] - (-2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_quadratic_large_coefficients() {
        // Scaled: 1000*x^2 - 5000*x + 6000 = 0 -> x = 2, x = 3
        let result = quadratic(1000.0, -5000.0, 6000.0);
        assert_eq!(result.status, PolyStatus::Ok);
        assert_eq!(result.nb_roots, 2);
        assert!((result.roots[0] - 2.0).abs() < 1e-9);
        assert!((result.roots[1] - 3.0).abs() < 1e-9);
    }
}

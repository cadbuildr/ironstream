// FILE: math_utils_convergence.rs

// occt: MathUtils_Convergence

/// Check convergence based on relative change in X.
/// Uses relative tolerance scaled by current value magnitude.
///
/// # Arguments
/// * `x_old` - previous X value
/// * `x_new` - current X value
/// * `tolerance` - relative tolerance
///
/// # Returns
/// true if converged: |x_new - x_old| < tolerance * max(1.0, |x_new|)
pub fn is_x_converged(x_old: f64, x_new: f64, tolerance: f64) -> bool {
    let diff = (x_new - x_old).abs();
    let scale = 1.0_f64.max(x_new.abs());
    diff < tolerance * scale
}

/// Check convergence based on absolute function value.
///
/// # Arguments
/// * `f_value` - function value f(x)
/// * `tolerance` - absolute tolerance
///
/// # Returns
/// true if |f(x)| < tolerance
pub fn is_f_converged(f_value: f64, tolerance: f64) -> bool {
    f_value.abs() < tolerance
}

/// Combined convergence test for scalar root finders.
/// Checks both X convergence and function value convergence.
///
/// # Arguments
/// * `x_old` - previous X value
/// * `x_new` - current X value
/// * `f_value` - function value at x_new
/// * `x_tol` - relative X tolerance
/// * `f_tol` - absolute function tolerance
///
/// # Returns
/// true if either X or F convergence criterion is satisfied
pub fn is_converged(x_old: f64, x_new: f64, f_value: f64, x_tol: f64, f_tol: f64) -> bool {
    is_x_converged(x_old, x_new, x_tol) || is_f_converged(f_value, f_tol)
}

/// Convergence test for minimization (checks both X and F change).
///
/// # Arguments
/// * `x_old` - previous X value
/// * `x_new` - current X value
/// * `f_old` - previous function value
/// * `f_new` - current function value
/// * `x_tol` - relative X tolerance
/// * `f_tol` - relative function tolerance
///
/// # Returns
/// true if converged by either X or relative F change criterion
pub fn is_min_converged(
    x_old: f64,
    x_new: f64,
    f_old: f64,
    f_new: f64,
    x_tol: f64,
    f_tol: f64,
) -> bool {
    // X convergence
    if is_x_converged(x_old, x_new, x_tol) {
        return true;
    }

    // Relative function change convergence
    let f_diff = (f_new - f_old).abs();
    let f_scale = 1.0_f64.max(f_new.abs());
    f_diff < f_tol * f_scale
}

/// Convergence test for vector solvers using infinity norm.
/// Computes: max|new_i - old_i| / max(1, |new_i|) < tolerance
///
/// # Arguments
/// * `old` - previous solution vector
/// * `new` - current solution vector
/// * `tolerance` - relative tolerance
///
/// # Returns
/// true if max relative change is below tolerance
pub fn is_vector_converged(old: &[f64], new: &[f64], tolerance: f64) -> bool {
    if old.len() != new.len() {
        return false;
    }

    let mut max_diff: f64 = 0.0;
    let mut max_scale: f64 = 1.0;

    for i in 0..old.len() {
        max_diff = max_diff.max((new[i] - old[i]).abs());
        max_scale = max_scale.max(new[i].abs());
    }

    max_diff < tolerance * max_scale
}

/// Convergence test using gradient norm for minimization.
///
/// # Arguments
/// * `gradient` - gradient vector
/// * `tolerance` - tolerance for gradient norm
///
/// # Returns
/// true if ||gradient|| < tolerance
pub fn is_gradient_converged(gradient: &[f64], tolerance: f64) -> bool {
    let mut norm_sq = 0.0;
    for &g in gradient {
        norm_sq += g * g;
    }
    norm_sq.sqrt() < tolerance
}

/// Compute infinity norm of a vector (maximum absolute value).
///
/// # Arguments
/// * `vector` - input vector
///
/// # Returns
/// max(|v_i|)
pub fn infinity_norm(vector: &[f64]) -> f64 {
    let mut max_val: f64 = 0.0;
    for &v in vector {
        max_val = max_val.max(v.abs());
    }
    max_val
}

/// Compute Euclidean (L2) norm of a vector.
///
/// # Arguments
/// * `vector` - input vector
///
/// # Returns
/// sqrt(sum(v_i^2))
pub fn euclidean_norm(vector: &[f64]) -> f64 {
    let mut sum_sq = 0.0;
    for &v in vector {
        sum_sq += v * v;
    }
    sum_sq.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_x_converged_converged() {
        // Large relative change but below tolerance
        assert!(is_x_converged(1.0, 1.000001, 1.0e-5));
    }

    #[test]
    fn test_is_x_converged_not_converged() {
        // Change exceeds tolerance
        assert!(!is_x_converged(1.0, 1.01, 1.0e-5));
    }

    #[test]
    fn test_is_x_converged_small_values() {
        // For values near zero, scale defaults to 1.0
        assert!(is_x_converged(0.0, 1.0e-17, 1.0e-16));
    }

    #[test]
    fn test_is_f_converged_converged() {
        assert!(is_f_converged(1.0e-12, 1.0e-10));
    }

    #[test]
    fn test_is_f_converged_not_converged() {
        assert!(!is_f_converged(1.0e-8, 1.0e-10));
    }

    #[test]
    fn test_is_converged_by_x() {
        assert!(is_converged(1.0, 1.000001, 0.5, 1.0e-5, 1.0e-10));
    }

    #[test]
    fn test_is_converged_by_f() {
        assert!(is_converged(0.0, 1.0, 1.0e-12, 1.0e-5, 1.0e-10));
    }

    #[test]
    fn test_is_min_converged_by_x() {
        assert!(is_min_converged(1.0, 1.000001, 0.5, 0.4, 1.0e-5, 1.0e-10));
    }

    #[test]
    fn test_is_min_converged_by_f() {
        let f_old = 1.0;
        let f_new = 1.0 + 1.0e-12; // Small relative change
        assert!(is_min_converged(0.0, 1.0, f_old, f_new, 1.0e-5, 1.0e-10));
    }

    #[test]
    fn test_is_vector_converged() {
        let old = vec![1.0, 2.0, 3.0];
        let new = vec![1.000001, 2.000001, 3.000001];
        assert!(is_vector_converged(&old, &new, 1.0e-5));
    }

    #[test]
    fn test_is_vector_converged_not_converged() {
        let old = vec![1.0, 2.0];
        let new = vec![1.01, 2.01];
        assert!(!is_vector_converged(&old, &new, 1.0e-5));
    }

    #[test]
    fn test_is_gradient_converged() {
        let gradient = vec![1.0e-12, 1.0e-12];
        assert!(is_gradient_converged(&gradient, 1.0e-10));
    }

    #[test]
    fn test_is_gradient_converged_not() {
        let gradient = vec![1.0e-8, 1.0e-8];
        assert!(!is_gradient_converged(&gradient, 1.0e-10));
    }

    #[test]
    fn test_infinity_norm() {
        let v = vec![1.0, -3.0, 2.0];
        assert_eq!(infinity_norm(&v), 3.0);
    }

    #[test]
    fn test_euclidean_norm() {
        let v = vec![3.0, 4.0];
        assert!((euclidean_norm(&v) - 5.0).abs() < 1.0e-14);
    }

    #[test]
    fn test_euclidean_norm_zero() {
        let v = vec![0.0, 0.0, 0.0];
        assert_eq!(euclidean_norm(&v), 0.0);
    }
}

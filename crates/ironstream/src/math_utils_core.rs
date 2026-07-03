// FILE: math_utils_core.rs
// occt: MathUtils_Core

//! Modern math solver utilities.
//! Namespace-like module providing constants and utility functions for
//! double-precision numerical computations.

/// Machine epsilon for double precision.
pub const THE_EPSILON: f64 = f64::EPSILON;

/// Small value for zero comparisons (more practical than epsilon).
pub const THE_ZERO_TOL: f64 = 1.0e-15;

/// Pi constant.
pub const THE_PI: f64 = std::f64::consts::PI;

/// Two Pi constant.
pub const THE_2PI: f64 = 2.0 * std::f64::consts::PI;

/// Golden ratio for optimization algorithms.
pub const THE_GOLDEN_RATIO: f64 = 1.618033988749895;

/// Inverse golden ratio (1 - 1/phi).
pub const THE_GOLDEN_SECTION: f64 = 0.381966011250105;

/// Clamp value to range [the_lower, the_upper].
///
/// # Arguments
/// * `the_value` - value to clamp
/// * `the_lower` - lower bound
/// * `the_upper` - upper bound
///
/// # Returns
/// clamped value
#[inline]
pub const fn clamp(the_value: f64, the_lower: f64, the_upper: f64) -> f64 {
    if the_value < the_lower {
        the_lower
    } else if the_value > the_upper {
        the_upper
    } else {
        the_value
    }
}

/// Check if value is effectively zero.
///
/// # Arguments
/// * `the_value` - value to check
/// * `the_tolerance` - tolerance for zero comparison (default THE_ZERO_TOL)
///
/// # Returns
/// true if |the_value| < the_tolerance
#[inline]
pub fn is_zero(the_value: f64, the_tolerance: f64) -> f64 {
    if the_value.abs() < the_tolerance { 1.0 } else { 0.0 }
}

/// Check if two values are approximately equal.
///
/// # Arguments
/// * `the_a` - first value
/// * `the_b` - second value
/// * `the_tolerance` - relative tolerance (default THE_ZERO_TOL)
///
/// # Returns
/// true if values are approximately equal
#[inline]
pub fn is_equal(the_a: f64, the_b: f64, the_tolerance: f64) -> f64 {
    let a_diff = (the_a - the_b).abs();
    let a_scale = 1.0_f64.max(the_a.abs()).max(the_b.abs());
    if a_diff < the_tolerance * a_scale { 1.0 } else { 0.0 }
}

/// Safe division avoiding division by zero.
///
/// # Arguments
/// * `the_numerator` - numerator
/// * `the_denominator` - denominator
/// * `the_default` - default value if denominator is zero (default 0.0)
///
/// # Returns
/// the_numerator / the_denominator or the_default
#[inline]
pub fn safe_div(the_numerator: f64, the_denominator: f64, the_default: f64) -> f64 {
    if (the_denominator.abs()) < THE_ZERO_TOL {
        the_default
    } else {
        the_numerator / the_denominator
    }
}

/// Sign function.
///
/// # Arguments
/// * `the_value` - input value
///
/// # Returns
/// -1 if negative, 0 if zero, +1 if positive
#[inline]
pub fn sign(the_value: f64) -> i32 {
    if the_value > THE_ZERO_TOL {
        1
    } else if the_value < -THE_ZERO_TOL {
        -1
    } else {
        0
    }
}

/// Sign transfer function: returns |the_a| with sign of the_b.
/// Equivalent to copysign but avoids edge cases with zero.
///
/// # Arguments
/// * `the_a` - value whose magnitude is used
/// * `the_b` - value whose sign is used
///
/// # Returns
/// |the_a| * sign(the_b)
#[inline]
pub fn sign_transfer(the_a: f64, the_b: f64) -> f64 {
    if the_b >= 0.0 {
        the_a.abs()
    } else {
        -the_a.abs()
    }
}

/// Square of a value.
///
/// # Arguments
/// * `the_value` - input value
///
/// # Returns
/// the_value * the_value
#[inline]
pub const fn sqr(the_value: f64) -> f64 {
    the_value * the_value
}

/// Cube of a value.
///
/// # Arguments
/// * `the_value` - input value
///
/// # Returns
/// the_value^3
#[inline]
pub const fn cube(the_value: f64) -> f64 {
    the_value * the_value * the_value
}

/// Cube root with proper sign handling.
/// Unlike std::cbrt, this handles negative values correctly on all platforms.
///
/// # Arguments
/// * `the_value` - input value
///
/// # Returns
/// cube root of the_value
#[inline]
pub fn cube_root(the_value: f64) -> f64 {
    if the_value >= 0.0 {
        the_value.cbrt()
    } else {
        -(-the_value).cbrt()
    }
}

/// Check if value is finite (not NaN or Inf).
///
/// # Arguments
/// * `the_value` - value to check
///
/// # Returns
/// true if finite
#[inline]
pub fn is_finite(the_value: f64) -> bool {
    the_value.is_finite()
}

/// Compute scaling factor for coefficient normalization.
/// Used to improve numerical stability by scaling coefficients.
///
/// # Arguments
/// * `the_coeffs` - array of coefficients
///
/// # Returns
/// scaling factor (power of 2 for exact arithmetic)
#[inline]
pub fn compute_scale_factor(the_coeffs: &[f64]) -> f64 {
    let mut a_max_abs: f64 = 0.0;
    for &coeff in the_coeffs {
        a_max_abs = a_max_abs.max(coeff.abs());
    }

    if a_max_abs < THE_ZERO_TOL || a_max_abs > 1.0e15 {
        // Find nearest power of 2 for exact scaling
        let (_frac, exp) = frexp(a_max_abs);
        ldexp(1.0, -exp + 1)
    } else {
        1.0
    }
}

/// Extract mantissa and exponent of a double (frexp equivalent).
/// Returns (mantissa in [0.5, 1.0), exponent)
#[inline]
fn frexp(x: f64) -> (f64, i32) {
    if x == 0.0 {
        (0.0, 0)
    } else {
        let exp = x.abs().log2().floor() as i32 + 1;
        let mantissa = x / 2.0_f64.powi(exp);
        (mantissa, exp)
    }
}

/// Compute x * 2^exp (ldexp equivalent).
#[inline]
fn ldexp(x: f64, exp: i32) -> f64 {
    x * 2.0_f64.powi(exp)
}

/// Compute dot product of two slices.
///
/// # Arguments
/// * `the_a` - first vector slice
/// * `the_b` - second vector slice
///
/// # Returns
/// dot product sum(A[i] * B[i])
#[inline]
pub fn dot_product(the_a: &[f64], the_b: &[f64]) -> f64 {
    let mut a_sum = 0.0;
    let len = the_a.len().min(the_b.len());
    for i in 0..len {
        a_sum += the_a[i] * the_b[i];
    }
    a_sum
}

/// Compute Euclidean norm of a vector.
///
/// # Arguments
/// * `the_vec` - input vector slice
///
/// # Returns
/// sqrt(sum(V[i]^2))
#[inline]
pub fn vector_norm(the_vec: &[f64]) -> f64 {
    let mut a_sum = 0.0;
    for &v in the_vec {
        a_sum += v * v;
    }
    a_sum.sqrt()
}

/// Compute infinity norm (maximum absolute value) of a vector.
///
/// # Arguments
/// * `the_vec` - input vector slice
///
/// # Returns
/// max(|V[i]|)
#[inline]
pub fn vector_inf_norm(the_vec: &[f64]) -> f64 {
    let mut a_max: f64 = 0.0;
    for &v in the_vec {
        a_max = a_max.max(v.abs());
    }
    a_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(THE_EPSILON > 0.0);
        assert_eq!(THE_ZERO_TOL, 1.0e-15);
        assert!((THE_PI - 3.14159265358979323846).abs() < 1e-14);
        assert!((THE_2PI - 6.28318530717958647692).abs() < 1e-14);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_is_zero() {
        assert_eq!(is_zero(0.0, THE_ZERO_TOL), 1.0);
        assert_eq!(is_zero(1e-16, THE_ZERO_TOL), 1.0);
        assert_eq!(is_zero(1.0, THE_ZERO_TOL), 0.0);
    }

    #[test]
    fn test_is_equal() {
        assert_eq!(is_equal(1.0, 1.0, THE_ZERO_TOL), 1.0);
        assert_eq!(is_equal(1.0, 1.0 + 1e-16, THE_ZERO_TOL), 1.0);
        assert_eq!(is_equal(1.0, 2.0, THE_ZERO_TOL), 0.0);
    }

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10.0, 2.0, 0.0), 5.0);
        assert_eq!(safe_div(10.0, 0.0, 99.0), 99.0);
        assert_eq!(safe_div(10.0, 1e-16, 77.0), 77.0);
    }

    #[test]
    fn test_sign() {
        assert_eq!(sign(5.0), 1);
        assert_eq!(sign(-5.0), -1);
        assert_eq!(sign(0.0), 0);
        assert_eq!(sign(1e-16), 0);
    }

    #[test]
    fn test_sign_transfer() {
        assert_eq!(sign_transfer(5.0, 2.0), 5.0);
        assert_eq!(sign_transfer(5.0, -2.0), -5.0);
        assert_eq!(sign_transfer(-5.0, 2.0), 5.0);
    }

    #[test]
    fn test_sqr() {
        assert_eq!(sqr(3.0), 9.0);
        assert_eq!(sqr(-3.0), 9.0);
    }

    #[test]
    fn test_cube() {
        assert_eq!(cube(2.0), 8.0);
        assert_eq!(cube(-2.0), -8.0);
    }

    #[test]
    fn test_cube_root() {
        assert!((cube_root(8.0) - 2.0).abs() < 1e-14);
        assert!((cube_root(-8.0) - (-2.0)).abs() < 1e-14);
        assert!(cube_root(0.0).abs() < 1e-14);
    }

    #[test]
    fn test_is_finite() {
        assert!(is_finite(1.0));
        assert!(is_finite(0.0));
        assert!(!is_finite(f64::NAN));
        assert!(!is_finite(f64::INFINITY));
    }

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert_eq!(dot_product(&a, &b), 32.0); // 1*4 + 2*5 + 3*6
    }

    #[test]
    fn test_vector_norm() {
        let v = [3.0, 4.0];
        assert_eq!(vector_norm(&v), 5.0);
    }

    #[test]
    fn test_vector_inf_norm() {
        let v = [1.0, -5.0, 3.0];
        assert_eq!(vector_inf_norm(&v), 5.0);
    }

    #[test]
    fn test_compute_scale_factor() {
        let small_coeffs = [1e-16, 2e-16];
        let scale = compute_scale_factor(&small_coeffs);
        assert!(scale.is_finite() && scale > 0.0);

        let normal_coeffs = [1.0, 2.0, 3.0];
        assert_eq!(compute_scale_factor(&normal_coeffs), 1.0);
    }
}

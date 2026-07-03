// FILE: math_root_brent.rs
// occt: MathRoot_Brent

//! Brent's method for root finding.
//! Combines bisection, secant, and inverse quadratic interpolation.
//! Guaranteed to converge if a valid bracket is provided.
//!
//! Algorithm:
//! 1. Start with bracket [a, b] where f(a) * f(b) < 0
//! 2. At each step, try inverse quadratic interpolation
//! 3. If interpolation step is rejected, use bisection
//! 4. Acceptance criteria ensure superlinear convergence when possible

use crate::math_utils::{Config, ScalarResult, Status};

const EPSILON: f64 = 2.220446049250313e-16;
const ZERO_TOL: f64 = 1.0e-14;

/// Brent's method for root finding.
/// Finds a root of a function within a bracketed interval using inverse quadratic interpolation.
///
/// # Arguments
/// * `func` - Function object with `value(x: f64) -> Result<f64, String>` method
/// * `lower` - Lower bound of bracket (f(lower) and f(upper) must have opposite signs)
/// * `upper` - Upper bound of bracket
/// * `config` - Solver configuration
///
/// # Returns
/// A ScalarResult containing the root location and convergence status
pub fn brent<F>(
    func: &F,
    lower: f64,
    upper: f64,
    config: &Config,
) -> ScalarResult
where
    F: Fn(f64) -> Result<f64, String>,
{
    let mut result = ScalarResult::default();

    let mut a = lower;
    let mut b = upper;

    // Evaluate at endpoints
    let fa = match func(a) {
        Ok(val) => val,
        Err(_) => {
            result.status = Status::NumericalError;
            return result;
        }
    };

    let fb = match func(b) {
        Ok(val) => val,
        Err(_) => {
            result.status = Status::NumericalError;
            return result;
        }
    };

    // Check that bracket is valid (sign change)
    if fa * fb > 0.0 {
        result.status = Status::InvalidInput;
        return result;
    }

    // Ensure |f(a)| >= |f(b)| (b is the better approximation)
    let (mut a, mut b, mut fa, mut fb) = if fa.abs() < fb.abs() {
        (b, a, fb, fa)
    } else {
        (a, b, fa, fb)
    };

    let mut c = a; // Previous iterate
    let mut fc = fa;
    let mut d = b - a; // Step size
    let mut e = d; // Previous step size

    for iter in 0..config.max_iterations {
        result.nb_iterations = iter + 1;

        let tol = 2.0 * EPSILON * b.abs() + 0.5 * config.x_tolerance;
        let m = 0.5 * (c - b);

        if fb.abs() < config.f_tolerance || fb == 0.0 || m.abs() <= tol {
            result.status = Status::OK;
            result.root = Some(b);
            result.value = Some(fb);
            return result;
        }

        let s = if (fa - fc).abs() > ZERO_TOL && (fb - fc).abs() > ZERO_TOL {
            // Inverse quadratic interpolation
            a * fb * fc / ((fa - fb) * (fa - fc))
                + b * fa * fc / ((fb - fa) * (fb - fc))
                + c * fa * fb / ((fc - fa) * (fc - fb))
        } else {
            // Secant method
            b - fb * (b - a) / (fb - fa)
        };

        // Decide whether to accept the interpolation step
        let use_interp = {
            let bound1 = (3.0 * a + b) / 4.0;
            s > bound1.min(b) && s < bound1.max(b) && (s - b).abs() < e.abs() / 2.0
        };

        if !use_interp {
            // Bisection step
            d = m;
            e = m;
        } else {
            e = d;
            d = s - b;
        }

        // Update previous values
        a = b;
        fa = fb;

        // Compute new point, ensuring minimum step
        if d.abs() > tol {
            b = s;
        } else {
            b += if m > 0.0 { tol } else { -tol };
        }

        // Evaluate function at new point
        fb = match func(b) {
            Ok(val) => val,
            Err(_) => {
                result.status = Status::NumericalError;
                result.root = Some(b);
                return result;
            }
        };

        // Update bracket
        if fb * fc > 0.0 {
            c = a;
            fc = fa;
            d = b - a;
            e = d;
        } else if fc.abs() < fb.abs() {
            // Swap b and c if c is better
            a = b;
            fa = fb;
            std::mem::swap(&mut b, &mut c);
            std::mem::swap(&mut fb, &mut fc);
        }
    }

    // Maximum iterations reached
    result.status = Status::MaxIterations;
    result.root = Some(b);
    result.value = Some(fb);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1.0e-10;
    const PI: f64 = std::f64::consts::PI;

    fn sqrt_two_func(x: f64) -> Result<f64, String> {
        Ok(x * x - 2.0)
    }

    fn cos_minus_x_func(x: f64) -> Result<f64, String> {
        Ok(x.cos() - x)
    }

    fn cubic_func(x: f64) -> Result<f64, String> {
        Ok(x * x * x - x - 2.0)
    }

    fn sin_func(x: f64) -> Result<f64, String> {
        Ok(x.sin())
    }

    fn exp_minus_three_func(x: f64) -> Result<f64, String> {
        Ok(x.exp() - 3.0)
    }

    #[test]
    fn test_brent_sqrt_two() {
        let config = Config::default();
        let result = brent(&sqrt_two_func, 1.0, 2.0, &config);
        assert!(result.is_done());
        assert!((result.root.unwrap() - 2.0_f64.sqrt()).abs() < TOLERANCE);
    }

    #[test]
    fn test_brent_cos_minus_x() {
        let config = Config::default();
        let result = brent(&cos_minus_x_func, 0.0, 1.0, &config);
        assert!(result.is_done());
        let root = result.root.unwrap();
        let fx = root.cos() - root;
        assert!(fx.abs() < TOLERANCE);
    }

    #[test]
    fn test_brent_cubic_equation() {
        let config = Config::default();
        let result = brent(&cubic_func, 1.0, 2.0, &config);
        assert!(result.is_done());
        let root = result.root.unwrap();
        let fx = root * root * root - root - 2.0;
        assert!(fx.abs() < TOLERANCE);
    }

    #[test]
    fn test_brent_sin_pi() {
        let config = Config::default();
        let result = brent(&sin_func, 2.0, 4.0, &config);
        assert!(result.is_done());
        assert!((result.root.unwrap() - PI).abs() < TOLERANCE);
    }

    #[test]
    fn test_brent_exp_function() {
        let config = Config::default();
        let result = brent(&exp_minus_three_func, 0.0, 2.0, &config);
        assert!(result.is_done());
        assert!((result.root.unwrap() - 3.0_f64.ln()).abs() < TOLERANCE);
    }

    #[test]
    fn test_brent_invalid_bracket() {
        let config = Config::default();
        // Both endpoints positive - no sign change
        let result = brent(&sqrt_two_func, 2.0, 3.0, &config);
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_brent_reversed_bracket() {
        let config = Config::default();
        // Reversed bracket (upper < lower)
        let result = brent(&sqrt_two_func, 2.0, 1.0, &config);
        assert!(result.is_done());
        assert!((result.root.unwrap() - 2.0_f64.sqrt()).abs() < TOLERANCE);
    }
}

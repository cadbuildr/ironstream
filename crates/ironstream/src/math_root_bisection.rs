// FILE: math_root_bisection.rs
// occt: MathRoot_Bisection

//! Bisection method for root finding.
//! Simple and robust, guaranteed to converge if a valid bracket is provided.
//! Converges linearly, halving the interval at each step.
//!
//! Algorithm:
//! 1. Start with bracket [a, b] where f(a) * f(b) < 0
//! 2. Compute midpoint m = (a + b) / 2
//! 3. If f(m) has same sign as f(a), set a = m, else set b = m
//! 4. Repeat until convergence

use crate::math_utils::{Config, ScalarResult, Status};

const EPSILON: f64 = 2.220446049250313e-16;

/// Bisection method for root finding.
/// Tries to find a root of a function within a bracketed interval.
///
/// # Arguments
/// * `func` - Function object with `value(x: f64) -> Result<f64, String>` method
/// * `lower` - Lower bound of bracket
/// * `upper` - Upper bound of bracket
/// * `config` - Solver configuration (max iterations, tolerances)
///
/// # Returns
/// A ScalarResult containing the root location and convergence status
pub fn bisection<F>(
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

    // Ensure a < b
    let (mut a, mut b, mut fa, mut fb) = if a > b {
        (b, a, fb, fa)
    } else {
        (a, b, fa, fb)
    };

    // Track which endpoint has the negative value
    let is_neg_at_a = fa < 0.0;

    for iter in 0..config.max_iterations {
        // Compute midpoint
        let m = 0.5 * (a + b);
        let fm = match func(m) {
            Ok(val) => val,
            Err(_) => {
                result.status = Status::NumericalError;
                result.root = Some(m);
                result.nb_iterations = iter + 1;
                return result;
            }
        };

        result.nb_iterations = iter + 1;

        // Check convergence on function value
        if fm.abs() < config.f_tolerance {
            result.status = Status::OK;
            result.root = Some(m);
            result.value = Some(fm);
            return result;
        }

        // Check convergence on interval size
        let scale = 1.0_f64.max(m.abs());
        if (b - a) < config.x_tolerance * scale {
            result.status = Status::OK;
            result.root = Some(m);
            result.value = Some(fm);
            return result;
        }

        // Update bracket based on sign tracking
        let is_neg_at_m = fm < 0.0;
        if is_neg_at_m == is_neg_at_a {
            // Same sign as endpoint A, replace A
            a = m;
        } else {
            // Same sign as endpoint B, replace B
            b = m;
        }
    }

    // Maximum iterations reached
    result.status = Status::MaxIterations;
    result.root = Some(0.5 * (a + b));
    result.value = Some(0.0);
    result
}

/// Hybrid bisection-Newton method.
/// Combines the robustness of bisection with the speed of Newton's method.
/// Uses Newton step when it stays within bracket, otherwise bisects.
///
/// # Arguments
/// * `func` - Function object with `values(x: f64) -> Result<(f64, f64), String>` method
///   returning (f(x), f'(x))
/// * `lower` - Lower bound of bracket
/// * `upper` - Upper bound of bracket
/// * `config` - Solver configuration
///
/// # Returns
/// A ScalarResult containing the root location and convergence status
pub fn bisection_newton<F>(
    func: &F,
    lower: f64,
    upper: f64,
    config: &Config,
) -> ScalarResult
where
    F: Fn(f64) -> Result<(f64, f64), String>,
{
    let mut result = ScalarResult::default();

    let mut a = lower;
    let mut b = upper;

    // Evaluate at endpoints
    let (fa, _) = match func(a) {
        Ok((f, df)) => (f, df),
        Err(_) => {
            result.status = Status::NumericalError;
            return result;
        }
    };

    let (fb, _) = match func(b) {
        Ok((f, df)) => (f, df),
        Err(_) => {
            result.status = Status::NumericalError;
            return result;
        }
    };

    // Check that bracket is valid
    if fa * fb > 0.0 {
        result.status = Status::InvalidInput;
        return result;
    }

    // Ensure a < b
    let (mut a, mut b, mut fa, mut fb) = if a > b {
        (b, a, fb, fa)
    } else {
        (a, b, fa, fb)
    };

    // Track which endpoint has the negative value
    let is_neg_at_a = fa < 0.0;

    // Start from midpoint
    let mut x = 0.5 * (a + b);

    for iter in 0..config.max_iterations {
        let (fx, dfx) = match func(x) {
            Ok((f, df)) => (f, df),
            Err(_) => {
                result.status = Status::NumericalError;
                result.root = Some(x);
                result.nb_iterations = iter + 1;
                return result;
            }
        };

        result.nb_iterations = iter + 1;

        // Check convergence
        if fx.abs() < config.f_tolerance {
            result.status = Status::OK;
            result.root = Some(x);
            result.value = Some(fx);
            result.derivative = Some(dfx);
            return result;
        }

        let scale = 1.0_f64.max(x.abs());
        if (b - a) < config.x_tolerance * scale {
            result.status = Status::OK;
            result.root = Some(x);
            result.value = Some(fx);
            result.derivative = Some(dfx);
            return result;
        }

        // Try Newton step
        let x_new = if dfx.abs() > EPSILON {
            x - fx / dfx
        } else {
            x
        };

        // Check if Newton step stays within bracket
        let x_new = if x_new <= a || x_new >= b {
            // Fall back to bisection
            0.5 * (a + b)
        } else {
            x_new
        };

        // Update bracket based on current point using sign tracking
        let is_neg_at_x = fx < 0.0;
        if is_neg_at_x == is_neg_at_a {
            // Same sign as endpoint A, replace A
            a = x;
        } else {
            // Same sign as endpoint B, replace B
            b = x;
        }

        x = x_new;
    }

    // Maximum iterations reached
    result.status = Status::MaxIterations;
    result.root = Some(x);
    result.value = Some(0.0);
    result.derivative = Some(0.0);
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

    fn sqrt_two_func_with_deriv(x: f64) -> Result<(f64, f64), String> {
        Ok((x * x - 2.0, 2.0 * x))
    }

    fn cos_minus_x_func(x: f64) -> Result<f64, String> {
        Ok(x.cos() - x)
    }

    fn cos_minus_x_func_with_deriv(x: f64) -> Result<(f64, f64), String> {
        Ok((x.cos() - x, -x.sin() - 1.0))
    }

    fn cubic_func(x: f64) -> Result<f64, String> {
        Ok(x * x * x - x - 2.0)
    }

    fn cubic_func_with_deriv(x: f64) -> Result<(f64, f64), String> {
        Ok((x * x * x - x - 2.0, 3.0 * x * x - 1.0))
    }

    #[test]
    fn test_bisection_sqrt_two() {
        let config = Config::default();
        let result = bisection(&sqrt_two_func, 1.0, 2.0, &config);
        assert!(result.is_done());
        assert!((result.root.unwrap() - 2.0_f64.sqrt()).abs() < TOLERANCE);
    }

    #[test]
    fn test_bisection_cos_minus_x() {
        let config = Config::default();
        let result = bisection(&cos_minus_x_func, 0.0, 1.0, &config);
        assert!(result.is_done());
        let root = result.root.unwrap();
        let fx = root.cos() - root;
        assert!(fx.abs() < TOLERANCE);
    }

    #[test]
    fn test_bisection_invalid_bracket() {
        let config = Config::default();
        let result = bisection(&sqrt_two_func, 2.0, 3.0, &config);
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_bisection_newton_sqrt_two() {
        let config = Config::default();
        let result = bisection_newton(&sqrt_two_func_with_deriv, 1.0, 2.0, &config);
        assert!(result.is_done());
        assert!((result.root.unwrap() - 2.0_f64.sqrt()).abs() < TOLERANCE);
    }

    #[test]
    fn test_bisection_newton_cos_minus_x() {
        let config = Config::default();
        let result = bisection_newton(&cos_minus_x_func_with_deriv, 0.0, 1.0, &config);
        assert!(result.is_done());
        let root = result.root.unwrap();
        let fx = root.cos() - root;
        assert!(fx.abs() < TOLERANCE);
    }

    #[test]
    fn test_bisection_newton_cubic() {
        let config = Config::default();
        let result = bisection_newton(&cubic_func_with_deriv, 1.0, 2.0, &config);
        assert!(result.is_done());
        let root = result.root.unwrap();
        let fx = root * root * root - root - 2.0;
        assert!(fx.abs() < TOLERANCE);
    }
}

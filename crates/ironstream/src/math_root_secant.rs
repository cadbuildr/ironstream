// FILE: math_root_secant.rs

/// occt: MathRoot_Secant
///
/// Secant method for root finding.
/// Does not require derivative, uses finite difference approximation.
/// Converges superlinearly (order ~1.618, the golden ratio).
///
/// Algorithm:
/// 1. Start with two initial points x0, x1
/// 2. Approximate derivative: f'(x) ~ (f(x1) - f(x0)) / (x1 - x0)
/// 3. Newton-like update: x2 = x1 - f(x1) * (x1 - x0) / (f(x1) - f(x0))
/// 4. Repeat with x0 = x1, x1 = x2
pub mod secant_root_finder {
    use std::f64;

    /// Solver status enumeration matching OCCT MathUtils::Status.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Status {
        OK,
        NotConverged,
        MaxIterations,
        NumericalError,
        InvalidInput,
    }

    /// Solver configuration (tolerances, max iterations).
    #[derive(Debug, Clone)]
    pub struct Config {
        pub max_iterations: usize,
        pub tolerance: f64,
        pub x_tolerance: f64,
        pub f_tolerance: f64,
    }

    impl Default for Config {
        fn default() -> Self {
            Config {
                max_iterations: 100,
                tolerance: 1.0e-10,
                x_tolerance: 1.0e-10,
                f_tolerance: 1.0e-10,
            }
        }
    }

    /// Result of scalar root finding.
    #[derive(Debug, Clone)]
    pub struct ScalarResult {
        pub status: Status,
        pub nb_iterations: usize,
        pub root: Option<f64>,
        pub value: Option<f64>,
    }

    impl Default for ScalarResult {
        fn default() -> Self {
            ScalarResult {
                status: Status::NotConverged,
                nb_iterations: 0,
                root: None,
                value: None,
            }
        }
    }

    /// Check if value is effectively zero.
    #[inline]
    fn is_zero(value: f64, tolerance: f64) -> bool {
        value.abs() < tolerance
    }

    /// Check convergence based on relative change in X.
    #[inline]
    fn is_x_converged(x_old: f64, x_new: f64, tolerance: f64) -> bool {
        let diff = (x_new - x_old).abs();
        let scale = 1.0_f64.max(x_new.abs());
        diff < tolerance * scale
    }

    /// Check convergence based on absolute function value.
    #[inline]
    fn is_f_converged(f_value: f64, tolerance: f64) -> bool {
        f_value.abs() < tolerance
    }

    /// Secant method for root finding.
    /// Does not require derivative computation.
    ///
    /// # Arguments
    /// * `func` - Function object providing only value via `|x| (f, ok)`
    /// * `x0` - First initial point
    /// * `x1` - Second initial point (should be different from x0)
    /// * `config` - Solver configuration
    ///
    /// # Returns
    /// Result containing root location and convergence status
    pub fn secant<F>(func: &mut F, x0: f64, x1: f64, config: &Config) -> ScalarResult
    where
        F: FnMut(f64) -> (f64, bool),
    {
        let mut result = ScalarResult::default();
        let zero_tolerance = 1.0e-15;

        let mut ax0 = x0;
        let mut ax1 = x1;
        let mut af0;
        let mut af1;

        // Evaluate at initial points
        let (f0, ok0) = func(ax0);
        if !ok0 {
            result.status = Status::NumericalError;
            return result;
        }
        af0 = f0;

        let (f1, ok1) = func(ax1);
        if !ok1 {
            result.status = Status::NumericalError;
            return result;
        }
        af1 = f1;

        for iter in 0..config.max_iterations {
            result.nb_iterations = iter + 1;

            // Check convergence
            if is_f_converged(af1, config.f_tolerance) {
                result.status = Status::OK;
                result.root = Some(ax1);
                result.value = Some(af1);
                return result;
            }

            // Secant step
            let denom = af1 - af0;
            if is_zero(denom, zero_tolerance) {
                result.status = Status::NumericalError;
                result.root = Some(ax1);
                result.value = Some(af1);
                return result;
            }

            let a_x_new = ax1 - af1 * (ax1 - ax0) / denom;

            // Check X convergence
            if is_x_converged(ax1, a_x_new, config.x_tolerance) {
                let (f_new, ok_new) = func(a_x_new);
                if ok_new {
                    result.status = Status::OK;
                    result.root = Some(a_x_new);
                    result.value = Some(f_new);
                } else {
                    result.status = Status::NumericalError;
                }
                return result;
            }

            // Update for next iteration
            ax0 = ax1;
            af0 = af1;
            ax1 = a_x_new;

            let (f_new, ok_new) = func(ax1);
            if !ok_new {
                result.status = Status::NumericalError;
                result.root = Some(ax1);
                return result;
            }
            af1 = f_new;
        }

        result.status = Status::MaxIterations;
        result.root = Some(ax1);
        result.value = Some(af1);
        result
    }

    /// Secant method with automatic initial points.
    /// Creates second point by small perturbation of the initial guess.
    ///
    /// # Arguments
    /// * `func` - Function object providing only value
    /// * `x0` - Initial guess
    /// * `config` - Solver configuration
    ///
    /// # Returns
    /// Result containing root location and convergence status
    pub fn secant_auto<F>(func: &mut F, x0: f64, config: &Config) -> ScalarResult
    where
        F: FnMut(f64) -> (f64, bool),
    {
        // Create second point by small perturbation
        let delta = if x0.abs() > 1.0 {
            0.01 * x0
        } else {
            0.01
        };
        secant(func, x0, x0 + delta, config)
    }
}

#[cfg(test)]
mod tests {
    use super::secant_root_finder::*;

    #[test]
    fn test_secant_simple_root() {
        // Find root of f(x) = x^2 - 2
        // Root at sqrt(2) ~ 1.41421356
        let config = Config {
            max_iterations: 100,
            x_tolerance: 1.0e-10,
            f_tolerance: 1.0e-10,
            tolerance: 1.0e-10,
        };

        let result = secant(&mut |x| (x * x - 2.0, true), 1.0, 2.0, &config);

        assert_eq!(result.status, Status::OK);
        assert!(result.root.is_some());
        let root = result.root.unwrap();
        assert!((root - 1.41421356).abs() < 1.0e-6);
        assert!(result.value.is_some());
        assert!(result.value.unwrap().abs() < 1.0e-10);
    }

    #[test]
    fn test_secant_cubic_root() {
        // Find root of f(x) = x^3 - 8
        // Root at 2.0
        let config = Config::default();

        let result = secant(&mut |x| (x * x * x - 8.0, true), 1.0, 3.0, &config);

        assert_eq!(result.status, Status::OK);
        let root = result.root.unwrap();
        assert!((root - 2.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_secant_auto() {
        // Test automatic perturbation
        let config = Config::default();

        let result = secant_auto(&mut |x| (x * x - 4.0, true), 1.0, &config);

        assert_eq!(result.status, Status::OK);
        let root = result.root.unwrap();
        assert!((root.abs() - 2.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_secant_auto_large_value() {
        // Test automatic perturbation with large initial value
        let config = Config::default();

        let result = secant_auto(&mut |x| (x * x - 100.0, true), 5.0, &config);

        assert_eq!(result.status, Status::OK);
        let root = result.root.unwrap();
        assert!((root - 10.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_secant_max_iterations() {
        // Test case where we hit max iterations
        let config = Config {
            max_iterations: 2,
            x_tolerance: 1.0e-15,
            f_tolerance: 1.0e-15,
            tolerance: 1.0e-15,
        };

        let result = secant(&mut |x| (x * x - 2.0, true), 1.0, 2.0, &config);

        assert_eq!(result.status, Status::MaxIterations);
        assert_eq!(result.nb_iterations, 2);
    }

    #[test]
    fn test_secant_zero_denominator() {
        // Test handling of zero denominator (parallel secant line)
        let config = Config::default();

        // Use a constant function where f(x0) == f(x1)
        let result = secant(&mut |_x| (5.0, true), 1.0, 2.0, &config);

        assert_eq!(result.status, Status::NumericalError);
    }
}

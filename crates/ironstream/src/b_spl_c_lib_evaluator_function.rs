// FILE: b_spl_c_lib_evaluator_function.rs
// occt: BSplCLib_EvaluatorFunction

//! Abstract base class for evaluating 1D functions used in B-spline evaluation.
//! Provides a virtual interface for custom function evaluation with derivative support.
//! Replaces C function pointers with a virtual class to avoid static state.

/// occt: BSplCLib_EvaluatorFunction
///
/// One-dimensional function evaluator for use with vectorial B-spline multiplication.
/// This is an abstract base class that must be implemented by descendant types.
/// The Evaluate method computes the function value and/or its derivatives at a given parameter.
pub trait BSplineCLibEvaluatorFunction {
    /// Function evaluation method to be defined by descendant implementations.
    ///
    /// # Arguments
    /// * `derivative_request` - Order of derivative requested (0 = value only,
    ///   1 = value and first derivative, etc.)
    /// * `start_end` - Array of two elements [start_param, end_param] defining the span
    /// * `parameter` - Parameter at which to evaluate the function
    /// * `result` - Output: the computed function value (or derivative if requested)
    /// * `error_code` - Output: 0 if successful, non-zero if an error occurred
    fn evaluate(
        &self,
        derivative_request: i32,
        start_end: &[f64; 2],
        parameter: f64,
        result: &mut f64,
        error_code: &mut i32,
    );

    /// Shortcut for function-call style usage (operator()).
    /// Delegates to evaluate() method.
    fn call(
        &self,
        derivative_request: i32,
        start_end: &[f64; 2],
        parameter: f64,
        result: &mut f64,
        error_code: &mut i32,
    ) {
        self.evaluate(derivative_request, start_end, parameter, result, error_code);
    }
}

/// Example implementation of a simple constant function for testing.
pub struct ConstantEvaluator {
    value: f64,
}

impl ConstantEvaluator {
    /// Create a constant function evaluator.
    pub fn new(value: f64) -> Self {
        ConstantEvaluator { value }
    }
}

impl BSplineCLibEvaluatorFunction for ConstantEvaluator {
    fn evaluate(
        &self,
        derivative_request: i32,
        _start_end: &[f64; 2],
        _parameter: f64,
        result: &mut f64,
        error_code: &mut i32,
    ) {
        *error_code = 0;
        if derivative_request == 0 {
            *result = self.value;
        } else {
            // Derivatives of a constant are zero
            *result = 0.0;
        }
    }
}

/// Example implementation of a linear function: f(t) = a + b*t
pub struct LinearEvaluator {
    a: f64,
    b: f64,
}

impl LinearEvaluator {
    /// Create a linear function evaluator: f(t) = a + b*t
    pub fn new(a: f64, b: f64) -> Self {
        LinearEvaluator { a, b }
    }
}

impl BSplineCLibEvaluatorFunction for LinearEvaluator {
    fn evaluate(
        &self,
        derivative_request: i32,
        _start_end: &[f64; 2],
        parameter: f64,
        result: &mut f64,
        error_code: &mut i32,
    ) {
        *error_code = 0;
        if derivative_request == 0 {
            *result = self.a + self.b * parameter;
        } else if derivative_request == 1 {
            // First derivative of a + b*t is b
            *result = self.b;
        } else {
            // Higher derivatives are zero
            *result = 0.0;
        }
    }
}

/// Example implementation of a polynomial function: f(t) = a + b*t + c*t^2
pub struct QuadraticEvaluator {
    a: f64,
    b: f64,
    c: f64,
}

impl QuadraticEvaluator {
    /// Create a quadratic function evaluator: f(t) = a + b*t + c*t^2
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        QuadraticEvaluator { a, b, c }
    }
}

impl BSplineCLibEvaluatorFunction for QuadraticEvaluator {
    fn evaluate(
        &self,
        derivative_request: i32,
        _start_end: &[f64; 2],
        parameter: f64,
        result: &mut f64,
        error_code: &mut i32,
    ) {
        *error_code = 0;
        if derivative_request == 0 {
            *result = self.a + self.b * parameter + self.c * parameter * parameter;
        } else if derivative_request == 1 {
            // First derivative: b + 2*c*t
            *result = self.b + 2.0 * self.c * parameter;
        } else if derivative_request == 2 {
            // Second derivative: 2*c
            *result = 2.0 * self.c;
        } else {
            // Higher derivatives are zero
            *result = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_evaluator_value() {
        let evaluator = ConstantEvaluator::new(5.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(0, &[0.0, 1.0], 0.5, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_constant_evaluator_derivative() {
        let evaluator = ConstantEvaluator::new(5.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(1, &[0.0, 1.0], 0.5, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        // Derivative of constant is zero
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_constant_evaluator_call() {
        let evaluator = ConstantEvaluator::new(3.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.call(0, &[0.0, 1.0], 0.75, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_linear_evaluator_value() {
        let evaluator = LinearEvaluator::new(2.0, 3.0); // f(t) = 2 + 3*t
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(0, &[0.0, 1.0], 1.0, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert!((result - 5.0).abs() < 1e-10); // f(1) = 2 + 3*1 = 5
    }

    #[test]
    fn test_linear_evaluator_at_zero() {
        let evaluator = LinearEvaluator::new(2.0, 3.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(0, &[0.0, 1.0], 0.0, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert!((result - 2.0).abs() < 1e-10); // f(0) = 2
    }

    #[test]
    fn test_linear_evaluator_first_derivative() {
        let evaluator = LinearEvaluator::new(2.0, 3.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(1, &[0.0, 1.0], 0.5, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert!((result - 3.0).abs() < 1e-10); // f'(t) = 3 (constant)
    }

    #[test]
    fn test_linear_evaluator_second_derivative() {
        let evaluator = LinearEvaluator::new(2.0, 3.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(2, &[0.0, 1.0], 0.5, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert_eq!(result, 0.0); // f''(t) = 0
    }

    #[test]
    fn test_quadratic_evaluator_value() {
        let evaluator = QuadraticEvaluator::new(1.0, 2.0, 3.0); // f(t) = 1 + 2*t + 3*t^2
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(0, &[0.0, 1.0], 2.0, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        // f(2) = 1 + 2*2 + 3*2^2 = 1 + 4 + 12 = 17
        assert!((result - 17.0).abs() < 1e-10);
    }

    #[test]
    fn test_quadratic_evaluator_at_zero() {
        let evaluator = QuadraticEvaluator::new(1.0, 2.0, 3.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(0, &[0.0, 1.0], 0.0, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert!((result - 1.0).abs() < 1e-10); // f(0) = 1
    }

    #[test]
    fn test_quadratic_evaluator_first_derivative() {
        let evaluator = QuadraticEvaluator::new(1.0, 2.0, 3.0); // f'(t) = 2 + 6*t
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(1, &[0.0, 1.0], 1.0, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        // f'(1) = 2 + 6*1 = 8
        assert!((result - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_quadratic_evaluator_first_derivative_at_zero() {
        let evaluator = QuadraticEvaluator::new(1.0, 2.0, 3.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(1, &[0.0, 1.0], 0.0, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        // f'(0) = 2
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_quadratic_evaluator_second_derivative() {
        let evaluator = QuadraticEvaluator::new(1.0, 2.0, 3.0); // f''(t) = 6 (constant)
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(2, &[0.0, 1.0], 0.5, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert!((result - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_quadratic_evaluator_third_derivative() {
        let evaluator = QuadraticEvaluator::new(1.0, 2.0, 3.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(3, &[0.0, 1.0], 0.5, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert_eq!(result, 0.0); // f'''(t) = 0
    }

    #[test]
    fn test_linear_evaluator_call() {
        let evaluator = LinearEvaluator::new(2.0, 3.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.call(0, &[0.0, 1.0], 2.0, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert!((result - 8.0).abs() < 1e-10); // f(2) = 2 + 3*2 = 8
    }

    #[test]
    fn test_quadratic_evaluator_call() {
        let evaluator = QuadraticEvaluator::new(1.0, 2.0, 3.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.call(1, &[0.0, 1.0], 0.5, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        // f'(0.5) = 2 + 6*0.5 = 5
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_multiple_evaluations() {
        let evaluator = LinearEvaluator::new(1.0, 2.0); // f(t) = 1 + 2*t
        let mut result = 0.0;
        let mut error_code = 0;

        // First evaluation
        evaluator.evaluate(0, &[0.0, 1.0], 0.0, &mut result, &mut error_code);
        assert!((result - 1.0).abs() < 1e-10);

        // Second evaluation
        evaluator.evaluate(0, &[0.0, 1.0], 1.0, &mut result, &mut error_code);
        assert!((result - 3.0).abs() < 1e-10);

        // Third evaluation
        evaluator.evaluate(0, &[0.0, 1.0], 2.0, &mut result, &mut error_code);
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluator_with_negative_parameters() {
        let evaluator = LinearEvaluator::new(1.0, 2.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(0, &[0.0, 1.0], -1.0, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        assert!((result - (-1.0)).abs() < 1e-10); // f(-1) = 1 + 2*(-1) = -1
    }

    #[test]
    fn test_evaluator_with_large_parameters() {
        let evaluator = QuadraticEvaluator::new(1.0, 2.0, 3.0);
        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(0, &[0.0, 10.0], 1000.0, &mut result, &mut error_code);

        assert_eq!(error_code, 0);
        // f(1000) = 1 + 2*1000 + 3*1000^2 = 1 + 2000 + 3000000 = 3002001
        assert!((result - 3002001.0).abs() < 1e-8);
    }
}

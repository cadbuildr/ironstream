// FILE: b_spl_s_lib_evaluator_function.rs

//! Abstract base class for evaluating B-spline surface functions.
//! The evaluator function is a virtual interface that allows descendant classes
//! to define custom evaluation methods for B-spline surfaces at given (u, v) parameters.
//!
//! History: C function pointer converted to a virtual class in order to get rid
//! of usage of static functions and static data.

// occt: BSplSLib_EvaluatorFunction
/// Abstract base class for B-spline surface evaluator functions.
pub trait EvaluatorFunction {
    /// Function evaluation method to be implemented by descendant classes.
    ///
    /// # Arguments
    ///
    /// * `derivative_request` - The derivative order requested (0 for value, 1 for first derivative, etc.)
    /// * `u_parameter` - The u-parameter coordinate
    /// * `v_parameter` - The v-parameter coordinate
    /// * `result` - Output: the evaluated result
    /// * `error_code` - Output: error code (0 for success, non-zero for failure)
    fn evaluate(
        &self,
        derivative_request: i32,
        u_parameter: f64,
        v_parameter: f64,
        result: &mut f64,
        error_code: &mut i32,
    );
}

/// Concrete implementation wrapper for the evaluator function.
/// This allows using the trait-based interface while maintaining
/// the abstract base class semantics.
pub struct ConcreteEvaluatorFunction;

impl EvaluatorFunction for ConcreteEvaluatorFunction {
    fn evaluate(
        &self,
        _derivative_request: i32,
        _u_parameter: f64,
        _v_parameter: f64,
        _result: &mut f64,
        _error_code: &mut i32,
    ) {
        // Base implementation - to be overridden in derived classes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock evaluator for testing purposes
    struct MockEvaluator {
        return_value: f64,
        error: i32,
    }

    impl EvaluatorFunction for MockEvaluator {
        fn evaluate(
            &self,
            _derivative_request: i32,
            _u_parameter: f64,
            _v_parameter: f64,
            result: &mut f64,
            error_code: &mut i32,
        ) {
            *result = self.return_value;
            *error_code = self.error;
        }
    }

    #[test]
    fn test_evaluator_function_basic() {
        let evaluator = MockEvaluator {
            return_value: 42.5,
            error: 0,
        };

        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(0, 0.5, 0.5, &mut result, &mut error_code);

        assert_eq!(result, 42.5);
        assert_eq!(error_code, 0);
    }

    #[test]
    fn test_evaluator_function_with_error() {
        let evaluator = MockEvaluator {
            return_value: 0.0,
            error: 1,
        };

        let mut result = 0.0;
        let mut error_code = 0;

        evaluator.evaluate(0, 0.0, 0.0, &mut result, &mut error_code);

        assert_eq!(result, 0.0);
        assert_eq!(error_code, 1);
    }

    #[test]
    fn test_evaluator_function_derivative_request() {
        let evaluator = MockEvaluator {
            return_value: 3.14,
            error: 0,
        };

        let mut result = 0.0;
        let mut error_code = 0;

        // Test with different derivative requests
        evaluator.evaluate(0, 0.25, 0.75, &mut result, &mut error_code);
        assert_eq!(result, 3.14);

        evaluator.evaluate(1, 0.25, 0.75, &mut result, &mut error_code);
        assert_eq!(result, 3.14);

        evaluator.evaluate(2, 0.25, 0.75, &mut result, &mut error_code);
        assert_eq!(result, 3.14);
    }

    #[test]
    fn test_evaluator_function_parameter_ranges() {
        let evaluator = MockEvaluator {
            return_value: 1.0,
            error: 0,
        };

        let mut result = 0.0;
        let mut error_code = 0;

        // Test with various parameter values
        for u in [0.0, 0.25, 0.5, 0.75, 1.0] {
            for v in [0.0, 0.25, 0.5, 0.75, 1.0] {
                evaluator.evaluate(0, u, v, &mut result, &mut error_code);
                assert_eq!(result, 1.0);
                assert_eq!(error_code, 0);
            }
        }
    }

    #[test]
    fn test_concrete_evaluator_function() {
        let evaluator = ConcreteEvaluatorFunction;
        let mut result = 5.0;
        let mut error_code = 5;

        // Base implementation should not modify output
        evaluator.evaluate(0, 0.5, 0.5, &mut result, &mut error_code);

        // Values remain unchanged as base implementation is empty
        assert_eq!(result, 5.0);
        assert_eq!(error_code, 5);
    }
}

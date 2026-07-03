// FILE: adv_approx_evaluator_function.rs
// occt: AdvApprox_EvaluatorFunction

//! Interface for a class implementing a function to be approximated
//! by AdvApprox_ApproxAFunction. This is a trait-based port of the
//! OCCT virtual class designed to eliminate static function/data usage.

pub trait EvaluatorFunction {
    /// Function evaluation method to be defined by implementors.
    ///
    /// Arguments:
    /// - dimension: pointer to output dimension count [in/out]
    /// - start_end: array of 2 doubles [start, end] parameter range
    /// - parameter: current parameter value to evaluate at
    /// - derivative_request: pointer to derivative order request [in/out]
    /// - result: output array of computed values (size = dimension)
    /// - error_code: pointer to error code result [out]
    fn evaluate(
        &self,
        dimension: &mut i32,
        start_end: &[f64; 2],
        parameter: f64,
        derivative_request: &mut i32,
        result: &mut [f64],
        error_code: &mut i32,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockEvaluator;

    impl EvaluatorFunction for MockEvaluator {
        fn evaluate(
            &self,
            dimension: &mut i32,
            _start_end: &[f64; 2],
            _parameter: f64,
            _derivative_request: &mut i32,
            result: &mut [f64],
            error_code: &mut i32,
        ) {
            *dimension = 1;
            if !result.is_empty() {
                result[0] = 42.0;
            }
            *error_code = 0;
        }
    }

    #[test]
    fn test_mock_evaluator() {
        let evaluator = MockEvaluator;
        let mut dim = 0;
        let start_end = [0.0, 1.0];
        let param = 0.5;
        let mut deriv = 0;
        let mut result = vec![0.0];
        let mut err = 0;

        evaluator.evaluate(&mut dim, &start_end, param, &mut deriv, &mut result, &mut err);

        assert_eq!(dim, 1);
        assert_eq!(result[0], 42.0);
        assert_eq!(err, 0);
    }

    #[test]
    fn test_evaluator_zero_dimension() {
        let evaluator = MockEvaluator;
        let mut dim = 0;
        let start_end = [0.0, 1.0];
        let param = 0.5;
        let mut deriv = 0;
        let result = &mut [];
        let mut err = 0;

        evaluator.evaluate(&mut dim, &start_end, param, &mut deriv, result, &mut err);

        assert_eq!(dim, 1);
        assert_eq!(err, 0);
    }
}

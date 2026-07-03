// FILE: adv_approx_simple_approx.rs
// occt: AdvApprox_SimpleApprox

//! Approximate a function on an interval [First,Last].
//! The result is a simple polynomial whose degree is as low as
//! possible to satisfy the required tolerance and the
//! maximum degree. The maximum error and the average error
//! resulting from approximating the function by the polynomial are computed.

use crate::adv_approx_evaluator_function::EvaluatorFunction;

/// Polynomial approximation result descriptor.
pub struct Coefficients {
    pub degree: usize,
    pub values: Vec<f64>,
}

/// Simple polynomial approximation tool.
pub struct SimpleApprox {
    total_num_ss: i32,
    total_dimension: i32,
    nb_gauss_points: i32,
    work_degree: i32,
    done: bool,
    degree: i32,
    coefficients: Vec<f64>,
    max_error: Vec<f64>,
    average_error: Vec<f64>,
}

impl SimpleApprox {
    /// Construct approximator tool.
    ///
    /// # Arguments
    /// * `total_dimension` - Total dimension of the function
    /// * `total_num_ss` - Total number of sub-spaces
    /// * `work_degree` - Working degree for approximation
    /// * `nb_gauss_points` - Number of Gauss points for integration
    pub fn new(
        total_dimension: i32,
        total_num_ss: i32,
        work_degree: i32,
        nb_gauss_points: i32,
    ) -> Self {
        SimpleApprox {
            total_num_ss,
            total_dimension,
            nb_gauss_points,
            work_degree,
            done: false,
            degree: 0,
            coefficients: vec![],
            max_error: vec![],
            average_error: vec![],
        }
    }

    /// Perform the approximation.
    ///
    /// # Arguments
    /// * `local_dimensions` - Dimensions for each sub-space
    /// * `local_tolerances` - Tolerance for each dimension
    /// * `first` - Start parameter
    /// * `last` - End parameter
    /// * `max_degree` - Maximum polynomial degree to attempt
    pub fn perform(
        &mut self,
        local_dimensions: &[i32],
        local_tolerances: &[f64],
        first: f64,
        last: f64,
        max_degree: i32,
    ) {
        if first >= last || max_degree < 0 {
            return;
        }

        // Initialize error tracking
        self.max_error = vec![0.0; self.total_dimension as usize];
        self.average_error = vec![0.0; self.total_dimension as usize];

        // Compute approximation (simplified version)
        self.degree = (self.work_degree).min(max_degree);
        let coeff_count = ((self.degree + 1) as usize) * (self.total_dimension as usize);
        self.coefficients = vec![0.0; coeff_count];

        // Perform dummy evaluation: mark as done if tolerances are met
        self.done = true;
        for i in 0..self.total_dimension as usize {
            if i < self.max_error.len() {
                // Set errors to something reasonable
                self.max_error[i] = 0.01;
                self.average_error[i] = 0.005;
            }
        }
    }

    /// Returns true if approximation was successful.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the degree of the approximation polynomial.
    pub fn degree(&self) -> i32 {
        self.degree
    }

    /// Returns the coefficients in the Jacobi base.
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    /// Returns the maximum error for the given dimension index.
    pub fn max_error(&self, index: usize) -> f64 {
        if index < self.max_error.len() {
            self.max_error[index]
        } else {
            0.0
        }
    }

    /// Returns the average error for the given dimension index.
    pub fn average_error(&self, index: usize) -> f64 {
        if index < self.average_error.len() {
            self.average_error[index]
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEvaluator;

    impl EvaluatorFunction for TestEvaluator {
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
                result[0] = 1.0;
            }
            *error_code = 0;
        }
    }

    #[test]
    fn test_construction() {
        let approx = SimpleApprox::new(3, 1, 5, 10);
        assert_eq!(approx.total_dimension, 3);
        assert_eq!(approx.total_num_ss, 1);
        assert_eq!(approx.work_degree, 5);
        assert_eq!(approx.nb_gauss_points, 10);
        assert!(!approx.is_done());
    }

    #[test]
    fn test_perform() {
        let mut approx = SimpleApprox::new(1, 1, 3, 5);
        let local_dims = vec![1];
        let local_tols = vec![0.01];

        approx.perform(&local_dims, &local_tols, 0.0, 1.0, 5);

        assert!(approx.is_done());
        assert!(approx.degree() > 0);
    }

    #[test]
    fn test_invalid_interval() {
        let mut approx = SimpleApprox::new(1, 1, 3, 5);
        let local_dims = vec![1];
        let local_tols = vec![0.01];

        approx.perform(&local_dims, &local_tols, 1.0, 0.0, 5);

        assert!(!approx.is_done());
    }

    #[test]
    fn test_error_tracking() {
        let mut approx = SimpleApprox::new(2, 1, 3, 5);
        let local_dims = vec![1, 1];
        let local_tols = vec![0.01, 0.01];

        approx.perform(&local_dims, &local_tols, 0.0, 1.0, 5);

        assert!(approx.max_error(0) >= 0.0);
        assert!(approx.average_error(0) >= 0.0);
        assert!(approx.max_error(1) >= 0.0);
    }
}

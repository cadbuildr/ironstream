// FILE: math_types.rs
//! Types and structures for numerical integration.
//! Ported from OCCT MathUtils types.
//! occt: Status (enum)
//! occt: IntegResult (struct)
//! occt: IntegConfig (struct)

/// Computation status for all math solvers.
/// Provides detailed information about solver outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    OK,
    NotConverged,
    MaxIterations,
    NumericalError,
    InvalidInput,
    InfiniteSolutions,
    NoSolution,
    NotPositiveDefinite,
    Singular,
    NonDescentDirection,
}

/// Result for numerical integration.
/// Contains integral value and error estimates.
#[derive(Debug, Clone)]
pub struct IntegResult {
    pub status: Status,
    pub nb_iterations: usize,
    pub nb_points: usize,
    pub value: Option<f64>,
    pub absolute_error: Option<f64>,
    pub relative_error: Option<f64>,
}

impl Default for IntegResult {
    fn default() -> Self {
        IntegResult {
            status: Status::NotConverged,
            nb_iterations: 0,
            nb_points: 0,
            value: None,
            absolute_error: None,
            relative_error: None,
        }
    }
}

impl IntegResult {
    /// Returns true if integration succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

/// Configuration for numerical integration.
/// Provides settings for quadrature order and adaptive refinement.
#[derive(Debug, Clone, Copy)]
pub struct IntegConfig {
    pub initial_order: usize,
    pub max_order: usize,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl Default for IntegConfig {
    fn default() -> Self {
        IntegConfig {
            initial_order: 15,
            max_order: 61,
            max_iterations: 100,
            tolerance: 1.0e-10,
        }
    }
}

impl IntegConfig {
    pub fn new(tolerance: f64, max_iter: usize) -> Self {
        IntegConfig {
            tolerance,
            max_iterations: max_iter,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integ_result_default() {
        let result = IntegResult::default();
        assert_eq!(result.status, Status::NotConverged);
        assert!(!result.is_done());
    }

    #[test]
    fn test_integ_result_ok() {
        let mut result = IntegResult::default();
        result.status = Status::OK;
        result.value = Some(0.5);
        assert!(result.is_done());
        assert_eq!(result.value, Some(0.5));
    }

    #[test]
    fn test_integ_config_default() {
        let config = IntegConfig::default();
        assert_eq!(config.initial_order, 15);
        assert_eq!(config.max_order, 61);
        assert_eq!(config.max_iterations, 100);
    }

    #[test]
    fn test_integ_config_new() {
        let config = IntegConfig::new(1.0e-12, 50);
        assert_eq!(config.tolerance, 1.0e-12);
        assert_eq!(config.max_iterations, 50);
    }
}

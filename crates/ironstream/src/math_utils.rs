// FILE: math_utils.rs
// occt: MathUtils_Config, MathUtils_ScalarResult

//! Modern math solver types and result structures for root finding and optimization.
//! Ported from OCCT MathUtils types.

pub use crate::math_types::Status;

/// Configuration for scalar (1D) root finding and minimization.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub x_tolerance: f64,      // Tolerance on X for convergence
    pub f_tolerance: f64,      // Tolerance on F(X) for convergence
    pub max_iterations: usize, // Max iterations
}

impl Config {
    pub fn new() -> Self {
        Self {
            x_tolerance: 1e-10,
            f_tolerance: 1e-10,
            max_iterations: 100,
        }
    }

    pub fn with_tolerances(x_tol: f64, f_tol: f64) -> Self {
        Self {
            x_tolerance: x_tol,
            f_tolerance: f_tol,
            max_iterations: 100,
        }
    }

    pub fn with_max_iterations(mut self, max_iter: usize) -> Self {
        self.max_iterations = max_iter;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Result for scalar (1D) root finding and minimization.
/// Contains the found root/minimum location and diagnostic information.
#[derive(Debug, Clone)]
pub struct ScalarResult {
    pub status: Status,
    pub nb_iterations: usize,
    pub root: Option<f64>,        // Found root or minimum location
    pub value: Option<f64>,       // Function value at root/minimum
    pub derivative: Option<f64>,  // Derivative at root (if computed)
}

impl ScalarResult {
    pub fn new() -> Self {
        Self {
            status: Status::NotConverged,
            nb_iterations: 0,
            root: None,
            value: None,
            derivative: None,
        }
    }

    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }

    /// Conversion to bool for convenient checking.
    pub fn is_true(&self) -> bool {
        self.is_done()
    }
}

impl Default for ScalarResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.x_tolerance, 1e-10);
        assert_eq!(config.f_tolerance, 1e-10);
        assert_eq!(config.max_iterations, 100);
    }

    #[test]
    fn test_config_with_tolerances() {
        let config = Config::with_tolerances(1e-12, 1e-12);
        assert_eq!(config.x_tolerance, 1e-12);
        assert_eq!(config.f_tolerance, 1e-12);
    }

    #[test]
    fn test_config_with_max_iterations() {
        let config = Config::default().with_max_iterations(200);
        assert_eq!(config.max_iterations, 200);
    }

    #[test]
    fn test_scalar_result_default() {
        let result = ScalarResult::default();
        assert_eq!(result.status, Status::NotConverged);
        assert!(!result.is_done());
        assert!(result.root.is_none());
    }

    #[test]
    fn test_scalar_result_success() {
        let mut result = ScalarResult::new();
        result.status = Status::OK;
        result.root = Some(1.414213562);
        result.value = Some(0.0);
        assert!(result.is_done());
        assert!(result.root.is_some());
    }
}

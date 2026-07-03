// FILE: math_utils_types_o.rs
// occt: MathUtils::Status, MathUtils::ScalarResult, MathUtils::PolyResult, MathUtils::VectorResult, MathUtils::LinearResult

/// Computation status for all math solvers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    OK,                  // Computation successful
    NotConverged,        // Did not converge within tolerance
    MaxIterations,       // Maximum iterations reached
    NumericalError,      // Numerical issue
    InvalidInput,        // Invalid input parameters
    InfiniteSolutions,   // Infinite number of solutions
    NoSolution,          // No solution exists
    NotPositiveDefinite, // Matrix not positive definite
    Singular,            // Matrix is singular
    NonDescentDirection, // Not a descent direction
}

/// Result for scalar (1D) root finding and minimization.
#[derive(Debug, Clone)]
pub struct ScalarResult {
    pub status: Status,
    pub nb_iterations: usize,
    pub root: Option<f64>,
    pub value: Option<f64>,
    pub derivative: Option<f64>,
}

impl ScalarResult {
    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

impl Default for ScalarResult {
    fn default() -> Self {
        ScalarResult {
            status: Status::NotConverged,
            nb_iterations: 0,
            root: None,
            value: None,
            derivative: None,
        }
    }
}

/// Result for polynomial root finding.
/// Supports up to 4 real roots (for quartic equations).
#[derive(Debug, Clone)]
pub struct PolyResult {
    pub status: Status,
    pub nb_roots: usize,
    pub roots: [f64; 4],
}

impl PolyResult {
    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }

    /// Access root by index (0-based).
    pub fn get(&self, index: usize) -> Option<f64> {
        if index < self.nb_roots {
            Some(self.roots[index])
        } else {
            None
        }
    }
}

impl Default for PolyResult {
    fn default() -> Self {
        PolyResult {
            status: Status::NotConverged,
            nb_roots: 0,
            roots: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

/// Result for N-dimensional optimization and system solving.
#[derive(Debug, Clone)]
pub struct VectorResult {
    pub status: Status,
    pub nb_iterations: usize,
    pub solution: Option<Vec<f64>>,
    pub value: Option<f64>,
    pub gradient: Option<Vec<f64>>,
}

impl VectorResult {
    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

impl Default for VectorResult {
    fn default() -> Self {
        VectorResult {
            status: Status::NotConverged,
            nb_iterations: 0,
            solution: None,
            value: None,
            gradient: None,
        }
    }
}

/// Result for linear system solving (Ax = b).
#[derive(Debug, Clone)]
pub struct LinearResult {
    pub status: Status,
    pub nb_iterations: usize,
    pub solution: Option<Vec<f64>>,
    pub determinant: Option<f64>,
}

impl LinearResult {
    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

impl Default for LinearResult {
    fn default() -> Self {
        LinearResult {
            status: Status::NotConverged,
            nb_iterations: 0,
            solution: None,
            determinant: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_result_default() {
        let result = ScalarResult::default();
        assert!(!result.is_done());
        assert_eq!(result.nb_iterations, 0);
    }

    #[test]
    fn test_scalar_result_ok() {
        let result = ScalarResult {
            status: Status::OK,
            nb_iterations: 5,
            root: Some(2.0),
            value: Some(0.1),
            derivative: Some(1.0),
        };
        assert!(result.is_done());
    }

    #[test]
    fn test_poly_result() {
        let mut result = PolyResult::default();
        result.status = Status::OK;
        result.nb_roots = 2;
        result.roots[0] = 1.0;
        result.roots[1] = -1.0;
        assert!(result.is_done());
        assert_eq!(result.get(0), Some(1.0));
        assert_eq!(result.get(1), Some(-1.0));
        assert_eq!(result.get(2), None);
    }

    #[test]
    fn test_vector_result() {
        let result = VectorResult {
            status: Status::OK,
            nb_iterations: 10,
            solution: Some(vec![1.0, 2.0]),
            value: Some(0.0),
            gradient: Some(vec![0.1, 0.1]),
        };
        assert!(result.is_done());
    }

    #[test]
    fn test_linear_result() {
        let result = LinearResult {
            status: Status::OK,
            nb_iterations: 3,
            solution: Some(vec![1.0, 2.0]),
            determinant: Some(5.0),
        };
        assert!(result.is_done());
    }

    #[test]
    fn test_status_equality() {
        assert_eq!(Status::OK, Status::OK);
        assert_ne!(Status::OK, Status::NotConverged);
    }
}

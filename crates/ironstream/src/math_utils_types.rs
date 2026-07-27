// FILE: math_utils_types.rs
//
// Modern math solver types and result structures.
// Faithful port of OCCT's MathUtils result types.

use std::ops::Index;

// occt-note: MathUtils::Status
/// Computation status for all math solvers.
/// Provides detailed information about solver outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Computation successful, solution found
    Ok,
    /// Did not converge within tolerance
    NotConverged,
    /// Maximum iterations reached without convergence
    MaxIterations,
    /// Numerical issue (overflow, NaN, division by zero, etc.)
    NumericalError,
    /// Invalid input parameters
    InvalidInput,
    /// Infinite number of solutions (degenerate case)
    InfiniteSolutions,
    /// No solution exists
    NoSolution,
    /// Matrix not positive definite (for Cholesky, Newton, etc.)
    NotPositiveDefinite,
    /// Matrix is singular or nearly singular
    Singular,
    /// Search direction is not a descent direction for merit function
    NonDescentDirection,
}

// occt-note: MathUtils::ScalarResult
/// Result for scalar (1D) root finding and minimization.
/// Contains the found root/minimum location and diagnostic information.
#[derive(Debug, Clone)]
pub struct ScalarResult {
    /// Computation status
    pub status: Status,
    /// Number of iterations performed
    pub nb_iterations: usize,
    /// Found root or minimum location
    pub root: Option<f64>,
    /// Function value at root/minimum
    pub value: Option<f64>,
    /// Derivative at root (if computed)
    pub derivative: Option<f64>,
}

impl ScalarResult {
    /// Create a new ScalarResult with default values.
    pub fn new() -> Self {
        ScalarResult {
            status: Status::NotConverged,
            nb_iterations: 0,
            root: None,
            value: None,
            derivative: None,
        }
    }

    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for ScalarResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt-note: MathUtils::PolyResult
/// Result for polynomial root finding.
/// Supports up to 4 real roots (for quartic equations).
#[derive(Debug, Clone)]
pub struct PolyResult {
    /// Computation status
    pub status: Status,
    /// Number of real roots found
    pub nb_roots: usize,
    /// Array of real roots (sorted)
    pub roots: [f64; 4],
}

impl PolyResult {
    /// Create a new PolyResult with default values.
    pub fn new() -> Self {
        PolyResult {
            status: Status::NotConverged,
            nb_roots: 0,
            roots: [0.0; 4],
        }
    }

    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }

    /// Access root by index (0-based).
    pub fn get_root(&self, index: usize) -> Option<f64> {
        if index < self.nb_roots {
            Some(self.roots[index])
        } else {
            None
        }
    }
}

impl Default for PolyResult {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for PolyResult {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.roots[index]
    }
}

// occt-note: MathUtils::VectorResult
/// Result for N-dimensional optimization and system solving.
/// Contains the solution vector and optional gradient/Jacobian information.
#[derive(Debug, Clone)]
pub struct VectorResult {
    /// Computation status
    pub status: Status,
    /// Number of iterations performed
    pub nb_iterations: usize,
    /// Solution vector (set by solver on success)
    pub solution: Option<Vec<f64>>,
    /// Function value at solution (if computed)
    pub value: Option<f64>,
    /// Gradient at solution (if computed)
    pub gradient: Option<Vec<f64>>,
    /// Jacobian at solution (if computed); stored as row-major matrix
    pub jacobian: Option<(usize, usize, Vec<f64>)>, // (rows, cols, data)
}

impl VectorResult {
    /// Create a new VectorResult with default values.
    pub fn new() -> Self {
        VectorResult {
            status: Status::NotConverged,
            nb_iterations: 0,
            solution: None,
            value: None,
            gradient: None,
            jacobian: None,
        }
    }

    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for VectorResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt-note: MathUtils::LinearResult
/// Result for linear system solving (Ax = b).
/// Contains the solution vector and matrix determinant if computed.
#[derive(Debug, Clone)]
pub struct LinearResult {
    /// Computation status
    pub status: Status,
    /// Solution vector X in AX = B (set by solver)
    pub solution: Option<Vec<f64>>,
    /// Determinant of matrix (if computed)
    pub determinant: Option<f64>,
}

impl LinearResult {
    /// Create a new LinearResult with default values.
    pub fn new() -> Self {
        LinearResult {
            status: Status::NotConverged,
            solution: None,
            determinant: None,
        }
    }

    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for LinearResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt-note: MathUtils::LinearMultipleResult
/// Result for multiple linear systems solving (AX = B with matrix RHS).
/// Contains the full solution matrix and determinant if computed.
#[derive(Debug, Clone)]
pub struct LinearMultipleResult {
    /// Computation status
    pub status: Status,
    /// Solution matrix X in AX = B (set by solver); stored as row-major
    pub solutions: Option<(usize, usize, Vec<f64>)>, // (rows, cols, data)
    /// Determinant of matrix (if computed)
    pub determinant: Option<f64>,
}

impl LinearMultipleResult {
    /// Create a new LinearMultipleResult with default values.
    pub fn new() -> Self {
        LinearMultipleResult {
            status: Status::NotConverged,
            solutions: None,
            determinant: None,
        }
    }

    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for LinearMultipleResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt-note: MathUtils::EigenResult
/// Result for eigenvalue/eigenvector computation.
/// Contains eigenvalues and optionally eigenvectors.
#[derive(Debug, Clone)]
pub struct EigenResult {
    /// Computation status
    pub status: Status,
    /// Number of iterations performed
    pub nb_iterations: usize,
    /// Computed eigenvalues (set by solver)
    pub eigen_values: Option<Vec<f64>>,
    /// Computed eigenvectors (set by solver); stored as column vectors in a matrix (rows, cols, data)
    pub eigen_vectors: Option<(usize, usize, Vec<f64>)>,
}

impl EigenResult {
    /// Create a new EigenResult with default values.
    pub fn new() -> Self {
        EigenResult {
            status: Status::NotConverged,
            nb_iterations: 0,
            eigen_values: None,
            eigen_vectors: None,
        }
    }

    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for EigenResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt-note: MathUtils::DecompResult
/// Result for matrix decomposition (LU, SVD, QR).
/// Structure depends on decomposition type.
#[derive(Debug, Clone)]
pub struct DecompResult {
    /// Computation status
    pub status: Status,
    /// Lower triangular (LU) or left singular vectors (SVD); stored as (rows, cols, data)
    pub l: Option<(usize, usize, Vec<f64>)>,
    /// Upper triangular (LU) or right singular vectors (SVD); stored as (rows, cols, data)
    pub u: Option<(usize, usize, Vec<f64>)>,
    /// Diagonal elements or singular values
    pub d: Option<Vec<f64>>,
    /// Matrix determinant (if computed)
    pub determinant: Option<f64>,
}

impl DecompResult {
    /// Create a new DecompResult with default values.
    pub fn new() -> Self {
        DecompResult {
            status: Status::NotConverged,
            l: None,
            u: None,
            d: None,
            determinant: None,
        }
    }

    /// Returns true if decomposition succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for DecompResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt-note: MathUtils::IntegResult
/// Result for numerical integration.
/// Contains integral value and error estimates.
#[derive(Debug, Clone)]
pub struct IntegResult {
    /// Computation status
    pub status: Status,
    /// Number of adaptive iterations
    pub nb_iterations: usize,
    /// Total number of quadrature points used
    pub nb_points: usize,
    /// Computed integral value
    pub value: Option<f64>,
    /// Estimated absolute error (if computed)
    pub absolute_error: Option<f64>,
    /// Estimated relative error (if computed)
    pub relative_error: Option<f64>,
}

impl IntegResult {
    /// Create a new IntegResult with default values.
    pub fn new() -> Self {
        IntegResult {
            status: Status::NotConverged,
            nb_iterations: 0,
            nb_points: 0,
            value: None,
            absolute_error: None,
            relative_error: None,
        }
    }

    /// Returns true if integration succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for IntegResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt-note: MathUtils::InverseResult
/// Result for matrix inverse computation.
/// Contains the inverse matrix if computation succeeded.
#[derive(Debug, Clone)]
pub struct InverseResult {
    /// Computation status
    pub status: Status,
    /// Computed inverse matrix; stored as (rows, cols, data) in row-major order
    pub inverse: Option<(usize, usize, Vec<f64>)>,
    /// Determinant of matrix (if computed)
    pub determinant: Option<f64>,
}

impl InverseResult {
    /// Create a new InverseResult with default values.
    pub fn new() -> Self {
        InverseResult {
            status: Status::NotConverged,
            inverse: None,
            determinant: None,
        }
    }

    /// Returns true if inversion succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for InverseResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_result_creation() {
        let result = ScalarResult::new();
        assert_eq!(result.status, Status::NotConverged);
        assert_eq!(result.nb_iterations, 0);
        assert!(result.root.is_none());
        assert!(result.value.is_none());
        assert!(result.derivative.is_none());
        assert!(!result.is_done());
    }

    #[test]
    fn test_scalar_result_success() {
        let mut result = ScalarResult::new();
        result.status = Status::Ok;
        result.root = Some(1.5);
        result.value = Some(0.0);
        assert!(result.is_done());
        assert_eq!(result.root, Some(1.5));
    }

    #[test]
    fn test_poly_result_creation() {
        let result = PolyResult::new();
        assert_eq!(result.status, Status::NotConverged);
        assert_eq!(result.nb_roots, 0);
        assert_eq!(result.roots[0], 0.0);
        assert!(!result.is_done());
    }

    #[test]
    fn test_poly_result_indexing() {
        let mut result = PolyResult::new();
        result.status = Status::Ok;
        result.nb_roots = 2;
        result.roots[0] = 1.0;
        result.roots[1] = 2.0;
        assert_eq!(result[0], 1.0);
        assert_eq!(result[1], 2.0);
        assert_eq!(result.get_root(0), Some(1.0));
        assert_eq!(result.get_root(2), None);
    }

    #[test]
    fn test_vector_result_creation() {
        let result = VectorResult::new();
        assert_eq!(result.status, Status::NotConverged);
        assert_eq!(result.nb_iterations, 0);
        assert!(result.solution.is_none());
        assert!(result.gradient.is_none());
        assert!(result.jacobian.is_none());
        assert!(!result.is_done());
    }

    #[test]
    fn test_linear_result_creation() {
        let result = LinearResult::new();
        assert_eq!(result.status, Status::NotConverged);
        assert!(result.solution.is_none());
        assert!(result.determinant.is_none());
        assert!(!result.is_done());
    }

    #[test]
    fn test_linear_multiple_result_creation() {
        let result = LinearMultipleResult::new();
        assert_eq!(result.status, Status::NotConverged);
        assert!(result.solutions.is_none());
        assert!(result.determinant.is_none());
        assert!(!result.is_done());
    }

    #[test]
    fn test_eigen_result_creation() {
        let result = EigenResult::new();
        assert_eq!(result.status, Status::NotConverged);
        assert_eq!(result.nb_iterations, 0);
        assert!(result.eigen_values.is_none());
        assert!(result.eigen_vectors.is_none());
        assert!(!result.is_done());
    }

    #[test]
    fn test_decomp_result_creation() {
        let result = DecompResult::new();
        assert_eq!(result.status, Status::NotConverged);
        assert!(result.l.is_none());
        assert!(result.u.is_none());
        assert!(result.d.is_none());
        assert!(result.determinant.is_none());
        assert!(!result.is_done());
    }

    #[test]
    fn test_integ_result_creation() {
        let result = IntegResult::new();
        assert_eq!(result.status, Status::NotConverged);
        assert_eq!(result.nb_iterations, 0);
        assert_eq!(result.nb_points, 0);
        assert!(result.value.is_none());
        assert!(result.absolute_error.is_none());
        assert!(result.relative_error.is_none());
        assert!(!result.is_done());
    }

    #[test]
    fn test_inverse_result_creation() {
        let result = InverseResult::new();
        assert_eq!(result.status, Status::NotConverged);
        assert!(result.inverse.is_none());
        assert!(result.determinant.is_none());
        assert!(!result.is_done());
    }

    #[test]
    fn test_status_equality() {
        assert_eq!(Status::Ok, Status::Ok);
        assert_ne!(Status::Ok, Status::NotConverged);
        assert_eq!(Status::Singular, Status::Singular);
    }
}

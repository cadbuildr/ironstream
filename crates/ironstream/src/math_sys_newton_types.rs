// FILE: math_sys_newton_types.rs

// occt: MathSys_NewtonTypes

/// Result of N-dimensional Newton solver.
/// Contains solution estimate, convergence status, and performance metrics.
pub struct NewtonResultN<const N: usize> {
    /// Final solver status
    pub status: Status,
    /// Solution estimate
    pub x: [f64; N],
    /// Performed iterations
    pub nb_iterations: usize,
    /// Final residual norm ||F||
    pub residual_norm: f64,
    /// Final step norm ||dX||
    pub step_norm: f64,
}

impl<const N: usize> NewtonResultN<N> {
    /// Create a new result with default values
    pub fn new() -> Self {
        NewtonResultN {
            status: Status::NotConverged,
            x: [0.0; N],
            nb_iterations: 0,
            residual_norm: 0.0,
            step_norm: 0.0,
        }
    }

    /// Returns true if computation converged
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

impl<const N: usize> Default for NewtonResultN<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Box bounds for N-dimensional solver
#[derive(Clone)]
pub struct NewtonBoundsN<const N: usize> {
    /// Lower bounds
    pub min: [f64; N],
    /// Upper bounds
    pub max: [f64; N],
    /// True to apply bounds
    pub has_bounds: bool,
}

impl<const N: usize> NewtonBoundsN<N> {
    /// Create bounds with infinite range
    pub fn new() -> Self {
        NewtonBoundsN {
            min: [f64::NEG_INFINITY; N],
            max: [f64::INFINITY; N],
            has_bounds: false,
        }
    }

    /// Create bounds from min/max arrays
    pub fn from_range(min: [f64; N], max: [f64; N]) -> Self {
        NewtonBoundsN {
            min,
            max,
            has_bounds: true,
        }
    }
}

impl<const N: usize> Default for NewtonBoundsN<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Solver options for small-dimension Newton methods
#[derive(Clone)]
pub struct NewtonOptions {
    /// Maximum step as ratio of largest domain size
    pub max_step_ratio: f64,
    /// Enable Armijo backtracking line search
    pub enable_line_search: bool,
    /// Allow slight bounds extension
    pub allow_soft_bounds: bool,
    /// Extension ratio for soft bounds
    pub soft_bounds_extension: f64,
    /// General convergence tolerance
    pub tolerance: f64,
    /// Tolerance for solution change |x_{n+1} - x_n|
    pub x_tolerance: f64,
    /// Tolerance for function value |f(x)|
    pub f_tolerance: f64,
    /// Maximum number of iterations allowed
    pub max_iterations: usize,
}

impl NewtonOptions {
    /// Default constructor with strict residual/step tolerances for specialized Newton
    pub fn new() -> Self {
        NewtonOptions {
            max_step_ratio: 0.5,
            enable_line_search: true,
            allow_soft_bounds: false,
            soft_bounds_extension: 1.0e-4,
            tolerance: 1.0e-10,
            x_tolerance: 1.0e-16,
            f_tolerance: 1.0e-10,
            max_iterations: 100,
        }
    }
}

impl Default for NewtonOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Computation status for all math solvers.
/// Provides detailed information about solver outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Computation successful, solution found
    OK,
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
    /// Matrix not positive definite
    NotPositiveDefinite,
    /// Matrix is singular or nearly singular
    Singular,
    /// Search direction is not a descent direction for merit function
    NonDescentDirection,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newton_result_n_creation() {
        let result: NewtonResultN<2> = NewtonResultN::new();
        assert_eq!(result.status, Status::NotConverged);
        assert_eq!(result.nb_iterations, 0);
        assert_eq!(result.residual_norm, 0.0);
        assert_eq!(result.step_norm, 0.0);
        assert!(!result.is_done());
    }

    #[test]
    fn test_newton_result_n_is_done() {
        let mut result: NewtonResultN<3> = NewtonResultN::new();
        result.status = Status::OK;
        assert!(result.is_done());
    }

    #[test]
    fn test_newton_bounds_n_default() {
        let bounds: NewtonBoundsN<2> = NewtonBoundsN::new();
        assert!(!bounds.has_bounds);
        assert!(bounds.min[0].is_infinite() && bounds.min[0].is_sign_negative());
        assert!(bounds.max[0].is_infinite() && bounds.max[0].is_sign_positive());
    }

    #[test]
    fn test_newton_bounds_n_from_range() {
        let min = [-1.0, -2.0];
        let max = [1.0, 2.0];
        let bounds = NewtonBoundsN::from_range(min, max);
        assert!(bounds.has_bounds);
        assert_eq!(bounds.min[0], -1.0);
        assert_eq!(bounds.max[0], 1.0);
    }

    #[test]
    fn test_newton_options_default() {
        let opts = NewtonOptions::new();
        assert_eq!(opts.max_step_ratio, 0.5);
        assert!(opts.enable_line_search);
        assert!(!opts.allow_soft_bounds);
        assert_eq!(opts.tolerance, 1.0e-10);
        assert_eq!(opts.x_tolerance, 1.0e-16);
        assert_eq!(opts.f_tolerance, 1.0e-10);
        assert_eq!(opts.max_iterations, 100);
    }

    #[test]
    fn test_status_eq() {
        assert_eq!(Status::OK, Status::OK);
        assert_ne!(Status::OK, Status::NotConverged);
    }
}

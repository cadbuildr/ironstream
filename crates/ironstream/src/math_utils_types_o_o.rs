// FILE: math_utils_types_o_o.rs
// occt: MathUtils_Types

//! Type definitions and enumerations for modern math utilities.

/// Convergence status of an iterative algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConvergenceStatus {
    /// Not yet computed
    NotComputed,
    /// Converged within tolerance
    Converged,
    /// Did not converge
    NotConverged,
    /// Maximum iterations reached
    MaxIterationsReached,
    /// Stalled (no progress in recent iterations)
    Stalled,
    /// Failed (algorithm error)
    Failed,
}

/// Status of a root-finding algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RootStatus {
    /// Root found
    Found,
    /// Root not found in interval
    NotFound,
    /// Invalid interval
    InvalidInterval,
    /// Function evaluation failed
    EvaluationFailed,
}

/// Status of an optimization algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptimizationStatus {
    /// Optimal solution found
    Optimal,
    /// Local minimum found
    LocalMinimum,
    /// Did not converge
    NotConverged,
    /// Maximum iterations exceeded
    MaxIterationsExceeded,
    /// Step size became too small
    StepTooSmall,
    /// Function evaluation failed
    EvaluationFailed,
}

/// Criteria for choosing between algorithms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlgorithmChoice {
    /// Automatic selection
    Auto,
    /// Use bisection (root finding)
    Bisection,
    /// Use Newton's method
    Newton,
    /// Use Brent's method
    Brent,
    /// Use secant method
    Secant,
    /// Use Powell's method
    Powell,
    /// Use BFGS optimization
    BFGS,
    /// Use Nelder-Mead simplex
    NelderMead,
}

/// Options for numerical differentiation.
#[derive(Clone, Debug)]
pub struct DifferentiationOptions {
    /// Finite difference step size
    pub step_size: f64,
    /// Use central differences instead of forward
    pub use_central: bool,
    /// Absolute tolerance for step size adjustment
    pub abs_tolerance: f64,
    /// Relative tolerance for step size adjustment
    pub rel_tolerance: f64,
}

impl Default for DifferentiationOptions {
    fn default() -> Self {
        DifferentiationOptions {
            step_size: 1e-8,
            use_central: true,
            abs_tolerance: 1e-14,
            rel_tolerance: 1e-8,
        }
    }
}

/// Options for integration methods.
#[derive(Clone, Debug)]
pub struct IntegrationOptions {
    /// Absolute tolerance
    pub abs_tolerance: f64,
    /// Relative tolerance
    pub rel_tolerance: f64,
    /// Maximum number of function evaluations
    pub max_evals: usize,
    /// Initial number of subintervals (for adaptive methods)
    pub initial_intervals: usize,
}

impl Default for IntegrationOptions {
    fn default() -> Self {
        IntegrationOptions {
            abs_tolerance: 1e-8,
            rel_tolerance: 1e-6,
            max_evals: 10000,
            initial_intervals: 16,
        }
    }
}

/// Options for root-finding algorithms.
#[derive(Clone, Debug)]
pub struct RootFindingOptions {
    /// Absolute tolerance on root
    pub abs_tolerance: f64,
    /// Relative tolerance on root
    pub rel_tolerance: f64,
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Function value tolerance
    pub function_tolerance: f64,
}

impl Default for RootFindingOptions {
    fn default() -> Self {
        RootFindingOptions {
            abs_tolerance: 1e-10,
            rel_tolerance: 1e-8,
            max_iterations: 100,
            function_tolerance: 1e-12,
        }
    }
}

/// Options for optimization algorithms.
#[derive(Clone, Debug)]
pub struct OptimizationOptions {
    /// Gradient tolerance
    pub gradient_tolerance: f64,
    /// Step size tolerance
    pub step_tolerance: f64,
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Maximum number of function evaluations
    pub max_evals: usize,
    /// Line search c1 parameter (Armijo)
    pub c1: f64,
    /// Line search c2 parameter (curvature)
    pub c2: f64,
}

impl Default for OptimizationOptions {
    fn default() -> Self {
        OptimizationOptions {
            gradient_tolerance: 1e-6,
            step_tolerance: 1e-10,
            max_iterations: 1000,
            max_evals: 10000,
            c1: 1e-4,
            c2: 0.9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergence_status_eq() {
        assert_eq!(ConvergenceStatus::Converged, ConvergenceStatus::Converged);
        assert_ne!(ConvergenceStatus::Converged, ConvergenceStatus::NotConverged);
    }

    #[test]
    fn test_root_status() {
        let status = RootStatus::Found;
        assert_eq!(status, RootStatus::Found);
    }

    #[test]
    fn test_optimization_status() {
        let status = OptimizationStatus::Optimal;
        assert_eq!(status, OptimizationStatus::Optimal);
    }

    #[test]
    fn test_differentiation_options_default() {
        let opts = DifferentiationOptions::default();
        assert!(opts.step_size > 0.0);
        assert_eq!(opts.use_central, true);
    }

    #[test]
    fn test_integration_options_default() {
        let opts = IntegrationOptions::default();
        assert!(opts.abs_tolerance > 0.0);
        assert!(opts.max_evals > 0);
    }

    #[test]
    fn test_root_finding_options_default() {
        let opts = RootFindingOptions::default();
        assert!(opts.abs_tolerance > 0.0);
        assert!(opts.max_iterations > 0);
    }

    #[test]
    fn test_optimization_options_default() {
        let opts = OptimizationOptions::default();
        assert!(opts.gradient_tolerance > 0.0);
        assert!(opts.max_iterations > 0);
    }
}

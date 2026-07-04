// FILE: math_opt_glob_opt.rs
// occt: MathOpt_GlobOpt

/// Global optimization strategy selection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GlobalStrategy {
    Pso,
    MultiStart,
    PsoHybrid,
    DifferentialEvolution,
}

/// Configuration for global optimization.
pub struct GlobalConfig {
    pub strategy: GlobalStrategy,
    pub nb_population: usize,
    pub nb_starts: usize,
    pub mutation_scale: f64,
    pub crossover_prob: f64,
    pub seed: u32,
    pub polish_budget_per_dim: usize,
    pub tolerance: f64,
    pub max_iterations: usize,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            strategy: GlobalStrategy::PsoHybrid,
            nb_population: 40,
            nb_starts: 10,
            mutation_scale: 0.8,
            crossover_prob: 0.9,
            seed: 6,
            polish_budget_per_dim: 50,
            tolerance: 1.0e-8,
            max_iterations: 200,
        }
    }
}

/// Result of global optimization.
pub struct VectorResult {
    pub solution: Option<Vec<f64>>,
    pub value: Option<f64>,
    pub nb_iterations: usize,
    pub status: OptimizationStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationStatus {
    Ok,
    NotConverged,
    InvalidInput,
    NumericalError,
    MaxIterations,
}

impl VectorResult {
    pub fn is_done(&self) -> bool {
        self.status == OptimizationStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = GlobalConfig::default();
        assert_eq!(cfg.strategy, GlobalStrategy::PsoHybrid);
        assert_eq!(cfg.nb_population, 40);
        assert_eq!(cfg.tolerance, 1.0e-8);
    }

    #[test]
    fn test_result_status() {
        let mut result = VectorResult {
            solution: Some(vec![1.0, 2.0]),
            value: Some(0.5),
            nb_iterations: 100,
            status: OptimizationStatus::Ok,
        };
        assert!(result.is_done());

        result.status = OptimizationStatus::NotConverged;
        assert!(!result.is_done());
    }
}

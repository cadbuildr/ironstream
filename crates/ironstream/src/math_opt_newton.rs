// FILE: math_opt_newton.rs
// occt: MathOpt_Newton

/// Newton's method for unconstrained optimization.
pub struct NewtonOptimizer {
    tolerance: f64,
    max_iterations: usize,
}

impl NewtonOptimizer {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self { tolerance, max_iterations }
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }
}

impl Default for NewtonOptimizer {
    fn default() -> Self {
        Self::new(1.0e-8, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_newton() {
        let opt = NewtonOptimizer::default();
        assert_eq!(opt.tolerance(), 1.0e-8);
        assert_eq!(opt.max_iterations(), 100);
    }

    #[test]
    fn test_custom_newton() {
        let opt = NewtonOptimizer::new(1.0e-10, 500);
        assert_eq!(opt.tolerance(), 1.0e-10);
        assert_eq!(opt.max_iterations(), 500);
    }
}

// FILE: math_opt_bfgs.rs
// occt: MathOpt_BFGS

/// BFGS quasi-Newton optimization.
pub struct BFGS {
    tolerance: f64,
    max_iterations: usize,
}

impl BFGS {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self { tolerance, max_iterations }
    }

    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn max_iterations(&self) -> usize { self.max_iterations }
}

impl Default for BFGS {
    fn default() -> Self {
        Self::new(1.0e-8, 100)
    }
}

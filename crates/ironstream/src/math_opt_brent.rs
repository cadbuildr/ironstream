// FILE: math_opt_brent.rs
// occt: MathOpt_Brent

/// Brent's method for 1D optimization.
pub struct Brent {
    tolerance: f64,
    max_iterations: usize,
}

impl Brent {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self { tolerance, max_iterations }
    }

    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn max_iterations(&self) -> usize { self.max_iterations }
}

impl Default for Brent {
    fn default() -> Self {
        Self::new(1.0e-8, 100)
    }
}

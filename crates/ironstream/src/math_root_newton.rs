// FILE: math_root_newton.rs
// occt: MathRoot_Newton

/// Newton's method for root finding.
pub struct Newton {
    tolerance: f64,
    max_iterations: usize,
}

impl Newton {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self { tolerance, max_iterations }
    }

    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn max_iterations(&self) -> usize { self.max_iterations }
}

impl Default for Newton {
    fn default() -> Self {
        Self::new(1.0e-8, 100)
    }
}

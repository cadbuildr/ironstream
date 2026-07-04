// FILE: math_lin_svd.rs
// occt: MathLin_SVD

/// Singular Value Decomposition.
pub struct SVD {
    tolerance: f64,
    max_iterations: usize,
}

impl SVD {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self { tolerance, max_iterations }
    }

    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn max_iterations(&self) -> usize { self.max_iterations }
}

impl Default for SVD {
    fn default() -> Self {
        Self::new(1.0e-8, 100)
    }
}

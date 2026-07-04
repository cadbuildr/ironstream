// FILE: math_eigen_values_searcher.rs
// occt: math_EigenValuesSearcher

/// Eigenvalue searcher for matrices.
pub struct EigenValuesSearcher {
    tolerance: f64,
    max_iterations: usize,
}

impl EigenValuesSearcher {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self { tolerance, max_iterations }
    }

    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn max_iterations(&self) -> usize { self.max_iterations }
}

impl Default for EigenValuesSearcher {
    fn default() -> Self {
        Self::new(1.0e-8, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let s = EigenValuesSearcher::default();
        assert_eq!(s.tolerance(), 1.0e-8);
        assert_eq!(s.max_iterations(), 100);
    }
}

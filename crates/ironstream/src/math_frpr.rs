// FILE: math_frpr.rs
// occt: math_FRPR

/// Fletcher-Reeves Powell-Reeves conjugate gradient optimization.
pub struct FRPR {
    tolerance: f64,
    max_iterations: usize,
}

impl FRPR {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self { tolerance, max_iterations }
    }

    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn max_iterations(&self) -> usize { self.max_iterations }
}

impl Default for FRPR {
    fn default() -> Self {
        Self::new(1.0e-8, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let f = FRPR::default();
        assert_eq!(f.tolerance(), 1.0e-8);
    }
}

// FILE: math_root_multiple.rs
// occt: MathRoot_Multiple

/// Root finding for multiple roots.
pub struct MultipleRoot {
    tolerance: f64,
    max_iterations: usize,
}

impl MultipleRoot {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self { tolerance, max_iterations }
    }

    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn max_iterations(&self) -> usize { self.max_iterations }
}

impl Default for MultipleRoot {
    fn default() -> Self {
        Self::new(1.0e-8, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let mr = MultipleRoot::default();
        assert_eq!(mr.tolerance(), 1.0e-8);
        assert_eq!(mr.max_iterations(), 100);
    }

    #[test]
    fn test_custom() {
        let mr = MultipleRoot::new(1.0e-12, 200);
        assert_eq!(mr.tolerance(), 1.0e-12);
        assert_eq!(mr.max_iterations(), 200);
    }
}

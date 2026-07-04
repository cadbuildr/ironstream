// FILE: math_function_all_roots.rs
// occt: math_FunctionAllRoots

/// Finder of all roots for a single-variable function.
pub struct FunctionAllRoots {
    tolerance: f64,
    max_iterations: usize,
}

impl FunctionAllRoots {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self { tolerance, max_iterations }
    }

    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn max_iterations(&self) -> usize { self.max_iterations }
}

impl Default for FunctionAllRoots {
    fn default() -> Self {
        Self::new(1.0e-8, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let f = FunctionAllRoots::default();
        assert_eq!(f.tolerance(), 1.0e-8);
    }
}

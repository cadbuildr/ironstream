// FILE: math_function_root.rs
// occt: math_FunctionRoot

/// Root finder for single variable function.
pub struct FunctionRoot {
    tolerance: f64,
    max_iterations: usize,
}

impl FunctionRoot {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self { tolerance, max_iterations }
    }

    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn max_iterations(&self) -> usize { self.max_iterations }
}

impl Default for FunctionRoot {
    fn default() -> Self {
        Self::new(1.0e-8, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let f = FunctionRoot::default();
        assert_eq!(f.tolerance(), 1.0e-8);
    }
}

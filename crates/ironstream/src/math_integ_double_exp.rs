// FILE: math_integ_double_exp.rs
// occt: MathInteg_DoubleExp

/// Double exponential integration method.
pub struct DoubleExpIntegration {
    tolerance: f64,
    max_intervals: usize,
}

impl DoubleExpIntegration {
    pub fn new(tolerance: f64, max_intervals: usize) -> Self {
        Self { tolerance, max_intervals }
    }

    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn max_intervals(&self) -> usize { self.max_intervals }
}

impl Default for DoubleExpIntegration {
    fn default() -> Self {
        Self::new(1.0e-10, 1000)
    }
}

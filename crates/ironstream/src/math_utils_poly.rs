// FILE: math_utils_poly.rs
// occt: MathUtils_Poly

/// Polynomial utility functions.
pub struct Poly;

impl Poly {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(coeffs: &[f64], x: f64) -> f64 {
        let mut result = 0.0;
        for (i, &coeff) in coeffs.iter().enumerate() {
            result = result * x + coeff;
        }
        result
    }
}

impl Default for Poly {
    fn default() -> Self {
        Self::new()
    }
}

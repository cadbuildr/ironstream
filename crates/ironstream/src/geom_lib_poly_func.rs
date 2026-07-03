// FILE: geom_lib_poly_func.rs
// occt: GeomLib_PolyFunc

/// Polynomial function for geometric algorithms.
pub struct GeomLibPolyFunc {
    coefficients: Vec<f64>,
    degree: usize,
}

impl GeomLibPolyFunc {
    /// Create a polynomial with given coefficients.
    pub fn new(coeffs: Vec<f64>) -> Self {
        let degree = if coeffs.is_empty() { 0 } else { coeffs.len() - 1 };
        GeomLibPolyFunc {
            coefficients: coeffs,
            degree,
        }
    }

    /// Evaluate the polynomial at x.
    pub fn evaluate(&self, x: f64) -> f64 {
        let mut result = 0.0;
        let mut x_pow = 1.0;
        for coeff in &self.coefficients {
            result += coeff * x_pow;
            x_pow *= x;
        }
        result
    }

    /// Get the degree.
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// Get a coefficient.
    pub fn coefficient(&self, index: usize) -> Option<f64> {
        if index < self.coefficients.len() {
            Some(self.coefficients[index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_polynomial() {
        let poly = GeomLibPolyFunc::new(vec![5.0]);
        assert_eq!(poly.evaluate(0.0), 5.0);
        assert_eq!(poly.evaluate(10.0), 5.0);
        assert_eq!(poly.degree(), 0);
    }

    #[test]
    fn test_linear_polynomial() {
        // 2 + 3x
        let poly = GeomLibPolyFunc::new(vec![2.0, 3.0]);
        assert_eq!(poly.evaluate(0.0), 2.0);
        assert_eq!(poly.evaluate(1.0), 5.0);
        assert_eq!(poly.evaluate(2.0), 8.0);
        assert_eq!(poly.degree(), 1);
    }

    #[test]
    fn test_quadratic_polynomial() {
        // 1 + 2x + x^2
        let poly = GeomLibPolyFunc::new(vec![1.0, 2.0, 1.0]);
        assert_eq!(poly.evaluate(0.0), 1.0);
        assert_eq!(poly.evaluate(1.0), 4.0);
        assert_eq!(poly.evaluate(2.0), 9.0);
        assert_eq!(poly.degree(), 2);
    }

    #[test]
    fn test_coefficient_access() {
        let poly = GeomLibPolyFunc::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(poly.coefficient(0), Some(1.0));
        assert_eq!(poly.coefficient(1), Some(2.0));
        assert_eq!(poly.coefficient(3), None);
    }
}

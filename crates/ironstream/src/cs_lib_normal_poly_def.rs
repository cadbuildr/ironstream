// FILE: cs_lib_normal_poly_def.rs
// occt: CSLib_NormalPolyDef

//! Polynomial definition for surface normal computation at singular points.
//!
//! This class defines a polynomial function F(X) and its derivative for use
//! with numerical root-finding algorithms. The function represents a trigonometric
//! polynomial in terms of cos(X) and sin(X) with binomial coefficients.
//!
//! The polynomial has the form:
//!   F(X) = Sum_{i=0}^{k0} C(k0,i) * cos^i(X) * sin^(k0-i)(X) * li(i)
//!
//! where C(k0,i) is the binomial coefficient and li(i) are user-provided coefficients.
//!
//! This is used internally by CSLib::Normal() to find the normal direction
//! at singular surface points by solving for zeros of this polynomial.

/// Computes binomial coefficient C(n, k)
fn binomial(n: i32, k: i32) -> f64 {
    if k < 0 || k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }
    let k = k.min(n - k);
    let mut result = 1.0;
    for i in 0..k {
        result *= (n - i) as f64 / (i + 1) as f64;
    }
    result
}

pub struct CSLibNormalPolyDef {
    k0: i32,
    li: Vec<f64>,
}

impl CSLibNormalPolyDef {
    /// Constructs a polynomial definition for normal computation.
    ///
    /// # Arguments
    /// * `k0` - Polynomial degree (must be >= 0)
    /// * `li` - Array of coefficients with indices 0 to k0 (inclusive)
    ///
    /// # Panics
    /// Panics if `li.len()` != `k0 + 1`
    pub fn new(k0: i32, li: Vec<f64>) -> Self {
        assert!(
            li.len() == (k0 + 1) as usize,
            "li array length must be k0 + 1"
        );
        CSLibNormalPolyDef { k0, li }
    }

    /// Computes the value of the function for the given variable.
    ///
    /// Evaluates F(X) = Sum_{i=0}^{k0} C(k0,i) * cos^i(X) * sin^(k0-i)(X) * li(i)
    ///
    /// # Arguments
    /// * `x` - Input variable (angle in radians)
    ///
    /// # Returns
    /// Computed function value
    pub fn value(&self, x: f64) -> f64 {
        let cos_x = x.cos();
        let sin_x = x.sin();

        // At singular points (cos or sin near zero), return zero to avoid numerical instability.
        // RealSmall() in OCCT is approximately f64::MIN_POSITIVE
        const REAL_SMALL: f64 = 1.0e-20;
        if cos_x.abs() <= REAL_SMALL || sin_x.abs() <= REAL_SMALL {
            return 0.0;
        }

        let mut f = 0.0;
        for i in 0..=self.k0 {
            let i_usize = i as usize;
            let bin_coeff = binomial(self.k0, i);
            let cos_power = cos_x.powi(i);
            let sin_power = sin_x.powi(self.k0 - i);
            f += bin_coeff * cos_power * sin_power * self.li[i_usize];
        }

        f
    }

    /// Computes the derivative of the function for the given variable.
    ///
    /// Evaluates dF/dX using the chain rule on the trigonometric polynomial.
    ///
    /// # Arguments
    /// * `x` - Input variable (angle in radians)
    ///
    /// # Returns
    /// Computed derivative value
    pub fn derivative(&self, x: f64) -> f64 {
        let cos_x = x.cos();
        let sin_x = x.sin();

        // At singular points, return zero derivative.
        const REAL_SMALL: f64 = 1.0e-20;
        if cos_x.abs() <= REAL_SMALL || sin_x.abs() <= REAL_SMALL {
            return 0.0;
        }

        // Evaluate the derivative using the chain rule.
        // dF/dX = Sum_{i=0}^{k0} C(k0,i) * cos^(i-1)(X) * sin^(k0-i-1)(X) * (k0*cos^2(X) - i) * li(i)
        let mut d = 0.0;
        for i in 0..=self.k0 {
            let i_usize = i as usize;
            let bin_coeff = binomial(self.k0, i);
            let cos_power = cos_x.powi(i - 1);
            let sin_power = sin_x.powi(self.k0 - i - 1);
            let factor = (self.k0 as f64) * cos_x * cos_x - (i as f64);
            d += bin_coeff * cos_power * sin_power * factor * self.li[i_usize];
        }

        d
    }

    /// Computes both the value and derivative of the function.
    ///
    /// More efficient than calling value() and derivative() separately
    /// as common subexpressions are computed only once.
    ///
    /// # Arguments
    /// * `x` - Input variable (angle in radians)
    ///
    /// # Returns
    /// Tuple of (function value, derivative value)
    pub fn values(&self, x: f64) -> (f64, f64) {
        let cos_x = x.cos();
        let sin_x = x.sin();

        // At singular points, return zeros.
        const REAL_SMALL: f64 = 1.0e-20;
        if cos_x.abs() <= REAL_SMALL || sin_x.abs() <= REAL_SMALL {
            return (0.0, 0.0);
        }

        let mut f = 0.0;
        let mut d = 0.0;

        // Compute both function value and derivative in a single loop for efficiency.
        for i in 0..=self.k0 {
            let i_usize = i as usize;
            let bin_coeff = binomial(self.k0, i);
            let li_coeff = self.li[i_usize];

            let cos_power = cos_x.powi(i);
            let sin_power = sin_x.powi(self.k0 - i);

            f += bin_coeff * cos_power * sin_power * li_coeff;

            let cos_power_d = cos_x.powi(i - 1);
            let sin_power_d = sin_x.powi(self.k0 - i - 1);
            let factor = (self.k0 as f64) * cos_x * cos_x - (i as f64);
            d += bin_coeff * cos_power_d * sin_power_d * factor * li_coeff;
        }

        (f, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_coefficients() {
        assert_eq!(binomial(0, 0), 1.0);
        assert_eq!(binomial(1, 0), 1.0);
        assert_eq!(binomial(1, 1), 1.0);
        assert_eq!(binomial(2, 0), 1.0);
        assert_eq!(binomial(2, 1), 2.0);
        assert_eq!(binomial(2, 2), 1.0);
        assert_eq!(binomial(4, 2), 6.0);
        assert_eq!(binomial(5, 3), 10.0);
    }

    #[test]
    fn test_normal_poly_def_construction() {
        let coeffs = vec![1.0, 2.0, 3.0];
        let poly = CSLibNormalPolyDef::new(2, coeffs);
        assert_eq!(poly.k0, 2);
        assert_eq!(poly.li.len(), 3);
    }

    #[test]
    fn test_value_at_zero() {
        // F(0) with k0=1, li=[1, 1]
        // F(0) = C(1,0)*cos^0(0)*sin^1(0)*1 + C(1,1)*cos^1(0)*sin^0(0)*1
        //      = 1*1*0*1 + 1*1*1*1 = 1
        let coeffs = vec![1.0, 1.0];
        let poly = CSLibNormalPolyDef::new(1, coeffs);
        let val = poly.value(0.0);
        // At theta=0, sin(0)=0, so we hit the singular point guard and return 0
        assert!(val.abs() < 1e-10);
    }

    #[test]
    fn test_value_at_pi_quarter() {
        // F(pi/4) with k0=1, li=[1, 1]
        // At pi/4: cos(pi/4) = sin(pi/4) = sqrt(2)/2
        let coeffs = vec![1.0, 1.0];
        let poly = CSLibNormalPolyDef::new(1, coeffs);
        let val = poly.value(std::f64::consts::PI / 4.0);
        // F(pi/4) = C(1,0)*1*sin(pi/4)*1 + C(1,1)*cos(pi/4)*1*1
        //         = 1 * sqrt(2)/2 + 1 * sqrt(2)/2 = sqrt(2)
        let expected = 2.0 * (std::f64::consts::PI / 4.0).cos();
        assert!((val - expected).abs() < 1e-10);
    }

    #[test]
    fn test_derivative_consistency() {
        let coeffs = vec![1.0, 2.0, 1.0];
        let poly = CSLibNormalPolyDef::new(2, coeffs);

        let x = 0.5;
        let (f1, d1) = poly.values(x);
        let f2 = poly.value(x);
        let d2 = poly.derivative(x);

        // values() and separate calls should give same results
        assert!((f1 - f2).abs() < 1e-14);
        assert!((d1 - d2).abs() < 1e-14);
    }

    #[test]
    fn test_derivative_numerical() {
        let coeffs = vec![1.0, 0.5];
        let poly = CSLibNormalPolyDef::new(1, coeffs);

        let x = 0.3;
        let dx = 1e-8;
        let f_left = poly.value(x - dx);
        let f_right = poly.value(x + dx);
        let numerical_deriv = (f_right - f_left) / (2.0 * dx);

        let analytical_deriv = poly.derivative(x);

        // Numerical derivative should approximate analytical within reasonable tolerance
        assert!((numerical_deriv - analytical_deriv).abs() < 1e-6);
    }

    #[test]
    fn test_symmetric_coefficients() {
        // Test with k0=2, symmetric coefficients [1, 1, 1]
        let coeffs = vec![1.0, 1.0, 1.0];
        let poly = CSLibNormalPolyDef::new(2, coeffs);

        // By symmetry, F(x) + F(pi/2 - x) should exhibit certain properties
        let x1 = 0.2;
        let x2 = std::f64::consts::PI / 2.0 - x1;

        let f1 = poly.value(x1);
        let f2 = poly.value(x2);

        // Both should be computed without error
        assert!(f1.is_finite());
        assert!(f2.is_finite());
    }

    #[test]
    fn test_zero_coefficients() {
        let coeffs = vec![0.0, 0.0, 0.0];
        let poly = CSLibNormalPolyDef::new(2, coeffs);

        let x = 1.0;
        let f = poly.value(x);
        let d = poly.derivative(x);
        let (f2, d2) = poly.values(x);

        assert_eq!(f, 0.0);
        assert_eq!(d, 0.0);
        assert_eq!(f2, 0.0);
        assert_eq!(d2, 0.0);
    }

    #[test]
    #[should_panic(expected = "li array length must be k0 + 1")]
    fn test_invalid_length() {
        let coeffs = vec![1.0, 2.0];
        let _ = CSLibNormalPolyDef::new(3, coeffs); // k0=3 requires 4 coeffs
    }
}

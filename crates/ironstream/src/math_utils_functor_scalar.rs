// FILE: math_utils_functor_scalar.rs

//! Non-virtual functor classes for scalar (1D) functions.
//!
//! Provides ready-to-use functor classes that work with the template-based
//! math API without virtual dispatch overhead.

// occt-ref: MathUtils_ScalarLambda
/// Lambda wrapper for scalar functions with value only.
/// Wraps a lambda/callable into a functor with Value() method.
pub struct ScalarLambda<F>
where
    F: Fn(f64) -> Option<f64>,
{
    lambda: F,
}

impl<F> ScalarLambda<F>
where
    F: Fn(f64) -> Option<f64>,
{
    /// Constructor from lambda/callable.
    pub fn new(lambda: F) -> Self {
        ScalarLambda { lambda }
    }

    /// Evaluates the function at x.
    /// Returns Some(y) if evaluation succeeded, None otherwise.
    pub fn value(&self, x: f64) -> Option<f64> {
        (self.lambda)(x)
    }
}

// occt-ref: MathUtils_ScalarLambdaWithDerivative
/// Lambda wrapper for scalar functions with value and derivative.
/// Wraps a lambda/callable into a functor with Values() method.
pub struct ScalarLambdaWithDerivative<F>
where
    F: Fn(f64) -> Option<(f64, f64)>,
{
    lambda: F,
}

impl<F> ScalarLambdaWithDerivative<F>
where
    F: Fn(f64) -> Option<(f64, f64)>,
{
    /// Constructor from lambda/callable.
    pub fn new(lambda: F) -> Self {
        ScalarLambdaWithDerivative { lambda }
    }

    /// Evaluates function and derivative at x.
    /// Returns Some((y, dy)) if evaluation succeeded, None otherwise.
    pub fn values(&self, x: f64) -> Option<(f64, f64)> {
        (self.lambda)(x)
    }

    /// Evaluates only the function value (for algorithms that don't need derivative).
    pub fn value(&self, x: f64) -> Option<f64> {
        (self.lambda)(x).map(|(y, _)| y)
    }
}

// occt-ref: MathUtils_Polynomial
/// Polynomial functor: f(x) = sum(a[i] * x^i).
/// Coefficients are stored in order: a[0] + a[1]*x + a[2]*x^2 + ...
#[derive(Clone)]
pub struct Polynomial {
    coeffs: Vec<f64>,
}

impl Polynomial {
    /// Constructor from a slice of coefficients (ascending power order).
    pub fn new(coeffs: &[f64]) -> Self {
        Polynomial {
            coeffs: coeffs.to_vec(),
        }
    }

    /// Evaluates polynomial at x using Horner's method.
    pub fn value(&self, x: f64) -> f64 {
        if self.coeffs.is_empty() {
            return 0.0;
        }

        // Horner's method: p(x) = a[0] + x*(a[1] + x*(a[2] + ...))
        let mut result = self.coeffs[self.coeffs.len() - 1];
        for i in (0..self.coeffs.len() - 1).rev() {
            result = result * x + self.coeffs[i];
        }
        result
    }

    /// Evaluates polynomial and its derivative at x.
    pub fn values(&self, x: f64) -> (f64, f64) {
        if self.coeffs.is_empty() {
            return (0.0, 0.0);
        }

        if self.coeffs.len() == 1 {
            return (self.coeffs[0], 0.0);
        }

        // Horner's method for value and derivative simultaneously
        let mut y = self.coeffs[self.coeffs.len() - 1];
        let mut dy = 0.0;

        for i in (1..self.coeffs.len()).rev() {
            dy = dy * x + y;
            y = y * x + self.coeffs[i - 1];
        }

        (y, dy)
    }

    /// Returns the degree of the polynomial.
    pub fn degree(&self) -> usize {
        if self.coeffs.is_empty() {
            0
        } else {
            self.coeffs.len() - 1
        }
    }

    /// Returns coefficient by index (0 = constant term).
    pub fn coefficient(&self, index: usize) -> f64 {
        if index < self.coeffs.len() {
            self.coeffs[index]
        } else {
            0.0
        }
    }
}

// occt-ref: MathUtils_Rational
/// Rational function functor: f(x) = P(x) / Q(x).
/// Both numerator P and denominator Q are polynomials.
#[derive(Clone)]
pub struct Rational {
    numerator: Polynomial,
    denominator: Polynomial,
}

impl Rational {
    /// Constructor from numerator and denominator coefficient arrays.
    pub fn new(num_coeffs: &[f64], denom_coeffs: &[f64]) -> Self {
        Rational {
            numerator: Polynomial::new(num_coeffs),
            denominator: Polynomial::new(denom_coeffs),
        }
    }

    /// Evaluates rational function at x.
    /// Returns Some(y) if denominator is nonzero, None otherwise.
    pub fn value(&self, x: f64) -> Option<f64> {
        let num = self.numerator.value(x);
        let denom = self.denominator.value(x);

        if denom.abs() < 1e-15 {
            None
        } else {
            Some(num / denom)
        }
    }
}

// occt-ref: MathUtils_Composite
/// Composite functor: f(g(x)).
/// Evaluates the outer function at the result of the inner function.
pub struct Composite<Outer, Inner> {
    outer: Outer,
    inner: Inner,
}

impl<Outer, Inner> Composite<Outer, Inner>
where
    Outer: Fn(f64) -> Option<f64>,
    Inner: Fn(f64) -> Option<f64>,
{
    /// Constructor from outer and inner functions.
    pub fn new(outer: Outer, inner: Inner) -> Self {
        Composite { outer, inner }
    }

    /// Evaluates composite function f(g(x)).
    pub fn value(&self, x: f64) -> Option<f64> {
        let inner_result = (self.inner)(x)?;
        (self.outer)(inner_result)
    }
}

// occt-ref: MathUtils_Sum
/// Sum of functions functor: f(x) + g(x).
pub struct Sum<F, G> {
    f: F,
    g: G,
}

impl<F, G> Sum<F, G>
where
    F: Fn(f64) -> Option<f64>,
    G: Fn(f64) -> Option<f64>,
{
    /// Constructor from two functions.
    pub fn new(f: F, g: G) -> Self {
        Sum { f, g }
    }

    /// Evaluates sum f(x) + g(x).
    pub fn value(&self, x: f64) -> Option<f64> {
        let f_val = (self.f)(x)?;
        let g_val = (self.g)(x)?;
        Some(f_val + g_val)
    }
}

// occt-ref: MathUtils_Difference
/// Difference of functions functor: f(x) - g(x).
pub struct Difference<F, G> {
    f: F,
    g: G,
}

impl<F, G> Difference<F, G>
where
    F: Fn(f64) -> Option<f64>,
    G: Fn(f64) -> Option<f64>,
{
    /// Constructor from two functions.
    pub fn new(f: F, g: G) -> Self {
        Difference { f, g }
    }

    /// Evaluates difference f(x) - g(x).
    pub fn value(&self, x: f64) -> Option<f64> {
        let f_val = (self.f)(x)?;
        let g_val = (self.g)(x)?;
        Some(f_val - g_val)
    }
}

// occt-ref: MathUtils_Product
/// Product of functions functor: f(x) * g(x).
pub struct Product<F, G> {
    f: F,
    g: G,
}

impl<F, G> Product<F, G>
where
    F: Fn(f64) -> Option<f64>,
    G: Fn(f64) -> Option<f64>,
{
    /// Constructor from two functions.
    pub fn new(f: F, g: G) -> Self {
        Product { f, g }
    }

    /// Evaluates product f(x) * g(x).
    pub fn value(&self, x: f64) -> Option<f64> {
        let f_val = (self.f)(x)?;
        let g_val = (self.g)(x)?;
        Some(f_val * g_val)
    }
}

// occt-ref: MathUtils_Quotient
/// Quotient of functions functor: f(x) / g(x).
pub struct Quotient<F, G> {
    f: F,
    g: G,
}

impl<F, G> Quotient<F, G>
where
    F: Fn(f64) -> Option<f64>,
    G: Fn(f64) -> Option<f64>,
{
    /// Constructor from two functions.
    pub fn new(f: F, g: G) -> Self {
        Quotient { f, g }
    }

    /// Evaluates quotient f(x) / g(x).
    /// Returns None if evaluation fails or denominator is zero.
    pub fn value(&self, x: f64) -> Option<f64> {
        let f_val = (self.f)(x)?;
        let g_val = (self.g)(x)?;

        if g_val.abs() < 1e-15 {
            None
        } else {
            Some(f_val / g_val)
        }
    }
}

// occt-ref: MathUtils_Scaled
/// Scaled function functor: c * f(x).
pub struct Scaled<F> {
    f: F,
    scale: f64,
}

impl<F> Scaled<F>
where
    F: Fn(f64) -> Option<f64>,
{
    /// Constructor from function and scale factor.
    pub fn new(f: F, scale: f64) -> Self {
        Scaled { f, scale }
    }

    /// Evaluates scaled function c * f(x).
    pub fn value(&self, x: f64) -> Option<f64> {
        (self.f)(x).map(|y| self.scale * y)
    }
}

// occt-ref: MathUtils_Shifted
/// Shifted function functor: f(x) + c.
pub struct Shifted<F> {
    f: F,
    shift: f64,
}

impl<F> Shifted<F>
where
    F: Fn(f64) -> Option<f64>,
{
    /// Constructor from function and shift value.
    pub fn new(f: F, shift: f64) -> Self {
        Shifted { f, shift }
    }

    /// Evaluates shifted function f(x) + c.
    pub fn value(&self, x: f64) -> Option<f64> {
        (self.f)(x).map(|y| y + self.shift)
    }
}

// occt-ref: MathUtils_Negated
/// Negated function functor: -f(x).
pub struct Negated<F> {
    f: F,
}

impl<F> Negated<F>
where
    F: Fn(f64) -> Option<f64>,
{
    /// Constructor from function.
    pub fn new(f: F) -> Self {
        Negated { f }
    }

    /// Evaluates negated function -f(x).
    pub fn value(&self, x: f64) -> Option<f64> {
        (self.f)(x).map(|y| -y)
    }
}

// occt-ref: MathUtils_Constant
/// Constant function functor: f(x) = c.
#[derive(Clone, Copy)]
pub struct Constant {
    value: f64,
}

impl Constant {
    /// Constructor from constant value.
    pub fn new(value: f64) -> Self {
        Constant { value }
    }

    /// Evaluates constant function.
    pub fn value(&self, _x: f64) -> f64 {
        self.value
    }

    /// Evaluates constant and derivative (derivative is always 0).
    pub fn values(&self, _x: f64) -> (f64, f64) {
        (self.value, 0.0)
    }
}

// occt-ref: MathUtils_Linear
/// Linear function functor: f(x) = a*x + b.
#[derive(Clone, Copy)]
pub struct Linear {
    slope: f64,
    intercept: f64,
}

impl Linear {
    /// Constructor from slope and intercept.
    pub fn new(slope: f64, intercept: f64) -> Self {
        Linear { slope, intercept }
    }

    /// Evaluates linear function a*x + b.
    pub fn value(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }

    /// Evaluates linear function and derivative.
    pub fn values(&self, x: f64) -> (f64, f64) {
        (self.slope * x + self.intercept, self.slope)
    }
}

// occt-ref: MathUtils_Sine
/// Sine function functor: f(x) = a * sin(b*x + c) + d.
#[derive(Clone, Copy)]
pub struct Sine {
    amplitude: f64,
    frequency: f64,
    phase: f64,
    offset: f64,
}

impl Sine {
    /// Constructor with full parameters.
    pub fn new(
        amplitude: f64,
        frequency: f64,
        phase: f64,
        offset: f64,
    ) -> Self {
        Sine {
            amplitude,
            frequency,
            phase,
            offset,
        }
    }

    /// Default constructor (amplitude=1, frequency=1, phase=0, offset=0).
    pub fn default() -> Self {
        Sine {
            amplitude: 1.0,
            frequency: 1.0,
            phase: 0.0,
            offset: 0.0,
        }
    }

    /// Evaluates sine function.
    pub fn value(&self, x: f64) -> f64 {
        self.amplitude * (self.frequency * x + self.phase).sin() + self.offset
    }

    /// Evaluates sine function and derivative.
    pub fn values(&self, x: f64) -> (f64, f64) {
        let arg = self.frequency * x + self.phase;
        let y = self.amplitude * arg.sin() + self.offset;
        let dy = self.amplitude * self.frequency * arg.cos();
        (y, dy)
    }
}

// occt-ref: MathUtils_Cosine
/// Cosine function functor: f(x) = a * cos(b*x + c) + d.
#[derive(Clone, Copy)]
pub struct Cosine {
    amplitude: f64,
    frequency: f64,
    phase: f64,
    offset: f64,
}

impl Cosine {
    /// Constructor with full parameters.
    pub fn new(
        amplitude: f64,
        frequency: f64,
        phase: f64,
        offset: f64,
    ) -> Self {
        Cosine {
            amplitude,
            frequency,
            phase,
            offset,
        }
    }

    /// Default constructor.
    pub fn default() -> Self {
        Cosine {
            amplitude: 1.0,
            frequency: 1.0,
            phase: 0.0,
            offset: 0.0,
        }
    }

    /// Evaluates cosine function.
    pub fn value(&self, x: f64) -> f64 {
        self.amplitude * (self.frequency * x + self.phase).cos() + self.offset
    }

    /// Evaluates cosine function and derivative.
    pub fn values(&self, x: f64) -> (f64, f64) {
        let arg = self.frequency * x + self.phase;
        let y = self.amplitude * arg.cos() + self.offset;
        let dy = -self.amplitude * self.frequency * arg.sin();
        (y, dy)
    }
}

// occt-ref: MathUtils_Exponential
/// Exponential function functor: f(x) = a * exp(b*x) + c.
#[derive(Clone, Copy)]
pub struct Exponential {
    scale: f64,
    rate: f64,
    offset: f64,
}

impl Exponential {
    /// Constructor with full parameters.
    pub fn new(scale: f64, rate: f64, offset: f64) -> Self {
        Exponential {
            scale,
            rate,
            offset,
        }
    }

    /// Default constructor.
    pub fn default() -> Self {
        Exponential {
            scale: 1.0,
            rate: 1.0,
            offset: 0.0,
        }
    }

    /// Evaluates exponential function.
    pub fn value(&self, x: f64) -> f64 {
        self.scale * (self.rate * x).exp() + self.offset
    }

    /// Evaluates exponential function and derivative.
    pub fn values(&self, x: f64) -> (f64, f64) {
        let exp_val = (self.rate * x).exp();
        let y = self.scale * exp_val + self.offset;
        let dy = self.scale * self.rate * exp_val;
        (y, dy)
    }
}

// occt-ref: MathUtils_Power
/// Power function functor: f(x) = a * x^n + b.
#[derive(Clone, Copy)]
pub struct Power {
    exponent: f64,
    scale: f64,
    offset: f64,
}

impl Power {
    /// Constructor with full parameters.
    pub fn new(exponent: f64, scale: f64, offset: f64) -> Self {
        Power {
            exponent,
            scale,
            offset,
        }
    }

    /// Evaluates power function.
    /// Returns None if x < 0 and exponent is non-integer.
    pub fn value(&self, x: f64) -> Option<f64> {
        if x < 0.0 && self.exponent != self.exponent.floor() {
            return None;
        }
        Some(self.scale * x.powf(self.exponent) + self.offset)
    }

    /// Evaluates power function and derivative.
    /// Returns None if x < 0 and exponent is non-integer.
    pub fn values(&self, x: f64) -> Option<(f64, f64)> {
        if x < 0.0 && self.exponent != self.exponent.floor() {
            return None;
        }

        let pow_val = x.powf(self.exponent);
        let y = self.scale * pow_val + self.offset;

        let dy = if x.abs() < 1e-15 {
            if (self.exponent - 1.0).abs() < 1e-10 {
                self.scale
            } else {
                0.0
            }
        } else {
            self.scale * self.exponent * pow_val / x
        };

        Some((y, dy))
    }
}

// occt-ref: MathUtils_Gaussian
/// Gaussian function functor: f(x) = a * exp(-((x-mu)^2)/(2*sigma^2)).
#[derive(Clone, Copy)]
pub struct Gaussian {
    amplitude: f64,
    mean: f64,
    sigma: f64,
}

impl Gaussian {
    /// Constructor with full parameters.
    pub fn new(amplitude: f64, mean: f64, sigma: f64) -> Self {
        Gaussian {
            amplitude,
            mean,
            sigma,
        }
    }

    /// Default constructor.
    pub fn default() -> Self {
        Gaussian {
            amplitude: 1.0,
            mean: 0.0,
            sigma: 1.0,
        }
    }

    /// Evaluates Gaussian function.
    /// Returns None if sigma is zero.
    pub fn value(&self, x: f64) -> Option<f64> {
        if self.sigma.abs() < 1e-15 {
            return None;
        }

        let z = (x - self.mean) / self.sigma;
        Some(self.amplitude * (-0.5 * z * z).exp())
    }

    /// Evaluates Gaussian function and derivative.
    /// Returns None if sigma is zero.
    pub fn values(&self, x: f64) -> Option<(f64, f64)> {
        if self.sigma.abs() < 1e-15 {
            return None;
        }

        let z = (x - self.mean) / self.sigma;
        let exp_val = (-0.5 * z * z).exp();
        let y = self.amplitude * exp_val;
        let dy = -self.amplitude * z * exp_val / self.sigma;

        Some((y, dy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant() {
        let c = Constant::new(5.0);
        assert_eq!(c.value(0.0), 5.0);
        assert_eq!(c.value(10.0), 5.0);
        let (y, dy) = c.values(3.0);
        assert_eq!(y, 5.0);
        assert_eq!(dy, 0.0);
    }

    #[test]
    fn test_linear() {
        let lin = Linear::new(2.0, 3.0);
        assert_eq!(lin.value(0.0), 3.0);
        assert_eq!(lin.value(1.0), 5.0);
        assert_eq!(lin.value(2.0), 7.0);

        let (y, dy) = lin.values(5.0);
        assert_eq!(y, 13.0);
        assert_eq!(dy, 2.0);
    }

    #[test]
    fn test_polynomial_quadratic() {
        // f(x) = x^2 - 2
        let poly = Polynomial::new(&[-2.0, 0.0, 1.0]);
        assert_eq!(poly.degree(), 2);

        // At x=0: f(0)=−2
        assert_eq!(poly.value(0.0), -2.0);

        // At x=2: f(2)=4−2=2
        assert_eq!(poly.value(2.0), 2.0);

        // sqrt(2) ≈ 1.414...
        let val = poly.value(1.414213562373095);
        assert!((val).abs() < 0.01);
    }

    #[test]
    fn test_polynomial_derivative() {
        // f(x) = x^3 (so f'(x) = 3x^2)
        let poly = Polynomial::new(&[0.0, 0.0, 0.0, 1.0]);
        let (y, dy) = poly.values(2.0);
        assert_eq!(y, 8.0);
        assert!((dy - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_sine() {
        let s = Sine::default();
        let pi = std::f64::consts::PI;

        // sin(0) = 0
        assert!((s.value(0.0)).abs() < 1e-10);

        // sin(pi/2) = 1
        assert!((s.value(pi / 2.0) - 1.0).abs() < 1e-10);

        // sin(pi) = 0
        assert!((s.value(pi)).abs() < 1e-10);
    }

    #[test]
    fn test_sine_with_derivative() {
        let s = Sine::new(1.0, 1.0, 0.0, 0.0);
        let (y, dy) = s.values(0.0);
        assert!((y).abs() < 1e-10);
        assert!((dy - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cosine() {
        let c = Cosine::default();
        let pi = std::f64::consts::PI;

        // cos(0) = 1
        assert!((c.value(0.0) - 1.0).abs() < 1e-10);

        // cos(pi/2) = 0
        assert!((c.value(pi / 2.0)).abs() < 1e-10);

        // cos(pi) = -1
        assert!((c.value(pi) + 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_exponential() {
        let exp = Exponential::default();

        // e^0 = 1
        assert!((exp.value(0.0) - 1.0).abs() < 1e-10);

        // e^1 ≈ 2.71828
        assert!((exp.value(1.0) - std::f64::consts::E).abs() < 1e-10);
    }

    #[test]
    fn test_power() {
        let pow = Power::new(2.0, 1.0, 0.0);

        assert_eq!(pow.value(0.0), Some(0.0));
        assert_eq!(pow.value(2.0), Some(4.0));
        assert_eq!(pow.value(3.0), Some(9.0));
    }

    #[test]
    fn test_power_with_negative_base_and_non_integer_exponent() {
        let pow = Power::new(0.5, 1.0, 0.0);

        // Should handle x >= 0
        assert!(pow.value(1.0).is_some());

        // Should reject x < 0 with non-integer exponent
        assert!(pow.value(-1.0).is_none());
    }

    #[test]
    fn test_gaussian() {
        let gauss = Gaussian::default();

        // Peak at x = mean
        let y_peak = gauss.value(0.0).unwrap();
        assert!((y_peak - 1.0).abs() < 1e-10);

        // Symmetric
        let y_pos = gauss.value(1.0).unwrap();
        let y_neg = gauss.value(-1.0).unwrap();
        assert!((y_pos - y_neg).abs() < 1e-10);
    }

    #[test]
    fn test_gaussian_zero_sigma_fails() {
        let gauss = Gaussian::new(1.0, 0.0, 0.0);
        assert!(gauss.value(0.0).is_none());
    }

    #[test]
    fn test_rational() {
        // (x + 1) / (x^2 + 1)
        let rat = Rational::new(&[1.0, 1.0], &[1.0, 0.0, 1.0]);

        // At x=0: (1) / (1) = 1
        assert_eq!(rat.value(0.0), Some(1.0));

        // At x=1: (2) / (2) = 1
        assert_eq!(rat.value(1.0), Some(1.0));
    }
}

// FILE: math_utils_functor_scalar_o.rs
// occt: MathUtils_FunctorScalar

//! Non-virtual functor classes for scalar (1D) functions.
//! Provides ready-to-use functor classes that work with template-based
//! math API without virtual dispatch overhead.

/// Wrapper for a scalar function with only value evaluation.
/// Wraps a callable into a functor with value() method.
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
    /// Constructor from callable.
    pub fn new(lambda: F) -> Self {
        ScalarLambda { lambda }
    }

    /// Evaluates the function at x.
    pub fn value(&self, x: f64) -> Option<f64> {
        (self.lambda)(x)
    }
}

/// Wrapper for a scalar function with value and derivative.
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
    /// Constructor from callable with signature bool(f64) -> Option<(f64, f64)>.
    pub fn new(lambda: F) -> Self {
        ScalarLambdaWithDerivative { lambda }
    }

    /// Evaluates function and derivative at x.
    /// Returns (value, derivative) or None if evaluation failed.
    pub fn values(&self, x: f64) -> Option<(f64, f64)> {
        (self.lambda)(x)
    }

    /// Evaluates only the function value.
    pub fn value(&self, x: f64) -> Option<f64> {
        (self.lambda)(x).map(|(y, _)| y)
    }
}

/// Polynomial functor: f(x) = sum(a[i] * x^i).
/// Coefficients stored in ascending power order: a[0] + a[1]*x + a[2]*x^2 + ...
pub struct Polynomial {
    coeffs: Vec<f64>,
}

impl Polynomial {
    /// Constructor from coefficient vector.
    pub fn new(coeffs: Vec<f64>) -> Self {
        Polynomial { coeffs }
    }

    /// Evaluates polynomial at x using Horner's method.
    pub fn value(&self, x: f64) -> f64 {
        if self.coeffs.is_empty() {
            return 0.0;
        }

        // Horner's method: a[0] + x*(a[1] + x*(a[2] + ...))
        let n = self.coeffs.len();
        let mut y = self.coeffs[n - 1];
        for i in (0..n - 1).rev() {
            y = y * x + self.coeffs[i];
        }
        y
    }

    /// Evaluates polynomial and its derivative at x.
    pub fn values(&self, x: f64) -> (f64, f64) {
        if self.coeffs.is_empty() {
            return (0.0, 0.0);
        }

        let n = self.coeffs.len();
        if n == 1 {
            return (self.coeffs[0], 0.0);
        }

        // Horner's method for value and derivative simultaneously
        let mut y = self.coeffs[n - 1];
        let mut dy = 0.0;
        for i in (1..n).rev() {
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

    /// Returns coefficient by index.
    pub fn coefficient(&self, index: usize) -> f64 {
        if index < self.coeffs.len() {
            self.coeffs[index]
        } else {
            0.0
        }
    }
}

/// Rational function functor: f(x) = P(x) / Q(x).
/// Both numerator P and denominator Q are polynomials.
pub struct Rational {
    numerator: Vec<f64>,
    denominator: Vec<f64>,
}

impl Rational {
    /// Constructor from numerator and denominator coefficients.
    pub fn new(numerator: Vec<f64>, denominator: Vec<f64>) -> Self {
        Rational {
            numerator,
            denominator,
        }
    }

    /// Evaluates rational function at x.
    /// Returns None if denominator is zero.
    pub fn value(&self, x: f64) -> Option<f64> {
        let num = horner_eval(&self.numerator, x);
        let denom = horner_eval(&self.denominator, x);

        if denom.abs() < 1e-15 {
            None
        } else {
            Some(num / denom)
        }
    }
}

/// Composite functor: f(g(x)).
/// Evaluates outer function at result of inner function.
pub struct Composite<O, I> {
    outer: O,
    inner: I,
}

impl<O, I> Composite<O, I>
where
    O: Fn(f64) -> Option<f64>,
    I: Fn(f64) -> Option<f64>,
{
    /// Constructor from outer and inner functions.
    pub fn new(outer: O, inner: I) -> Self {
        Composite { outer, inner }
    }

    /// Evaluates composite function f(g(x)).
    pub fn value(&self, x: f64) -> Option<f64> {
        let inner_val = (self.inner)(x)?;
        (self.outer)(inner_val)
    }
}

/// Sum of functions: f(x) + g(x).
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

/// Difference of functions: f(x) - g(x).
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

/// Product of functions: f(x) * g(x).
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

/// Quotient of functions: f(x) / g(x).
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

/// Scaled function: c * f(x).
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

/// Shifted function: f(x) + c.
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

/// Negated function: -f(x).
pub struct Negated<F>
where
    F: Fn(f64) -> Option<f64>,
{
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

/// Constant function: f(x) = c.
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

    /// Evaluates constant and derivative.
    pub fn values(&self, _x: f64) -> (f64, f64) {
        (self.value, 0.0)
    }
}

/// Linear function: f(x) = a*x + b.
pub struct Linear {
    slope: f64,
    intercept: f64,
}

impl Linear {
    /// Constructor from slope and intercept.
    pub fn new(slope: f64, intercept: f64) -> Self {
        Linear { slope, intercept }
    }

    /// Evaluates linear function.
    pub fn value(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }

    /// Evaluates linear function and derivative.
    pub fn values(&self, x: f64) -> (f64, f64) {
        (self.slope * x + self.intercept, self.slope)
    }
}

/// Sine function: f(x) = a * sin(b*x + c) + d.
pub struct Sine {
    amplitude: f64,
    frequency: f64,
    phase: f64,
    offset: f64,
}

impl Sine {
    /// Constructor with full parameters.
    pub fn new(amplitude: f64, frequency: f64, phase: f64, offset: f64) -> Self {
        Sine {
            amplitude,
            frequency,
            phase,
            offset,
        }
    }

    /// Evaluates sine function.
    pub fn value(&self, x: f64) -> f64 {
        self.amplitude * (self.frequency * x + self.phase).sin() + self.offset
    }

    /// Evaluates sine function and derivative.
    pub fn values(&self, x: f64) -> (f64, f64) {
        let arg = self.frequency * x + self.phase;
        (
            self.amplitude * arg.sin() + self.offset,
            self.amplitude * self.frequency * arg.cos(),
        )
    }
}

/// Cosine function: f(x) = a * cos(b*x + c) + d.
pub struct Cosine {
    amplitude: f64,
    frequency: f64,
    phase: f64,
    offset: f64,
}

impl Cosine {
    /// Constructor with full parameters.
    pub fn new(amplitude: f64, frequency: f64, phase: f64, offset: f64) -> Self {
        Cosine {
            amplitude,
            frequency,
            phase,
            offset,
        }
    }

    /// Evaluates cosine function.
    pub fn value(&self, x: f64) -> f64 {
        self.amplitude * (self.frequency * x + self.phase).cos() + self.offset
    }

    /// Evaluates cosine function and derivative.
    pub fn values(&self, x: f64) -> (f64, f64) {
        let arg = self.frequency * x + self.phase;
        (
            self.amplitude * arg.cos() + self.offset,
            -self.amplitude * self.frequency * arg.sin(),
        )
    }
}

/// Exponential function: f(x) = a * exp(b*x) + c.
pub struct Exponential {
    scale: f64,
    rate: f64,
    offset: f64,
}

impl Exponential {
    /// Constructor with full parameters.
    pub fn new(scale: f64, rate: f64, offset: f64) -> Self {
        Exponential { scale, rate, offset }
    }

    /// Evaluates exponential function.
    pub fn value(&self, x: f64) -> f64 {
        self.scale * (self.rate * x).exp() + self.offset
    }

    /// Evaluates exponential function and derivative.
    pub fn values(&self, x: f64) -> (f64, f64) {
        let exp_val = (self.rate * x).exp();
        (
            self.scale * exp_val + self.offset,
            self.scale * self.rate * exp_val,
        )
    }
}

/// Power function: f(x) = a * x^n + b.
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
        if x < 0.0 && self.exponent.fract() != 0.0 {
            return None;
        }
        Some(self.scale * x.powf(self.exponent) + self.offset)
    }

    /// Evaluates power function and derivative.
    pub fn values(&self, x: f64) -> Option<(f64, f64)> {
        if x < 0.0 && self.exponent.fract() != 0.0 {
            return None;
        }
        let pow = x.powf(self.exponent);
        let y = self.scale * pow + self.offset;
        let dy = if x.abs() < 1e-15 {
            if (self.exponent - 1.0).abs() < 1e-15 {
                self.scale
            } else {
                0.0
            }
        } else {
            self.scale * self.exponent * pow / x
        };
        Some((y, dy))
    }
}

/// Gaussian function: f(x) = a * exp(-((x-mu)^2)/(2*sigma^2)).
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

// Helper function for Horner's method evaluation
fn horner_eval(coeffs: &[f64], x: f64) -> f64 {
    if coeffs.is_empty() {
        return 0.0;
    }
    let n = coeffs.len();
    let mut y = coeffs[n - 1];
    for i in (0..n - 1).rev() {
        y = y * x + coeffs[i];
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_value() {
        // x^2 - 2 (coefficients [−2, 0, 1])
        let poly = Polynomial::new(vec![-2.0, 0.0, 1.0]);
        assert!((poly.value(2.0) - 2.0).abs() < 1e-9); // 4 - 2 = 2
        assert!((poly.value(0.0) - (-2.0)).abs() < 1e-9); // -2
    }

    #[test]
    fn test_polynomial_derivative() {
        // x^2 - 2, derivative = 2x
        let poly = Polynomial::new(vec![-2.0, 0.0, 1.0]);
        let (y, dy) = poly.values(3.0);
        assert!((y - 7.0).abs() < 1e-9); // 9 - 2 = 7
        assert!((dy - 6.0).abs() < 1e-9); // 2 * 3 = 6
    }

    #[test]
    fn test_rational_function() {
        let rat = Rational::new(vec![1.0, 1.0], vec![1.0, 0.0, 1.0]); // (1 + x) / (1 + x^2)
        let result = rat.value(0.0);
        assert!(result.is_some());
        assert!((result.unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_constant_function() {
        let c = Constant::new(5.0);
        assert!((c.value(0.0) - 5.0).abs() < 1e-9);
        assert!((c.value(100.0) - 5.0).abs() < 1e-9);
        let (y, dy) = c.values(3.0);
        assert!((y - 5.0).abs() < 1e-9);
        assert!(dy.abs() < 1e-9);
    }

    #[test]
    fn test_linear_function() {
        let lin = Linear::new(2.0, 3.0); // 2x + 3
        assert!((lin.value(0.0) - 3.0).abs() < 1e-9);
        assert!((lin.value(2.0) - 7.0).abs() < 1e-9);
        let (y, dy) = lin.values(5.0);
        assert!((y - 13.0).abs() < 1e-9);
        assert!((dy - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_sine_function() {
        let sine = Sine::new(1.0, 1.0, 0.0, 0.0);
        assert!(sine.value(0.0).abs() < 1e-9);
        assert!((sine.value(std::f64::consts::PI / 2.0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_exponential_function() {
        let exp = Exponential::new(1.0, 1.0, 0.0); // e^x
        assert!((exp.value(0.0) - 1.0).abs() < 1e-9);
        let (y, dy) = exp.values(0.0);
        assert!((y - 1.0).abs() < 1e-9);
        assert!((dy - 1.0).abs() < 1e-9);
    }
}

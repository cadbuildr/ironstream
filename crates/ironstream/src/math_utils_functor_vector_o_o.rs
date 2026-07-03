// FILE: math_utils_functor_vector_o_o.rs
// occt: MathUtils_FunctorVector

//! Non-virtual functor classes for N-dimensional (vector) functions.
//! Provides ready-to-use functor classes for template-based math API
//! without virtual dispatch overhead.

/// Wrapper for N-D objective functions (value only).
pub struct VectorLambda<F>
where
    F: Fn(&[f64]) -> Option<f64>,
{
    lambda: F,
}

impl<F> VectorLambda<F>
where
    F: Fn(&[f64]) -> Option<f64>,
{
    /// Constructor from callable.
    pub fn new(lambda: F) -> Self {
        VectorLambda { lambda }
    }

    /// Evaluates the function at x vector.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        (self.lambda)(x)
    }
}

/// Wrapper for N-D objective functions with gradient.
pub struct VectorLambdaWithGradient<F, G>
where
    F: Fn(&[f64]) -> Option<f64>,
    G: Fn(&[f64], &mut [f64]) -> bool,
{
    value_lambda: F,
    grad_lambda: G,
}

impl<F, G> VectorLambdaWithGradient<F, G>
where
    F: Fn(&[f64]) -> Option<f64>,
    G: Fn(&[f64], &mut [f64]) -> bool,
{
    /// Constructor from value and gradient lambdas.
    pub fn new(value_lambda: F, grad_lambda: G) -> Self {
        VectorLambdaWithGradient {
            value_lambda,
            grad_lambda,
        }
    }

    /// Evaluates the function value at x.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        (self.value_lambda)(x)
    }

    /// Evaluates the gradient at x.
    pub fn gradient(&self, x: &[f64], g: &mut [f64]) -> bool {
        (self.grad_lambda)(x, g)
    }

    /// Evaluates both value and gradient at x.
    pub fn values(&self, x: &[f64], g: &mut [f64]) -> Option<f64> {
        if !self.gradient(x, g) {
            return None;
        }
        self.value(x)
    }
}

/// Quadratic form functor: f(x) = x^T A x + b^T x + c.
/// Commonly used for testing optimization algorithms.
pub struct QuadraticForm {
    a: Vec<Vec<f64>>,
    b: Vec<f64>,
    c: f64,
}

impl QuadraticForm {
    /// Constructor from matrix, vector, and constant.
    /// a is stored as row-major (Vec<Vec<f64>>).
    pub fn new(a: Vec<Vec<f64>>, b: Vec<f64>, c: f64) -> Self {
        QuadraticForm { a, b, c }
    }

    /// Evaluates the quadratic form f(x) = x^T A x + b^T x + c.
    pub fn value(&self, x: &[f64]) -> f64 {
        let mut y = self.c;

        // x^T A x
        for i in 0..x.len() {
            for j in 0..x.len() {
                if i < self.a.len() && j < self.a[i].len() {
                    y += x[i] * self.a[i][j] * x[j];
                }
            }
        }

        // b^T x
        for i in 0..x.len() {
            if i < self.b.len() {
                y += self.b[i] * x[i];
            }
        }

        y
    }

    /// Evaluates the gradient: g = (A + A^T) * x + b.
    pub fn gradient(&self, x: &[f64], g: &mut [f64]) -> bool {
        if g.len() != x.len() {
            return false;
        }

        for i in 0..x.len() {
            g[i] = if i < self.b.len() { self.b[i] } else { 0.0 };

            for j in 0..x.len() {
                let aij = if i < self.a.len() && j < self.a[i].len() {
                    self.a[i][j]
                } else {
                    0.0
                };
                let aji = if j < self.a.len() && i < self.a[j].len() {
                    self.a[j][i]
                } else {
                    0.0
                };
                g[i] += (aij + aji) * x[j];
            }
        }

        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &[f64], g: &mut [f64]) -> Option<f64> {
        if !self.gradient(x, g) {
            return None;
        }
        Some(self.value(x))
    }
}

/// Rosenbrock function: f(x,y) = (a - x)^2 + b*(y - x^2)^2.
/// Global minimum at (a, a^2).
pub struct Rosenbrock {
    a: f64,
    b: f64,
}

impl Rosenbrock {
    /// Constructor with parameters (default a=1.0, b=100.0).
    pub fn new(a: f64, b: f64) -> Self {
        Rosenbrock { a, b }
    }

    /// Evaluates the Rosenbrock function.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        if x.len() < 2 {
            return None;
        }
        let x_val = x[0];
        let y_val = x[1];
        let t1 = self.a - x_val;
        let t2 = y_val - x_val * x_val;
        Some(t1 * t1 + self.b * t2 * t2)
    }

    /// Evaluates the gradient of the Rosenbrock function.
    pub fn gradient(&self, x: &[f64], g: &mut [f64]) -> bool {
        if x.len() < 2 || g.len() < 2 {
            return false;
        }
        let x_val = x[0];
        let y_val = x[1];
        let t2 = y_val - x_val * x_val;
        g[0] = -2.0 * (self.a - x_val) - 4.0 * self.b * x_val * t2;
        g[1] = 2.0 * self.b * t2;
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &[f64], g: &mut [f64]) -> Option<f64> {
        if !self.gradient(x, g) {
            return None;
        }
        self.value(x)
    }
}

/// Sphere function: f(x) = sum(x[i]^2).
/// Global minimum at origin with f = 0.
pub struct Sphere;

impl Sphere {
    /// Evaluates the sphere function.
    pub fn value(x: &[f64]) -> f64 {
        x.iter().fold(0.0, |acc, &xi| acc + xi * xi)
    }

    /// Evaluates the gradient: g[i] = 2*x[i].
    pub fn gradient(x: &[f64], g: &mut [f64]) -> bool {
        if g.len() != x.len() {
            return false;
        }
        for i in 0..x.len() {
            g[i] = 2.0 * x[i];
        }
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(x: &[f64], g: &mut [f64]) -> Option<f64> {
        if !Self::gradient(x, g) {
            return None;
        }
        Some(Self::value(x))
    }
}

/// Booth function: f(x,y) = (x + 2y - 7)^2 + (2x + y - 5)^2.
/// Global minimum at (1, 3) with f = 0.
pub struct Booth;

impl Booth {
    /// Evaluates the Booth function.
    pub fn value(x: &[f64]) -> Option<f64> {
        if x.len() < 2 {
            return None;
        }
        let x_val = x[0];
        let y_val = x[1];
        let t1 = x_val + 2.0 * y_val - 7.0;
        let t2 = 2.0 * x_val + y_val - 5.0;
        Some(t1 * t1 + t2 * t2)
    }

    /// Evaluates the gradient of the Booth function.
    pub fn gradient(x: &[f64], g: &mut [f64]) -> bool {
        if x.len() < 2 || g.len() < 2 {
            return false;
        }
        let x_val = x[0];
        let y_val = x[1];
        let t1 = x_val + 2.0 * y_val - 7.0;
        let t2 = 2.0 * x_val + y_val - 5.0;
        g[0] = 2.0 * t1 + 4.0 * t2;
        g[1] = 4.0 * t1 + 2.0 * t2;
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(x: &[f64], g: &mut [f64]) -> Option<f64> {
        if !Self::gradient(x, g) {
            return None;
        }
        Self::value(x)
    }
}

/// Beale function: f(x,y) = (1.5 - x + xy)^2 + (2.25 - x + xy^2)^2 + (2.625 - x + xy^3)^2.
/// Global minimum at (3, 0.5) with f = 0.
pub struct Beale;

impl Beale {
    /// Evaluates the Beale function.
    pub fn value(x: &[f64]) -> Option<f64> {
        if x.len() < 2 {
            return None;
        }
        let x_val = x[0];
        let y_val = x[1];
        let y2 = y_val * y_val;
        let y3 = y2 * y_val;
        let t1 = 1.5 - x_val + x_val * y_val;
        let t2 = 2.25 - x_val + x_val * y2;
        let t3 = 2.625 - x_val + x_val * y3;
        Some(t1 * t1 + t2 * t2 + t3 * t3)
    }

    /// Evaluates the gradient of the Beale function.
    pub fn gradient(x: &[f64], g: &mut [f64]) -> bool {
        if x.len() < 2 || g.len() < 2 {
            return false;
        }
        let x_val = x[0];
        let y_val = x[1];
        let y2 = y_val * y_val;
        let y3 = y2 * y_val;
        let t1 = 1.5 - x_val + x_val * y_val;
        let t2 = 2.25 - x_val + x_val * y2;
        let t3 = 2.625 - x_val + x_val * y3;
        g[0] = 2.0 * ((y_val - 1.0) * t1 + (y2 - 1.0) * t2 + (y3 - 1.0) * t3);
        g[1] = 2.0 * x_val * (t1 + 2.0 * y_val * t2 + 3.0 * y2 * t3);
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(x: &[f64], g: &mut [f64]) -> Option<f64> {
        if !Self::gradient(x, g) {
            return None;
        }
        Self::value(x)
    }
}

/// Himmelblau function: f(x,y) = (x^2 + y - 11)^2 + (x + y^2 - 7)^2.
/// Has four local minima, all with f = 0.
pub struct Himmelblau;

impl Himmelblau {
    /// Evaluates the Himmelblau function.
    pub fn value(x: &[f64]) -> Option<f64> {
        if x.len() < 2 {
            return None;
        }
        let x_val = x[0];
        let y_val = x[1];
        let t1 = x_val * x_val + y_val - 11.0;
        let t2 = x_val + y_val * y_val - 7.0;
        Some(t1 * t1 + t2 * t2)
    }

    /// Evaluates the gradient of the Himmelblau function.
    pub fn gradient(x: &[f64], g: &mut [f64]) -> bool {
        if x.len() < 2 || g.len() < 2 {
            return false;
        }
        let x_val = x[0];
        let y_val = x[1];
        let t1 = x_val * x_val + y_val - 11.0;
        let t2 = x_val + y_val * y_val - 7.0;
        g[0] = 4.0 * x_val * t1 + 2.0 * t2;
        g[1] = 2.0 * t1 + 4.0 * y_val * t2;
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(x: &[f64], g: &mut [f64]) -> Option<f64> {
        if !Self::gradient(x, g) {
            return None;
        }
        Self::value(x)
    }
}

/// Rastrigin function: f(x) = A*n + sum(x[i]^2 - A*cos(2*pi*x[i])).
/// Default A = 10. Global minimum at origin with f = 0.
/// Highly multimodal - challenging for local optimizers.
pub struct Rastrigin {
    a: f64,
}

impl Rastrigin {
    /// Constructor with parameter.
    pub fn new(a: f64) -> Self {
        Rastrigin { a }
    }

    /// Evaluates the Rastrigin function.
    pub fn value(&self, x: &[f64]) -> f64 {
        const TWO_PI: f64 = 6.283185307179586;
        let n = x.len() as f64;
        let mut y = self.a * n;
        for &xi in x {
            y += xi * xi - self.a * (TWO_PI * xi).cos();
        }
        y
    }

    /// Evaluates the gradient of the Rastrigin function.
    pub fn gradient(&self, x: &[f64], g: &mut [f64]) -> bool {
        if g.len() != x.len() {
            return false;
        }
        const TWO_PI: f64 = 6.283185307179586;
        for i in 0..x.len() {
            g[i] = 2.0 * x[i] + self.a * TWO_PI * (TWO_PI * x[i]).sin();
        }
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &[f64], g: &mut [f64]) -> Option<f64> {
        if !self.gradient(x, g) {
            return None;
        }
        Some(self.value(x))
    }
}

/// Ackley function: f(x) = -a*exp(-b*sqrt(sum(x[i]^2)/n)) - exp(sum(cos(c*x[i]))/n) + a + e.
/// Default: a = 20, b = 0.2, c = 2*pi. Global minimum at origin with f = 0.
pub struct Ackley {
    a: f64,
    b: f64,
    c: f64,
}

impl Ackley {
    /// Constructor with parameters.
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Ackley { a, b, c }
    }

    /// Evaluates the Ackley function.
    pub fn value(&self, x: &[f64]) -> f64 {
        const E: f64 = 2.718281828459045;
        let n = x.len() as f64;
        let mut sum_sq = 0.0;
        let mut sum_cos = 0.0;
        for &xi in x {
            sum_sq += xi * xi;
            sum_cos += (self.c * xi).cos();
        }
        -self.a * (-self.b * (sum_sq / n).sqrt()).exp() - (sum_cos / n).exp() + self.a + E
    }
}

/// Linear system residual functor: f(x) = ||Ax - b||^2.
/// Useful for solving overdetermined linear systems via optimization.
pub struct LinearResidual {
    a: Vec<Vec<f64>>,
    b: Vec<f64>,
}

impl LinearResidual {
    /// Constructor from matrix and right-hand side.
    pub fn new(a: Vec<Vec<f64>>, b: Vec<f64>) -> Self {
        LinearResidual { a, b }
    }

    /// Evaluates the residual ||Ax - b||^2.
    pub fn value(&self, x: &[f64]) -> f64 {
        let mut y = 0.0;
        for i in 0..self.b.len() {
            let mut residual = -self.b[i];
            if i < self.a.len() {
                for j in 0..x.len() {
                    if j < self.a[i].len() {
                        residual += self.a[i][j] * x[j];
                    }
                }
            }
            y += residual * residual;
        }
        y
    }

    /// Evaluates the gradient: g = 2 * A^T * (Ax - b).
    pub fn gradient(&self, x: &[f64], g: &mut [f64]) -> bool {
        // Compute residual r = Ax - b
        let mut residual = vec![0.0; self.b.len()];
        for i in 0..self.b.len() {
            residual[i] = -self.b[i];
            if i < self.a.len() {
                for j in 0..x.len() {
                    if j < self.a[i].len() {
                        residual[i] += self.a[i][j] * x[j];
                    }
                }
            }
        }

        // Compute g = 2 * A^T * residual
        for j in 0..x.len() {
            g[j] = 0.0;
            for i in 0..self.b.len() {
                if i < self.a.len() && j < self.a[i].len() {
                    g[j] += 2.0 * self.a[i][j] * residual[i];
                }
            }
        }
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &[f64], g: &mut [f64]) -> Option<f64> {
        if !self.gradient(x, g) {
            return None;
        }
        Some(self.value(x))
    }
}

/// Nonlinear system functor: F(x) = [f1(x), f2(x), ..., fn(x)].
pub struct SystemLambda<F>
where
    F: Fn(&[f64], &mut [f64]) -> bool,
{
    lambda: F,
    nb_equations: usize,
}

impl<F> SystemLambda<F>
where
    F: Fn(&[f64], &mut [f64]) -> bool,
{
    /// Constructor from lambda.
    pub fn new(lambda: F, nb_equations: usize) -> Self {
        SystemLambda {
            lambda,
            nb_equations,
        }
    }

    /// Returns the number of equations.
    pub fn nb_equations(&self) -> usize {
        self.nb_equations
    }

    /// Evaluates the system F(x).
    pub fn value(&self, x: &[f64], f: &mut [f64]) -> bool {
        (self.lambda)(x, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_function() {
        let x = vec![1.0, 2.0, 3.0];
        assert!((Sphere::value(&x) - 14.0).abs() < 1e-9);
    }

    #[test]
    fn test_sphere_gradient() {
        let x = vec![2.0, 3.0];
        let mut g = vec![0.0; 2];
        assert!(Sphere::gradient(&x, &mut g));
        assert!((g[0] - 4.0).abs() < 1e-9);
        assert!((g[1] - 6.0).abs() < 1e-9);
    }

    #[test]
    fn test_rosenbrock_at_minimum() {
        let rosen = Rosenbrock::new(1.0, 100.0);
        let x = vec![1.0, 1.0];
        assert!((rosen.value(&x).unwrap() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_booth_at_minimum() {
        let x = vec![1.0, 3.0];
        assert!((Booth::value(&x).unwrap() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_beale_at_minimum() {
        let x = vec![3.0, 0.5];
        assert!((Beale::value(&x).unwrap() - 0.0).abs() < 1e-8);
    }

    #[test]
    fn test_himmelblau_at_minimum() {
        let x = vec![3.0, 2.0];
        assert!((Himmelblau::value(&x).unwrap() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_rastrigin_at_origin() {
        let rastrigin = Rastrigin::new(10.0);
        let x = vec![0.0, 0.0, 0.0];
        assert!((rastrigin.value(&x) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_quadratic_form() {
        let a = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
        let b = vec![-4.0, -4.0];
        let qf = QuadraticForm::new(a, b, 8.0);
        let x = vec![1.0, 1.0];
        assert!((qf.value(&x) - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_linear_residual() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0]];
        let b = vec![1.0, 1.0, 2.0];
        let lr = LinearResidual::new(a, b);
        let x = vec![1.0, 1.0];
        assert!((lr.value(&x) - 0.0).abs() < 1e-9);
    }
}

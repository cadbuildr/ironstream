// FILE: math_utils_functor_vector.rs

//! Non-virtual functor classes for N-dimensional (vector) functions.
//!
//! Provides ready-to-use functor classes that work with the template-based
//! math API (MathOpt::Powell, MathOpt::BFGS, MathSys::Newton) without
//! virtual dispatch overhead.

// Simple vector type using Vec<f64> for 1-indexed compatibility
#[derive(Clone, Debug)]
pub struct Vector {
    data: Vec<f64>,
    lower: usize,
}

impl Vector {
    /// Create a vector with the given lower bound and length.
    pub fn new(lower: usize, upper: usize) -> Self {
        let len = upper - lower + 1;
        Vector {
            data: vec![0.0; len],
            lower,
        }
    }

    /// Get the lower index bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Get the upper index bound.
    pub fn upper(&self) -> usize {
        self.lower + self.data.len() - 1
    }

    /// Get the length of the vector.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get element at index (1-indexed in OCCT style).
    pub fn at(&self, i: usize) -> f64 {
        self.data[i - self.lower]
    }

    /// Set element at index (1-indexed in OCCT style).
    pub fn set(&mut self, i: usize, val: f64) {
        self.data[i - self.lower] = val;
    }

    /// Initialize all elements with a value.
    pub fn init(&mut self, val: f64) {
        for elem in &mut self.data {
            *elem = val;
        }
    }
}

impl std::ops::Index<usize> for Vector {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.data[i - self.lower]
    }
}

impl std::ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.data[i - self.lower]
    }
}

// Simple matrix type using Vec<f64> for 1-indexed compatibility
#[derive(Clone, Debug)]
pub struct Matrix {
    data: Vec<f64>,
    lower_row: usize,
    upper_row: usize,
    lower_col: usize,
    upper_col: usize,
}

impl Matrix {
    /// Create a matrix with the given bounds.
    pub fn new(lower_row: usize, upper_row: usize, lower_col: usize, upper_col: usize) -> Self {
        let num_rows = upper_row - lower_row + 1;
        let num_cols = upper_col - lower_col + 1;
        Matrix {
            data: vec![0.0; num_rows * num_cols],
            lower_row,
            upper_row,
            lower_col,
            upper_col,
        }
    }

    pub fn lower_row(&self) -> usize {
        self.lower_row
    }

    pub fn upper_row(&self) -> usize {
        self.upper_row
    }

    pub fn lower_col(&self) -> usize {
        self.lower_col
    }

    pub fn upper_col(&self) -> usize {
        self.upper_col
    }

    /// Get element at (i, j) (1-indexed in OCCT style).
    pub fn at(&self, i: usize, j: usize) -> f64 {
        let row_idx = i - self.lower_row;
        let col_idx = j - self.lower_col;
        let num_cols = self.upper_col - self.lower_col + 1;
        self.data[row_idx * num_cols + col_idx]
    }

    /// Set element at (i, j).
    pub fn set(&mut self, i: usize, j: usize, val: f64) {
        let row_idx = i - self.lower_row;
        let col_idx = j - self.lower_col;
        let num_cols = self.upper_col - self.lower_col + 1;
        self.data[row_idx * num_cols + col_idx] = val;
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (i, j): (usize, usize)) -> &f64 {
        let row_idx = i - self.lower_row;
        let col_idx = j - self.lower_col;
        let num_cols = self.upper_col - self.lower_col + 1;
        &self.data[row_idx * num_cols + col_idx]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut f64 {
        let row_idx = i - self.lower_row;
        let col_idx = j - self.lower_col;
        let num_cols = self.upper_col - self.lower_col + 1;
        &mut self.data[row_idx * num_cols + col_idx]
    }
}

// occt-ref: MathUtils_VectorLambda
/// Lambda wrapper for N-D objective functions (value only).
pub struct VectorLambda<F>
where
    F: Fn(&Vector) -> Option<f64>,
{
    lambda: F,
}

impl<F> VectorLambda<F>
where
    F: Fn(&Vector) -> Option<f64>,
{
    /// Constructor from lambda/callable.
    pub fn new(lambda: F) -> Self {
        VectorLambda { lambda }
    }

    /// Evaluates the function at x.
    pub fn value(&self, x: &Vector) -> Option<f64> {
        (self.lambda)(x)
    }
}

// occt-ref: MathUtils_VectorLambdaWithGradient
/// Lambda wrapper for N-D objective functions with gradient.
pub struct VectorLambdaWithGradient<FValue, FGrad>
where
    FValue: Fn(&Vector) -> Option<f64>,
    FGrad: Fn(&Vector, &mut Vector) -> bool,
{
    value_lambda: FValue,
    grad_lambda: FGrad,
}

impl<FValue, FGrad> VectorLambdaWithGradient<FValue, FGrad>
where
    FValue: Fn(&Vector) -> Option<f64>,
    FGrad: Fn(&Vector, &mut Vector) -> bool,
{
    /// Constructor from value and gradient lambdas.
    pub fn new(value_lambda: FValue, grad_lambda: FGrad) -> Self {
        VectorLambdaWithGradient {
            value_lambda,
            grad_lambda,
        }
    }

    /// Evaluates the function value at x.
    pub fn value(&self, x: &Vector) -> Option<f64> {
        (self.value_lambda)(x)
    }

    /// Evaluates the gradient at x.
    pub fn gradient(&self, x: &Vector, g: &mut Vector) -> bool {
        (self.grad_lambda)(x, g)
    }

    /// Evaluates both value and gradient at x.
    pub fn values(&self, x: &Vector, g: &mut Vector) -> Option<f64> {
        let y = (self.value_lambda)(x)?;
        if (self.grad_lambda)(x, g) {
            Some(y)
        } else {
            None
        }
    }
}

// occt-ref: MathUtils_QuadraticForm
/// Quadratic form functor: f(x) = x^T A x + b^T x + c.
#[derive(Clone)]
pub struct QuadraticForm {
    matrix_a: Matrix,
    vector_b: Vector,
    c: f64,
}

impl QuadraticForm {
    /// Constructor from matrix, vector, and constant.
    pub fn new(matrix_a: Matrix, vector_b: Vector, c: f64) -> Self {
        QuadraticForm {
            matrix_a,
            vector_b,
            c,
        }
    }

    /// Evaluates the quadratic form f(x) = x^T A x + b^T x + c.
    pub fn value(&self, x: &Vector) -> f64 {
        let mut result = self.c;

        // x^T A x
        for i in self.matrix_a.lower_row()..=self.matrix_a.upper_row() {
            for j in self.matrix_a.lower_col()..=self.matrix_a.upper_col() {
                result += x[i] * self.matrix_a[(i, j)] * x[j];
            }
        }

        // b^T x
        for i in self.vector_b.lower()..=self.vector_b.upper() {
            result += self.vector_b[i] * x[i];
        }

        result
    }

    /// Evaluates the gradient: g = 2*A*x + b (for symmetric A).
    pub fn gradient(&self, x: &Vector, g: &mut Vector) -> bool {
        for i in self.matrix_a.lower_row()..=self.matrix_a.upper_row() {
            g[i] = self.vector_b[i];
            for j in self.matrix_a.lower_col()..=self.matrix_a.upper_col() {
                g[i] += (self.matrix_a[(i, j)] + self.matrix_a[(j, i)]) * x[j];
            }
        }
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &Vector, g: &mut Vector) -> (f64, bool) {
        let y = self.value(x);
        let success = self.gradient(x, g);
        (y, success)
    }
}

// occt-ref: MathUtils_Rosenbrock
/// Rosenbrock function functor (for testing optimization).
/// f(x,y) = (a - x)^2 + b*(y - x^2)^2
/// Global minimum at (a, a^2) with f = 0.
#[derive(Clone, Copy)]
pub struct Rosenbrock {
    a: f64,
    b: f64,
}

impl Rosenbrock {
    /// Constructor with parameters.
    pub fn new(a: f64, b: f64) -> Self {
        Rosenbrock { a, b }
    }

    /// Default: a=1, b=100
    pub fn default() -> Self {
        Rosenbrock { a: 1.0, b: 100.0 }
    }

    /// Evaluates the Rosenbrock function.
    pub fn value(&self, x: &Vector) -> f64 {
        let x_val = x[x.lower()];
        let y_val = x[x.lower() + 1];
        let t1 = self.a - x_val;
        let t2 = y_val - x_val * x_val;
        t1 * t1 + self.b * t2 * t2
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &Vector, g: &mut Vector) -> bool {
        let x_val = x[x.lower()];
        let y_val = x[x.lower() + 1];
        let t2 = y_val - x_val * x_val;
        let g_lower = g.lower();
        g[g_lower] = -2.0 * (self.a - x_val) - 4.0 * self.b * x_val * t2;
        g[g_lower + 1] = 2.0 * self.b * t2;
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &Vector, g: &mut Vector) -> (f64, bool) {
        let y = self.value(x);
        let success = self.gradient(x, g);
        (y, success)
    }
}

// occt-ref: MathUtils_Sphere
/// Sphere function functor (for testing optimization).
/// f(x) = sum(x[i]^2) for all i.
/// Global minimum at origin with f = 0.
pub struct Sphere;

impl Sphere {
    /// Evaluates the sphere function.
    pub fn value(&self, x: &Vector) -> f64 {
        let mut result = 0.0;
        for i in x.lower()..=x.upper() {
            result += x[i] * x[i];
        }
        result
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &Vector, g: &mut Vector) -> bool {
        for i in x.lower()..=x.upper() {
            g[i] = 2.0 * x[i];
        }
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &Vector, g: &mut Vector) -> (f64, bool) {
        let y = self.value(x);
        let success = self.gradient(x, g);
        (y, success)
    }
}

// occt-ref: MathUtils_Booth
/// Booth function functor (for testing optimization).
/// f(x,y) = (x + 2y - 7)^2 + (2x + y - 5)^2
/// Global minimum at (1, 3) with f = 0.
pub struct Booth;

impl Booth {
    /// Evaluates the Booth function.
    pub fn value(&self, x: &Vector) -> f64 {
        let x_val = x[x.lower()];
        let y_val = x[x.lower() + 1];
        let t1 = x_val + 2.0 * y_val - 7.0;
        let t2 = 2.0 * x_val + y_val - 5.0;
        t1 * t1 + t2 * t2
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &Vector, g: &mut Vector) -> bool {
        let x_val = x[x.lower()];
        let y_val = x[x.lower() + 1];
        let t1 = x_val + 2.0 * y_val - 7.0;
        let t2 = 2.0 * x_val + y_val - 5.0;
        let g_lower = g.lower();
        g[g_lower] = 2.0 * t1 + 4.0 * t2;
        g[g_lower + 1] = 4.0 * t1 + 2.0 * t2;
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &Vector, g: &mut Vector) -> (f64, bool) {
        let y = self.value(x);
        let success = self.gradient(x, g);
        (y, success)
    }
}

// occt-ref: MathUtils_Beale
/// Beale function functor (for testing optimization).
/// f(x,y) = (1.5 - x + xy)^2 + (2.25 - x + xy^2)^2 + (2.625 - x + xy^3)^2
/// Global minimum at (3, 0.5) with f = 0.
pub struct Beale;

impl Beale {
    /// Evaluates the Beale function.
    pub fn value(&self, x: &Vector) -> f64 {
        let x_val = x[x.lower()];
        let y_val = x[x.lower() + 1];
        let t1 = 1.5 - x_val + x_val * y_val;
        let t2 = 2.25 - x_val + x_val * y_val * y_val;
        let t3 = 2.625 - x_val + x_val * y_val * y_val * y_val;
        t1 * t1 + t2 * t2 + t3 * t3
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &Vector, g: &mut Vector) -> bool {
        let x_val = x[x.lower()];
        let y_val = x[x.lower() + 1];
        let y2 = y_val * y_val;
        let y3 = y2 * y_val;
        let t1 = 1.5 - x_val + x_val * y_val;
        let t2 = 2.25 - x_val + x_val * y2;
        let t3 = 2.625 - x_val + x_val * y3;

        let g_lower = g.lower();
        g[g_lower] = 2.0 * ((y_val - 1.0) * t1 + (y2 - 1.0) * t2 + (y3 - 1.0) * t3);
        g[g_lower + 1] = 2.0 * x_val * (t1 + 2.0 * y_val * t2 + 3.0 * y2 * t3);
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &Vector, g: &mut Vector) -> (f64, bool) {
        let y = self.value(x);
        let success = self.gradient(x, g);
        (y, success)
    }
}

// occt-ref: MathUtils_Himmelblau
/// Himmelblau function functor (for testing optimization).
/// f(x,y) = (x^2 + y - 11)^2 + (x + y^2 - 7)^2
/// Has four local minima, all with f = 0.
pub struct Himmelblau;

impl Himmelblau {
    /// Evaluates the Himmelblau function.
    pub fn value(&self, x: &Vector) -> f64 {
        let x_val = x[x.lower()];
        let y_val = x[x.lower() + 1];
        let t1 = x_val * x_val + y_val - 11.0;
        let t2 = x_val + y_val * y_val - 7.0;
        t1 * t1 + t2 * t2
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &Vector, g: &mut Vector) -> bool {
        let x_val = x[x.lower()];
        let y_val = x[x.lower() + 1];
        let t1 = x_val * x_val + y_val - 11.0;
        let t2 = x_val + y_val * y_val - 7.0;
        let g_lower = g.lower();
        g[g_lower] = 4.0 * x_val * t1 + 2.0 * t2;
        g[g_lower + 1] = 2.0 * t1 + 4.0 * y_val * t2;
        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &Vector, g: &mut Vector) -> (f64, bool) {
        let y = self.value(x);
        let success = self.gradient(x, g);
        (y, success)
    }
}

// occt-ref: MathUtils_Rastrigin
/// Rastrigin function functor (for testing global optimization).
/// f(x) = A*n + sum(x[i]^2 - A*cos(2*pi*x[i])) for all i
/// Default: A = 10
/// Global minimum at origin with f = 0.
#[derive(Clone, Copy)]
pub struct Rastrigin {
    a: f64,
}

impl Rastrigin {
    /// Constructor with parameter.
    pub fn new(a: f64) -> Self {
        Rastrigin { a }
    }

    /// Default: A = 10
    pub fn default() -> Self {
        Rastrigin { a: 10.0 }
    }

    /// Evaluates the Rastrigin function.
    pub fn value(&self, x: &Vector) -> f64 {
        const TWO_PI: f64 = 2.0 * std::f64::consts::PI;
        let n = (x.upper() - x.lower() + 1) as f64;
        let mut result = self.a * n;

        for i in x.lower()..=x.upper() {
            result += x[i] * x[i] - self.a * (TWO_PI * x[i]).cos();
        }

        result
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &Vector, g: &mut Vector) -> bool {
        const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

        for i in x.lower()..=x.upper() {
            g[i] = 2.0 * x[i] + self.a * TWO_PI * (TWO_PI * x[i]).sin();
        }

        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &Vector, g: &mut Vector) -> (f64, bool) {
        let y = self.value(x);
        let success = self.gradient(x, g);
        (y, success)
    }
}

// occt-ref: MathUtils_Ackley
/// Ackley function functor (for testing global optimization).
/// f(x) = -a*exp(-b*sqrt(sum(x[i]^2)/n)) - exp(sum(cos(c*x[i]))/n) + a + e
/// Default: a = 20, b = 0.2, c = 2*pi
/// Global minimum at origin with f = 0.
#[derive(Clone, Copy)]
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

    /// Default: a=20, b=0.2, c=2*pi
    pub fn default() -> Self {
        Ackley {
            a: 20.0,
            b: 0.2,
            c: 2.0 * std::f64::consts::PI,
        }
    }

    /// Evaluates the Ackley function.
    pub fn value(&self, x: &Vector) -> f64 {
        const E: f64 = std::f64::consts::E;
        let n = (x.upper() - x.lower() + 1) as f64;

        let mut sum_sq = 0.0;
        let mut sum_cos = 0.0;

        for i in x.lower()..=x.upper() {
            sum_sq += x[i] * x[i];
            sum_cos += (self.c * x[i]).cos();
        }

        -self.a * (-self.b * (sum_sq / n).sqrt()).exp() - (sum_cos / n).exp() + self.a + E
    }
}

// occt-ref: MathUtils_LinearResidual
/// Linear system residual functor: f(x) = ||Ax - b||^2.
#[derive(Clone)]
pub struct LinearResidual {
    matrix_a: Matrix,
    vector_b: Vector,
}

impl LinearResidual {
    /// Constructor from matrix and right-hand side.
    pub fn new(matrix_a: Matrix, vector_b: Vector) -> Self {
        LinearResidual { matrix_a, vector_b }
    }

    /// Evaluates the residual ||Ax - b||^2.
    pub fn value(&self, x: &Vector) -> f64 {
        let mut result = 0.0;

        for i in self.matrix_a.lower_row()..=self.matrix_a.upper_row() {
            let mut residual = -self.vector_b[i];
            for j in self.matrix_a.lower_col()..=self.matrix_a.upper_col() {
                residual += self.matrix_a[(i, j)] * x[j];
            }
            result += residual * residual;
        }

        result
    }

    /// Evaluates the gradient: g = 2 * A^T * (Ax - b).
    pub fn gradient(&self, x: &Vector, g: &mut Vector) -> bool {
        let m = self.matrix_a.upper_row() - self.matrix_a.lower_row() + 1;
        let mut residual = Vector::new(1, m);

        // Compute residual r = Ax - b
        for i in self.matrix_a.lower_row()..=self.matrix_a.upper_row() {
            residual[i] = -self.vector_b[i];
            for j in self.matrix_a.lower_col()..=self.matrix_a.upper_col() {
                residual[i] += self.matrix_a[(i, j)] * x[j];
            }
        }

        // Compute g = 2 * A^T * r
        for j in self.matrix_a.lower_col()..=self.matrix_a.upper_col() {
            g[j] = 0.0;
            for i in self.matrix_a.lower_row()..=self.matrix_a.upper_row() {
                g[j] += 2.0 * self.matrix_a[(i, j)] * residual[i];
            }
        }

        true
    }

    /// Evaluates both value and gradient.
    pub fn values(&self, x: &Vector, g: &mut Vector) -> (f64, bool) {
        let y = self.value(x);
        let success = self.gradient(x, g);
        (y, success)
    }
}

// occt-ref: MathUtils_SystemLambda
/// Nonlinear system functor: F(x) = [f1(x), f2(x), ..., fn(x)].
pub struct SystemLambda<F>
where
    F: Fn(&Vector, &mut Vector) -> bool,
{
    lambda: F,
    nb_equations: usize,
}

impl<F> SystemLambda<F>
where
    F: Fn(&Vector, &mut Vector) -> bool,
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
    pub fn value(&self, x: &Vector, f: &mut Vector) -> bool {
        (self.lambda)(x, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let mut v = Vector::new(1, 3);
        assert_eq!(v.lower(), 1);
        assert_eq!(v.upper(), 3);
        assert_eq!(v.len(), 3);

        v.set(1, 1.5);
        v.set(2, 2.5);
        v.set(3, 3.5);

        assert_eq!(v.at(1), 1.5);
        assert_eq!(v[2], 2.5);
        assert_eq!(v[3], 3.5);
    }

    #[test]
    fn test_sphere() {
        let sphere = Sphere;
        let mut x = Vector::new(1, 3);
        x.set(1, 1.0);
        x.set(2, 2.0);
        x.set(3, 2.0);

        // f(1,2,2) = 1 + 4 + 4 = 9
        assert_eq!(sphere.value(&x), 9.0);

        let mut g = Vector::new(1, 3);
        sphere.gradient(&x, &mut g);
        assert_eq!(g[1], 2.0);
        assert_eq!(g[2], 4.0);
        assert_eq!(g[3], 4.0);
    }

    #[test]
    fn test_rosenbrock() {
        let rosen = Rosenbrock::default(); // a=1, b=100
        let mut x = Vector::new(1, 2);
        x.set(1, 1.0);
        x.set(2, 1.0);

        // At (1,1): f(1,1) = (1-1)^2 + 100*(1-1)^2 = 0
        assert_eq!(rosen.value(&x), 0.0);

        let mut g = Vector::new(1, 2);
        rosen.gradient(&x, &mut g);
        assert_eq!(g[1], 0.0);
        assert_eq!(g[2], 0.0);
    }

    #[test]
    fn test_booth() {
        let booth = Booth;
        let mut x = Vector::new(1, 2);
        x.set(1, 1.0);
        x.set(2, 3.0);

        // At (1,3): f(1,3) = (1+6-7)^2 + (2+3-5)^2 = 0
        assert_eq!(booth.value(&x), 0.0);

        let mut g = Vector::new(1, 2);
        booth.gradient(&x, &mut g);
        assert_eq!(g[1], 0.0);
        assert_eq!(g[2], 0.0);
    }

    #[test]
    fn test_rastrigin() {
        let rast = Rastrigin::default();
        let mut x = Vector::new(1, 2);
        x.set(1, 0.0);
        x.set(2, 0.0);

        // At origin: f(0,0) = A*n + sum(x[i]^2 - A*cos(2*pi*x[i]))
        //                   = 10*2 + (0 - 10*cos(0)) + (0 - 10*cos(0))
        //                   = 20 + (-10) + (-10) = 0
        assert_eq!(rast.value(&x), 0.0);

        let mut g = Vector::new(1, 2);
        rast.gradient(&x, &mut g);
        // df/dx1 = 2*0 + 10*2pi*sin(0) = 0
        // df/dx2 = 2*0 + 10*2pi*sin(0) = 0
        assert!((g[1]).abs() < 1e-10);
        assert!((g[2]).abs() < 1e-10);
    }

    #[test]
    fn test_quadratic_form() {
        let mut a = Matrix::new(1, 2, 1, 2);
        a.set(1, 1, 2.0);
        a.set(1, 2, 0.0);
        a.set(2, 1, 0.0);
        a.set(2, 2, 2.0);

        let mut b = Vector::new(1, 2);
        b.set(1, -4.0);
        b.set(2, -4.0);

        let quad = QuadraticForm::new(a, b, 8.0);

        let mut x = Vector::new(1, 2);
        x.set(1, 1.0);
        x.set(2, 1.0);

        // f(1,1) = (1,1) * [[2,0],[0,2]] * (1,1) + (-4,-4)^T (1,1) + 8
        //        = (2 + 2) + (-4 - 4) + 8 = 4
        assert_eq!(quad.value(&x), 4.0);
    }

    #[test]
    fn test_himmelblau() {
        let hbl = Himmelblau;
        let mut x = Vector::new(1, 2);
        x.set(1, 3.0);
        x.set(2, 2.0);

        // At (3,2): f(3,2) = (9+2-11)^2 + (3+4-7)^2 = 0
        assert_eq!(hbl.value(&x), 0.0);

        let mut g = Vector::new(1, 2);
        hbl.gradient(&x, &mut g);
        assert_eq!(g[1], 0.0);
        assert_eq!(g[2], 0.0);
    }

    #[test]
    fn test_beale() {
        let beale = Beale;
        let mut x = Vector::new(1, 2);
        x.set(1, 3.0);
        x.set(2, 0.5);

        // At (3, 0.5):
        // t1 = 1.5 - 3 + 3*0.5 = 0
        // t2 = 2.25 - 3 + 3*0.25 = 0
        // t3 = 2.625 - 3 + 3*0.125 = 0
        // f = 0
        assert!((beale.value(&x)).abs() < 1e-10);
    }

    #[test]
    fn test_linear_residual() {
        let mut a = Matrix::new(1, 2, 1, 1);
        a.set(1, 1, 1.0);
        a.set(2, 1, 2.0);

        let mut b = Vector::new(1, 2);
        b.set(1, 1.0);
        b.set(2, 2.0);

        let linear_res = LinearResidual::new(a, b);

        let mut x = Vector::new(1, 1);
        x.set(1, 1.0);

        // Residual: (1*1 - 1)^2 + (2*1 - 2)^2 = 0
        assert_eq!(linear_res.value(&x), 0.0);
    }
}

// FILE: math_utils_functor_vector_o.rs
// occt: MathUtils::QuadraticForm, MathUtils::Rosenbrock, MathUtils::Sphere, MathUtils::Booth, MathUtils::Beale, MathUtils::Himmelblau, MathUtils::Rastrigin, MathUtils::Ackley, MathUtils::LinearResidual

/// Quadratic form functor: f(x) = x^T A x + b^T x + c.
pub struct QuadraticForm {
    a: Vec<Vec<f64>>,
    b: Vec<f64>,
    c: f64,
}

impl QuadraticForm {
    /// Constructor from matrix, vector, and constant.
    pub fn new(a: &[&[f64]], b: &[f64], c: f64) -> Self {
        QuadraticForm {
            a: a.iter().map(|row| row.to_vec()).collect(),
            b: b.to_vec(),
            c,
        }
    }

    /// Evaluates f(x) = x^T A x + b^T x + c.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        let n = x.len();
        if self.a.is_empty() || self.a[0].len() != n || self.b.len() != n {
            return None;
        }

        let mut result = self.c;
        for i in 0..n {
            for j in 0..n {
                result += x[i] * self.a[i][j] * x[j];
            }
        }
        for i in 0..n {
            result += self.b[i] * x[i];
        }
        Some(result)
    }

    /// Evaluates gradient: g = (A + A^T) * x + b.
    pub fn gradient(&self, x: &[f64]) -> Option<Vec<f64>> {
        let n = x.len();
        if self.a.is_empty() || self.a[0].len() != n || self.b.len() != n {
            return None;
        }

        let mut g = vec![0.0; n];
        for i in 0..n {
            g[i] = self.b[i];
            for j in 0..n {
                g[i] += (self.a[i][j] + self.a[j][i]) * x[j];
            }
        }
        Some(g)
    }
}

/// Rosenbrock function: f(x,y) = (a - x)^2 + b*(y - x^2)^2
pub struct Rosenbrock {
    a: f64,
    b: f64,
}

impl Rosenbrock {
    /// Constructor with parameters.
    pub fn new(a: f64, b: f64) -> Self {
        Rosenbrock { a, b }
    }

    /// Evaluates the function.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        if x.len() < 2 {
            return None;
        }
        let t1 = self.a - x[0];
        let t2 = x[1] - x[0] * x[0];
        Some(t1 * t1 + self.b * t2 * t2)
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &[f64]) -> Option<Vec<f64>> {
        if x.len() < 2 {
            return None;
        }
        let t2 = x[1] - x[0] * x[0];
        let g0 = -2.0 * (self.a - x[0]) - 4.0 * self.b * x[0] * t2;
        let g1 = 2.0 * self.b * t2;
        Some(vec![g0, g1])
    }
}

/// Sphere function: f(x) = sum(x[i]^2)
pub struct Sphere;

impl Sphere {
    /// Evaluates the function.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        let mut result = 0.0;
        for xi in x {
            result += xi * xi;
        }
        Some(result)
    }

    /// Evaluates the gradient: g[i] = 2*x[i]
    pub fn gradient(&self, x: &[f64]) -> Option<Vec<f64>> {
        Some(x.iter().map(|xi| 2.0 * xi).collect())
    }
}

/// Booth function: f(x,y) = (x + 2y - 7)^2 + (2x + y - 5)^2
pub struct Booth;

impl Booth {
    /// Evaluates the function.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        if x.len() < 2 {
            return None;
        }
        let t1 = x[0] + 2.0 * x[1] - 7.0;
        let t2 = 2.0 * x[0] + x[1] - 5.0;
        Some(t1 * t1 + t2 * t2)
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &[f64]) -> Option<Vec<f64>> {
        if x.len() < 2 {
            return None;
        }
        let t1 = x[0] + 2.0 * x[1] - 7.0;
        let t2 = 2.0 * x[0] + x[1] - 5.0;
        Some(vec![2.0 * t1 + 4.0 * t2, 4.0 * t1 + 2.0 * t2])
    }
}

/// Beale function: f(x,y) = (1.5 - x + xy)^2 + (2.25 - x + xy^2)^2 + (2.625 - x + xy^3)^2
pub struct Beale;

impl Beale {
    /// Evaluates the function.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        if x.len() < 2 {
            return None;
        }
        let x0 = x[0];
        let x1 = x[1];
        let t1 = 1.5 - x0 + x0 * x1;
        let t2 = 2.25 - x0 + x0 * x1 * x1;
        let t3 = 2.625 - x0 + x0 * x1 * x1 * x1;
        Some(t1 * t1 + t2 * t2 + t3 * t3)
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &[f64]) -> Option<Vec<f64>> {
        if x.len() < 2 {
            return None;
        }
        let x0 = x[0];
        let x1 = x[1];
        let x1_2 = x1 * x1;
        let x1_3 = x1_2 * x1;
        let t1 = 1.5 - x0 + x0 * x1;
        let t2 = 2.25 - x0 + x0 * x1_2;
        let t3 = 2.625 - x0 + x0 * x1_3;
        let g0 = 2.0 * ((x1 - 1.0) * t1 + (x1_2 - 1.0) * t2 + (x1_3 - 1.0) * t3);
        let g1 = 2.0 * x0 * (t1 + 2.0 * x1 * t2 + 3.0 * x1_2 * t3);
        Some(vec![g0, g1])
    }
}

/// Himmelblau function: f(x,y) = (x^2 + y - 11)^2 + (x + y^2 - 7)^2
pub struct Himmelblau;

impl Himmelblau {
    /// Evaluates the function.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        if x.len() < 2 {
            return None;
        }
        let t1 = x[0] * x[0] + x[1] - 11.0;
        let t2 = x[0] + x[1] * x[1] - 7.0;
        Some(t1 * t1 + t2 * t2)
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &[f64]) -> Option<Vec<f64>> {
        if x.len() < 2 {
            return None;
        }
        let t1 = x[0] * x[0] + x[1] - 11.0;
        let t2 = x[0] + x[1] * x[1] - 7.0;
        let g0 = 4.0 * x[0] * t1 + 2.0 * t2;
        let g1 = 2.0 * t1 + 4.0 * x[1] * t2;
        Some(vec![g0, g1])
    }
}

/// Rastrigin function: f(x) = A*n + sum(x[i]^2 - A*cos(2*pi*x[i]))
pub struct Rastrigin {
    a: f64,
}

impl Rastrigin {
    /// Constructor with parameter.
    pub fn new(a: f64) -> Self {
        Rastrigin { a }
    }

    /// Evaluates the function.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        let two_pi = 2.0 * std::f64::consts::PI;
        let n = x.len() as f64;
        let mut result = self.a * n;
        for xi in x {
            result += xi * xi - self.a * (two_pi * xi).cos();
        }
        Some(result)
    }

    /// Evaluates the gradient.
    pub fn gradient(&self, x: &[f64]) -> Option<Vec<f64>> {
        let two_pi = 2.0 * std::f64::consts::PI;
        Some(
            x.iter()
                .map(|xi| 2.0 * xi + self.a * two_pi * (two_pi * xi).sin())
                .collect(),
        )
    }
}

/// Ackley function: f(x) = -a*exp(-b*sqrt(sum(x[i]^2)/n)) - exp(sum(cos(c*x[i]))/n) + a + e
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

    /// Evaluates the function.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        let n = x.len() as f64;
        let mut sum_sq = 0.0;
        let mut sum_cos = 0.0;
        for xi in x {
            sum_sq += xi * xi;
            sum_cos += (self.c * xi).cos();
        }
        let e = std::f64::consts::E;
        Some(-self.a * (-self.b * (sum_sq / n).sqrt()).exp() - (sum_cos / n).exp() + self.a + e)
    }
}

/// Linear residual: f(x) = ||Ax - b||^2
pub struct LinearResidual {
    a: Vec<Vec<f64>>,
    b: Vec<f64>,
}

impl LinearResidual {
    /// Constructor from matrix and rhs.
    pub fn new(a: &[&[f64]], b: &[f64]) -> Self {
        LinearResidual {
            a: a.iter().map(|row| row.to_vec()).collect(),
            b: b.to_vec(),
        }
    }

    /// Evaluates ||Ax - b||^2.
    pub fn value(&self, x: &[f64]) -> Option<f64> {
        if self.a.is_empty() {
            return Some(0.0);
        }
        let m = self.a.len();
        let n = self.a[0].len();
        if x.len() != n || self.b.len() != m {
            return None;
        }

        let mut result = 0.0;
        for i in 0..m {
            let mut ax_i = -self.b[i];
            for j in 0..n {
                ax_i += self.a[i][j] * x[j];
            }
            result += ax_i * ax_i;
        }
        Some(result)
    }

    /// Evaluates gradient: g = 2 * A^T * (Ax - b).
    pub fn gradient(&self, x: &[f64]) -> Option<Vec<f64>> {
        if self.a.is_empty() {
            return Some(vec![0.0; x.len()]);
        }
        let m = self.a.len();
        let n = self.a[0].len();
        if x.len() != n || self.b.len() != m {
            return None;
        }

        let mut g = vec![0.0; n];
        for i in 0..m {
            let mut residual = -self.b[i];
            for j in 0..n {
                residual += self.a[i][j] * x[j];
            }
            for j in 0..n {
                g[j] += 2.0 * self.a[i][j] * residual;
            }
        }
        Some(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic_form() {
        let a = vec![&[2.0, 0.0][..], &[0.0, 2.0][..]];
        let b = vec![-4.0, -4.0];
        let qf = QuadraticForm::new(&a, &b, 8.0);
        let x = vec![1.0, 1.0];
        let val = qf.value(&x).unwrap();
        assert!((val - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_rosenbrock() {
        let ros = Rosenbrock::new(1.0, 100.0);
        let x = vec![1.0, 1.0];
        let val = ros.value(&x).unwrap();
        assert!(val.abs() < 1e-10);
    }

    #[test]
    fn test_sphere() {
        let sphere = Sphere;
        let x = vec![1.0, 2.0, 2.0];
        let val = sphere.value(&x).unwrap();
        assert!((val - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_booth() {
        let booth = Booth;
        let x = vec![1.0, 3.0];
        let val = booth.value(&x).unwrap();
        assert!(val.abs() < 1e-10);
    }

    #[test]
    fn test_himmelblau() {
        let himmel = Himmelblau;
        let x = vec![3.0, 2.0];
        let val = himmel.value(&x).unwrap();
        assert!(val.abs() < 1e-10);
    }

    #[test]
    fn test_rastrigin() {
        let rast = Rastrigin::new(10.0);
        let x = vec![0.0, 0.0];
        let val = rast.value(&x).unwrap();
        assert!(val.abs() < 1e-10);
    }

    #[test]
    fn test_linear_residual() {
        let a = vec![&[1.0, 0.0][..], &[0.0, 1.0][..]];
        let b = vec![1.0, 1.0];
        let lr = LinearResidual::new(&a, &b);
        let x = vec![1.0, 1.0];
        let val = lr.value(&x).unwrap();
        assert!(val.abs() < 1e-10);
    }
}

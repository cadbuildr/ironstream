// FILE: app_cont_least_square_o.rs
// occt: AppCont_LeastSquare

//! Least squares approximation for continuum functions.

pub struct LeastSquareApproximation {
    pub coefficients: Vec<f64>,
    pub residual: f64,
}

impl LeastSquareApproximation {
    pub fn new(points: &[(f64, f64)], degree: usize) -> Self {
        let n = points.len();
        let m = degree + 1;

        if n < m {
            return LeastSquareApproximation {
                coefficients: vec![0.0; m],
                residual: f64::INFINITY,
            };
        }

        // Build normal equations
        let mut A = vec![vec![0.0; m]; m];
        let mut b = vec![0.0; m];

        for (x, y) in points {
            let mut xp = 1.0;
            for i in 0..m {
                for j in 0..m {
                    A[i][j] += xp * (x.powi(j as i32));
                }
                b[i] += y * xp;
                xp *= x;
            }
        }

        // Solve normal equations (simple approach)
        let mut coefficients = vec![0.0; m];
        if let Some(sol) = gaussian_elimination(&A, &b) {
            coefficients = sol;
        }

        // Compute residual
        let mut residual = 0.0;
        for (x, y) in points {
            let mut yhat = 0.0;
            let mut xp = 1.0;
            for i in 0..m {
                yhat += coefficients[i] * xp;
                xp *= x;
            }
            residual += (y - yhat).powi(2);
        }

        LeastSquareApproximation {
            coefficients,
            residual,
        }
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        let mut result = 0.0;
        let mut xp = 1.0;
        for &coeff in &self.coefficients {
            result += coeff * xp;
            xp *= x;
        }
        result
    }
}

fn gaussian_elimination(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let n = a.len();
    let mut a_copy = a.to_vec();
    let mut b_copy = b.to_vec();

    for i in 0..n {
        let mut max_row = i;
        for k in (i + 1)..n {
            if a_copy[k][i].abs() > a_copy[max_row][i].abs() {
                max_row = k;
            }
        }

        if a_copy[max_row][i].abs() < 1e-15 {
            return None;
        }

        a_copy.swap(i, max_row);
        b_copy.swap(i, max_row);

        for k in (i + 1)..n {
            let factor = a_copy[k][i] / a_copy[i][i];
            for j in i..n {
                a_copy[k][j] -= factor * a_copy[i][j];
            }
            b_copy[k] -= factor * b_copy[i];
        }
    }

    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = b_copy[i];
        for j in (i + 1)..n {
            sum -= a_copy[i][j] * x[j];
        }
        x[i] = sum / a_copy[i][i];
    }

    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_square() {
        let points = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)];
        let approx = LeastSquareApproximation::new(&points, 2);
        assert!(!approx.coefficients.is_empty());
    }

    #[test]
    fn test_evaluate() {
        let points = vec![(0.0, 0.0), (1.0, 1.0)];
        let approx = LeastSquareApproximation::new(&points, 1);
        let y = approx.evaluate(0.5);
        assert!(y.is_finite());
    }
}

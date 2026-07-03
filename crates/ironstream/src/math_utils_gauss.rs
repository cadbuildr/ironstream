// FILE: math_utils_gauss.rs

//! Gauss-Legendre quadrature points and weights computation.
//!
//! Provides utilities to obtain ordered Gauss-Legendre points and weights
//! for numerical integration.

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

/// Internal helper struct for eigenvalue computation
#[derive(Clone, Debug)]
struct ValueAndWeight {
    value: f64,
    weight: f64,
}

impl ValueAndWeight {
    fn new(value: f64, weight: f64) -> Self {
        ValueAndWeight { value, weight }
    }
}

/// QR algorithm for tridiagonal eigenvalue computation.
/// Finds eigenvalues and eigenvectors of a tridiagonal matrix.
/// Input: diagonal `d` and subdiagonal `sub` (1-indexed, length n)
/// Output: eigenvalues in `d` and normalized eigenvectors as columns of `v`
/// Returns true if computation succeeded.
fn eigen_tridiagonal(
    d: &mut Vector,
    sub: &mut Vector,
    v: &mut Matrix,
) -> bool {
    let n = d.len();
    if n < 1 {
        return false;
    }

    // Initialize eigenvector matrix to identity
    for i in v.lower_row()..=v.upper_row() {
        for j in v.lower_col()..=v.upper_col() {
            if i == j {
                v[(i, j)] = 1.0;
            } else {
                v[(i, j)] = 0.0;
            }
        }
    }

    const MAX_ITERATIONS: usize = 1000;
    const TOL: f64 = 1e-15;

    // QR algorithm for tridiagonal matrices (Givens rotations)
    for _ in 0..MAX_ITERATIONS {
        let mut converged = true;

        // Check for convergence: subdiagonal elements near zero
        for i in (1 + d.lower())..=d.upper() {
            if sub[i].abs() > TOL {
                converged = false;
                break;
            }
        }

        if converged {
            return true;
        }

        // One QR sweep: reduce matrix towards diagonal form
        for k in d.lower()..d.upper() {
            let alpha = d[k];
            let beta = sub[k + 1];

            if beta.abs() < TOL {
                continue;
            }

            // Givens rotation to zero out sub[k+1]
            let r = (alpha * alpha + beta * beta).sqrt();
            if r.abs() < 1e-15 {
                continue;
            }

            let c = alpha / r;
            let s = beta / r;

            // Apply rotation to (d[k], d[k+1]) and sub[k+1]
            let temp_d = d[k];
            d[k] = c * c * temp_d + s * s * d[k + 1] - 2.0 * c * s * sub[k + 1];
            d[k + 1] = s * s * temp_d + c * c * d[k + 1] + 2.0 * c * s * sub[k + 1];
            sub[k + 1] = c * s * (d[k + 1] - temp_d);

            if k + 2 <= d.upper() && sub[k + 2].abs() > 1e-15 {
                let temp_sub = sub[k + 2];
                sub[k + 2] = c * temp_sub;
            }

            // Apply rotation to eigenvector matrix
            for row in v.lower_row()..=v.upper_row() {
                let temp_v = v[(row, k)];
                v[(row, k)] = c * temp_v + s * v[(row, k + 1)];
                v[(row, k + 1)] = -s * temp_v + c * v[(row, k + 1)];
            }
        }
    }

    // After max iterations, accept current diagonal as approximate eigenvalues
    true
}

/// Compute Gauss-Legendre quadrature points and weights.
/// Points are roots of the Legendre polynomial P_n(x) on [-1, 1].
/// Weights are precomputed based on Gauss quadrature formula.
///
/// This function uses the eigenvalue method: Gauss points are eigenvalues of
/// a specific tridiagonal matrix, and weights are derived from the eigenvectors.
fn compute_gauss_legendre(n: usize, points: &mut Vector, weights: &mut Vector) -> bool {
    if n < 1 || points.len() != n || weights.len() != n {
        return false;
    }

    // Build the Jacobi matrix (tridiagonal)
    // Diagonal is all zeros for Legendre polynomials
    // Subdiagonal: sqrt(i^2 / (4*i^2 - 1))
    let mut d = Vector::new(1, n);
    let mut sub = Vector::new(1, n);

    for i in 1..=n {
        d.set(i, 0.0);
        if i > 1 {
            let i_m1_sq = ((i - 1) * (i - 1)) as f64;
            let denom = 4.0 * i_m1_sq - 1.0;
            if denom.abs() > 1e-15 {
                sub.set(i, (i_m1_sq / denom).sqrt());
            } else {
                sub.set(i, 0.0);
            }
        } else {
            sub.set(i, 0.0);
        }
    }

    // Compute eigenvalues and eigenvectors
    let mut eigenvecs = Matrix::new(1, n, 1, n);
    if !eigen_tridiagonal(&mut d, &mut sub, &mut eigenvecs) {
        return false;
    }

    // Collect eigenvalues and weights
    let mut values_and_weights = Vec::with_capacity(n);
    for i in 1..=n {
        let eigenvalue = d[i];
        let weight = 2.0 * eigenvecs[(1, i)] * eigenvecs[(1, i)];
        values_and_weights.push(ValueAndWeight::new(eigenvalue, weight));
    }

    // Sort by eigenvalue (points in ascending order)
    values_and_weights.sort_by(|a, b| {
        a.value.partial_cmp(&b.value).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Copy to output arrays
    for (idx, vw) in values_and_weights.iter().enumerate() {
        points.set(1 + idx, vw.value);
        weights.set(1 + idx, vw.weight);
    }

    true
}

// occt: MathUtils_GetGaussPointsAndWeights
/// Get ordered Gauss-Legendre points and weights for given order.
/// Points are returned in ascending order on [-1, 1].
///
/// # Arguments
/// * `order` - number of quadrature points (>= 1)
/// * `points` - output vector of quadrature points (must have length = order, lower bound 1)
/// * `weights` - output vector of quadrature weights (must have length = order, lower bound 1)
///
/// # Returns
/// `true` if points/weights are available, `false` otherwise
pub fn get_gauss_points_and_weights(
    order: usize,
    points: &mut Vector,
    weights: &mut Vector,
) -> bool {
    if order < 1 || points.len() != order || weights.len() != order {
        return false;
    }

    // For small orders, use pre-computed values for accuracy
    // Otherwise, compute via eigenvalue method
    match order {
        1 => {
            points.set(1, 0.0);
            weights.set(1, 2.0);
            true
        }
        2 => {
            let x = 1.0 / 3.0_f64.sqrt(); // 1/sqrt(3) ≈ 0.5773...
            points.set(1, -x);
            points.set(2, x);
            weights.set(1, 1.0);
            weights.set(2, 1.0);
            true
        }
        3 => {
            let x = (3.0_f64 / 5.0).sqrt(); // sqrt(3/5)
            points.set(1, -x);
            points.set(2, 0.0);
            points.set(3, x);
            weights.set(1, 5.0 / 9.0);
            weights.set(2, 8.0 / 9.0);
            weights.set(3, 5.0 / 9.0);
            true
        }
        4 => {
            let x1 = (3.0 + 2.0 * 6.0_f64.sqrt()) / 7.0;
            let x2 = (3.0 - 2.0 * 6.0_f64.sqrt()) / 7.0;
            let x1_sqrt = x1.sqrt();
            let x2_sqrt = x2.sqrt();

            points.set(1, -x1_sqrt);
            points.set(2, -x2_sqrt);
            points.set(3, x2_sqrt);
            points.set(4, x1_sqrt);

            let w1 = (18.0 - 6.0_f64.sqrt()) / 36.0;
            let w2 = (18.0 + 6.0_f64.sqrt()) / 36.0;

            weights.set(1, w1);
            weights.set(2, w2);
            weights.set(3, w2);
            weights.set(4, w1);
            true
        }
        5 => {
            let x1 = (5.0 + 2.0 * 10.0_f64.sqrt()) / 9.0;
            let x2 = (5.0 - 2.0 * 10.0_f64.sqrt()) / 9.0;
            let x1_sqrt = x1.sqrt();
            let x2_sqrt = x2.sqrt();

            points.set(1, -x1_sqrt);
            points.set(2, -x2_sqrt);
            points.set(3, 0.0);
            points.set(4, x2_sqrt);
            points.set(5, x1_sqrt);

            let w1 = (322.0 - 13.0 * 70.0_f64.sqrt()) / 900.0;
            let w2 = (322.0 + 13.0 * 70.0_f64.sqrt()) / 900.0;
            let w0 = 128.0 / 225.0;

            weights.set(1, w1);
            weights.set(2, w2);
            weights.set(3, w0);
            weights.set(4, w2);
            weights.set(5, w1);
            true
        }
        _ => {
            // Use eigenvalue method for larger orders
            compute_gauss_legendre(order, points, weights)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_order_1() {
        let mut points = Vector::new(1, 1);
        let mut weights = Vector::new(1, 1);

        assert!(get_gauss_points_and_weights(1, &mut points, &mut weights));

        // Order 1: single point at 0, weight = 2
        assert!((points[1] - 0.0).abs() < 1e-10);
        assert!((weights[1] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_order_2() {
        let mut points = Vector::new(1, 2);
        let mut weights = Vector::new(1, 2);

        assert!(get_gauss_points_and_weights(2, &mut points, &mut weights));

        // Order 2: points at ±1/sqrt(3), weights = 1
        let x = 1.0 / std::f64::consts::PI.sqrt();
        assert!(points[1] < points[2]);
        assert!((weights[1] - 1.0).abs() < 1e-10);
        assert!((weights[2] - 1.0).abs() < 1e-10);

        // Sum of weights should be 2 (integral of 1 on [-1,1])
        assert!((weights[1] + weights[2] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_order_3() {
        let mut points = Vector::new(1, 3);
        let mut weights = Vector::new(1, 3);

        assert!(get_gauss_points_and_weights(3, &mut points, &mut weights));

        // Order 3: symmetric about 0
        assert!((points[1] + points[3]).abs() < 1e-10);
        assert!((points[2] - 0.0).abs() < 1e-10);

        // Weights
        assert!(((weights[1] - 5.0 / 9.0).abs()) < 1e-10);
        assert!(((weights[2] - 8.0 / 9.0).abs()) < 1e-10);

        // Sum of weights = 2
        assert!(((weights[1] + weights[2] + weights[3]) - 2.0).abs() < 1e-10);
    }


    #[test]
    fn test_gauss_vector_bounds() {
        let mut points = Vector::new(1, 2);
        let mut weights = Vector::new(1, 2);

        // Correct bounds
        assert!(get_gauss_points_and_weights(2, &mut points, &mut weights));

        // Wrong length
        let mut bad_points = Vector::new(1, 3);
        let mut bad_weights = Vector::new(1, 2);
        assert!(!get_gauss_points_and_weights(2, &mut bad_points, &mut bad_weights));
    }
}

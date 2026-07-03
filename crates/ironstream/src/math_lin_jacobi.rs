// FILE: math_lin_jacobi.rs
// occt: MathLin_Jacobi

use std::f64;

/// Minimal math_Vector implementation for Rust
#[derive(Clone, Debug)]
pub struct Vector {
    lower: i32,
    upper: i32,
    data: Vec<f64>,
}

impl Vector {
    pub fn new(lower: i32, upper: i32, val: f64) -> Self {
        let len = (upper - lower + 1) as usize;
        Vector {
            lower,
            upper,
            data: vec![val; len],
        }
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, i: i32) -> f64 {
        let idx = (i - self.lower) as usize;
        self.data[idx]
    }

    pub fn set(&mut self, i: i32, val: f64) {
        let idx = (i - self.lower) as usize;
        self.data[idx] = val;
    }
}

impl std::ops::Index<i32> for Vector {
    type Output = f64;
    fn index(&self, i: i32) -> &f64 {
        let idx = (i - self.lower) as usize;
        &self.data[idx]
    }
}

impl std::ops::IndexMut<i32> for Vector {
    fn index_mut(&mut self, i: i32) -> &mut f64 {
        let idx = (i - self.lower) as usize;
        &mut self.data[idx]
    }
}

/// Minimal math_Matrix implementation for Rust
#[derive(Clone, Debug)]
pub struct Matrix {
    row_lower: i32,
    row_upper: i32,
    col_lower: i32,
    col_upper: i32,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32, val: f64) -> Self {
        let rows = (row_upper - row_lower + 1) as usize;
        let cols = (col_upper - col_lower + 1) as usize;
        Matrix {
            row_lower,
            row_upper,
            col_lower,
            col_upper,
            data: vec![val; rows * cols],
        }
    }

    pub fn lower_row(&self) -> i32 {
        self.row_lower
    }

    pub fn upper_row(&self) -> i32 {
        self.row_upper
    }

    pub fn lower_col(&self) -> i32 {
        self.col_lower
    }

    pub fn upper_col(&self) -> i32 {
        self.col_upper
    }

    pub fn row_number(&self) -> usize {
        (self.row_upper - self.row_lower + 1) as usize
    }

    pub fn col_number(&self) -> usize {
        (self.col_upper - self.col_lower + 1) as usize
    }

    fn index_linear(&self, i: i32, j: i32) -> usize {
        let row = (i - self.row_lower) as usize;
        let col = (j - self.col_lower) as usize;
        let cols = self.col_number();
        row * cols + col
    }

    pub fn get(&self, i: i32, j: i32) -> f64 {
        let idx = self.index_linear(i, j);
        self.data[idx]
    }

    pub fn set(&mut self, i: i32, j: i32, val: f64) {
        let idx = self.index_linear(i, j);
        self.data[idx] = val;
    }
}

impl std::ops::Index<(i32, i32)> for Matrix {
    type Output = f64;
    fn index(&self, (i, j): (i32, i32)) -> &f64 {
        let idx = self.index_linear(i, j);
        &self.data[idx]
    }
}

impl std::ops::IndexMut<(i32, i32)> for Matrix {
    fn index_mut(&mut self, (i, j): (i32, i32)) -> &mut f64 {
        let idx = self.index_linear(i, j);
        &mut self.data[idx]
    }
}

/// Computation status for solvers
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    OK,
    NotConverged,
    InvalidInput,
    NumericalError,
}

impl Default for Status {
    fn default() -> Self {
        Status::NotConverged
    }
}

/// Result structure for eigenvalue/eigenvector computation
#[derive(Clone, Debug)]
pub struct EigenResult {
    pub status: Status,
    pub eigen_values: Option<Vector>,
    pub eigen_vectors: Option<Matrix>,
    pub nb_iterations: usize,
}

impl EigenResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

/// Compute eigenvalues and eigenvectors of a symmetric matrix
/// using the Jacobi iterative method.
///
/// The Jacobi method applies a sequence of plane rotations (Givens rotations)
/// to diagonalize the symmetric matrix A:
///   A' = R^T * A * R
/// where R is a rotation that zeroes one off-diagonal element.
///
/// After convergence, A is diagonal with eigenvalues on the diagonal,
/// and the accumulated rotations form the eigenvector matrix.
pub fn jacobi(a: &Matrix, sort_descending: bool) -> EigenResult {
    let mut result = EigenResult {
        status: Status::NotConverged,
        eigen_values: None,
        eigen_vectors: None,
        nb_iterations: 0,
    };

    let row_lower = a.lower_row();
    let row_upper = a.upper_row();
    let col_lower = a.lower_col();
    let col_upper = a.upper_col();
    let n = row_upper - row_lower + 1;

    // Check for square matrix
    if col_upper - col_lower + 1 != n {
        result.status = Status::InvalidInput;
        return result;
    }

    // Create 1-based working copies as required by the Jacobi algorithm
    let mut work_a = Matrix::new(1, n, 1, n, 0.0);
    let mut eigen_vals = Vector::new(1, n, 0.0);
    let mut eigen_vecs = Matrix::new(1, n, 1, n, 0.0);

    // Copy input to working matrix
    for i in row_lower..=row_upper {
        for j in col_lower..=col_upper {
            work_a.set(i - row_lower + 1, j - col_lower + 1, a.get(i, j));
        }
    }

    // Initialize eigenvector matrix as identity
    for i in 1..=n {
        for j in 1..=n {
            eigen_vecs.set(i, j, if i == j { 1.0 } else { 0.0 });
        }
    }

    // Jacobi eigenvalue algorithm
    let max_iterations = 50;
    let tolerance = 1e-15;

    for iteration in 0..max_iterations {
        // Find largest off-diagonal element
        let mut max_val = 0.0;
        let mut max_i = 1;
        let mut max_j = 2;

        for i in 1..=n {
            for j in (i + 1)..=n {
                let val = work_a.get(i, j).abs();
                if val > max_val {
                    max_val = val;
                    max_i = i;
                    max_j = j;
                }
            }
        }

        if max_val < tolerance {
            result.nb_iterations = iteration;
            break;
        }

        // Compute rotation angle to zero work_a[max_i, max_j]
        let aii = work_a.get(max_i, max_i);
        let ajj = work_a.get(max_j, max_j);
        let aij = work_a.get(max_i, max_j);

        let denom = aii - ajj;
        let theta = if denom.abs() < 1e-20 {
            std::f64::consts::PI / 4.0
        } else {
            0.5 * (2.0 * aij / denom).atan()
        };

        let c = theta.cos();
        let s = theta.sin();

        // Apply rotation to work_a
        let aii_new = c * c * aii + s * s * ajj - 2.0 * c * s * aij;
        let ajj_new = s * s * aii + c * c * ajj + 2.0 * c * s * aij;
        let aij_new = c * s * (aii - ajj) + (c * c - s * s) * aij;

        work_a.set(max_i, max_i, aii_new);
        work_a.set(max_j, max_j, ajj_new);
        work_a.set(max_i, max_j, aij_new);
        work_a.set(max_j, max_i, aij_new);

        // Update other elements
        for k in 1..=n {
            if k != max_i && k != max_j {
                let aik = work_a.get(k, max_i);
                let akj = work_a.get(k, max_j);

                let aik_new = c * aik - s * akj;
                let akj_new = s * aik + c * akj;

                work_a.set(k, max_i, aik_new);
                work_a.set(max_i, k, aik_new);
                work_a.set(k, max_j, akj_new);
                work_a.set(max_j, k, akj_new);
            }
        }

        // Update eigenvector matrix (accumulate rotations)
        for k in 1..=n {
            let vik = eigen_vecs.get(k, max_i);
            let vjk = eigen_vecs.get(k, max_j);

            let vik_new = c * vik - s * vjk;
            let vjk_new = s * vik + c * vjk;

            eigen_vecs.set(k, max_i, vik_new);
            eigen_vecs.set(k, max_j, vjk_new);
        }
    }

    // Extract eigenvalues from diagonal
    for i in 1..=n {
        eigen_vals.set(i, work_a.get(i, i));
    }

    // Sort eigenvalues and eigenvectors if requested
    if sort_descending && n > 1 {
        for i in 1..n {
            let mut max_idx = i;
            let mut max_val = eigen_vals.get(i);

            for j in (i + 1)..=n {
                if eigen_vals.get(j) > max_val {
                    max_val = eigen_vals.get(j);
                    max_idx = j;
                }
            }

            if max_idx != i {
                // Swap eigenvalues
                let tmp = eigen_vals.get(i);
                eigen_vals.set(i, eigen_vals.get(max_idx));
                eigen_vals.set(max_idx, tmp);

                // Swap corresponding eigenvector columns
                for k in 1..=n {
                    let tmp = eigen_vecs.get(k, i);
                    eigen_vecs.set(k, i, eigen_vecs.get(k, max_idx));
                    eigen_vecs.set(k, max_idx, tmp);
                }
            }
        }
    }

    // Copy results with original indexing
    let mut result_vals = Vector::new(row_lower, row_upper, 0.0);
    for i in 1..=n {
        result_vals.set(row_lower + i - 1, eigen_vals.get(i));
    }

    let mut result_vecs = Matrix::new(row_lower, row_upper, col_lower, col_upper, 0.0);
    for i in 1..=n {
        for j in 1..=n {
            result_vecs.set(row_lower + i - 1, col_lower + j - 1, eigen_vecs.get(i, j));
        }
    }

    result.eigen_values = Some(result_vals);
    result.eigen_vectors = Some(result_vecs);
    result.status = Status::OK;
    result
}

/// Compute only eigenvalues of a symmetric matrix (faster).
///
/// Uses the same Jacobi method but may be optimized to not store
/// eigenvectors if not needed.
pub fn eigen_values(a: &Matrix, sort_descending: bool) -> EigenResult {
    // Currently delegates to full Jacobi
    jacobi(a, sort_descending)
}

/// Compute spectral decomposition A = V * D * V^T.
///
/// For symmetric matrix A, decomposes into:
/// - V: orthogonal matrix of eigenvectors (columns)
/// - D: diagonal matrix of eigenvalues
///
/// Such that A = V * D * V^T
pub fn spectral_decomposition(a: &Matrix) -> EigenResult {
    jacobi(a, false)
}

/// Compute matrix power A^p for symmetric positive semi-definite matrix.
///
/// Uses spectral decomposition: A^p = V * D^p * V^T
/// where D^p is the diagonal matrix with eigenvalues raised to power p.
pub fn matrix_power(a: &Matrix, power: f64) -> Option<Matrix> {
    let eigen = jacobi(a, false);
    if !eigen.is_done() {
        return None;
    }

    let d = eigen.eigen_values.as_ref().unwrap();
    let v = eigen.eigen_vectors.as_ref().unwrap();

    let lower = d.lower();
    let upper = d.upper();

    // Compute D^p
    let mut dp = Vector::new(lower, upper, 0.0);
    for i in lower..=upper {
        let eigenval = d.get(i);
        if eigenval < 0.0 && power != power.floor() {
            // Can't take fractional power of negative eigenvalue
            return None;
        }
        let powered = if eigenval >= 0.0 {
            eigenval.powf(power)
        } else {
            -((-eigenval).powf(power))
        };
        dp.set(i, powered);
    }

    // Compute V * D^p * V^T
    let mut result = Matrix::new(lower, upper, lower, upper, 0.0);
    for i in lower..=upper {
        for j in lower..=upper {
            let mut sum = 0.0;
            for k in lower..=upper {
                sum += v.get(i, k) * dp.get(k) * v.get(j, k);
            }
            result.set(i, j, sum);
        }
    }

    Some(result)
}

/// Compute matrix square root of symmetric positive semi-definite matrix.
pub fn matrix_sqrt(a: &Matrix) -> Option<Matrix> {
    matrix_power(a, 0.5)
}

/// Compute matrix inverse square root of symmetric positive definite matrix.
pub fn matrix_inv_sqrt(a: &Matrix) -> Option<Matrix> {
    matrix_power(a, -0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jacobi_2x2_identity() {
        // Identity matrix has eigenvalues [1, 1]
        let mut a = Matrix::new(1, 2, 1, 2, 0.0);
        a.set(1, 1, 1.0);
        a.set(1, 2, 0.0);
        a.set(2, 1, 0.0);
        a.set(2, 2, 1.0);

        let result = jacobi(&a, true);
        assert!(result.is_done());
        assert!(result.eigen_values.is_some());
        assert!(result.eigen_vectors.is_some());

        let evals = result.eigen_values.unwrap();
        assert!((evals.get(1) - 1.0).abs() < 1e-10);
        assert!((evals.get(2) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_jacobi_2x2_diagonal() {
        // Diagonal matrix [[2, 0], [0, 3]]
        let mut a = Matrix::new(1, 2, 1, 2, 0.0);
        a.set(1, 1, 2.0);
        a.set(1, 2, 0.0);
        a.set(2, 1, 0.0);
        a.set(2, 2, 3.0);

        let result = jacobi(&a, true);
        assert!(result.is_done());

        let evals = result.eigen_values.unwrap();
        // Should be sorted descending
        assert!((evals.get(1) - 3.0).abs() < 1e-10);
        assert!((evals.get(2) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_jacobi_2x2_off_diagonal() {
        // [[1, 1], [1, 1]] has eigenvalues [2, 0]
        let mut a = Matrix::new(1, 2, 1, 2, 1.0);

        let result = jacobi(&a, true);
        assert!(result.is_done());

        let evals = result.eigen_values.unwrap();
        assert!((evals.get(1) - 2.0).abs() < 1e-10, "eval[1] = {}", evals.get(1));
        assert!(evals.get(2).abs() < 1e-10, "eval[2] = {}", evals.get(2));
    }

    #[test]
    fn test_jacobi_invalid_input() {
        // Non-square matrix should fail
        let a = Matrix::new(1, 2, 1, 3, 1.0);
        let result = jacobi(&a, true);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_jacobi_orthogonality() {
        // Check that eigenvectors are orthonormal
        let mut a = Matrix::new(1, 3, 1, 3, 0.0);
        a.set(1, 1, 4.0);
        a.set(1, 2, -1.0);
        a.set(1, 3, 0.0);
        a.set(2, 1, -1.0);
        a.set(2, 2, 3.0);
        a.set(2, 3, -1.0);
        a.set(3, 1, 0.0);
        a.set(3, 2, -1.0);
        a.set(3, 3, 2.0);

        let result = jacobi(&a, false);
        assert!(result.is_done());

        let evecs = result.eigen_vectors.unwrap();

        // Check orthonormality: V^T * V ≈ I
        for i in 1..=3 {
            for j in 1..=3 {
                let mut sum = 0.0;
                for k in 1..=3 {
                    sum += evecs.get(k, i) * evecs.get(k, j);
                }
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((sum - expected).abs() < 1e-10, "V^T*V[{},{}] = {}", i, j, sum);
            }
        }
    }

    #[test]
    fn test_matrix_sqrt() {
        // [[2, 0], [0, 2]] has sqrt [[sqrt(2), 0], [0, sqrt(2)]]
        let mut a = Matrix::new(1, 2, 1, 2, 0.0);
        a.set(1, 1, 2.0);
        a.set(1, 2, 0.0);
        a.set(2, 1, 0.0);
        a.set(2, 2, 2.0);

        let result = matrix_sqrt(&a);
        assert!(result.is_some());

        let sqrt_a = result.unwrap();
        let sqrt2 = 2.0_f64.sqrt();
        assert!((sqrt_a.get(1, 1) - sqrt2).abs() < 1e-10);
        assert!((sqrt_a.get(2, 2) - sqrt2).abs() < 1e-10);
        assert!(sqrt_a.get(1, 2).abs() < 1e-10);
    }

    #[test]
    fn test_eigen_values_sorting() {
        // Test that eigenvalues are sorted in descending order
        let mut a = Matrix::new(1, 3, 1, 3, 0.0);
        a.set(1, 1, 1.0);
        a.set(2, 2, 5.0);
        a.set(3, 3, 3.0);

        let result = jacobi(&a, true);
        assert!(result.is_done());

        let evals = result.eigen_values.unwrap();
        assert!((evals.get(1) - 5.0).abs() < 1e-10);
        assert!((evals.get(2) - 3.0).abs() < 1e-10);
        assert!((evals.get(3) - 1.0).abs() < 1e-10);
    }
}

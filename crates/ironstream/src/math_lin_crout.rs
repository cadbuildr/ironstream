// FILE: math_lin_crout.rs
// occt: MathLin_Crout

/// Computation status for linear solvers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Computation successful
    Ok,
    /// Did not converge
    NotConverged,
    /// Matrix is singular
    Singular,
    /// Invalid input
    InvalidInput,
}

/// Result for Crout LDL^T decomposition.
/// Specialized for symmetric matrices.
#[derive(Clone, Debug)]
pub struct CroutResult {
    pub status: Status,
    /// Lower triangular matrix (unit diagonal)
    pub l: Option<Vec<Vec<f64>>>,
    /// Diagonal elements
    pub d: Option<Vec<f64>>,
    /// Inverse matrix (lower triangle only)
    pub inverse: Option<Vec<Vec<f64>>>,
    /// Matrix determinant
    pub determinant: Option<f64>,
}

impl CroutResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for CroutResult {
    fn default() -> Self {
        Self {
            status: Status::NotConverged,
            l: None,
            d: None,
            inverse: None,
            determinant: None,
        }
    }
}

/// Helper function to create an n×n matrix with all zeros
fn zero_matrix(n: usize) -> Vec<Vec<f64>> {
    vec![vec![0.0; n]; n]
}

/// Helper function to create an n×n identity matrix
fn identity_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut mat = zero_matrix(n);
    for i in 0..n {
        mat[i][i] = 1.0;
    }
    mat
}

/// Crout decomposition for symmetric matrices: A = L * D * L^T.
///
/// This algorithm decomposes a symmetric matrix A into:
/// - L: lower triangular matrix with unit diagonal
/// - D: diagonal matrix
///
/// Properties:
/// - Only the lower triangle of A is used
/// - Faster than general LU for symmetric matrices
/// - Computes inverse efficiently
/// - Requires positive definiteness for stability
///
/// # Arguments
/// - `a`: Input symmetric matrix (only lower triangle used)
/// - `min_pivot`: Minimum pivot value (smaller treated as singular)
///
/// # Returns
/// CroutResult containing L, D, inverse, and determinant
pub fn crout(a: &[Vec<f64>], min_pivot: f64) -> CroutResult {
    let mut result = CroutResult::default();

    let n = a.len();
    if n == 0 {
        result.status = Status::InvalidInput;
        return result;
    }

    // Check square matrix
    for row in a {
        if row.len() != n {
            result.status = Status::InvalidInput;
            return result;
        }
    }

    // Working matrices
    let mut l = zero_matrix(n);
    let mut d = vec![0.0; n];
    let mut det = 1.0;

    // Crout decomposition: A = L * D * L^T
    for i in 0..n {
        // Compute L(i, j) for j < i
        for j in 0..i {
            let mut scale = 0.0;
            for k in 0..j {
                scale += l[i][k] * l[j][k] * d[k];
            }
            l[i][j] = (a[i][j] - scale) / d[j];
        }

        // Compute D(i)
        let mut scale = 0.0;
        for k in 0..i {
            scale += l[i][k] * l[i][k] * d[k];
        }
        d[i] = a[i][i] - scale;
        det *= d[i];

        // Check for singularity
        if d[i].abs() <= min_pivot {
            result.status = Status::Singular;
            return result;
        }

        l[i][i] = 1.0;
    }

    // Compute inverse of L
    let mut l_inv = identity_matrix(n);

    for i in 1..n {
        for k in 0..i {
            let mut scale = 0.0;
            for j in k..i {
                scale += l[i][j] * l_inv[j][k];
            }
            l_inv[i][k] = -scale / l[i][i];
        }
        l_inv[i][i] = 1.0 / l[i][i];
    }

    // Compute inverse of A (lower triangle only)
    let mut a_inv = zero_matrix(n);

    for j in 0..n {
        let mut scale = l_inv[j][j] * l_inv[j][j] / d[j];
        for k in (j + 1)..n {
            scale += l_inv[k][j] * l_inv[k][j] / d[k];
        }
        a_inv[j][j] = scale;

        for i in (j + 1)..n {
            let mut scale = l_inv[i][j] * l_inv[i][i] / d[i];
            for k in (i + 1)..n {
                scale += l_inv[k][j] * l_inv[k][i] / d[k];
            }
            a_inv[i][j] = scale;
        }
    }

    // Mirror to upper triangle for full symmetric matrix
    for i in 0..n {
        for j in (i + 1)..n {
            a_inv[i][j] = a_inv[j][i];
        }
    }

    result.l = Some(l);
    result.d = Some(d);
    result.inverse = Some(a_inv);
    result.determinant = Some(det);
    result.status = Status::Ok;
    result
}

/// Solve symmetric linear system Ax = b using Crout decomposition.
///
/// Uses precomputed Crout decomposition to solve for x.
/// More efficient than LU for symmetric positive definite matrices.
///
/// # Arguments
/// - `a`: Coefficient matrix (symmetric)
/// - `b`: Right-hand side vector
/// - `min_pivot`: Minimum pivot value
///
/// # Returns
/// Solution vector and determinant
pub fn solve_crout(a: &[Vec<f64>], b: &[f64], min_pivot: f64) -> Result<Vec<f64>, Status> {
    let n = a.len();

    if b.len() != n {
        return Err(Status::InvalidInput);
    }

    let crout_result = crout(a, min_pivot);
    if !crout_result.is_done() {
        return Err(crout_result.status);
    }

    let a_inv = crout_result.inverse.unwrap();

    // Compute X = Inverse * B using symmetry
    let mut x = vec![0.0; n];

    for i in 0..n {
        let mut asum = a_inv[i][0] * b[0];
        for j in 1..=i {
            asum += a_inv[i][j] * b[j];
        }
        for j in (i + 1)..n {
            asum += a_inv[j][i] * b[j];
        }
        x[i] = asum;
    }

    Ok(x)
}

/// Compute inverse of symmetric matrix using Crout decomposition.
///
/// # Arguments
/// - `a`: Input symmetric matrix
/// - `min_pivot`: Minimum pivot value
///
/// # Returns
/// Full symmetric inverse matrix
pub fn invert_crout(a: &[Vec<f64>], min_pivot: f64) -> Result<Vec<Vec<f64>>, Status> {
    let crout_result = crout(a, min_pivot);
    if !crout_result.is_done() {
        return Err(crout_result.status);
    }

    Ok(crout_result.inverse.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_matrix_close(a: &[Vec<f64>], b: &[Vec<f64>], tol: f64) {
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert_eq!(a[i].len(), b[i].len());
            for j in 0..a[i].len() {
                assert!(
                    (a[i][j] - b[i][j]).abs() < tol,
                    "Matrix mismatch at [{}, {}]: {} vs {}",
                    i,
                    j,
                    a[i][j],
                    b[i][j]
                );
            }
        }
    }

    fn assert_vector_close(a: &[f64], b: &[f64], tol: f64) {
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert!(
                (a[i] - b[i]).abs() < tol,
                "Vector mismatch at [{}]: {} vs {}",
                i,
                a[i],
                b[i]
            );
        }
    }

    #[test]
    fn test_crout_2x2() {
        // Simple 2x2 symmetric matrix
        // A = [[4, 1], [1, 3]]
        let a = vec![vec![4.0, 1.0], vec![1.0, 3.0]];

        let result = crout(&a, 1e-20);
        assert!(result.is_done());
        assert!(result.l.is_some());
        assert!(result.d.is_some());
        assert!(result.determinant.is_some());

        let d = result.d.unwrap();
        let l = result.l.unwrap();

        // D should be [4, 11/4]
        assert!((d[0] - 4.0).abs() < 1e-10);
        assert!((d[1] - 11.0 / 4.0).abs() < 1e-10);

        // L[1][0] = (a[1][0] - l[1][0]*l[0][0]*d[0]) / d[0] = 1/4
        assert!((l[1][0] - 0.25).abs() < 1e-10);
        assert!((l[1][1] - 1.0).abs() < 1e-10);

        // det = 4 * 11/4 = 11
        assert!((result.determinant.unwrap() - 11.0).abs() < 1e-10);
    }

    #[test]
    fn test_crout_inverse_2x2() {
        // A = [[4, 1], [1, 3]]
        // A^-1 = [[3/11, -1/11], [-1/11, 4/11]]
        let a = vec![vec![4.0, 1.0], vec![1.0, 3.0]];

        let inv = invert_crout(&a, 1e-20).unwrap();

        let expected = vec![vec![3.0 / 11.0, -1.0 / 11.0], vec![-1.0 / 11.0, 4.0 / 11.0]];

        assert_matrix_close(&inv, &expected, 1e-10);
    }

    #[test]
    fn test_crout_solve_2x2() {
        // Ax = b where A = [[4, 1], [1, 3]], b = [1, 2]
        // x = A^-1 * b
        let a = vec![vec![4.0, 1.0], vec![1.0, 3.0]];
        let b = vec![1.0, 2.0];

        let x = solve_crout(&a, &b, 1e-20).unwrap();

        // Manual solution: A^-1 = [[3/11, -1/11], [-1/11, 4/11]]
        // x[0] = 3/11 * 1 - 1/11 * 2 = 1/11
        // x[1] = -1/11 * 1 + 4/11 * 2 = 7/11
        let expected = vec![1.0 / 11.0, 7.0 / 11.0];

        assert_vector_close(&x, &expected, 1e-10);
    }

    #[test]
    fn test_crout_3x3() {
        // 3x3 symmetric positive definite matrix
        // A = [[6, 3, 1], [3, 5, 2], [1, 2, 4]]
        let a = vec![
            vec![6.0, 3.0, 1.0],
            vec![3.0, 5.0, 2.0],
            vec![1.0, 2.0, 4.0],
        ];

        let result = crout(&a, 1e-20);
        assert!(result.is_done());

        let d = result.d.unwrap();
        let l = result.l.unwrap();

        // First diagonal should be 6
        assert!((d[0] - 6.0).abs() < 1e-10);

        // Check L is lower triangular with unit diagonal
        for i in 0..3 {
            assert!((l[i][i] - 1.0).abs() < 1e-10);
        }
        assert!((l[1][0] - 0.5).abs() < 1e-10); // 3/6 = 0.5
    }

    #[test]
    fn test_crout_singular_matrix() {
        // Singular matrix
        // A = [[1, 2], [2, 4]]
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];

        let result = crout(&a, 1e-20);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::Singular);
    }

    #[test]
    fn test_crout_non_square() {
        // Non-square matrix
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];

        let result = crout(&a, 1e-20);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_crout_identity() {
        // Identity matrix
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];

        let result = crout(&a, 1e-20);
        assert!(result.is_done());

        let inv = result.inverse.unwrap();
        assert!((inv[0][0] - 1.0).abs() < 1e-10);
        assert!((inv[0][1] - 0.0).abs() < 1e-10);
        assert!((inv[1][0] - 0.0).abs() < 1e-10);
        assert!((inv[1][1] - 1.0).abs() < 1e-10);

        // det = 1
        assert!((result.determinant.unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_crout_solve_3x3() {
        // A = [[6, 3, 1], [3, 5, 2], [1, 2, 4]]
        // b = [6, 5, 4]
        let a = vec![
            vec![6.0, 3.0, 1.0],
            vec![3.0, 5.0, 2.0],
            vec![1.0, 2.0, 4.0],
        ];
        let b = vec![6.0, 5.0, 4.0];

        let x = solve_crout(&a, &b, 1e-20).unwrap();

        // Verify Ax = b
        for i in 0..3 {
            let mut sum = 0.0;
            for j in 0..3 {
                sum += a[i][j] * x[j];
            }
            assert!((sum - b[i]).abs() < 1e-9);
        }
    }

    #[test]
    fn test_crout_solve_dimension_mismatch() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![1.0, 2.0, 3.0]; // Wrong size

        let result = solve_crout(&a, &b, 1e-20);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Status::InvalidInput);
    }

    #[test]
    fn test_crout_empty_matrix() {
        let a: Vec<Vec<f64>> = vec![];

        let result = crout(&a, 1e-20);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::InvalidInput);
    }
}

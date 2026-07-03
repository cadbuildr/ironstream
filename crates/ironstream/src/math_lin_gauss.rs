// FILE: math_lin_gauss.rs
// occt: MathLin_Gauss

/// Computation status for linear solvers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Computation successful
    Ok,
    /// Did not converge
    NotConverged,
    /// Numerical error (overflow, singular, etc.)
    NumericalError,
    /// Invalid input
    InvalidInput,
}

/// Result for LU decomposition.
#[derive(Clone, Debug)]
pub struct LuResult {
    pub status: Status,
    /// Combined L and U matrices
    pub lu: Option<Vec<Vec<f64>>>,
    /// Pivot indices (row permutations)
    pub pivot: Option<Vec<usize>>,
    /// Matrix determinant
    pub determinant: Option<f64>,
    /// Sign from row interchanges
    pub sign: i32,
}

impl LuResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for LuResult {
    fn default() -> Self {
        Self {
            status: Status::NotConverged,
            lu: None,
            pivot: None,
            determinant: None,
            sign: 1,
        }
    }
}

/// Helper: Check if a value is zero
fn is_zero(val: f64, tol: f64) -> bool {
    val.abs() < tol
}

/// Helper: Create an n×n matrix with all zeros
fn zero_matrix(n: usize) -> Vec<Vec<f64>> {
    vec![vec![0.0; n]; n]
}

/// Perform LU decomposition of matrix A with partial pivoting.
///
/// Decomposes A into L*U where L is lower triangular with unit diagonal
/// and U is upper triangular. The result stores L and U in a combined matrix.
///
/// # Arguments
/// - `a`: Input square matrix
/// - `min_pivot`: Minimum pivot value (smaller treated as singular)
///
/// # Returns
/// LuResult containing combined LU matrix, pivot indices, determinant, and sign
pub fn lu(a: &[Vec<f64>], min_pivot: f64) -> LuResult {
    let mut result = LuResult::default();

    let n = a.len();
    if n == 0 {
        result.status = Status::InvalidInput;
        return result;
    }

    // Check for square matrix
    for row in a {
        if row.len() != n {
            result.status = Status::InvalidInput;
            return result;
        }
    }

    // Initialize LU matrix as copy of A
    let mut lu_mat = a.to_vec();
    let mut pivot = vec![0; n];
    let mut sign = 1i32;

    // Scaling factors for implicit pivoting
    let mut scale = vec![0.0; n];
    for i in 0..n {
        let mut max_val = 0.0;
        for j in 0..n {
            let abs_val = lu_mat[i][j].abs();
            if abs_val > max_val {
                max_val = abs_val;
            }
        }
        if max_val < min_pivot {
            result.status = Status::NumericalError;
            return result;
        }
        scale[i] = 1.0 / max_val;
    }

    // Crout's algorithm with partial pivoting
    for k in 0..n {
        // Find pivot
        let mut max_scaled = 0.0;
        let mut pivot_row = k;

        for i in k..n {
            let scaled = scale[i] * lu_mat[i][k].abs();
            if scaled > max_scaled {
                max_scaled = scaled;
                pivot_row = i;
            }
        }

        // Swap rows if needed
        if pivot_row != k {
            lu_mat.swap(pivot_row, k);
            scale.swap(pivot_row, k);
            sign = -sign;
        }
        pivot[k] = pivot_row;

        // Check for singular matrix
        if lu_mat[k][k].abs() < min_pivot {
            result.status = Status::NumericalError;
            return result;
        }

        // Eliminate below diagonal
        for i in (k + 1)..n {
            lu_mat[i][k] /= lu_mat[k][k];
            let factor = lu_mat[i][k];
            for j in (k + 1)..n {
                lu_mat[i][j] -= factor * lu_mat[k][j];
            }
        }
    }

    // Compute determinant
    let mut det = sign as f64;
    for i in 0..n {
        det *= lu_mat[i][i];
    }

    result.lu = Some(lu_mat);
    result.pivot = Some(pivot);
    result.determinant = Some(det);
    result.sign = sign;
    result.status = Status::Ok;
    result
}

/// Solve linear system AX = B using LU decomposition.
///
/// # Arguments
/// - `a`: Coefficient matrix (square)
/// - `b`: Right-hand side vector
/// - `min_pivot`: Minimum pivot value
///
/// # Returns
/// Solution vector
pub fn solve(a: &[Vec<f64>], b: &[f64], min_pivot: f64) -> Result<Vec<f64>, Status> {
    let n = a.len();

    if b.len() != n {
        return Err(Status::InvalidInput);
    }

    let lu_result = lu(a, min_pivot);
    if !lu_result.is_done() {
        return Err(lu_result.status);
    }

    let lu_mat = lu_result.lu.unwrap();
    let pivot = lu_result.pivot.unwrap();

    // Permute b according to pivot indices
    let mut x = vec![0.0; n];
    for i in 0..n {
        x[i] = b[pivot[i]];
    }

    // Forward substitution (solve Ly = Pb)
    for i in 0..n {
        for j in 0..i {
            x[i] -= lu_mat[i][j] * x[j];
        }
    }

    // Back substitution (solve Ux = y)
    for i in (0..n).rev() {
        for j in (i + 1)..n {
            x[i] -= lu_mat[i][j] * x[j];
        }
        if lu_mat[i][i].abs() < min_pivot {
            return Err(Status::NumericalError);
        }
        x[i] /= lu_mat[i][i];
    }

    Ok(x)
}

/// Solve multiple linear systems AX = B where B is a matrix.
/// Each column of B is a separate right-hand side.
///
/// # Arguments
/// - `a`: Coefficient matrix (square)
/// - `b`: Right-hand side matrix where rows are indexed by n and columns are RHS vectors
/// - `min_pivot`: Minimum pivot value
///
/// # Returns
/// Solution matrix X where each row is the solution vector
pub fn solve_multiple(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    min_pivot: f64,
) -> Result<Vec<Vec<f64>>, Status> {
    let n = a.len();
    if b.len() != n {
        return Err(Status::InvalidInput);
    }

    if b.is_empty() {
        return Err(Status::InvalidInput);
    }

    let m = b[0].len(); // Number of columns (RHS)

    let lu_result = lu(a, min_pivot);
    if !lu_result.is_done() {
        return Err(lu_result.status);
    }

    let lu_mat = lu_result.lu.unwrap();
    let pivot = lu_result.pivot.unwrap();

    // Solutions will be n x m matrix
    let mut solutions = vec![vec![0.0; m]; n];

    // Solve for each column (each RHS)
    for col in 0..m {
        // Permute b according to pivot indices
        let mut work = vec![0.0; n];
        for i in 0..n {
            work[i] = b[pivot[i]][col];
        }

        // Forward substitution (solve Ly = Pb)
        for i in 0..n {
            for j in 0..i {
                work[i] -= lu_mat[i][j] * work[j];
            }
        }

        // Back substitution (solve Ux = y)
        for i in (0..n).rev() {
            for j in (i + 1)..n {
                work[i] -= lu_mat[i][j] * work[j];
            }
            if lu_mat[i][i].abs() < min_pivot {
                return Err(Status::NumericalError);
            }
            work[i] /= lu_mat[i][i];
        }

        // Copy result to solution matrix
        for i in 0..n {
            solutions[i][col] = work[i];
        }
    }

    Ok(solutions)
}

/// Compute determinant of matrix A.
///
/// # Arguments
/// - `a`: Input square matrix
/// - `min_pivot`: Minimum pivot value
///
/// # Returns
/// Determinant value
pub fn determinant(a: &[Vec<f64>], min_pivot: f64) -> Result<f64, Status> {
    let lu_result = lu(a, min_pivot);
    if !lu_result.is_done() {
        return Err(lu_result.status);
    }

    Ok(lu_result.determinant.unwrap())
}

/// Compute inverse of matrix A.
///
/// # Arguments
/// - `a`: Input square matrix
/// - `min_pivot`: Minimum pivot value
///
/// # Returns
/// Inverse matrix
pub fn invert(a: &[Vec<f64>], min_pivot: f64) -> Result<Vec<Vec<f64>>, Status> {
    let n = a.len();
    if n == 0 {
        return Err(Status::InvalidInput);
    }

    let lu_result = lu(a, min_pivot);
    if !lu_result.is_done() {
        return Err(lu_result.status);
    }

    let lu_mat = lu_result.lu.unwrap();
    let pivot = lu_result.pivot.unwrap();

    // Compute inverse column by column
    let mut inv = zero_matrix(n);

    for col in 0..n {
        // Initialize unit vector (permuted by pivot)
        let mut work = vec![0.0; n];
        work[pivot[col]] = 1.0;

        // Forward substitution (solve Ly = e_col)
        for i in 0..n {
            for j in 0..i {
                work[i] -= lu_mat[i][j] * work[j];
            }
        }

        // Back substitution (solve Ux = y)
        for i in (0..n).rev() {
            for j in (i + 1)..n {
                work[i] -= lu_mat[i][j] * work[j];
            }
            if lu_mat[i][i].abs() < min_pivot {
                return Err(Status::NumericalError);
            }
            work[i] /= lu_mat[i][i];
        }

        // Copy result to inverse matrix
        for i in 0..n {
            inv[i][col] = work[i];
        }
    }

    Ok(inv)
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
    fn test_lu_2x2() {
        // A = [[4, 3], [6, 3]]
        let a = vec![vec![4.0, 3.0], vec![6.0, 3.0]];

        let result = lu(&a, 1e-20);
        assert!(result.is_done());
        assert!(result.lu.is_some());
        assert!(result.pivot.is_some());
        assert!(result.determinant.is_some());

        // det(A) = 4*3 - 3*6 = 12 - 18 = -6
        assert!((result.determinant.unwrap() - (-6.0)).abs() < 1e-10);
    }

    #[test]
    fn test_solve_2x2() {
        // Ax = b where A = [[4, 3], [6, 3]], b = [1, 0]
        let a = vec![vec![4.0, 3.0], vec![6.0, 3.0]];
        let b = vec![1.0, 0.0];

        let x = solve(&a, &b, 1e-20).unwrap();

        // Verify Ax = b
        assert!((a[0][0] * x[0] + a[0][1] * x[1] - b[0]).abs() < 1e-10);
        assert!((a[1][0] * x[0] + a[1][1] * x[1] - b[1]).abs() < 1e-10);
    }

    #[test]
    fn test_solve_3x3() {
        // A = [[3, 2, 1], [2, 3, 2], [1, 2, 3]]
        // b = [10, 13, 11]
        let a = vec![
            vec![3.0, 2.0, 1.0],
            vec![2.0, 3.0, 2.0],
            vec![1.0, 2.0, 3.0],
        ];
        let b = vec![10.0, 13.0, 11.0];

        let x = solve(&a, &b, 1e-20).unwrap();

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
    fn test_determinant_2x2() {
        // A = [[4, 3], [6, 3]]
        // det = 12 - 18 = -6
        let a = vec![vec![4.0, 3.0], vec![6.0, 3.0]];

        let det = determinant(&a, 1e-20).unwrap();
        assert!((det - (-6.0)).abs() < 1e-10);
    }

    #[test]
    fn test_determinant_identity() {
        // Identity matrix
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];

        let det = determinant(&a, 1e-20).unwrap();
        assert!((det - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_invert_2x2() {
        // A = [[4, 3], [6, 3]]
        // A^-1 = [[-0.5, 0.5], [1, -2/3]]
        let a = vec![vec![4.0, 3.0], vec![6.0, 3.0]];

        let inv = invert(&a, 1e-20).unwrap();

        let expected = vec![vec![-0.5, 0.5], vec![1.0, -2.0 / 3.0]];

        assert_matrix_close(&inv, &expected, 1e-10);
    }

    #[test]
    fn test_invert_identity() {
        // Identity matrix should invert to itself
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];

        let inv = invert(&a, 1e-20).unwrap();

        assert!((inv[0][0] - 1.0).abs() < 1e-10);
        assert!((inv[0][1] - 0.0).abs() < 1e-10);
        assert!((inv[1][0] - 0.0).abs() < 1e-10);
        assert!((inv[1][1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_invert_3x3() {
        // A = [[3, 2, 1], [2, 3, 2], [1, 2, 3]]
        let a = vec![
            vec![3.0, 2.0, 1.0],
            vec![2.0, 3.0, 2.0],
            vec![1.0, 2.0, 3.0],
        ];

        let inv = invert(&a, 1e-20).unwrap();

        // Verify A * A^-1 = I
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += a[i][k] * inv[k][j];
                }
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (sum - expected).abs() < 1e-9,
                    "A*A^-1[{},{}] = {}, expected {}",
                    i,
                    j,
                    sum,
                    expected
                );
            }
        }
    }

    #[test]
    fn test_lu_singular_matrix() {
        // Singular matrix: rows are linearly dependent
        // A = [[1, 2], [2, 4]]
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];

        let result = lu(&a, 1e-20);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::NumericalError);
    }

    #[test]
    fn test_solve_multiple() {
        // Solve AX = B where B has 2 columns
        let a = vec![
            vec![3.0, 2.0, 1.0],
            vec![2.0, 3.0, 2.0],
            vec![1.0, 2.0, 3.0],
        ];
        let b = vec![vec![10.0, 5.0], vec![13.0, 7.0], vec![11.0, 6.0]];

        let solutions = solve_multiple(&a, &b, 1e-20).unwrap();

        // Verify for each column
        for col in 0..2 {
            for i in 0..3 {
                let mut sum = 0.0;
                for j in 0..3 {
                    sum += a[i][j] * solutions[j][col];
                }
                assert!((sum - b[i][col]).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn test_solve_dimension_mismatch() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![1.0, 2.0, 3.0]; // Wrong size

        let result = solve(&a, &b, 1e-20);
        assert!(result.is_err());
    }

    #[test]
    fn test_lu_non_square() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];

        let result = lu(&a, 1e-20);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_lu_empty() {
        let a: Vec<Vec<f64>> = vec![];

        let result = lu(&a, 1e-20);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_solve_identity() {
        // Solving Ix = b should give x = b
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![5.0, 3.0];

        let x = solve(&a, &b, 1e-20).unwrap();
        assert_vector_close(&x, &b, 1e-10);
    }

    #[test]
    fn test_determinant_diagonal() {
        // Diagonal matrix: det = product of diagonal
        let a = vec![vec![2.0, 0.0], vec![0.0, 3.0]];

        let det = determinant(&a, 1e-20).unwrap();
        assert!((det - 6.0).abs() < 1e-10);
    }
}

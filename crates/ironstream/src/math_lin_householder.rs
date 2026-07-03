// FILE: math_lin_householder.rs
// occt: MathLin_Householder

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
    Singular,
    NumericalError,
}

impl Default for Status {
    fn default() -> Self {
        Status::NotConverged
    }
}

/// Result structure for QR decomposition
#[derive(Clone, Debug)]
pub struct QRResult {
    pub status: Status,
    pub q: Option<Matrix>,
    pub r: Option<Matrix>,
    pub rank: i32,
}

impl QRResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

/// Result structure for linear system solving
#[derive(Clone, Debug)]
pub struct LinearResult {
    pub status: Status,
    pub solution: Option<Vector>,
    pub determinant: Option<f64>,
}

impl LinearResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

/// Result structure for multiple linear systems
#[derive(Clone, Debug)]
pub struct LinearMultipleResult {
    pub status: Status,
    pub solutions: Option<Matrix>,
    pub determinant: Option<f64>,
}

impl LinearMultipleResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

const THE_ZERO_TOL: f64 = 1e-20;

fn sqr(x: f64) -> f64 {
    x * x
}

/// QR decomposition using Householder reflections: A = Q * R.
///
/// Decomposes an m x n matrix A (m >= n) into:
/// - Q: m x m orthogonal matrix (Q^T * Q = I)
/// - R: m x n upper triangular matrix
///
/// The Householder method applies orthogonal transformations
/// to reduce A to upper triangular form. It is more numerically
/// stable than Gram-Schmidt orthogonalization.
pub fn qr(a: &Matrix, tolerance: f64) -> QRResult {
    let mut result = QRResult {
        status: Status::NotConverged,
        q: None,
        r: None,
        rank: 0,
    };

    let row_lower = a.lower_row();
    let row_upper = a.upper_row();
    let col_lower = a.lower_col();
    let col_upper = a.upper_col();
    let m = row_upper - row_lower + 1; // Number of rows
    let n = col_upper - col_lower + 1; // Number of columns

    if m < n {
        result.status = Status::InvalidInput;
        return result;
    }

    // Working copy of A that will become R
    let mut r = Matrix::new(row_lower, row_upper, col_lower, col_upper, 0.0);
    for i in row_lower..=row_upper {
        for j in col_lower..=col_upper {
            r.set(i, j, a.get(i, j));
        }
    }

    // Q starts as identity
    let mut q = Matrix::new(row_lower, row_upper, row_lower, row_upper, 0.0);
    for i in row_lower..=row_upper {
        q.set(i, i, 1.0);
    }

    // Householder vector storage
    let mut v = Vector::new(row_lower, row_upper, 0.0);

    result.rank = 0;

    // Apply Householder reflections to each column
    for j in col_lower..=col_upper {
        let j_offset = j - col_lower;

        // Compute norm of column below diagonal
        let mut norm = 0.0;
        for i in (row_lower + j_offset)..=row_upper {
            norm += sqr(r.get(i, j));
        }
        norm = norm.sqrt();

        if norm < tolerance {
            // Column is essentially zero, skip
            continue;
        }

        result.rank += 1;

        // Compute Householder vector v
        let alpha = r.get(row_lower + j_offset, j);
        let beta = if alpha >= 0.0 { -norm } else { norm };

        // v = [0,...,0, x_j - beta, x_{j+1}, ..., x_m]
        for i in row_lower..(row_lower + j_offset) {
            v.set(i, 0.0);
        }
        v.set(row_lower + j_offset, alpha - beta);
        for i in (row_lower + j_offset + 1)..=row_upper {
            v.set(i, r.get(i, j));
        }

        // Compute tau = 2 / (v^T v)
        let mut v_norm_sq = 0.0;
        for i in (row_lower + j_offset)..=row_upper {
            v_norm_sq += sqr(v.get(i));
        }

        if v_norm_sq < THE_ZERO_TOL {
            continue;
        }

        let tau = 2.0 / v_norm_sq;

        // Apply H = I - tau * v * v^T to remaining columns of R
        // R := H * R
        for k in j..=col_upper {
            // Compute v^T * R(:,k)
            let mut v_dot_rk = 0.0;
            for i in (row_lower + j_offset)..=row_upper {
                v_dot_rk += v.get(i) * r.get(i, k);
            }

            // R(:,k) := R(:,k) - tau * (v^T * R(:,k)) * v
            for i in (row_lower + j_offset)..=row_upper {
                let val = r.get(i, k) - tau * v_dot_rk * v.get(i);
                r.set(i, k, val);
            }
        }

        // Apply H to Q: Q := Q * H = Q - tau * Q * v * v^T
        for k in row_lower..=row_upper {
            // Compute Q(k,:) * v
            let mut qkv = 0.0;
            for i in (row_lower + j_offset)..=row_upper {
                qkv += q.get(k, i) * v.get(i);
            }

            // Q(k,:) := Q(k,:) - tau * (Q(k,:) * v) * v^T
            for i in (row_lower + j_offset)..=row_upper {
                let val = q.get(k, i) - tau * qkv * v.get(i);
                q.set(k, i, val);
            }
        }
    }

    // Zero out below-diagonal elements of R for cleanliness
    for i in row_lower..=row_upper {
        for j in col_lower..(col_lower + (i - row_lower)) {
            if j <= col_upper {
                r.set(i, j, 0.0);
            }
        }
    }

    result.q = Some(q);
    result.r = Some(r);
    result.status = Status::OK;
    result
}

/// Solve overdetermined system Ax = b using QR decomposition (least squares).
///
/// For m x n system with m >= n, finds x that minimizes ||Ax - b||_2.
///
/// Algorithm:
/// 1. Decompose A = Q * R
/// 2. Compute c = Q^T * b
/// 3. Solve R * x = c[1:n] (back substitution)
pub fn solve_qr(a: &Matrix, b: &Vector, tolerance: f64) -> LinearResult {
    let mut result = LinearResult {
        status: Status::NotConverged,
        solution: None,
        determinant: None,
    };

    let row_lower = a.lower_row();
    let row_upper = a.upper_row();
    let col_lower = a.lower_col();
    let col_upper = a.upper_col();
    let m = row_upper - row_lower + 1;

    // Check dimensions
    if b.length() != m as usize {
        result.status = Status::InvalidInput;
        return result;
    }

    // Perform QR decomposition
    let qr_result = qr(a, tolerance);
    if !qr_result.is_done() {
        result.status = qr_result.status;
        return result;
    }

    let q = qr_result.q.as_ref().unwrap();
    let r = qr_result.r.as_ref().unwrap();

    // Compute c = Q^T * b
    let mut c = Vector::new(row_lower, row_upper, 0.0);
    for i in row_lower..=row_upper {
        let mut sum = 0.0;
        for k in row_lower..=row_upper {
            sum += q.get(k, i) * b.get(b.lower() + (k - row_lower));
        }
        c.set(i, sum);
    }

    // Back substitution: R[1:n, 1:n] * x = c[1:n]
    let mut solution = Vector::new(col_lower, col_upper, 0.0);

    for i in (col_lower..=col_upper).rev() {
        let i_offset = i - col_lower;
        let diag = r.get(row_lower + i_offset, i);

        if diag.abs() < tolerance {
            result.status = Status::Singular;
            return result;
        }

        let mut sum = c.get(row_lower + i_offset);
        for k in (i + 1)..=col_upper {
            sum -= r.get(row_lower + i_offset, k) * solution.get(k);
        }
        solution.set(i, sum / diag);
    }

    result.solution = Some(solution);
    result.status = Status::OK;
    result
}

/// Solve multiple right-hand sides using QR decomposition.
///
/// @param a coefficient matrix (m x n, m >= n)
/// @param b right-hand side matrix (m x p)
/// @param tolerance for singularity detection
/// @return result containing solution matrix (n x p)
pub fn solve_qr_multiple(a: &Matrix, b: &Matrix, tolerance: f64) -> LinearMultipleResult {
    let mut result = LinearMultipleResult {
        status: Status::NotConverged,
        solutions: None,
        determinant: None,
    };

    let row_lower = a.lower_row();
    let row_upper = a.upper_row();
    let col_lower = a.lower_col();
    let col_upper = a.upper_col();

    // Check dimensions
    if b.row_number() != a.row_number() {
        result.status = Status::InvalidInput;
        return result;
    }

    // Perform QR decomposition
    let qr_result = qr(a, tolerance);
    if !qr_result.is_done() {
        result.status = qr_result.status;
        return result;
    }

    let q = qr_result.q.as_ref().unwrap();
    let r = qr_result.r.as_ref().unwrap();

    // Solve for each column of B
    let mut x = Matrix::new(col_lower, col_upper, b.lower_col(), b.upper_col(), 0.0);

    for j in b.lower_col()..=b.upper_col() {
        // Compute c = Q^T * b_j
        let mut c = Vector::new(row_lower, row_upper, 0.0);
        for i in row_lower..=row_upper {
            let mut sum = 0.0;
            for k in row_lower..=row_upper {
                let b_row = b.lower_row() + (k - row_lower);
                sum += q.get(k, i) * b.get(b_row, j);
            }
            c.set(i, sum);
        }

        // Back substitution: R[1:n,1:n] * x_j = c[1:n]
        for i in (col_lower..=col_upper).rev() {
            let i_offset = i - col_lower;
            let r_row = row_lower + i_offset;
            let diag = r.get(r_row, i);

            if diag.abs() < tolerance {
                result.status = Status::Singular;
                return result;
            }

            let mut sum = c.get(r_row);
            for k in (i + 1)..=col_upper {
                sum -= r.get(r_row, k) * x.get(k, j);
            }
            x.set(i, j, sum / diag);
        }
    }

    result.solutions = Some(x);
    result.status = Status::OK;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_small_matrix() {
        // Test QR on a 3x2 overdetermined system
        let mut a = Matrix::new(1, 3, 1, 2, 0.0);
        a.set(1, 1, 1.0);
        a.set(1, 2, 0.0);
        a.set(2, 1, 1.0);
        a.set(2, 2, 1.0);
        a.set(3, 1, 0.0);
        a.set(3, 2, 1.0);

        let result = qr(&a, 1e-20);
        assert!(result.is_done());
        assert_eq!(result.rank, 2);
        assert!(result.q.is_some());
        assert!(result.r.is_some());

        let q = result.q.unwrap();
        let r = result.r.unwrap();

        // Check that R is upper triangular
        for i in 2..=3 {
            assert!((r.get(i, 1)).abs() < 1e-10);
        }

        // Check that Q is orthogonal: Q^T * Q ≈ I
        for i in 1..=3 {
            for j in 1..=3 {
                let mut sum = 0.0;
                for k in 1..=3 {
                    sum += q.get(k, i) * q.get(k, j);
                }
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((sum - expected).abs() < 1e-10, "Q^T*Q[{},{}] = {}, expected {}", i, j, sum, expected);
            }
        }
    }

    #[test]
    fn test_solve_qr_overdetermined() {
        // Overdetermined system: 3 equations, 2 unknowns
        // x + 0*y = 1
        // x + y = 2
        // 0*x + y = 1
        // Least squares solution: x ≈ 0.5, y ≈ 1.5
        let mut a = Matrix::new(1, 3, 1, 2, 0.0);
        a.set(1, 1, 1.0);
        a.set(1, 2, 0.0);
        a.set(2, 1, 1.0);
        a.set(2, 2, 1.0);
        a.set(3, 1, 0.0);
        a.set(3, 2, 1.0);

        let mut b = Vector::new(1, 3, 0.0);
        b.set(1, 1.0);
        b.set(2, 2.0);
        b.set(3, 1.0);

        let result = solve_qr(&a, &b, 1e-15);
        assert!(result.is_done());
        assert!(result.solution.is_some());

        let x = result.solution.unwrap();
        assert!((x.get(1) - 1.0).abs() < 1e-10, "x[1] = {}", x.get(1));
        assert!((x.get(2) - 1.0).abs() < 1e-10, "x[2] = {}", x.get(2));
    }

    #[test]
    fn test_qr_invalid_input() {
        // m < n should fail
        let a = Matrix::new(1, 2, 1, 3, 1.0);
        let result = qr(&a, 1e-20);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_solve_qr_multiple() {
        // Solve two systems with same A but different B
        let mut a = Matrix::new(1, 3, 1, 2, 0.0);
        a.set(1, 1, 1.0);
        a.set(1, 2, 0.0);
        a.set(2, 1, 1.0);
        a.set(2, 2, 1.0);
        a.set(3, 1, 0.0);
        a.set(3, 2, 1.0);

        let mut b = Matrix::new(1, 3, 1, 2, 0.0);
        b.set(1, 1, 1.0);
        b.set(2, 1, 2.0);
        b.set(3, 1, 1.0);
        b.set(1, 2, 2.0);
        b.set(2, 2, 3.0);
        b.set(3, 2, 2.0);

        let result = solve_qr_multiple(&a, &b, 1e-15);
        assert!(result.is_done());
        assert!(result.solutions.is_some());

        let x = result.solutions.unwrap();
        // First solution: x ≈ 1.0, y ≈ 1.0
        assert!((x.get(1, 1) - 1.0).abs() < 1e-10);
        assert!((x.get(2, 1) - 1.0).abs() < 1e-10);
    }
}

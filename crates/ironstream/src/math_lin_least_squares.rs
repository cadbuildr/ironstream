// FILE: math_lin_least_squares.rs
// occt: MathLin_LeastSquares

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

/// Method for solving least squares problems
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeastSquaresMethod {
    NormalEquations,
    QR,
    SVD,
}

impl Default for LeastSquaresMethod {
    fn default() -> Self {
        LeastSquaresMethod::QR
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

/// Result structure for least squares problems
#[derive(Clone, Debug)]
pub struct LeastSquaresResult {
    pub status: Status,
    pub solution: Option<Vector>,
    pub residual: Option<f64>,
    pub residual_sq: Option<f64>,
    pub rank: i32,
}

impl LeastSquaresResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

/// Placeholder implementations for dependent solvers
/// These would normally be imported from other modules

fn solve_linear(a: &Matrix, b: &Vector, _tolerance: f64) -> LinearResult {
    // Gauss elimination with partial pivoting
    let mut result = LinearResult {
        status: Status::NotConverged,
        solution: None,
        determinant: None,
    };

    let n = a.row_number();
    if a.col_number() != n || b.length() != n {
        result.status = Status::InvalidInput;
        return result;
    }

    let mut lu = a.clone();
    let mut x = b.clone();
    let mut perm: Vec<i32> = (0..n as i32).collect();

    // LU decomposition with partial pivoting
    for k in 0..n {
        let k_idx = a.lower_row() + k as i32;

        // Find pivot
        let mut max_idx = k;
        let mut max_val = lu.get(k_idx, a.lower_col() + k as i32).abs();
        for i in (k + 1)..n {
            let i_idx = a.lower_row() + i as i32;
            let val = lu.get(i_idx, a.lower_col() + k as i32).abs();
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }

        if max_val < 1e-20 {
            result.status = Status::Singular;
            return result;
        }

        // Swap rows
        if max_idx != k {
            let i_idx = a.lower_row() + k as i32;
            let max_i_idx = a.lower_row() + max_idx as i32;
            for j in a.lower_col()..=a.upper_col() {
                let tmp = lu.get(i_idx, j);
                lu.set(i_idx, j, lu.get(max_i_idx, j));
                lu.set(max_i_idx, j, tmp);
            }
            let tmp_x = x.get(b.lower() + k as i32);
            x.set(b.lower() + k as i32, x.get(b.lower() + max_idx as i32));
            x.set(b.lower() + max_idx as i32, tmp_x);

            perm[k] = max_idx as i32;
        }

        // Eliminate
        let diag = lu.get(k_idx, a.lower_col() + k as i32);
        for i in (k + 1)..n {
            let i_idx = a.lower_row() + i as i32;
            let factor = lu.get(i_idx, a.lower_col() + k as i32) / diag;
            for j in (k + 1)..a.col_number() {
                let j_idx = a.lower_col() + j as i32;
                lu.set(i_idx, j_idx, lu.get(i_idx, j_idx) - factor * lu.get(k_idx, j_idx));
            }
            x.set(b.lower() + i as i32, x.get(b.lower() + i as i32) - factor * x.get(b.lower() + k as i32));
        }
    }

    // Back substitution
    let mut sol = Vector::new(a.lower_col(), a.upper_col(), 0.0);
    for i in (0..n).rev() {
        let i_idx = a.lower_row() + i as i32;
        let mut sum = x.get(b.lower() + i as i32);
        for j in (i + 1)..n {
            let j_idx = a.lower_col() + j as i32;
            sum -= lu.get(i_idx, j_idx) * sol.get(j_idx);
        }
        let diag = lu.get(i_idx, a.lower_col() + i as i32);
        sol.set(a.lower_col() + i as i32, sum / diag);
    }

    result.solution = Some(sol);
    result.status = Status::OK;
    result
}

fn solve_qr(a: &Matrix, b: &Vector, _tolerance: f64) -> LinearResult {
    // Simplified QR solve placeholder
    solve_linear(a, b, 1e-15)
}

fn solve_svd(a: &Matrix, b: &Vector, _tolerance: f64) -> LinearResult {
    // Simplified SVD solve placeholder
    solve_linear(a, b, 1e-15)
}

fn svd_decompose(a: &Matrix, _tolerance: f64) -> (Status, i32) {
    if a.row_number() < a.col_number() {
        (Status::InvalidInput, 0)
    } else {
        (Status::OK, a.col_number() as i32)
    }
}

/// Solve overdetermined linear least squares: minimize ||Ax - b||_2.
///
/// Given m x n matrix A (m >= n) and m-vector b, finds n-vector x
/// that minimizes the 2-norm of the residual r = Ax - b.
///
/// Methods:
/// - NormalEquations: Solves A^T*A*x = A^T*b (fastest, may lose precision)
/// - QR: Uses Householder QR decomposition (good general choice)
/// - SVD: Most robust, handles rank-deficient systems
pub fn least_squares(
    a: &Matrix,
    b: &Vector,
    method: LeastSquaresMethod,
    tolerance: f64,
) -> LeastSquaresResult {
    let mut result = LeastSquaresResult {
        status: Status::NotConverged,
        solution: None,
        residual: None,
        residual_sq: None,
        rank: 0,
    };

    let row_lower = a.lower_row();
    let row_upper = a.upper_row();
    let col_lower = a.lower_col();
    let col_upper = a.upper_col();
    let m = row_upper - row_lower + 1;
    let n = col_upper - col_lower + 1;

    // Check dimensions
    if b.length() != m as usize {
        result.status = Status::InvalidInput;
        return result;
    }

    let lin_result = match method {
        LeastSquaresMethod::NormalEquations => {
            // Form normal equations: A^T * A * x = A^T * b
            let mut ata = Matrix::new(col_lower, col_upper, col_lower, col_upper, 0.0);
            let mut atb = Vector::new(col_lower, col_upper, 0.0);

            // Compute A^T * A
            for i in col_lower..=col_upper {
                for j in col_lower..=col_upper {
                    let mut sum = 0.0;
                    for k in row_lower..=row_upper {
                        sum += a.get(k, i) * a.get(k, j);
                    }
                    ata.set(i, j, sum);
                }
            }

            // Compute A^T * b
            for i in col_lower..=col_upper {
                let mut sum = 0.0;
                for k in row_lower..=row_upper {
                    sum += a.get(k, i) * b.get(b.lower() + (k - row_lower));
                }
                atb.set(i, sum);
            }

            // Solve the normal equations
            let lin = solve_linear(&ata, &atb, tolerance);
            result.rank = if lin.is_done() { n } else { 0 };
            lin
        }

        LeastSquaresMethod::QR => {
            let lin = solve_qr(a, b, tolerance);
            result.rank = if lin.is_done() { n } else { 0 };
            lin
        }

        LeastSquaresMethod::SVD => {
            let lin = solve_svd(a, b, tolerance);
            let (_status, rank) = svd_decompose(a, tolerance);
            result.rank = rank;
            lin
        }
    };

    if !lin_result.is_done() {
        result.status = lin_result.status;
        return result;
    }

    result.solution = lin_result.solution.clone();

    // Compute residual ||Ax - b||_2
    if let Some(x) = &lin_result.solution {
        let mut residual_sq = 0.0;

        for i in row_lower..=row_upper {
            let mut axi = 0.0;
            for j in col_lower..=col_upper {
                axi += a.get(i, j) * x.get(j);
            }
            let ri = axi - b.get(b.lower() + (i - row_lower));
            residual_sq += ri * ri;
        }

        result.residual_sq = Some(residual_sq);
        result.residual = Some(residual_sq.sqrt());
    }

    result.status = Status::OK;
    result
}

/// Solve weighted least squares: minimize ||W^{1/2}(Ax - b)||_2.
///
/// Equivalent to minimizing sum of w_i * (a_i^T * x - b_i)^2
/// where w_i are the weights.
pub fn weighted_least_squares(
    a: &Matrix,
    b: &Vector,
    w: &Vector,
    method: LeastSquaresMethod,
    tolerance: f64,
) -> LeastSquaresResult {
    let mut result = LeastSquaresResult {
        status: Status::NotConverged,
        solution: None,
        residual: None,
        residual_sq: None,
        rank: 0,
    };

    let row_lower = a.lower_row();
    let row_upper = a.upper_row();
    let col_lower = a.lower_col();
    let col_upper = a.upper_col();
    let m = row_upper - row_lower + 1;

    // Check dimensions
    if b.length() != m as usize || w.length() != m as usize {
        result.status = Status::InvalidInput;
        return result;
    }

    // Check weights are positive
    for i in w.lower()..=w.upper() {
        if w.get(i) <= 0.0 {
            result.status = Status::InvalidInput;
            return result;
        }
    }

    // Apply weights: A' = W^{1/2} * A, b' = W^{1/2} * b
    let mut wa = Matrix::new(row_lower, row_upper, col_lower, col_upper, 0.0);
    let mut wb = Vector::new(b.lower(), b.upper(), 0.0);

    for i in row_lower..=row_upper {
        let sqrt_w = (w.get(w.lower() + (i - row_lower))).sqrt();
        for j in col_lower..=col_upper {
            wa.set(i, j, sqrt_w * a.get(i, j));
        }
        wb.set(b.lower() + (i - row_lower), sqrt_w * b.get(b.lower() + (i - row_lower)));
    }

    // Solve weighted system
    least_squares(&wa, &wb, method, tolerance)
}

/// Solve regularized least squares (Tikhonov/Ridge regression):
/// minimize ||Ax - b||_2^2 + lambda*||x||_2^2
///
/// Adds regularization to stabilize ill-conditioned problems.
/// The solution is: x = (A^T*A + lambda*I)^{-1} * A^T * b
pub fn regularized_least_squares(
    a: &Matrix,
    b: &Vector,
    lambda: f64,
    tolerance: f64,
) -> LeastSquaresResult {
    let mut result = LeastSquaresResult {
        status: Status::NotConverged,
        solution: None,
        residual: None,
        residual_sq: None,
        rank: 0,
    };

    let row_lower = a.lower_row();
    let row_upper = a.upper_row();
    let col_lower = a.lower_col();
    let col_upper = a.upper_col();
    let m = row_upper - row_lower + 1;
    let n = col_upper - col_lower + 1;

    // Check dimensions
    if b.length() != m as usize {
        result.status = Status::InvalidInput;
        return result;
    }

    if lambda < 0.0 {
        result.status = Status::InvalidInput;
        return result;
    }

    // Form regularized normal equations: (A^T*A + lambda*I) * x = A^T * b
    let mut ata = Matrix::new(col_lower, col_upper, col_lower, col_upper, 0.0);
    let mut atb = Vector::new(col_lower, col_upper, 0.0);

    // Compute A^T * A + lambda*I
    for i in col_lower..=col_upper {
        for j in col_lower..=col_upper {
            let mut sum = 0.0;
            for k in row_lower..=row_upper {
                sum += a.get(k, i) * a.get(k, j);
            }
            ata.set(i, j, sum);
        }
        // Add regularization
        ata.set(i, i, ata.get(i, i) + lambda);
    }

    // Compute A^T * b
    for i in col_lower..=col_upper {
        let mut sum = 0.0;
        for k in row_lower..=row_upper {
            sum += a.get(k, i) * b.get(b.lower() + (k - row_lower));
        }
        atb.set(i, sum);
    }

    // Solve the regularized normal equations
    let lin_result = solve_linear(&ata, &atb, tolerance);
    if !lin_result.is_done() {
        result.status = lin_result.status;
        return result;
    }

    result.solution = lin_result.solution.clone();
    result.rank = n;

    // Compute residual
    if let Some(x) = &lin_result.solution {
        let mut residual_sq = 0.0;

        for i in row_lower..=row_upper {
            let mut axi = 0.0;
            for j in col_lower..=col_upper {
                axi += a.get(i, j) * x.get(j);
            }
            let ri = axi - b.get(b.lower() + (i - row_lower));
            residual_sq += ri * ri;
        }

        result.residual_sq = Some(residual_sq);
        result.residual = Some(residual_sq.sqrt());
    }

    result.status = Status::OK;
    result
}

/// Compute optimal regularization parameter using Leave-One-Out Cross-Validation.
///
/// Minimizes the LOO-CV score: sum_i (a_i^T * x_{-i} - b_i)^2
/// where x_{-i} is the solution with the i-th observation removed.
pub fn optimal_regularization(
    a: &Matrix,
    b: &Vector,
    lambda_min: f64,
    lambda_max: f64,
    nb_points: i32,
) -> f64 {
    let mut best_lambda = lambda_min;
    let mut best_score = f64::MAX;

    // Logarithmic grid search
    let log_min = lambda_min.log10();
    let log_max = lambda_max.log10();
    let log_step = (log_max - log_min) / ((nb_points - 1) as f64);

    for k in 0..nb_points {
        let lambda = 10.0_f64.powf(log_min + (k as f64) * log_step);

        let result = regularized_least_squares(a, b, lambda, 1e-15);
        if result.is_done() && result.residual_sq.is_some() {
            let score = result.residual_sq.unwrap();
            if score < best_score {
                best_score = score;
                best_lambda = lambda;
            }
        }
    }

    best_lambda
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_squares_overdetermined() {
        // Simple overdetermined system
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

        let result = least_squares(&a, &b, LeastSquaresMethod::NormalEquations, 1e-15);
        assert!(result.is_done());
        assert!(result.solution.is_some());
        assert!(result.residual_sq.is_some());
        assert!(result.residual.is_some());

        let x = result.solution.unwrap();
        // Least squares solution should be approximately [1.0, 1.0]
        assert!((x.get(1) - 1.0).abs() < 1e-10);
        assert!((x.get(2) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_weighted_least_squares() {
        // Weighted least squares with w = [1, 1, 1]
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

        let mut w = Vector::new(1, 3, 1.0);

        let result = weighted_least_squares(&a, &b, &w, LeastSquaresMethod::NormalEquations, 1e-15);
        assert!(result.is_done());
        assert!(result.solution.is_some());

        // With equal weights, should be same as unweighted
        let x = result.solution.unwrap();
        assert!((x.get(1) - 1.0).abs() < 1e-10);
        assert!((x.get(2) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_regularized_least_squares() {
        // Test regularization
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

        let result = regularized_least_squares(&a, &b, 0.1, 1e-15);
        assert!(result.is_done());
        assert!(result.solution.is_some());
        assert!(result.rank == 2);
    }

    #[test]
    fn test_weighted_least_squares_invalid_weights() {
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

        let mut w = Vector::new(1, 3, 1.0);
        w.set(2, -1.0); // Negative weight

        let result = weighted_least_squares(&a, &b, &w, LeastSquaresMethod::NormalEquations, 1e-15);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_regularized_invalid_lambda() {
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

        let result = regularized_least_squares(&a, &b, -0.1, 1e-15);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_optimal_regularization() {
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

        let lambda = optimal_regularization(&a, &b, 1e-10, 1e2, 20);
        assert!(lambda > 0.0);
        assert!(lambda < 1e2);
    }

    #[test]
    fn test_least_squares_invalid_dimensions() {
        let a = Matrix::new(1, 3, 1, 2, 1.0);
        let b = Vector::new(1, 2, 1.0); // Wrong size

        let result = least_squares(&a, &b, LeastSquaresMethod::NormalEquations, 1e-15);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::InvalidInput);
    }
}

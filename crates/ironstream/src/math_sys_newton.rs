// FILE: math_sys_newton.rs

// occt: MathSys_Newton

use crate::math_sys_newton_types::{Status, NewtonResultN};

/// Result of solving a system of nonlinear equations.
/// Contains solution vector, Jacobian, and convergence diagnostics.
pub struct VectorResult {
    /// Final solver status
    pub status: Status,
    /// Number of iterations performed
    pub nb_iterations: usize,
    /// Solution vector (set by solver on success)
    pub solution: Option<Vec<f64>>,
    /// Final function value at solution (if computed)
    pub value: Option<f64>,
    /// Gradient at solution (if computed)
    pub gradient: Option<Vec<f64>>,
    /// Jacobian at solution (if computed)
    pub jacobian: Option<Vec<Vec<f64>>>,
}

impl VectorResult {
    /// Create a new empty result
    pub fn new() -> Self {
        VectorResult {
            status: Status::NotConverged,
            nb_iterations: 0,
            solution: None,
            value: None,
            gradient: None,
            jacobian: None,
        }
    }

    /// Returns true if computation succeeded
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

impl Default for VectorResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for function sets with derivatives (Jacobian).
/// Implementations compute function values and Jacobian matrix.
pub trait FunctionSetWithDerivatives {
    /// Number of variables in the system
    fn nb_variables(&self) -> usize;

    /// Number of equations in the system
    fn nb_equations(&self) -> usize;

    /// Evaluate function at point X, returning vector F(X).
    /// Returns false if evaluation failed (NaN, Inf, etc.).
    fn value(&mut self, x: &[f64], f: &mut [f64]) -> bool;

    /// Evaluate function and Jacobian at point X.
    /// Sets f to F(X) and jacobian to J(X).
    /// Returns false if evaluation failed.
    fn values(&mut self, x: &[f64], f: &mut [f64], jacobian: &mut [Vec<f64>]) -> bool;
}

/// Solve a system of nonlinear equations using Newton-Raphson method.
///
/// Solves F(X) = 0 where F is a vector function of vector X.
/// The method iteratively improves an initial guess by solving
/// the linear system J(X)*dX = -F(X) where J is the Jacobian matrix.
///
/// # Arguments
/// * `func` - function set with derivatives (Jacobian)
/// * `start` - initial guess vector
/// * `tol_x` - tolerance for solution change ||X(n+1) - X(n)|| < tol_x (per component)
/// * `tol_f` - tolerance for function values ||F(X)|| < tol_f
/// * `max_iter` - maximum number of iterations (default 100)
///
/// # Returns
/// VectorResult containing solution vector if converged
pub fn newton<F: FunctionSetWithDerivatives>(
    func: &mut F,
    start: &[f64],
    tol_x: &[f64],
    tol_f: f64,
    max_iter: usize,
) -> VectorResult {
    let mut result = VectorResult::new();

    let n = func.nb_variables();
    let m = func.nb_equations();

    // Check dimensions
    if n != m || start.len() != n || tol_x.len() != n {
        result.status = Status::InvalidInput;
        return result;
    }

    // Working vectors and matrices
    let mut sol = start.to_vec();
    let mut f = vec![0.0; n];
    let mut delta_x = vec![0.0; n];
    let mut jacobian = vec![vec![0.0; n]; n];

    // Newton iteration
    for iter in 0..max_iter {
        // Evaluate function and Jacobian
        if !func.values(&sol, &mut f, &mut jacobian) {
            result.status = Status::NumericalError;
            result.nb_iterations = iter;
            return result;
        }

        // Solve J * dX = -F using Gaussian elimination
        // Create augmented matrix for solving
        let mut aug = jacobian.clone();
        for i in 0..n {
            aug[i].push(-f[i]);
        }

        // Gaussian elimination with partial pivoting
        if !gaussian_elimination(&mut aug, &mut delta_x) {
            result.status = Status::Singular;
            result.nb_iterations = iter;
            return result;
        }

        // Check X convergence before update
        let mut x_converged = true;
        for i in 0..n {
            if delta_x[i].abs() > tol_x[i] {
                x_converged = false;
                break;
            }
        }

        // Update solution
        for i in 0..n {
            sol[i] += delta_x[i];
        }

        // Re-evaluate function at new solution to check F convergence
        if !func.value(&sol, &mut f) {
            result.status = Status::NumericalError;
            result.nb_iterations = iter + 1;
            return result;
        }

        // Check F convergence with updated function values
        let mut f_converged = true;
        for i in 0..n {
            if f[i].abs() > tol_f {
                f_converged = false;
                break;
            }
        }

        result.nb_iterations = iter + 1;

        if x_converged && f_converged {
            result.status = Status::OK;
            result.solution = Some(sol);
            result.jacobian = Some(jacobian);
            return result;
        }
    }

    // Max iterations reached - return last solution
    result.status = Status::MaxIterations;
    result.solution = Some(sol);
    result.jacobian = Some(jacobian);
    result
}

/// Solve a system of nonlinear equations with bounds constraints.
///
/// Solves F(X) = 0 subject to InfBound <= X <= SupBound.
/// If the Newton step would take X outside bounds, the solution
/// is clamped to the boundary.
///
/// # Arguments
/// * `func` - function set with derivatives (Jacobian)
/// * `start` - initial guess vector
/// * `inf_bound` - lower bounds for solution
/// * `sup_bound` - upper bounds for solution
/// * `tol_x` - tolerance for solution change (per component)
/// * `tol_f` - tolerance for function values
/// * `max_iter` - maximum number of iterations (default 100)
///
/// # Returns
/// VectorResult containing solution vector if converged
pub fn newton_bounded<F: FunctionSetWithDerivatives>(
    func: &mut F,
    start: &[f64],
    inf_bound: &[f64],
    sup_bound: &[f64],
    tol_x: &[f64],
    tol_f: f64,
    max_iter: usize,
) -> VectorResult {
    let mut result = VectorResult::new();

    let n = func.nb_variables();
    let m = func.nb_equations();

    // Check dimensions
    if n != m
        || start.len() != n
        || tol_x.len() != n
        || inf_bound.len() != n
        || sup_bound.len() != n
    {
        result.status = Status::InvalidInput;
        return result;
    }

    // Working vectors and matrices
    let mut sol = start.to_vec();
    let mut f = vec![0.0; n];
    let mut delta_x = vec![0.0; n];
    let mut jacobian = vec![vec![0.0; n]; n];

    // Clamp initial solution to bounds
    for i in 0..n {
        if sol[i] < inf_bound[i] {
            sol[i] = inf_bound[i];
        }
        if sol[i] > sup_bound[i] {
            sol[i] = sup_bound[i];
        }
    }

    // Newton iteration
    for iter in 0..max_iter {
        // Evaluate function and Jacobian
        if !func.values(&sol, &mut f, &mut jacobian) {
            result.status = Status::NumericalError;
            result.nb_iterations = iter;
            return result;
        }

        // Solve J * dX = -F
        let mut aug = jacobian.clone();
        for i in 0..n {
            aug[i].push(-f[i]);
        }

        if !gaussian_elimination(&mut aug, &mut delta_x) {
            result.status = Status::Singular;
            result.nb_iterations = iter;
            return result;
        }

        // Check X convergence before update
        let mut x_converged = true;
        for i in 0..n {
            if delta_x[i].abs() > tol_x[i] {
                x_converged = false;
                break;
            }
        }

        // Update solution with bounds clamping
        for i in 0..n {
            sol[i] += delta_x[i];
            if sol[i] < inf_bound[i] {
                sol[i] = inf_bound[i];
            }
            if sol[i] > sup_bound[i] {
                sol[i] = sup_bound[i];
            }
        }

        // Re-evaluate function at new solution to check F convergence
        if !func.value(&sol, &mut f) {
            result.status = Status::NumericalError;
            result.nb_iterations = iter + 1;
            return result;
        }

        // Check F convergence with updated function values
        let mut f_converged = true;
        for i in 0..n {
            if f[i].abs() > tol_f {
                f_converged = false;
                break;
            }
        }

        result.nb_iterations = iter + 1;

        if x_converged && f_converged {
            result.status = Status::OK;
            result.solution = Some(sol);
            result.jacobian = Some(jacobian);
            return result;
        }
    }

    // Max iterations reached
    result.status = Status::MaxIterations;
    result.solution = Some(sol);
    result.jacobian = Some(jacobian);
    result
}

/// Simplified Newton method with uniform tolerances.
/// Delegates to newton() with a uniform tolerance vector.
///
/// # Arguments
/// * `func` - function set with derivatives
/// * `start` - initial guess vector
/// * `tol_x` - uniform tolerance for all variables
/// * `tol_f` - tolerance for function values
/// * `max_iter` - maximum number of iterations
///
/// # Returns
/// VectorResult containing solution vector if converged
pub fn newton_uniform<F: FunctionSetWithDerivatives>(
    func: &mut F,
    start: &[f64],
    tol_x: f64,
    tol_f: f64,
    max_iter: usize,
) -> VectorResult {
    let tol_x_vec = vec![tol_x; start.len()];
    newton(func, start, &tol_x_vec, tol_f, max_iter)
}

/// Gaussian elimination with partial pivoting for solving Ax=b.
/// The augmented matrix is [A|b], with b in the last column.
fn gaussian_elimination(aug: &mut [Vec<f64>], solution: &mut [f64]) -> bool {
    let n = aug.len();
    if n == 0 || aug[0].is_empty() || aug[0].len() != n + 1 {
        return false;
    }

    // Forward elimination with partial pivoting
    for col in 0..n {
        // Find pivot
        let mut pivot_row = col;
        let mut max_val = aug[col][col].abs();
        for row in (col + 1)..n {
            let val = aug[row][col].abs();
            if val > max_val {
                max_val = val;
                pivot_row = row;
            }
        }

        // Check for singular matrix
        if max_val < 1.0e-15 {
            return false;
        }

        // Swap rows
        aug.swap(col, pivot_row);

        // Eliminate below pivot
        for row in (col + 1)..n {
            if aug[col][col].abs() < 1.0e-15 {
                return false;
            }
            let factor = aug[row][col] / aug[col][col];
            for j in col..=n {
                aug[row][j] -= factor * aug[col][j];
            }
        }
    }

    // Back substitution
    for i in (0..n).rev() {
        if aug[i][i].abs() < 1.0e-15 {
            return false;
        }
        let mut sum = aug[i][n];
        for j in (i + 1)..n {
            sum -= aug[i][j] * solution[j];
        }
        solution[i] = sum / aug[i][i];
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple test function: f(x,y) = [x^2 + y^2 - 1, x - y]
    /// Solution near (sqrt(2)/2, sqrt(2)/2)
    struct TestFunc2D;

    impl FunctionSetWithDerivatives for TestFunc2D {
        fn nb_variables(&self) -> usize {
            2
        }

        fn nb_equations(&self) -> usize {
            2
        }

        fn value(&mut self, x: &[f64], f: &mut [f64]) -> bool {
            if x.len() < 2 || f.len() < 2 {
                return false;
            }
            // f1 = x^2 + y^2 - 1
            // f2 = x - y
            f[0] = x[0] * x[0] + x[1] * x[1] - 1.0;
            f[1] = x[0] - x[1];
            true
        }

        fn values(&mut self, x: &[f64], f: &mut [f64], jacobian: &mut [Vec<f64>]) -> bool {
            if !self.value(x, f) {
                return false;
            }

            // Jacobian:
            // J = [2x, 2y]
            //     [1,  -1]
            if jacobian.len() < 2 || jacobian[0].len() < 2 || jacobian[1].len() < 2 {
                return false;
            }

            jacobian[0][0] = 2.0 * x[0];
            jacobian[0][1] = 2.0 * x[1];
            jacobian[1][0] = 1.0;
            jacobian[1][1] = -1.0;
            true
        }
    }

    #[test]
    fn test_newton_simple() {
        let mut func = TestFunc2D;
        let start = vec![0.5, 0.5];
        let tol_x = vec![1.0e-10, 1.0e-10];
        let tol_f = 1.0e-10;

        let result = newton(&mut func, &start, &tol_x, tol_f, 100);

        assert_eq!(result.status, Status::OK);
        assert!(result.nb_iterations > 0);
        assert!(result.solution.is_some());

        let sol = result.solution.unwrap();
        assert_eq!(sol.len(), 2);

        // Check solution approximately solves the system
        let x = sol[0];
        let y = sol[1];
        let f1 = x * x + y * y - 1.0;
        let f2 = x - y;

        assert!(f1.abs() < 1.0e-9);
        assert!(f2.abs() < 1.0e-9);

        // Check approximate solution: x = y, x^2 + x^2 = 1 => x = 1/sqrt(2)
        let expected = std::f64::consts::SQRT_2 / 2.0;
        assert!((x - expected).abs() < 1.0e-8);
        assert!((y - expected).abs() < 1.0e-8);
    }

    #[test]
    fn test_newton_bounded() {
        let mut func = TestFunc2D;
        let start = vec![0.5, 0.5];
        let inf_bound = vec![0.0, 0.0];
        let sup_bound = vec![1.0, 1.0];
        let tol_x = vec![1.0e-10, 1.0e-10];
        let tol_f = 1.0e-10;

        let result = newton_bounded(&mut func, &start, &inf_bound, &sup_bound, &tol_x, tol_f, 100);

        assert_eq!(result.status, Status::OK);
        let sol = result.solution.unwrap();

        // Verify bounds are satisfied
        assert!(sol[0] >= inf_bound[0] && sol[0] <= sup_bound[0]);
        assert!(sol[1] >= inf_bound[1] && sol[1] <= sup_bound[1]);

        // Verify solution is correct
        let f1 = sol[0] * sol[0] + sol[1] * sol[1] - 1.0;
        let f2 = sol[0] - sol[1];
        assert!(f1.abs() < 1.0e-8);
        assert!(f2.abs() < 1.0e-8);
    }

    #[test]
    fn test_newton_uniform() {
        let mut func = TestFunc2D;
        let start = vec![0.5, 0.5];
        let tol = 1.0e-10;

        let result = newton_uniform(&mut func, &start, tol, tol, 100);

        assert_eq!(result.status, Status::OK);
        assert!(result.solution.is_some());
    }

    #[test]
    fn test_newton_invalid_dimensions() {
        let mut func = TestFunc2D;
        let start = vec![0.5]; // Wrong size
        let tol_x = vec![1.0e-10, 1.0e-10];
        let result = newton(&mut func, &start, &tol_x, 1.0e-10, 100);

        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_vector_result_is_done() {
        let mut result = VectorResult::new();
        assert!(!result.is_done());

        result.status = Status::OK;
        assert!(result.is_done());
    }

    #[test]
    fn test_gaussian_elimination_2x2() {
        // Solve [2, 3 | 5] => x=1, y=1
        //       [1, 1 | 2]
        let mut aug = vec![vec![2.0, 3.0, 5.0], vec![1.0, 1.0, 2.0]];
        let mut sol = vec![0.0, 0.0];

        assert!(gaussian_elimination(&mut aug, &mut sol));
        assert!((sol[0] - 1.0).abs() < 1.0e-14);
        assert!((sol[1] - 1.0).abs() < 1.0e-14);
    }

    #[test]
    fn test_gaussian_elimination_singular() {
        // Singular matrix: [1, 1 | 2]
        //                   [2, 2 | 4]
        let mut aug = vec![vec![1.0, 1.0, 2.0], vec![2.0, 2.0, 4.0]];
        let mut sol = vec![0.0, 0.0];

        assert!(!gaussian_elimination(&mut aug, &mut sol));
    }
}

// FILE: math_sys_levenberg_marquardt.rs

/// occt: MathSys_LevenbergMarquardt
///
/// Levenberg-Marquardt algorithm for nonlinear least squares.
///
/// Minimizes ||F(X)||^2 where F is a vector function of vector X.
/// Combines Gauss-Newton method (fast near minimum) with gradient
/// descent (robust far from minimum) using adaptive damping.
///
/// Algorithm:
/// 1. Compute F(X) and Jacobian J at current point
/// 2. Solve (J^T*J + lambda*I) * dX = -J^T*F for correction dX
/// 3. If ||F(X+dX)||^2 < ||F(X)||^2: accept step, decrease lambda
/// 4. Otherwise: reject step, increase lambda
/// 5. Repeat until convergence
pub mod lm_solver {
    use std::f64;

    /// Solver status enumeration matching OCCT MathUtils::Status.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Status {
        OK,
        NotConverged,
        MaxIterations,
        NumericalError,
        InvalidInput,
        Singular,
    }

    /// Base solver configuration.
    #[derive(Debug, Clone)]
    pub struct Config {
        pub max_iterations: usize,
        pub tolerance: f64,
        pub x_tolerance: f64,
        pub f_tolerance: f64,
    }

    impl Default for Config {
        fn default() -> Self {
            Config {
                max_iterations: 100,
                tolerance: 1.0e-10,
                x_tolerance: 1.0e-10,
                f_tolerance: 1.0e-10,
            }
        }
    }

    /// Configuration for Levenberg-Marquardt algorithm.
    /// Extends base Config with damping parameter settings.
    #[derive(Debug, Clone)]
    pub struct LMConfig {
        pub max_iterations: usize,
        pub tolerance: f64,
        pub x_tolerance: f64,
        pub f_tolerance: f64,
        pub lambda_init: f64,
        pub lambda_increase: f64,
        pub lambda_decrease: f64,
        pub lambda_max: f64,
        pub lambda_min: f64,
    }

    impl Default for LMConfig {
        fn default() -> Self {
            LMConfig {
                max_iterations: 100,
                tolerance: 1.0e-10,
                x_tolerance: 1.0e-10,
                f_tolerance: 1.0e-10,
                lambda_init: 1.0e-3,
                lambda_increase: 10.0,
                lambda_decrease: 0.1,
                lambda_max: 1.0e10,
                lambda_min: 1.0e-12,
            }
        }
    }

    /// Simple matrix type for local use (row-major, 1-indexed compatible).
    #[derive(Clone, Debug)]
    pub struct Matrix {
        data: Vec<f64>,
        rows: usize,
        cols: usize,
    }

    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Self {
            Matrix {
                data: vec![0.0; rows * cols],
                rows,
                cols,
            }
        }

        #[inline]
        pub fn get(&self, i: usize, j: usize) -> f64 {
            self.data[i * self.cols + j]
        }

        #[inline]
        pub fn set(&mut self, i: usize, j: usize, val: f64) {
            self.data[i * self.cols + j] = val;
        }

        pub fn rows(&self) -> usize {
            self.rows
        }

        pub fn cols(&self) -> usize {
            self.cols
        }

        pub fn clone_matrix(&self) -> Matrix {
            Matrix {
                data: self.data.clone(),
                rows: self.rows,
                cols: self.cols,
            }
        }
    }

    /// Simple vector type for local use.
    #[derive(Clone, Debug)]
    pub struct Vector {
        pub data: Vec<f64>,
    }

    impl Vector {
        pub fn new(size: usize) -> Self {
            Vector {
                data: vec![0.0; size],
            }
        }

        #[inline]
        pub fn get(&self, i: usize) -> f64 {
            self.data[i]
        }

        #[inline]
        pub fn set(&mut self, i: usize, val: f64) {
            self.data[i] = val;
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn dot_product(&self, other: &Vector) -> f64 {
            self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum()
        }

        pub fn norm(&self) -> f64 {
            self.dot_product(self).sqrt()
        }

        pub fn clone_vec(&self) -> Vector {
            Vector {
                data: self.data.clone(),
            }
        }
    }

    /// Result of N-dimensional optimization.
    #[derive(Debug, Clone)]
    pub struct VectorResult {
        pub status: Status,
        pub nb_iterations: usize,
        pub solution: Option<Vector>,
        pub value: Option<f64>,
        pub gradient: Option<Vector>,
    }

    impl Default for VectorResult {
        fn default() -> Self {
            VectorResult {
                status: Status::NotConverged,
                nb_iterations: 0,
                solution: None,
                value: None,
                gradient: None,
            }
        }
    }

    /// Clamp value to range [lower, upper].
    #[inline]
    fn clamp(value: f64, lower: f64, upper: f64) -> f64 {
        if value < lower {
            lower
        } else if value > upper {
            upper
        } else {
            value
        }
    }

    /// Solve 2x2 system by direct inversion.
    fn solve_2x2(a: &Matrix, b: &Vector) -> Option<Vector> {
        if a.rows() != 2 || a.cols() != 2 || b.len() != 2 {
            return None;
        }

        let a00 = a.get(0, 0);
        let a01 = a.get(0, 1);
        let a10 = a.get(1, 0);
        let a11 = a.get(1, 1);

        let det = a00 * a11 - a01 * a10;
        if det.abs() < 1.0e-15 {
            return None;
        }

        let mut x = Vector::new(2);
        let b0 = b.get(0);
        let b1 = b.get(1);

        x.set(0, (a11 * b0 - a01 * b1) / det);
        x.set(1, (a00 * b1 - a10 * b0) / det);
        Some(x)
    }

    /// Solve NxN system using Gaussian elimination with partial pivoting.
    fn solve_gauss(a: &Matrix, b: &Vector) -> Option<Vector> {
        let n = a.rows();
        if a.cols() != n || b.len() != n {
            return None;
        }

        // Special case for 2x2
        if n == 2 {
            return solve_2x2(a, b);
        }

        // Create LU decomposition with partial pivoting
        let mut lu = a.clone_matrix();
        let mut piv = vec![0_usize; n];
        for i in 0..n {
            piv[i] = i;
        }

        // Perform LU decomposition
        for i in 0..n {
            // Find pivot
            let mut pivot_row = i;
            let mut pivot_val = lu.get(i, i).abs();
            for k in (i + 1)..n {
                let val = lu.get(k, i).abs();
                if val > pivot_val {
                    pivot_val = val;
                    pivot_row = k;
                }
            }

            if pivot_val < 1.0e-15 {
                return None; // Matrix is singular
            }

            // Swap rows
            if pivot_row != i {
                for j in 0..n {
                    let tmp = lu.get(i, j);
                    lu.set(i, j, lu.get(pivot_row, j));
                    lu.set(pivot_row, j, tmp);
                }
                piv.swap(i, pivot_row);
            }

            // Eliminate column
            for k in (i + 1)..n {
                let factor = lu.get(k, i) / lu.get(i, i);
                lu.set(k, i, factor);
                for j in (i + 1)..n {
                    let val = lu.get(k, j) - factor * lu.get(i, j);
                    lu.set(k, j, val);
                }
            }
        }

        // Permute b according to pivoting
        let mut bp = Vector::new(n);
        for i in 0..n {
            bp.set(i, b.get(piv[i]));
        }

        // Forward substitution (solve Ly = bp)
        let mut y = Vector::new(n);
        for i in 0..n {
            let mut sum = bp.get(i);
            for j in 0..i {
                sum -= lu.get(i, j) * y.get(j);
            }
            y.set(i, sum);
        }

        // Backward substitution (solve Ux = y)
        let mut x = Vector::new(n);
        for i in (0..n).rev() {
            let mut sum = y.get(i);
            for j in (i + 1)..n {
                sum -= lu.get(i, j) * x.get(j);
            }
            let diag = lu.get(i, i);
            if diag.abs() < 1.0e-15 {
                return None;
            }
            x.set(i, sum / diag);
        }

        Some(x)
    }

    /// Levenberg-Marquardt algorithm for nonlinear least squares.
    ///
    /// # Arguments
    /// * `func` - Function set with methods:
    ///   - `nb_variables() -> usize`
    ///   - `nb_equations() -> usize`
    ///   - `value(x: &Vector) -> Option<Vector>` (residuals F)
    ///   - `derivatives(x: &Vector) -> Option<Matrix>` (Jacobian J)
    /// * `start` - Initial guess vector
    /// * `config` - Levenberg-Marquardt configuration
    ///
    /// # Returns
    /// Result containing solution vector if converged
    pub fn levenberg_marquardt<F>(
        func: &mut F,
        start: &Vector,
        config: &LMConfig,
    ) -> VectorResult
    where
        F: FnMut(&Vector) -> (Option<Vector>, Option<Matrix>),
    {
        let mut result = VectorResult::default();

        // Get dimensions from function
        let (f_val, _) = func(start);
        let nb_eqs = match &f_val {
            Some(f) => f.len(),
            None => {
                result.status = Status::NumericalError;
                return result;
            }
        };

        let nb_vars = start.len();

        // Check dimensions
        if nb_vars == 0 || nb_eqs == 0 {
            result.status = Status::InvalidInput;
            return result;
        }

        // Working vectors and matrices
        let mut sol = start.clone_vec();
        let mut f_vec = f_val.unwrap();
        #[allow(unused_assignments)]
        let mut delta_x = Vector::new(nb_vars);
        let mut grad = Vector::new(nb_vars);

        let mut lambda = config.lambda_init;

        // Compute initial chi^2 = ||F||^2
        let mut chi2 = f_vec.dot_product(&f_vec);

        // Main iteration loop
        for iter in 0..config.max_iterations {
            result.nb_iterations = iter + 1;

            // Check F convergence (all residuals small)
            let mut f_converged = true;
            for i in 0..nb_eqs {
                if f_vec.get(i).abs() > config.f_tolerance {
                    f_converged = false;
                    break;
                }
            }

            if f_converged {
                result.status = Status::OK;
                result.solution = Some(sol.clone_vec());
                result.value = Some(chi2);
                return result;
            }

            // Compute Jacobian
            let (_, jac_opt) = func(&sol);
            let jac = match jac_opt {
                Some(j) => j,
                None => {
                    result.status = Status::NumericalError;
                    return result;
                }
            };

            // Compute J^T * J
            let mut jtj = Matrix::new(nb_vars, nb_vars);
            for i in 0..nb_vars {
                for j in 0..nb_vars {
                    let mut sum = 0.0;
                    for k in 0..nb_eqs {
                        sum += jac.get(k, i) * jac.get(k, j);
                    }
                    jtj.set(i, j, sum);
                }
            }

            // Compute J^T * F and gradient
            let mut jtf = Vector::new(nb_vars);
            for i in 0..nb_vars {
                let mut sum = 0.0;
                for k in 0..nb_eqs {
                    sum += jac.get(k, i) * f_vec.get(k);
                }
                jtf.set(i, sum);
                grad.set(i, 2.0 * sum);
            }

            // Check gradient convergence
            let grad_norm = grad.norm();
            if grad_norm < config.tolerance {
                result.status = Status::OK;
                result.solution = Some(sol.clone_vec());
                result.value = Some(chi2);
                result.gradient = Some(grad);
                return result;
            }

            // Inner loop: try to find acceptable step
            let mut step_accepted = false;
            for _lam_iter in 0..20 {
                if step_accepted {
                    break;
                }

                // Add damping: J^T*J + lambda*I
                let mut damped = jtj.clone_matrix();
                for i in 0..nb_vars {
                    damped.set(i, i, damped.get(i, i) + lambda);
                }

                // Solve (J^T*J + lambda*I) * dX = -J^T*F
                let mut neg_jtf = Vector::new(nb_vars);
                for i in 0..nb_vars {
                    neg_jtf.set(i, -jtf.get(i));
                }

                let sol_opt = solve_gauss(&damped, &neg_jtf);
                let sol_delta = match sol_opt {
                    Some(s) => s,
                    None => {
                        // Matrix is singular, increase damping
                        lambda *= config.lambda_increase;
                        if lambda > config.lambda_max {
                            result.status = Status::Singular;
                            result.solution = Some(sol.clone_vec());
                            result.value = Some(chi2);
                            return result;
                        }
                        continue;
                    }
                };

                delta_x = sol_delta;

                // Compute new solution
                let mut sol_new = Vector::new(nb_vars);
                for i in 0..nb_vars {
                    sol_new.set(i, sol.get(i) + delta_x.get(i));
                }

                // Evaluate new residual
                let (f_new_opt, _) = func(&sol_new);
                let f_new_vec = match f_new_opt {
                    Some(f) => f,
                    None => {
                        // Function evaluation failed, increase damping
                        lambda *= config.lambda_increase;
                        if lambda > config.lambda_max {
                            result.status = Status::NumericalError;
                            result.solution = Some(sol.clone_vec());
                            result.value = Some(chi2);
                            return result;
                        }
                        continue;
                    }
                };

                // Compute new ||F||^2
                let chi2_new = f_new_vec.dot_product(&f_new_vec);

                // Accept or reject step
                if chi2_new < chi2 {
                    // Step accepted
                    sol = sol_new;
                    f_vec = f_new_vec;
                    chi2 = chi2_new;
                    lambda *= config.lambda_decrease;
                    if lambda < config.lambda_min {
                        lambda = config.lambda_min;
                    }
                    step_accepted = true;

                    // Check X convergence
                    let mut x_converged = true;
                    for i in 0..nb_vars {
                        let threshold = config.x_tolerance * (1.0 + sol.get(i).abs());
                        if delta_x.get(i).abs() > threshold {
                            x_converged = false;
                            break;
                        }
                    }

                    if x_converged {
                        result.status = Status::OK;
                        result.solution = Some(sol.clone_vec());
                        result.value = Some(chi2);
                        return result;
                    }
                } else {
                    // Step rejected, increase damping
                    lambda *= config.lambda_increase;
                    if lambda > config.lambda_max {
                        result.status = Status::NotConverged;
                        result.solution = Some(sol.clone_vec());
                        result.value = Some(chi2);
                        return result;
                    }
                }
            }

            if !step_accepted {
                // Failed to find acceptable step
                result.status = Status::NotConverged;
                result.solution = Some(sol.clone_vec());
                result.value = Some(chi2);
                return result;
            }
        }

        // Max iterations reached
        result.status = Status::MaxIterations;
        result.solution = Some(sol);
        result.value = Some(chi2);
        result
    }

    /// Levenberg-Marquardt with bounds constraints.
    ///
    /// Minimizes ||F(X)||^2 subject to inf_bound <= X <= sup_bound.
    /// Solution is clamped to bounds after each step.
    pub fn levenberg_marquardt_bounded<F>(
        func: &mut F,
        start: &Vector,
        inf_bound: &Vector,
        sup_bound: &Vector,
        config: &LMConfig,
    ) -> VectorResult
    where
        F: FnMut(&Vector) -> (Option<Vector>, Option<Matrix>),
    {
        let mut result = VectorResult::default();

        // Get dimensions from function
        let (f_val, _) = func(start);
        let nb_eqs = match &f_val {
            Some(f) => f.len(),
            None => {
                result.status = Status::NumericalError;
                return result;
            }
        };

        let nb_vars = start.len();

        // Check dimensions
        if nb_vars != inf_bound.len() || nb_vars != sup_bound.len() {
            result.status = Status::InvalidInput;
            return result;
        }

        // Clamp initial solution to bounds
        let mut sol = start.clone_vec();
        for i in 0..nb_vars {
            let val = sol.get(i);
            let lower = inf_bound.get(i);
            let upper = sup_bound.get(i);
            sol.set(i, clamp(val, lower, upper));
        }

        let mut f_vec = f_val.unwrap();
        #[allow(unused_assignments)]
        let mut delta_x = Vector::new(nb_vars);
        let mut grad = Vector::new(nb_vars);

        let mut lambda = config.lambda_init;

        // Compute initial chi^2
        let mut chi2 = f_vec.dot_product(&f_vec);

        // Main iteration loop
        for iter in 0..config.max_iterations {
            result.nb_iterations = iter + 1;

            // Check F convergence
            let mut f_converged = true;
            for i in 0..nb_eqs {
                if f_vec.get(i).abs() > config.f_tolerance {
                    f_converged = false;
                    break;
                }
            }

            if f_converged {
                result.status = Status::OK;
                result.solution = Some(sol.clone_vec());
                result.value = Some(chi2);
                return result;
            }

            // Compute Jacobian
            let (_, jac_opt) = func(&sol);
            let jac = match jac_opt {
                Some(j) => j,
                None => {
                    result.status = Status::NumericalError;
                    return result;
                }
            };

            // Compute J^T * J
            let mut jtj = Matrix::new(nb_vars, nb_vars);
            for i in 0..nb_vars {
                for j in 0..nb_vars {
                    let mut sum = 0.0;
                    for k in 0..nb_eqs {
                        sum += jac.get(k, i) * jac.get(k, j);
                    }
                    jtj.set(i, j, sum);
                }
            }

            // Compute J^T * F and gradient
            let mut jtf = Vector::new(nb_vars);
            for i in 0..nb_vars {
                let mut sum = 0.0;
                for k in 0..nb_eqs {
                    sum += jac.get(k, i) * f_vec.get(k);
                }
                jtf.set(i, sum);
                grad.set(i, 2.0 * sum);
            }

            // Check gradient convergence
            let grad_norm = grad.norm();
            if grad_norm < config.tolerance {
                result.status = Status::OK;
                result.solution = Some(sol.clone_vec());
                result.value = Some(chi2);
                result.gradient = Some(grad);
                return result;
            }

            // Inner loop: try to find acceptable step
            let mut step_accepted = false;
            for _lam_iter in 0..20 {
                if step_accepted {
                    break;
                }

                // Add damping
                let mut damped = jtj.clone_matrix();
                for i in 0..nb_vars {
                    damped.set(i, i, damped.get(i, i) + lambda);
                }

                // Solve (J^T*J + lambda*I) * dX = -J^T*F
                let mut neg_jtf = Vector::new(nb_vars);
                for i in 0..nb_vars {
                    neg_jtf.set(i, -jtf.get(i));
                }

                let sol_opt = solve_gauss(&damped, &neg_jtf);
                let sol_delta = match sol_opt {
                    Some(s) => s,
                    None => {
                        lambda *= config.lambda_increase;
                        if lambda > config.lambda_max {
                            result.status = Status::Singular;
                            result.solution = Some(sol.clone_vec());
                            result.value = Some(chi2);
                            return result;
                        }
                        continue;
                    }
                };

                delta_x = sol_delta;

                // Compute new solution with bounds clamping
                let mut sol_new = Vector::new(nb_vars);
                for i in 0..nb_vars {
                    let val = sol.get(i) + delta_x.get(i);
                    let lower = inf_bound.get(i);
                    let upper = sup_bound.get(i);
                    sol_new.set(i, clamp(val, lower, upper));
                }

                // Evaluate new residual
                let (f_new_opt, _) = func(&sol_new);
                let f_new_vec = match f_new_opt {
                    Some(f) => f,
                    None => {
                        lambda *= config.lambda_increase;
                        if lambda > config.lambda_max {
                            result.status = Status::NumericalError;
                            result.solution = Some(sol.clone_vec());
                            result.value = Some(chi2);
                            return result;
                        }
                        continue;
                    }
                };

                // Compute new ||F||^2
                let chi2_new = f_new_vec.dot_product(&f_new_vec);

                // Accept or reject step
                if chi2_new < chi2 {
                    // Step accepted
                    sol = sol_new;
                    f_vec = f_new_vec;
                    chi2 = chi2_new;
                    lambda *= config.lambda_decrease;
                    if lambda < config.lambda_min {
                        lambda = config.lambda_min;
                    }
                    step_accepted = true;

                    // Check X convergence
                    let mut x_converged = true;
                    for i in 0..nb_vars {
                        let threshold = config.x_tolerance * (1.0 + sol.get(i).abs());
                        if delta_x.get(i).abs() > threshold {
                            x_converged = false;
                            break;
                        }
                    }

                    if x_converged {
                        result.status = Status::OK;
                        result.solution = Some(sol.clone_vec());
                        result.value = Some(chi2);
                        return result;
                    }
                } else {
                    // Step rejected, increase damping
                    lambda *= config.lambda_increase;
                    if lambda > config.lambda_max {
                        result.status = Status::NotConverged;
                        result.solution = Some(sol.clone_vec());
                        result.value = Some(chi2);
                        return result;
                    }
                }
            }

            if !step_accepted {
                result.status = Status::NotConverged;
                result.solution = Some(sol.clone_vec());
                result.value = Some(chi2);
                return result;
            }
        }

        // Max iterations reached
        result.status = Status::MaxIterations;
        result.solution = Some(sol);
        result.value = Some(chi2);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::lm_solver::*;

    #[test]
    fn test_lm_linear_system() {
        // Solve linear system: 2*x - y = 1, x + y = 2
        // Solution: x = 1, y = 1
        let start = Vector {
            data: vec![0.0, 0.0],
        };

        let config = LMConfig::default();

        let mut call_count = 0;
        let result = levenberg_marquardt(
            &mut |x: &Vector| -> (Option<Vector>, Option<Matrix>) {
                call_count += 1;
                let x0 = x.get(0);
                let x1 = x.get(1);

                // Residuals
                let mut f = Vector::new(2);
                f.set(0, 2.0 * x0 - x1 - 1.0);
                f.set(1, x0 + x1 - 2.0);

                // Jacobian
                let mut j = Matrix::new(2, 2);
                j.set(0, 0, 2.0);
                j.set(0, 1, -1.0);
                j.set(1, 0, 1.0);
                j.set(1, 1, 1.0);

                (Some(f), Some(j))
            },
            &start,
            &config,
        );

        assert_eq!(result.status, Status::OK);
        assert!(result.solution.is_some());
        let sol = result.solution.unwrap();
        assert!((sol.get(0) - 1.0).abs() < 1.0e-8);
        assert!((sol.get(1) - 1.0).abs() < 1.0e-8);
    }

    #[test]
    fn test_lm_quadratic() {
        // Fit y = a*x^2 to data: (0, 0), (1, 1), (2, 4), (3, 9)
        // a should be 1.0
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0)];
        let start = Vector { data: vec![0.5] };
        let config = LMConfig::default();

        let result = levenberg_marquardt(
            &mut |x: &Vector| -> (Option<Vector>, Option<Matrix>) {
                let a = x.get(0);
                let mut f = Vector::new(data.len());
                let mut j = Matrix::new(data.len(), 1);

                for (idx, &(xi, yi)) in data.iter().enumerate() {
                    let residual = a * xi * xi - yi;
                    f.set(idx, residual);
                    j.set(idx, 0, xi * xi);
                }

                (Some(f), Some(j))
            },
            &start,
            &config,
        );

        assert_eq!(result.status, Status::OK);
        let sol = result.solution.unwrap();
        assert!((sol.get(0) - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_lm_bounded() {
        // Solve system with bounds: 0 <= x,y <= 10
        let start = Vector {
            data: vec![0.0, 0.0],
        };
        let inf_bound = Vector {
            data: vec![0.0, 0.0],
        };
        let sup_bound = Vector {
            data: vec![10.0, 10.0],
        };

        let config = LMConfig::default();

        let result = levenberg_marquardt_bounded(
            &mut |x: &Vector| -> (Option<Vector>, Option<Matrix>) {
                let x0 = x.get(0);
                let x1 = x.get(1);

                let mut f = Vector::new(2);
                f.set(0, 2.0 * x0 - x1 - 1.0);
                f.set(1, x0 + x1 - 2.0);

                let mut j = Matrix::new(2, 2);
                j.set(0, 0, 2.0);
                j.set(0, 1, -1.0);
                j.set(1, 0, 1.0);
                j.set(1, 1, 1.0);

                (Some(f), Some(j))
            },
            &start,
            &inf_bound,
            &sup_bound,
            &config,
        );

        assert_eq!(result.status, Status::OK);
        let sol = result.solution.unwrap();
        assert!(sol.get(0) >= 0.0 && sol.get(0) <= 10.0);
        assert!(sol.get(1) >= 0.0 && sol.get(1) <= 10.0);
        assert!((sol.get(0) - 1.0).abs() < 1.0e-6);
        assert!((sol.get(1) - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_lm_max_iterations() {
        let start = Vector {
            data: vec![0.0, 0.0],
        };
        let config = LMConfig {
            max_iterations: 1,
            ..Default::default()
        };

        let result = levenberg_marquardt(
            &mut |x: &Vector| -> (Option<Vector>, Option<Matrix>) {
                let x0 = x.get(0);
                let x1 = x.get(1);

                let mut f = Vector::new(2);
                f.set(0, 2.0 * x0 - x1 - 1.0);
                f.set(1, x0 + x1 - 2.0);

                let mut j = Matrix::new(2, 2);
                j.set(0, 0, 2.0);
                j.set(0, 1, -1.0);
                j.set(1, 0, 1.0);
                j.set(1, 1, 1.0);

                (Some(f), Some(j))
            },
            &start,
            &config,
        );

        assert_eq!(result.status, Status::MaxIterations);
    }
}

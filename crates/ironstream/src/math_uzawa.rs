// FILE: math_uzawa.rs
// occt: math_Uzawa

//! Uzawa algorithm solver for systems C*X = B with constraints.
//! Supports equality constraints and inequality constraints (<).
//! Minimizes Norm(X-X0).

use std::cmp::Ordering;

/// Simple matrix wrapper for this context
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /// Create a matrix with given dimensions (1-indexed compatibility)
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    /// Get element at (row, col) (1-indexed)
    pub fn get(&self, row: usize, col: usize) -> f64 {
        let r = row - 1;
        let c = col - 1;
        self.data[r * self.cols + c]
    }

    /// Set element at (row, col) (1-indexed)
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        let r = row - 1;
        let c = col - 1;
        self.data[r * self.cols + c] = val;
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Matrix {
            data: self.data.clone(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

/// Simple vector wrapper for this context
pub struct Vector {
    data: Vec<f64>,
}

impl Vector {
    /// Create a vector with given length (1-indexed compatibility)
    pub fn new(len: usize) -> Self {
        Vector {
            data: vec![0.0; len],
        }
    }

    /// Get element at index (1-indexed)
    pub fn get(&self, idx: usize) -> f64 {
        self.data[idx - 1]
    }

    /// Set element at index (1-indexed)
    pub fn set(&mut self, idx: usize, val: f64) {
        self.data[idx - 1] = val;
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Clone for Vector {
    fn clone(&self) -> Self {
        Vector {
            data: self.data.clone(),
        }
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        assert_eq!(self.len(), other.len());
        let mut result = Vector::new(self.len());
        for i in 0..self.len() {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }
}

impl std::ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        assert_eq!(self.len(), other.len());
        let mut result = Vector::new(self.len());
        for i in 0..self.len() {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }
}

impl std::ops::Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        assert_eq!(self.len(), other.len());
        let mut result = Vector::new(self.len());
        for i in 0..self.len() {
            result.data[i] = self.data[i] - other.data[i];
        }
        result
    }
}

/// Crout algorithm for symmetric matrix inversion
struct MathCrout {
    inv_a: Matrix,
    done: bool,
}

impl MathCrout {
    fn new(a: &Matrix) -> Self {
        let nctl = a.rows();
        assert_eq!(nctl, a.cols(), "Matrix must be square");

        let mut inv_a = Matrix::new(nctl, nctl);
        let mut done = true;

        let min_pivot = 1.0e-20;
        let mut l = Matrix::new(nctl, nctl);
        let mut diag = Vector::new(nctl);

        // Crout decomposition: A = L * D * T(L)
        for i in 1..=nctl {
            for j in 1..=i - 1 {
                let mut scale = 0.0;
                for k in 1..=j - 1 {
                    scale += l.get(i, k) * l.get(j, k) * diag.get(k);
                }
                l.set(i, j, (a.get(i, j) - scale) / diag.get(j));
            }

            let mut scale = 0.0;
            for k in 1..=i - 1 {
                scale += l.get(i, k) * l.get(i, k) * diag.get(k);
            }
            let diag_val = a.get(i, i) - scale;
            diag.set(i, diag_val);

            if diag_val.abs() <= min_pivot {
                done = false;
                return MathCrout { inv_a, done };
            }
            l.set(i, i, 1.0);
        }

        // Compute inverse of L
        l.set(1, 1, 1.0 / l.get(1, 1));
        for i in 2..=nctl {
            for k in 1..=i - 1 {
                let mut scale = 0.0;
                for j in k..=i - 1 {
                    scale += l.get(i, j) * l.get(j, k);
                }
                l.set(i, k, -scale / l.get(i, i));
            }
            l.set(i, i, 1.0 / l.get(i, i));
        }

        // Compute inverse of A
        for j in 1..=nctl {
            let mut scale = l.get(j, j) * l.get(j, j) / diag.get(j);
            for k in (j + 1)..=nctl {
                scale += l.get(k, j) * l.get(k, j) / diag.get(k);
            }
            inv_a.set(j, j, scale);

            for i in (j + 1)..=nctl {
                let mut scale = l.get(i, j) * l.get(i, i) / diag.get(i);
                for k in (i + 1)..=nctl {
                    scale += l.get(k, j) * l.get(k, i) / diag.get(k);
                }
                inv_a.set(i, j, scale);
            }
        }

        MathCrout { inv_a, done }
    }

    fn inverse(&self) -> Matrix {
        self.inv_a.clone()
    }
}

/// occt: math_Uzawa
pub struct MathUzawa {
    result: Vector,
    error_uzawa: Vector,      // Erruza: error in Uzawa iteration
    error_initial: Vector,    // Errinit: initial error
    dual_variables: Vector,   // Vardua: dual variables
    inverse_cont: Matrix,     // CTCinv: inverse of C * transpose(C)
    nb_iterations: i32,
    done: bool,
}

impl MathUzawa {
    /// Solve C*X = B with only equality constraints, minimizing ||X - X0||
    pub fn new(
        cont: &Matrix,
        secont: &Vector,
        starting_point: &Vector,
    ) -> Self {
        Self::new_with_params(cont, secont, starting_point, 1.0e-6, 1.0e-6, 500)
    }

    /// Solve C*X = B with custom tolerances and iteration limit
    pub fn new_with_params(
        cont: &Matrix,
        secont: &Vector,
        starting_point: &Vector,
        eps_lix: f64,
        eps_lic: f64,
        nb_iterations: i32,
    ) -> Self {
        let nce = cont.rows();
        let nci = 0;
        Self::perform(cont, secont, starting_point, nce, nci, eps_lix, eps_lic, nb_iterations)
    }

    /// Solve C*X = B with mixed equality and inequality constraints
    /// First Nce equations are equality, remaining Nci are inequality (<)
    pub fn new_with_constraints(
        cont: &Matrix,
        secont: &Vector,
        starting_point: &Vector,
        nce: usize,
        nci: usize,
        eps_lix: f64,
        eps_lic: f64,
        nb_iterations: i32,
    ) -> Self {
        Self::perform(cont, secont, starting_point, nce, nci, eps_lix, eps_lic, nb_iterations)
    }

    fn perform(
        cont: &Matrix,
        secont: &Vector,
        starting_point: &Vector,
        nce: usize,
        nci: usize,
        eps_lix: f64,
        eps_lic: f64,
        max_iterations: i32,
    ) -> Self {
        let nlig = cont.rows();
        let ncol = cont.cols();

        assert_eq!(secont.len(), nlig, "Secont length must equal matrix row count");
        assert_eq!(nce + nci, nlig, "Nce + Nci must equal matrix row count");

        let mut result = Vector::new(ncol);
        let mut error_uzawa = Vector::new(ncol);
        let mut error_initial = Vector::new(nlig);
        let mut dual_variables = Vector::new(nlig);
        let mut inverse_cont = Matrix::new(nlig, nlig);
        let mut nb_iterations = 0i32;
        let mut done = false;

        // Calculate initial error: Cont*X0 - Secont
        for i in 1..=nlig {
            let mut err = cont.get(i, 1) * starting_point.get(1);
            for j in 2..=ncol {
                err += cont.get(i, j) * starting_point.get(j);
            }
            error_initial.set(i, err - secont.get(i));
        }

        if nci == 0 {
            // Direct resolution case (only equality constraints)
            nb_iterations = 1;

            // Calculate C * transpose(C)
            for i in 1..=nlig {
                for j in 1..=i {
                    let mut ctc_val = cont.get(i, 1) * cont.get(j, 1);
                    for k in 2..=ncol {
                        ctc_val += cont.get(i, k) * cont.get(j, k);
                    }
                    inverse_cont.set(i, j, ctc_val);
                }
            }

            // Invert using Crout
            let crout = MathCrout::new(&inverse_cont);
            inverse_cont = crout.inverse();

            // Calculate dual variables: Vardua = inv(CTC) * Errinit
            for i in 1..=nlig {
                let mut scale = inverse_cont.get(i, 1) * error_initial.get(1);
                for j in 2..=i {
                    scale += inverse_cont.get(i, j) * error_initial.get(j);
                }
                for j in (i + 1)..=nlig {
                    scale += inverse_cont.get(j, i) * error_initial.get(j);
                }
                dual_variables.set(i, scale);
            }

            // Calculate error: Erruza = -C^T * Vardua
            for i in 1..=ncol {
                let mut scale = -cont.get(1, i) * dual_variables.get(1);
                for j in 2..=nlig {
                    scale -= cont.get(j, i) * dual_variables.get(j);
                }
                error_uzawa.set(i, scale);
            }

            // Restitute calculated values
            for i in 0..ncol {
                result.data[i] = starting_point.data[i] + error_uzawa.data[i];
            }
            done = true;
        } else {
            // Uzawa iteration case (with inequality constraints)

            // Initialize dual variables
            for i in 1..=nlig {
                if i <= nce {
                    dual_variables.set(i, 0.0);
                } else {
                    dual_variables.set(i, 1.0);
                }
            }

            // Calculate Rho coefficient
            let coef = 1.0 / std::f64::consts::SQRT_2;
            let mut normat = 0.0;
            for i in 1..=nlig {
                let mut normli = cont.get(i, 1) * cont.get(i, 1);
                for j in 2..=ncol {
                    normli += cont.get(i, j) * cont.get(i, j);
                }
                normat += normli;
            }
            let rho = coef / normat;

            // Uzawa iteration loop
            let mut xmax = 0.0;
            let mut err_max = 0.0;

            for iter in 1..=max_iterations {
                nb_iterations = iter;

                // Calculate Erruza and Xmax
                for i in 1..=ncol {
                    let xian = error_uzawa.get(i);
                    let mut erruza_val = -cont.get(1, i) * dual_variables.get(1);
                    for j in 2..=nlig {
                        erruza_val -= cont.get(j, i) * dual_variables.get(j);
                    }
                    error_uzawa.set(i, erruza_val);

                    if iter > 1 {
                        let diff = (erruza_val - xian).abs();
                        if i == 1 {
                            xmax = diff;
                        } else {
                            xmax = xmax.max(diff);
                        }
                    }
                }

                // Calculate error and update dual variables
                for i in 1..=nlig {
                    let mut err = cont.get(i, 1) * error_uzawa.get(1) + error_initial.get(i);
                    for j in 2..=ncol {
                        err += cont.get(i, j) * error_uzawa.get(j);
                    }

                    let err1 = if i <= nce {
                        dual_variables.set(i, dual_variables.get(i) + rho * err);
                        (rho * err).abs()
                    } else {
                        let xmuian = dual_variables.get(i);
                        let new_val = (dual_variables.get(i) + rho * err).max(0.0);
                        dual_variables.set(i, new_val);
                        (new_val - xmuian).abs()
                    };

                    if i == 1 {
                        err_max = err1;
                    } else {
                        err_max = err_max.max(err1);
                    }
                }

                // Check convergence
                if iter > 1 {
                    if xmax <= eps_lix && err_max <= eps_lic {
                        for i in 0..ncol {
                            result.data[i] = starting_point.data[i] + error_uzawa.data[i];
                        }
                        done = true;
                        return MathUzawa {
                            result,
                            error_uzawa,
                            error_initial,
                            dual_variables,
                            inverse_cont,
                            nb_iterations,
                            done,
                        };
                    }
                }
            }

            done = false;
        }

        MathUzawa {
            result,
            error_uzawa,
            error_initial,
            dual_variables,
            inverse_cont,
            nb_iterations,
            done,
        }
    }

    /// Returns true if computations were successful
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the solution vector X
    pub fn value(&self) -> &Vector {
        assert!(self.done, "Cannot get value: solver did not converge");
        &self.result
    }

    /// Returns the initial error: C*X0 - B
    pub fn initial_error(&self) -> &Vector {
        assert!(self.done, "Cannot get initial error: solver did not converge");
        &self.error_initial
    }

    /// Returns the dual variables
    pub fn duale(&self, v: &mut Vector) {
        assert_eq!(v.len(), self.dual_variables.len());
        for i in 0..v.len() {
            v.data[i] = self.dual_variables.data[i];
        }
    }

    /// Returns X - X0 (solution minus starting point)
    pub fn error(&self) -> &Vector {
        assert!(self.done, "Cannot get error: solver did not converge");
        &self.error_uzawa
    }

    /// Returns the number of iterations performed
    pub fn nb_iterations(&self) -> i32 {
        assert!(self.done, "Cannot get iterations: solver did not converge");
        self.nb_iterations
    }

    /// Returns the inverse of (C * transpose(C))
    pub fn inverse_cont(&self) -> &Matrix {
        assert!(self.done, "Cannot get inverse: solver did not converge");
        &self.inverse_cont
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1.0e-6;

    #[test]
    fn test_simple_equality_constraints() {
        // 2x + y = 5
        // x + 2y = 4
        // Expected: x = 2, y = 1
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 2.0);
        c.set(1, 2, 1.0);
        c.set(2, 1, 1.0);
        c.set(2, 2, 2.0);

        let mut b = Vector::new(2);
        b.set(1, 5.0);
        b.set(2, 4.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 0.0);
        x0.set(2, 0.0);

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());
        let solution = solver.value();
        assert!((solution.get(1) - 2.0).abs() < TOLERANCE);
        assert!((solution.get(2) - 1.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_overdetermined_system() {
        // x + y = 3
        // x - y = 1
        // Expected: x = 2, y = 1
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 1.0);
        c.set(2, 1, 1.0);
        c.set(2, 2, -1.0);

        let mut b = Vector::new(2);
        b.set(1, 3.0);
        b.set(2, 1.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 0.0);
        x0.set(2, 0.0);

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());
        let solution = solver.value();
        assert!((solution.get(1) - 2.0).abs() < TOLERANCE);
        assert!((solution.get(2) - 1.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_with_inequality_constraints() {
        // x + y = 2
        let mut c = Matrix::new(1, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 1.0);

        let mut b = Vector::new(1);
        b.set(1, 2.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 1.0);
        x0.set(2, 1.0);

        let solver = MathUzawa::new_with_constraints(&c, &b, &x0, 1, 0, 1.0e-6, 1.0e-6, 500);

        assert!(solver.is_done());
        let solution = solver.value();
        assert!((solution.get(1) + solution.get(2) - 2.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_custom_tolerances() {
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 1.0);
        c.set(2, 1, 1.0);
        c.set(2, 2, -1.0);

        let mut b = Vector::new(2);
        b.set(1, 2.0);
        b.set(2, 0.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 0.0);
        x0.set(2, 0.0);

        let solver = MathUzawa::new_with_params(&c, &b, &x0, 1.0e-8, 1.0e-8, 500);

        assert!(solver.is_done());
        let solution = solver.value();
        assert!((solution.get(1) - 1.0).abs() < 1.0e-6);
        assert!((solution.get(2) - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_custom_iterations() {
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 1.0);
        c.set(2, 1, 1.0);
        c.set(2, 2, -1.0);

        let mut b = Vector::new(2);
        b.set(1, 2.0);
        b.set(2, 0.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 10.0);
        x0.set(2, 10.0);

        let solver = MathUzawa::new_with_params(&c, &b, &x0, 1.0e-6, 1.0e-6, 50);

        assert!(solver.is_done());
        assert!(solver.nb_iterations() <= 50);
    }

    #[test]
    fn test_initial_error() {
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 1.0);
        c.set(2, 1, 1.0);
        c.set(2, 2, -1.0);

        let mut b = Vector::new(2);
        b.set(1, 2.0);
        b.set(2, 0.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 5.0);
        x0.set(2, 3.0);

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());
        let initial_error = solver.initial_error();
        // C*[5,3] - [2,0] = [8,2] - [2,0] = [6,2]
        assert!((initial_error.get(1) - 6.0).abs() < TOLERANCE);
        assert!((initial_error.get(2) - 2.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_error_vector() {
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 1.0);
        c.set(2, 1, 1.0);
        c.set(2, 2, -1.0);

        let mut b = Vector::new(2);
        b.set(1, 2.0);
        b.set(2, 0.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 0.0);
        x0.set(2, 0.0);

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());
        let error = solver.error();
        let solution = solver.value();

        assert!((error.get(1) - (solution.get(1) - x0.get(1))).abs() < TOLERANCE);
        assert!((error.get(2) - (solution.get(2) - x0.get(2))).abs() < TOLERANCE);
    }

    #[test]
    fn test_dual_variables() {
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 1.0);
        c.set(2, 1, 1.0);
        c.set(2, 2, -1.0);

        let mut b = Vector::new(2);
        b.set(1, 2.0);
        b.set(2, 0.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 0.0);
        x0.set(2, 0.0);

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());

        let mut dual_var = Vector::new(2);
        solver.duale(&mut dual_var);

        assert!(dual_var.get(1).is_finite());
        assert!(dual_var.get(2).is_finite());
    }

    #[test]
    fn test_inverse_matrix() {
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 1.0);
        c.set(2, 1, 1.0);
        c.set(2, 2, -1.0);

        let mut b = Vector::new(2);
        b.set(1, 2.0);
        b.set(2, 0.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 0.0);
        x0.set(2, 0.0);

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());
        let inv_matrix = solver.inverse_cont();

        assert_eq!(inv_matrix.rows(), 2);
        assert_eq!(inv_matrix.cols(), 2);

        for i in 1..=2 {
            for j in 1..=2 {
                assert!(inv_matrix.get(i, j).is_finite());
            }
        }
    }

    #[test]
    fn test_large_system() {
        let n = 4;
        let mut c = Matrix::new(n, n);
        for i in 1..=n {
            for j in 1..=n {
                if i == j {
                    c.set(i, j, 2.0);
                } else {
                    c.set(i, j, 0.5);
                }
            }
        }

        let mut b = Vector::new(n);
        for i in 1..=n {
            b.set(i, i as f64);
        }

        let mut x0 = Vector::new(n);
        for i in 1..=n {
            x0.set(i, 0.0);
        }

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());

        let solution = solver.value();
        for i in 1..=n {
            assert!(solution.get(i).is_finite());
        }
    }

    #[test]
    fn test_starting_point_near_solution() {
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 1.0);
        c.set(2, 1, 1.0);
        c.set(2, 2, -1.0);

        let mut b = Vector::new(2);
        b.set(1, 2.0);
        b.set(2, 0.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 1.001);
        x0.set(2, 0.999);

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());
        assert!(solver.nb_iterations() <= 10);

        let solution = solver.value();
        assert!((solution.get(1) - 1.0).abs() < TOLERANCE);
        assert!((solution.get(2) - 1.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_consistent_system() {
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 2.0);
        c.set(2, 1, 2.0);
        c.set(2, 2, 1.0);

        let mut b = Vector::new(2);
        b.set(1, 5.0);
        b.set(2, 7.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 0.0);
        x0.set(2, 0.0);

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());

        let solution = solver.value();
        let eq1 = solution.get(1) + 2.0 * solution.get(2);
        let eq2 = 2.0 * solution.get(1) + solution.get(2);
        assert!((eq1 - 5.0).abs() < TOLERANCE);
        assert!((eq2 - 7.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_single_variable() {
        // 2x = 4
        let mut c = Matrix::new(1, 1);
        c.set(1, 1, 2.0);

        let mut b = Vector::new(1);
        b.set(1, 4.0);

        let mut x0 = Vector::new(1);
        x0.set(1, 0.0);

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());
        let solution = solver.value();
        assert!((solution.get(1) - 2.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_iteration_count() {
        let mut c = Matrix::new(2, 2);
        c.set(1, 1, 1.0);
        c.set(1, 2, 1.0);
        c.set(2, 1, 1.0);
        c.set(2, 2, -1.0);

        let mut b = Vector::new(2);
        b.set(1, 2.0);
        b.set(2, 0.0);

        let mut x0 = Vector::new(2);
        x0.set(1, 0.0);
        x0.set(2, 0.0);

        let solver = MathUzawa::new(&c, &b, &x0);

        assert!(solver.is_done());
        assert!(solver.nb_iterations() > 0);
        assert!(solver.nb_iterations() <= 500);
    }
}

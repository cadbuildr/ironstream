// FILE: math_opt_uzawa.rs

/// Uzawa constrained optimization solver.
/// occt: MathOpt_Uzawa
///
/// Solves constrained least squares: min ||X - X0||^2 subject to C*X = S
///
/// For equality constraints only, uses direct Crout decomposition.
/// For mixed equality/inequality constraints, uses iterative Uzawa method.
///
/// The Uzawa algorithm is a dual decomposition method that:
/// 1. Updates primal variables X to minimize Lagrangian
/// 2. Updates dual variables (Lagrange multipliers) for constraint violations

use core::f64;

/// Status code for optimization result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    OK,
    NotConverged,
    MaxIterations,
    NumericalError,
    InvalidInput,
    InfiniteSolutions,
    NoSolution,
    NotPositiveDefinite,
    Singular,
    NonDescentDirection,
}

/// Configuration for Uzawa algorithm
#[derive(Debug, Clone)]
pub struct UzawaConfig {
    pub eps_lix: f64,         // Tolerance for X convergence
    pub eps_lic: f64,         // Tolerance for dual variable convergence
    pub max_iterations: usize, // Maximum iterations
}

impl Default for UzawaConfig {
    fn default() -> Self {
        UzawaConfig {
            eps_lix: 1.0e-6,
            eps_lic: 1.0e-6,
            max_iterations: 500,
        }
    }
}

/// Result for Uzawa constrained optimization
#[derive(Debug, Clone)]
pub struct UzawaResult {
    pub status: Status,
    pub nb_iterations: usize,
    pub solution: Option<Vec<f64>>,
    pub dual: Option<Vec<f64>>,
    pub error: Option<Vec<f64>>,
    pub initial_error: Option<Vec<f64>>,
    pub inverse_ctc: Option<Vec<Vec<f64>>>,
}

impl UzawaResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

/// Constants for numerical computation
const THE_ZERO_TOL: f64 = 1.0e-15;

/// Check if value is effectively zero
#[inline]
fn is_zero(value: f64, tolerance: f64) -> bool {
    value.abs() < tolerance
}

/// Invert symmetric matrix using Crout decomposition
fn invert_crout(a: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    if n == 0 {
        return None;
    }

    // Check square
    if a.iter().any(|row| row.len() != n) {
        return None;
    }

    // Working matrices with 1-based-like indexing (0-based here)
    let mut l = vec![vec![0.0; n]; n];
    let mut diag = vec![0.0; n];

    let min_pivot = 1.0e-20;

    // Crout decomposition: A = L * D * L^T
    for i in 0..n {
        // Compute L(i,j) for j < i
        for j in 0..i {
            let mut scale = 0.0;
            for k in 0..j {
                scale += l[i][k] * l[j][k] * diag[k];
            }
            l[i][j] = (a[i][j] - scale) / diag[j];
        }

        // Compute D(i)
        let mut scale = 0.0;
        for k in 0..i {
            scale += l[i][k] * l[i][k] * diag[k];
        }
        diag[i] = a[i][i] - scale;

        // Check for singularity
        if diag[i].abs() <= min_pivot {
            return None;
        }

        l[i][i] = 1.0;
    }

    // Compute inverse of L
    l[0][0] = 1.0 / l[0][0];
    for i in 1..n {
        for k in 0..i {
            let mut scale = 0.0;
            for j in k..i {
                scale += l[i][j] * l[j][k];
            }
            l[i][k] = -scale / l[i][i];
        }
        l[i][i] = 1.0 / l[i][i];
    }

    // Compute inverse of A (lower triangle only, then fill symmetric)
    let mut inv = vec![vec![0.0; n]; n];
    for j in 0..n {
        let mut scale = l[j][j] * l[j][j] / diag[j];
        for k in (j + 1)..n {
            scale += l[k][j] * l[k][j] / diag[k];
        }
        inv[j][j] = scale;

        for i in (j + 1)..n {
            let mut scale = l[i][j] * l[i][i] / diag[i];
            for k in (i + 1)..n {
                scale += l[k][j] * l[k][i] / diag[k];
            }
            inv[i][j] = scale;
        }
    }

    // Mirror to upper triangle for symmetric matrix
    for i in 0..n {
        for j in (i + 1)..n {
            inv[i][j] = inv[j][i];
        }
    }

    Some(inv)
}

/// Uzawa constrained least squares solver
pub fn uzawa(
    constraint: &[Vec<f64>],
    rhs: &[f64],
    starting_point: &[f64],
    n_eq: usize,
    n_ineq: usize,
    config: Option<&UzawaConfig>,
) -> UzawaResult {
    let default_cfg = UzawaConfig::default();
    let cfg = config.unwrap_or(&default_cfg);
    let mut result = UzawaResult {
        status: Status::NotConverged,
        nb_iterations: 0,
        solution: None,
        dual: None,
        error: None,
        initial_error: None,
        inverse_ctc: None,
    };

    let n_lig = constraint.len();
    let n_col = if n_lig > 0 { constraint[0].len() } else { 0 };

    // Validate dimensions
    if rhs.len() != n_lig
        || (n_eq + n_ineq) != n_lig
        || starting_point.len() != n_col
        || n_col == 0
    {
        result.status = Status::InvalidInput;
        return result;
    }

    // Compute initial error: C*X0 - S
    let mut err_init = vec![0.0; n_lig];
    for i in 0..n_lig {
        let mut sum = 0.0;
        for j in 0..n_col {
            sum += constraint[i][j] * starting_point[j];
        }
        err_init[i] = sum - rhs[i];
    }

    result.initial_error = Some(err_init.clone());

    // Initialize dual variables and error
    let mut var_dua = vec![0.0; n_lig];
    let mut err_uza = vec![0.0; n_col];
    let mut ctc_inv = vec![vec![0.0; n_lig]; n_lig];

    // Check if equality constraints only
    if n_ineq == 0 {
        // Direct solution for equality constraints only
        result.nb_iterations = 1;

        // Compute C * C^T (symmetric)
        for i in 0..n_lig {
            for j in 0..=i {
                let mut sum = 0.0;
                for k in 0..n_col {
                    sum += constraint[i][k] * constraint[j][k];
                }
                ctc_inv[i][j] = sum;
                if i != j {
                    ctc_inv[j][i] = sum;
                }
            }
        }

        // Invert using Crout
        let inv = match invert_crout(&ctc_inv) {
            Some(v) => v,
            None => {
                result.status = Status::Singular;
                return result;
            }
        };
        ctc_inv = inv;

        // Compute dual variables: (C*C^T)^-1 * (C*X0 - S)
        for i in 0..n_lig {
            let mut sum = 0.0;
            for j in 0..n_lig {
                sum += ctc_inv[i][j] * err_init[j];
            }
            var_dua[i] = sum;
        }

        // Compute error: -C^T * dual
        for i in 0..n_col {
            let mut sum = 0.0;
            for j in 0..n_lig {
                sum -= constraint[j][i] * var_dua[j];
            }
            err_uza[i] = sum;
        }

        // Compute solution
        let mut resul = vec![0.0; n_col];
        for i in 0..n_col {
            resul[i] = starting_point[i] + err_uza[i];
        }

        result.solution = Some(resul);
        result.dual = Some(var_dua);
        result.error = Some(err_uza);
        result.inverse_ctc = Some(ctc_inv);
        result.status = Status::OK;
        return result;
    }

    // Iterative Uzawa for mixed equality/inequality constraints
    // Initialize dual variables
    for i in 0..n_lig {
        var_dua[i] = if i < n_eq { 0.0 } else { 1.0 };
    }

    // Compute step size rho
    let mut nor_mat = 0.0;
    for i in 0..n_lig {
        let mut norm_li = 0.0;
        for j in 0..n_col {
            let val = constraint[i][j];
            norm_li += val * val;
        }
        nor_mat += norm_li;
    }
    let rho = 1.0 / (2.0_f64.sqrt() * nor_mat);

    // Uzawa iterations
    for an_iter in 0..cfg.max_iterations {
        result.nb_iterations = an_iter + 1;

        let mut x_max: f64 = 0.0;

        // Update primal: X = X0 - C^T * dual
        for i in 0..n_col {
            let x_prev = err_uza[i];
            let mut sum = 0.0;
            for j in 0..n_lig {
                sum -= constraint[j][i] * var_dua[j];
            }
            err_uza[i] = sum;

            if an_iter > 0 {
                x_max = x_max.max((err_uza[i] - x_prev).abs());
            }
        }

        // Update dual variables and compute constraint error
        let mut err_max: f64 = 0.0;
        for i in 0..n_lig {
            // Constraint violation: C*X - S
            let mut err = err_init[i];
            for j in 0..n_col {
                err += constraint[i][j] * err_uza[j];
            }

            let err1;
            if i < n_eq {
                // Equality constraint: update freely
                var_dua[i] += rho * err;
                err1 = (rho * err).abs();
            } else {
                // Inequality constraint: project to non-negative
                let x_mu_prev = var_dua[i];
                var_dua[i] = (var_dua[i] + rho * err).max(0.0);
                err1 = (var_dua[i] - x_mu_prev).abs();
            }
            err_max = err_max.max(err1);
        }

        // Check convergence
        if an_iter > 0 {
            if x_max <= cfg.eps_lix {
                if err_max <= cfg.eps_lic {
                    // Converged
                    let mut resul = vec![0.0; n_col];
                    for i in 0..n_col {
                        resul[i] = starting_point[i] + err_uza[i];
                    }

                    result.solution = Some(resul);
                    result.dual = Some(var_dua);
                    result.error = Some(err_uza);
                    result.status = Status::OK;
                    return result;
                } else {
                    // Dual did not converge
                    result.status = Status::NotConverged;
                    return result;
                }
            }
        }
    }

    result.status = Status::MaxIterations;
    result
}

/// Uzawa constrained least squares solver with equality constraints only
pub fn uzawa_equality(
    constraint: &[Vec<f64>],
    rhs: &[f64],
    starting_point: &[f64],
    config: Option<&UzawaConfig>,
) -> UzawaResult {
    uzawa(
        constraint,
        rhs,
        starting_point,
        constraint.len(),
        0,
        config,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uzawa_equality() {
        // Solve: min ||X - X0||^2 subject to C*X = S
        // Let X0 = [1, 1], C = [1, 1], S = [2]
        // Minimum should be X = [1, 1] on the line x + y = 2

        let constraint = vec![vec![1.0, 1.0]];
        let rhs = vec![2.0];
        let starting_point = vec![1.0, 1.0];

        let result = uzawa_equality(&constraint, &rhs, &starting_point, None);

        assert_eq!(result.status, Status::OK);
        assert!(result.solution.is_some());

        let sol = result.solution.unwrap();
        // Check constraint satisfaction
        let c_x = sol[0] + sol[1];
        assert!((c_x - 2.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_uzawa_2x2_system() {
        // Solve: min ||X - X0||^2 subject to C*X = S
        // C = [[1, 0], [0, 1]], S = [2, 3]
        // X0 = [0, 0]
        // Minimum should be X = [2, 3]

        let constraint = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let rhs = vec![2.0, 3.0];
        let starting_point = vec![0.0, 0.0];

        let result = uzawa_equality(&constraint, &rhs, &starting_point, None);

        assert_eq!(result.status, Status::OK);
        assert!(result.solution.is_some());

        let sol = result.solution.unwrap();
        assert!((sol[0] - 2.0).abs() < 1.0e-6);
        assert!((sol[1] - 3.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_uzawa_underdetermined() {
        // Solve: min ||X - X0||^2 subject to C*X = S
        // C = [1, 1] (1 constraint, 2 unknowns)
        // S = [2]
        // X0 = [0, 0]
        // Minimum should be X = [1, 1] (closest point on line x + y = 2 to origin)

        let constraint = vec![vec![1.0, 1.0]];
        let rhs = vec![2.0];
        let starting_point = vec![0.0, 0.0];

        let result = uzawa_equality(&constraint, &rhs, &starting_point, None);

        assert_eq!(result.status, Status::OK);
        let sol = result.solution.unwrap();

        // Check constraint satisfaction
        let c_x = sol[0] + sol[1];
        assert!((c_x - 2.0).abs() < 1.0e-6, "Constraint not satisfied: {} != 2.0", c_x);
    }

    #[test]
    fn test_uzawa_invalid_dimensions() {
        let constraint = vec![vec![1.0, 1.0]];
        let rhs = vec![2.0, 3.0]; // Wrong size
        let starting_point = vec![0.0, 0.0];

        let result = uzawa_equality(&constraint, &rhs, &starting_point, None);
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_uzawa_singular() {
        // Create a singular matrix (zero constraint)
        let constraint = vec![vec![0.0, 0.0]];
        let rhs = vec![0.0];
        let starting_point = vec![0.0, 0.0];

        let result = uzawa_equality(&constraint, &rhs, &starting_point, None);
        assert_eq!(result.status, Status::Singular);
    }

    #[test]
    fn test_uzawa_3d_system() {
        // 3D system with 2 constraints
        // C = [[1, 0, 0], [0, 1, 1]]
        // S = [1, 3]
        // X0 = [0, 0, 0]
        // Constraints: x = 1, y + z = 3
        // Minimum should be X = [1, 1.5, 1.5]

        let constraint = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 1.0]];
        let rhs = vec![1.0, 3.0];
        let starting_point = vec![0.0, 0.0, 0.0];

        let result = uzawa_equality(&constraint, &rhs, &starting_point, None);

        assert_eq!(result.status, Status::OK);
        let sol = result.solution.unwrap();

        // Check constraints
        assert!((sol[0] - 1.0).abs() < 1.0e-6);
        assert!((sol[1] + sol[2] - 3.0).abs() < 1.0e-6);

        // Check that minimum is achieved at y = z = 1.5
        // by symmetry (starting from origin with constraint y + z = 3)
        assert!((sol[1] - 1.5).abs() < 1.0e-6 || (sol[2] - 1.5).abs() < 1.0e-6);
    }

    #[test]
    fn test_uzawa_with_starting_point() {
        // Solve with non-zero starting point
        // C = [1, 1], S = [2]
        // X0 = [2, 0] (violates constraint)

        let constraint = vec![vec![1.0, 1.0]];
        let rhs = vec![2.0];
        let starting_point = vec![2.0, 0.0];

        let result = uzawa_equality(&constraint, &rhs, &starting_point, None);

        assert_eq!(result.status, Status::OK);
        let sol = result.solution.unwrap();

        // Check constraint
        assert!((sol[0] + sol[1] - 2.0).abs() < 1.0e-6);
    }
}

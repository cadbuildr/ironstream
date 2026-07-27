// FILE: math_sys_newton4_d.rs
// occt: MathSys_Newton4D

// Status enum (shared with Newton3D)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok,
    NotConverged,
    MaxIterations,
    NumericalError,
    InvalidInput,
    Singular,
}

// Result structure for 4D Newton solver
// occt-note: NewtonResultN<4>
#[derive(Debug, Clone)]
pub struct NewtonResult4D {
    pub status: Status,
    pub x: [f64; 4],
    pub nb_iterations: usize,
    pub residual_norm: f64,
    pub step_norm: f64,
}

impl NewtonResult4D {
    pub fn new() -> Self {
        NewtonResult4D {
            status: Status::NotConverged,
            x: [0.0; 4],
            nb_iterations: 0,
            residual_norm: 0.0,
            step_norm: 0.0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for NewtonResult4D {
    fn default() -> Self {
        Self::new()
    }
}

// Bounds structure for 4D
// occt-note: NewtonBoundsN<4>
#[derive(Debug, Clone)]
pub struct NewtonBounds4D {
    pub min: [f64; 4],
    pub max: [f64; 4],
    pub has_bounds: bool,
}

impl NewtonBounds4D {
    pub fn new() -> Self {
        NewtonBounds4D {
            min: [0.0; 4],
            max: [0.0; 4],
            has_bounds: true,
        }
    }

    pub fn unbounded() -> Self {
        NewtonBounds4D {
            min: [0.0; 4],
            max: [0.0; 4],
            has_bounds: false,
        }
    }
}

impl Default for NewtonBounds4D {
    fn default() -> Self {
        Self::new()
    }
}

// Options structure for Newton solver (shared)
#[derive(Debug, Clone)]
pub struct NewtonOptions {
    pub max_iterations: usize,
    pub tolerance: f64,
    pub x_tolerance: f64,
    pub f_tolerance: f64,
    pub max_step_ratio: f64,
    pub enable_line_search: bool,
    pub allow_soft_bounds: bool,
    pub soft_bounds_extension: f64,
}

impl NewtonOptions {
    pub fn new() -> Self {
        NewtonOptions {
            max_iterations: 100,
            tolerance: 1.0e-10,
            x_tolerance: 1.0e-16,
            f_tolerance: 1.0e-10,
            max_step_ratio: 0.5,
            enable_line_search: true,
            allow_soft_bounds: false,
            soft_bounds_extension: 1.0e-4,
        }
    }
}

impl Default for NewtonOptions {
    fn default() -> Self {
        Self::new()
    }
}

// Helper: Check bounds validity
fn is_bounds_valid_4d(bounds: &NewtonBounds4D) -> bool {
    if !bounds.has_bounds {
        return true;
    }
    bounds.min[0] <= bounds.max[0]
        && bounds.min[1] <= bounds.max[1]
        && bounds.min[2] <= bounds.max[2]
        && bounds.min[3] <= bounds.max[3]
}

// Helper: Check options validity
fn is_options_valid_4d(options: &NewtonOptions) -> bool {
    options.f_tolerance > 0.0
        && options.x_tolerance > 0.0
        && options.max_iterations > 0
        && options.max_step_ratio > 0.0
        && options.soft_bounds_extension >= 0.0
}

// Helper: Get max domain size
fn max_domain_size_4d(bounds: &NewtonBounds4D) -> f64 {
    if !bounds.has_bounds {
        return 1.0;
    }

    let d0 = bounds.max[0] - bounds.min[0];
    let d1 = bounds.max[1] - bounds.min[1];
    let d2 = bounds.max[2] - bounds.min[2];
    let d3 = bounds.max[3] - bounds.min[3];
    1.0_f64.max(d0.max(d1.max(d2.max(d3))))
}

// Helper: Clamp solution to bounds
fn clamp_4d(x: &mut [f64; 4], bounds: &NewtonBounds4D, use_soft_bounds: bool, soft_ext_ratio: f64) {
    if !bounds.has_bounds {
        return;
    }

    for i in 0..4 {
        let ext = if use_soft_bounds {
            (bounds.max[i] - bounds.min[i]) * soft_ext_ratio
        } else {
            0.0
        };
        x[i] = x[i].max(bounds.min[i] - ext).min(bounds.max[i] + ext);
    }
}

// Solve 4x4 linear system using Gaussian elimination with partial pivoting
// occt-ref: Solve4x4
fn solve_4x4(j: &[[f64; 4]; 4], f: &[f64; 4], delta: &mut [f64; 4]) -> bool {
    // Augmented matrix [J | -F]
    let mut a = [[0.0; 5]; 4];
    for i in 0..4 {
        for j_col in 0..4 {
            a[i][j_col] = j[i][j_col];
        }
        a[i][4] = -f[i];
    }

    // Forward elimination with partial pivoting
    for k in 0..4 {
        // Find pivot
        let mut max_row = k;
        let mut max_val = a[k][k].abs();
        for i in (k + 1)..4 {
            let val = a[i][k].abs();
            if val > max_val {
                max_val = val;
                max_row = i;
            }
        }

        // Check for singularity
        if max_val < 1.0e-30 {
            return false;
        }

        // Swap rows if needed
        if max_row != k {
            let temp = a[k];
            a[k] = a[max_row];
            a[max_row] = temp;
        }

        // Eliminate column
        let inv_pivot = 1.0 / a[k][k];
        for i in (k + 1)..4 {
            let factor = a[i][k] * inv_pivot;
            for j_col in (k + 1)..=4 {
                a[i][j_col] -= factor * a[k][j_col];
            }
            a[i][k] = 0.0;
        }
    }

    // Back substitution
    for i in (0..4).rev() {
        let mut sum = a[i][4];
        for j_col in (i + 1)..4 {
            sum -= a[i][j_col] * delta[j_col];
        }
        delta[i] = sum / a[i][i];
    }

    true
}

// Main solver function
// occt-ref: Solve4D
pub fn solve_4d<F: Fn(f64, f64, f64, f64, &mut [f64; 4], &mut [[f64; 4]; 4]) -> bool>(
    func: &F,
    x0: [f64; 4],
    bounds: &NewtonBounds4D,
    options: &NewtonOptions,
) -> NewtonResult4D {
    let mut result = NewtonResult4D::new();
    result.x = x0;

    if !is_options_valid_4d(options) || !is_bounds_valid_4d(bounds) {
        result.status = Status::InvalidInput;
        return result;
    }

    clamp_4d(&mut result.x, bounds, options.allow_soft_bounds, options.soft_bounds_extension);

    let tol_sq = options.f_tolerance * options.f_tolerance;
    let max_step = options.max_step_ratio * max_domain_size_4d(bounds);

    for iter in 0..options.max_iterations {
        result.nb_iterations = iter + 1;

        let mut f = [0.0; 4];
        let mut j = [[0.0; 4]; 4];
        if !func(result.x[0], result.x[1], result.x[2], result.x[3], &mut f, &mut j) {
            result.status = Status::NumericalError;
            return result;
        }

        // Check convergence using squared norm
        let f_norm_sq = f[0] * f[0] + f[1] * f[1] + f[2] * f[2] + f[3] * f[3];
        result.residual_norm = f_norm_sq.sqrt();
        if f_norm_sq <= tol_sq {
            result.status = Status::Ok;
            return result;
        }

        // Solve 4x4 linear system
        let mut delta = [0.0; 4];
        if !solve_4x4(&j, &f, &mut delta) {
            // Singular Jacobian - try steepest descent
            let mut grad = [0.0; 4];
            for c in 0..4 {
                for r in 0..4 {
                    grad[c] += j[r][c] * f[r];
                }
            }

            let grad_sq = grad[0] * grad[0] + grad[1] * grad[1] + grad[2] * grad[2] + grad[3] * grad[3];
            if grad_sq < 1.0e-60 {
                result.status = Status::Singular;
                return result;
            }

            let alpha = 1.0_f64.min((f_norm_sq / grad_sq).sqrt() * 0.1);
            for i in 0..4 {
                delta[i] = -alpha * grad[i];
            }
        }

        // Limit step size
        let step_norm_sq = delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2] + delta[3] * delta[3];
        let step_norm = step_norm_sq.sqrt();
        if step_norm > max_step {
            let scale = max_step / step_norm;
            for i in 0..4 {
                delta[i] *= scale;
            }
        }

        // Update and clamp
        let mut new_x = [
            result.x[0] + delta[0],
            result.x[1] + delta[1],
            result.x[2] + delta[2],
            result.x[3] + delta[3],
        ];
        clamp_4d(&mut new_x, bounds, options.allow_soft_bounds, options.soft_bounds_extension);

        result.step_norm = ((new_x[0] - result.x[0]).powi(2)
            + (new_x[1] - result.x[1]).powi(2)
            + (new_x[2] - result.x[2]).powi(2)
            + (new_x[3] - result.x[3]).powi(2))
            .sqrt();
        result.x = new_x;

        let scale_ref = 1.0_f64.max(
            result.x[0].abs()
                .max(result.x[1].abs().max(result.x[2].abs().max(result.x[3].abs())))
        );
        if result.step_norm <= options.x_tolerance * scale_ref {
            let mut check_f = [0.0; 4];
            let mut check_j = [[0.0; 4]; 4];
            if !func(result.x[0], result.x[1], result.x[2], result.x[3], &mut check_f, &mut check_j) {
                result.status = Status::NumericalError;
                return result;
            }

            result.residual_norm = (check_f[0] * check_f[0]
                + check_f[1] * check_f[1]
                + check_f[2] * check_f[2]
                + check_f[3] * check_f[3])
                .sqrt();
            result.status = if result.residual_norm <= options.f_tolerance {
                Status::Ok
            } else {
                Status::MaxIterations
            };
            return result;
        }
    }

    // Final convergence check
    let mut f = [0.0; 4];
    let mut j = [[0.0; 4]; 4];
    if !func(result.x[0], result.x[1], result.x[2], result.x[3], &mut f, &mut j) {
        result.status = Status::NumericalError;
        return result;
    }

    result.residual_norm = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2] + f[3] * f[3]).sqrt();
    result.status = if result.residual_norm <= options.f_tolerance {
        Status::Ok
    } else {
        Status::MaxIterations
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const THE_TOL: f64 = 1.0e-10;

    #[test]
    fn solve_4d_linear_system() {
        let func = |x1: f64, x2: f64, x3: f64, x4: f64, f: &mut [f64; 4], j: &mut [[f64; 4]; 4]| {
            f[0] = x1 - 1.0;
            f[1] = x2 - 2.0;
            f[2] = x3 - 3.0;
            f[3] = x4 - 4.0;

            j[0][0] = 1.0;
            j[0][1] = 0.0;
            j[0][2] = 0.0;
            j[0][3] = 0.0;

            j[1][0] = 0.0;
            j[1][1] = 1.0;
            j[1][2] = 0.0;
            j[1][3] = 0.0;

            j[2][0] = 0.0;
            j[2][1] = 0.0;
            j[2][2] = 1.0;
            j[2][3] = 0.0;

            j[3][0] = 0.0;
            j[3][1] = 0.0;
            j[3][2] = 0.0;
            j[3][3] = 1.0;
            true
        };

        let bounds = NewtonBounds4D::unbounded();
        let mut options = NewtonOptions::new();
        options.f_tolerance = THE_TOL;

        let result = solve_4d(&func, [0.0, 0.0, 0.0, 0.0], &bounds, &options);

        assert!(result.is_done());
        assert!((result.x[0] - 1.0).abs() < 1.0e-12);
        assert!((result.x[1] - 2.0).abs() < 1.0e-12);
        assert!((result.x[2] - 3.0).abs() < 1.0e-12);
        assert!((result.x[3] - 4.0).abs() < 1.0e-12);
    }

    #[test]
    fn solve_4d_bounded() {
        let func = |x1: f64, x2: f64, x3: f64, x4: f64, f: &mut [f64; 4], j: &mut [[f64; 4]; 4]| {
            f[0] = x1 - 1.0;
            f[1] = x2 - 2.0;
            f[2] = x3 - 3.0;
            f[3] = x4 - 4.0;

            for r in 0..4 {
                for c in 0..4 {
                    j[r][c] = if r == c { 1.0 } else { 0.0 };
                }
            }
            true
        };

        let mut bounds = NewtonBounds4D::new();
        bounds.min = [-10.0, -10.0, -10.0, -10.0];
        bounds.max = [10.0, 10.0, 10.0, 10.0];

        let mut options = NewtonOptions::new();
        options.f_tolerance = THE_TOL;

        let result = solve_4d(&func, [9.0, 8.0, 7.0, 6.0], &bounds, &options);

        assert!(result.is_done());
        assert!((result.x[0] - 1.0).abs() < 1.0e-12);
        assert!((result.x[1] - 2.0).abs() < 1.0e-12);
        assert!((result.x[2] - 3.0).abs() < 1.0e-12);
        assert!((result.x[3] - 4.0).abs() < 1.0e-12);
    }

    #[test]
    fn solve_4d_small_step_at_root_returns_ok() {
        let func = |x1: f64, x2: f64, x3: f64, x4: f64, f: &mut [f64; 4], j: &mut [[f64; 4]; 4]| {
            f[0] = x1 - 1.0;
            f[1] = x2 - 2.0;
            f[2] = x3 - 3.0;
            f[3] = x4 - 4.0;

            for r in 0..4 {
                for c in 0..4 {
                    j[r][c] = if r == c { 1.0 } else { 0.0 };
                }
            }
            true
        };

        let bounds = NewtonBounds4D::unbounded();
        let mut options = NewtonOptions::new();
        options.f_tolerance = 1.0e-12;
        options.x_tolerance = 100.0;
        options.max_iterations = 5;
        options.max_step_ratio = 100.0;

        let result = solve_4d(&func, [0.0, 0.0, 0.0, 0.0], &bounds, &options);
        assert!(result.is_done());
        assert_eq!(result.status, Status::Ok);
        assert!((result.x[0] - 1.0).abs() < 1.0e-14);
        assert!((result.x[1] - 2.0).abs() < 1.0e-14);
        assert!((result.x[2] - 3.0).abs() < 1.0e-14);
        assert!((result.x[3] - 4.0).abs() < 1.0e-14);
    }

    #[test]
    fn solve_4d_tiny_step_large_residual_returns_max_iterations() {
        let func = |_x1: f64, _x2: f64, _x3: f64, _x4: f64, f: &mut [f64; 4], j: &mut [[f64; 4]; 4]| {
            f[0] = 1.0;
            f[1] = 1.0;
            f[2] = 1.0;
            f[3] = 1.0;

            for r in 0..4 {
                for c in 0..4 {
                    j[r][c] = if r == c { 1.0e20 } else { 0.0 };
                }
            }
            true
        };

        let bounds = NewtonBounds4D::unbounded();
        let mut options = NewtonOptions::new();
        options.f_tolerance = 1.0e-8;
        options.x_tolerance = 1.0e-16;
        options.max_iterations = 10;

        let result = solve_4d(&func, [0.0, 0.0, 0.0, 0.0], &bounds, &options);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::MaxIterations);
        assert!(result.residual_norm > 1.0e-2);
    }

    #[test]
    fn solve_4d_invalid_input() {
        let func = |_x1: f64, _x2: f64, _x3: f64, _x4: f64, f: &mut [f64; 4], j: &mut [[f64; 4]; 4]| {
            for i in 0..4 {
                f[i] = 0.0;
                for j_col in 0..4 {
                    j[i][j_col] = if i == j_col { 1.0 } else { 0.0 };
                }
            }
            true
        };

        let mut bounds = NewtonBounds4D::new();
        bounds.min = [1.0, 0.0, 0.0, 0.0];
        bounds.max = [0.0, 1.0, 1.0, 1.0];

        let mut options = NewtonOptions::new();
        options.f_tolerance = THE_TOL;

        let result = solve_4d(&func, [0.0, 0.0, 0.0, 0.0], &bounds, &options);

        assert_eq!(result.status, Status::InvalidInput);
        assert!(!result.is_done());
    }
}

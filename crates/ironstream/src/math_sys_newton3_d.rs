// FILE: math_sys_newton3_d.rs
// occt: MathSys_Newton3D

use std::array;

// Status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok,
    NotConverged,
    MaxIterations,
    NumericalError,
    InvalidInput,
    Singular,
}

// Result structure for 3D Newton solver
// occt: NewtonResultN<3>
#[derive(Debug, Clone)]
pub struct NewtonResult3D {
    pub status: Status,
    pub x: [f64; 3],
    pub nb_iterations: usize,
    pub residual_norm: f64,
    pub step_norm: f64,
}

impl NewtonResult3D {
    pub fn new() -> Self {
        NewtonResult3D {
            status: Status::NotConverged,
            x: [0.0; 3],
            nb_iterations: 0,
            residual_norm: 0.0,
            step_norm: 0.0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for NewtonResult3D {
    fn default() -> Self {
        Self::new()
    }
}

// Bounds structure for 3D
// occt: NewtonBoundsN<3>
#[derive(Debug, Clone)]
pub struct NewtonBounds3D {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub has_bounds: bool,
}

impl NewtonBounds3D {
    pub fn new() -> Self {
        NewtonBounds3D {
            min: [0.0; 3],
            max: [0.0; 3],
            has_bounds: true,
        }
    }

    pub fn unbounded() -> Self {
        NewtonBounds3D {
            min: [0.0; 3],
            max: [0.0; 3],
            has_bounds: false,
        }
    }
}

impl Default for NewtonBounds3D {
    fn default() -> Self {
        Self::new()
    }
}

// Options structure for Newton solver
// occt: NewtonOptions
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
fn is_bounds_valid_3d(bounds: &NewtonBounds3D) -> bool {
    if !bounds.has_bounds {
        return true;
    }
    bounds.min[0] <= bounds.max[0]
        && bounds.min[1] <= bounds.max[1]
        && bounds.min[2] <= bounds.max[2]
}

// Helper: Check options validity
fn is_options_valid_3d(options: &NewtonOptions) -> bool {
    options.f_tolerance > 0.0
        && options.x_tolerance > 0.0
        && options.max_iterations > 0
        && options.max_step_ratio > 0.0
        && options.soft_bounds_extension >= 0.0
}

// Helper: Get max domain size
fn max_domain_size_3d(bounds: &NewtonBounds3D) -> f64 {
    if !bounds.has_bounds {
        return 1.0;
    }

    let dx = bounds.max[0] - bounds.min[0];
    let dy = bounds.max[1] - bounds.min[1];
    let dz = bounds.max[2] - bounds.min[2];
    1.0_f64.max(dx.max(dy.max(dz)))
}

// Helper: Clamp solution to bounds
fn clamp_3d(x: &mut [f64; 3], bounds: &NewtonBounds3D, use_soft_bounds: bool, soft_ext_ratio: f64) {
    if !bounds.has_bounds {
        return;
    }

    let ext_x = if use_soft_bounds {
        (bounds.max[0] - bounds.min[0]) * soft_ext_ratio
    } else {
        0.0
    };
    let ext_y = if use_soft_bounds {
        (bounds.max[1] - bounds.min[1]) * soft_ext_ratio
    } else {
        0.0
    };
    let ext_z = if use_soft_bounds {
        (bounds.max[2] - bounds.min[2]) * soft_ext_ratio
    } else {
        0.0
    };

    x[0] = x[0].max(bounds.min[0] - ext_x).min(bounds.max[0] + ext_x);
    x[1] = x[1].max(bounds.min[1] - ext_y).min(bounds.max[1] + ext_y);
    x[2] = x[2].max(bounds.min[2] - ext_z).min(bounds.max[2] + ext_z);
}

// Solve 3x3 linear system using Cramer's rule
// occt: Solve3x3
fn solve_3x3(j: &[[f64; 3]; 3], f: &[f64; 3], delta: &mut [f64; 3]) -> bool {
    // Compute cofactors for first row
    let cof00 = j[1][1] * j[2][2] - j[1][2] * j[2][1];
    let cof01 = j[1][2] * j[2][0] - j[1][0] * j[2][2];
    let cof02 = j[1][0] * j[2][1] - j[1][1] * j[2][0];

    // Determinant by first row expansion
    let det = j[0][0] * cof00 + j[0][1] * cof01 + j[0][2] * cof02;

    // Check for singularity
    if det.abs() < 1.0e-30 {
        return false;
    }

    let inv_det = 1.0 / det;

    // Remaining cofactors
    let cof10 = j[0][2] * j[2][1] - j[0][1] * j[2][2];
    let cof11 = j[0][0] * j[2][2] - j[0][2] * j[2][0];
    let cof12 = j[0][1] * j[2][0] - j[0][0] * j[2][1];

    let cof20 = j[0][1] * j[1][2] - j[0][2] * j[1][1];
    let cof21 = j[0][2] * j[1][0] - j[0][0] * j[1][2];
    let cof22 = j[0][0] * j[1][1] - j[0][1] * j[1][0];

    // Solve: delta = -J^(-1) * F = -adjugate(J)^T * F / det
    delta[0] = -(cof00 * f[0] + cof10 * f[1] + cof20 * f[2]) * inv_det;
    delta[1] = -(cof01 * f[0] + cof11 * f[1] + cof21 * f[2]) * inv_det;
    delta[2] = -(cof02 * f[0] + cof12 * f[1] + cof22 * f[2]) * inv_det;

    true
}

// Main solver function
// occt: Solve3D
pub fn solve_3d<F: Fn(f64, f64, f64, &mut [f64; 3], &mut [[f64; 3]; 3]) -> bool>(
    func: &F,
    x0: [f64; 3],
    bounds: &NewtonBounds3D,
    options: &NewtonOptions,
) -> NewtonResult3D {
    let mut result = NewtonResult3D::new();
    result.x = x0;

    if !is_options_valid_3d(options) || !is_bounds_valid_3d(bounds) {
        result.status = Status::InvalidInput;
        return result;
    }

    clamp_3d(&mut result.x, bounds, options.allow_soft_bounds, options.soft_bounds_extension);

    let tol_sq = options.f_tolerance * options.f_tolerance;
    let max_step = options.max_step_ratio * max_domain_size_3d(bounds);

    for iter in 0..options.max_iterations {
        result.nb_iterations = iter + 1;

        let mut f = [0.0; 3];
        let mut j = [[0.0; 3]; 3];
        if !func(result.x[0], result.x[1], result.x[2], &mut f, &mut j) {
            result.status = Status::NumericalError;
            return result;
        }

        // Check convergence using squared norm
        let f_norm_sq = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        result.residual_norm = f_norm_sq.sqrt();
        if f_norm_sq <= tol_sq {
            result.status = Status::Ok;
            return result;
        }

        // Solve 3x3 linear system
        let mut delta = [0.0; 3];
        if !solve_3x3(&j, &f, &mut delta) {
            // Singular Jacobian - try steepest descent
            let grad_x = j[0][0] * f[0] + j[1][0] * f[1] + j[2][0] * f[2];
            let grad_y = j[0][1] * f[0] + j[1][1] * f[1] + j[2][1] * f[2];
            let grad_z = j[0][2] * f[0] + j[1][2] * f[1] + j[2][2] * f[2];
            let grad_sq = grad_x * grad_x + grad_y * grad_y + grad_z * grad_z;

            if grad_sq < 1.0e-60 {
                result.status = Status::Singular;
                return result;
            }

            let alpha = 1.0_f64.min((f_norm_sq / grad_sq).sqrt() * 0.1);
            delta[0] = -alpha * grad_x;
            delta[1] = -alpha * grad_y;
            delta[2] = -alpha * grad_z;
        }

        // Limit step size
        let step_norm_sq = delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2];
        let step_norm = step_norm_sq.sqrt();
        if step_norm > max_step {
            let scale = max_step / step_norm;
            delta[0] *= scale;
            delta[1] *= scale;
            delta[2] *= scale;
        }

        // Update and clamp
        let mut new_x = [
            result.x[0] + delta[0],
            result.x[1] + delta[1],
            result.x[2] + delta[2],
        ];
        clamp_3d(&mut new_x, bounds, options.allow_soft_bounds, options.soft_bounds_extension);

        result.step_norm = ((new_x[0] - result.x[0]).powi(2)
            + (new_x[1] - result.x[1]).powi(2)
            + (new_x[2] - result.x[2]).powi(2))
            .sqrt();
        result.x = new_x;

        let scale_ref = 1.0_f64.max(
            result.x[0].abs().max(result.x[1].abs().max(result.x[2].abs()))
        );
        if result.step_norm <= options.x_tolerance * scale_ref {
            let mut check_f = [0.0; 3];
            let mut check_j = [[0.0; 3]; 3];
            if !func(result.x[0], result.x[1], result.x[2], &mut check_f, &mut check_j) {
                result.status = Status::NumericalError;
                return result;
            }

            result.residual_norm = (check_f[0] * check_f[0]
                + check_f[1] * check_f[1]
                + check_f[2] * check_f[2])
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
    let mut f = [0.0; 3];
    let mut j = [[0.0; 3]; 3];
    if !func(result.x[0], result.x[1], result.x[2], &mut f, &mut j) {
        result.status = Status::NumericalError;
        return result;
    }

    result.residual_norm = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt();
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
    fn solve_3d_linear_system() {
        let func = |x1: f64, x2: f64, x3: f64, f: &mut [f64; 3], j: &mut [[f64; 3]; 3]| {
            f[0] = 2.0 * x1 + x2 + x3 - 4.0;
            f[1] = x1 + 3.0 * x2 + x3 - 5.0;
            f[2] = x1 + x2 + 4.0 * x3 - 6.0;

            j[0][0] = 2.0;
            j[0][1] = 1.0;
            j[0][2] = 1.0;
            j[1][0] = 1.0;
            j[1][1] = 3.0;
            j[1][2] = 1.0;
            j[2][0] = 1.0;
            j[2][1] = 1.0;
            j[2][2] = 4.0;
            true
        };

        let bounds = NewtonBounds3D::unbounded();
        let mut options = NewtonOptions::new();
        options.f_tolerance = THE_TOL;

        let result = solve_3d(&func, [0.0, 0.0, 0.0], &bounds, &options);

        assert!(result.is_done());
        assert!((result.x[0] - 1.0).abs() < 1.0e-8);
        assert!((result.x[1] - 1.0).abs() < 1.0e-8);
        assert!((result.x[2] - 1.0).abs() < 1.0e-8);
    }

    #[test]
    fn solve_3d_nonlinear_system() {
        let func = |x1: f64, x2: f64, x3: f64, f: &mut [f64; 3], j: &mut [[f64; 3]; 3]| {
            f[0] = x1 * x1 + x2 * x2 + x3 * x3 - 3.0;
            f[1] = x1 + x2 + x3 - 3.0;
            f[2] = x1 - x2;

            j[0][0] = 2.0 * x1;
            j[0][1] = 2.0 * x2;
            j[0][2] = 2.0 * x3;
            j[1][0] = 1.0;
            j[1][1] = 1.0;
            j[1][2] = 1.0;
            j[2][0] = 1.0;
            j[2][1] = -1.0;
            j[2][2] = 0.0;
            true
        };

        let bounds = NewtonBounds3D::unbounded();
        let mut options = NewtonOptions::new();
        options.f_tolerance = 1.0e-10;
        options.max_iterations = 50;

        let result = solve_3d(&func, [0.8, 0.8, 1.4], &bounds, &options);

        assert!(result.is_done());
        assert!((result.x[0] - 1.0).abs() < 1.0e-5);
        assert!((result.x[1] - 1.0).abs() < 1.0e-5);
        assert!((result.x[2] - 1.0).abs() < 1.0e-5);
    }

    #[test]
    fn solve_3d_small_step_at_root_returns_ok() {
        let func = |x1: f64, x2: f64, x3: f64, f: &mut [f64; 3], j: &mut [[f64; 3]; 3]| {
            f[0] = x1 - 1.0;
            f[1] = x2 - 2.0;
            f[2] = x3 - 3.0;

            j[0][0] = 1.0;
            j[0][1] = 0.0;
            j[0][2] = 0.0;
            j[1][0] = 0.0;
            j[1][1] = 1.0;
            j[1][2] = 0.0;
            j[2][0] = 0.0;
            j[2][1] = 0.0;
            j[2][2] = 1.0;
            true
        };

        let bounds = NewtonBounds3D::unbounded();
        let mut options = NewtonOptions::new();
        options.f_tolerance = 1.0e-12;
        options.x_tolerance = 100.0;
        options.max_iterations = 5;
        options.max_step_ratio = 100.0;

        let result = solve_3d(&func, [0.0, 0.0, 0.0], &bounds, &options);
        assert!(result.is_done());
        assert_eq!(result.status, Status::Ok);
        assert!((result.x[0] - 1.0).abs() < 1.0e-14);
        assert!((result.x[1] - 2.0).abs() < 1.0e-14);
        assert!((result.x[2] - 3.0).abs() < 1.0e-14);
    }

    #[test]
    fn solve_3d_tiny_step_large_residual_returns_max_iterations() {
        let func = |_x1: f64, _x2: f64, _x3: f64, f: &mut [f64; 3], j: &mut [[f64; 3]; 3]| {
            f[0] = 1.0;
            f[1] = 1.0;
            f[2] = 1.0;
            j[0][0] = 1.0e20;
            j[0][1] = 0.0;
            j[0][2] = 0.0;
            j[1][0] = 0.0;
            j[1][1] = 1.0e20;
            j[1][2] = 0.0;
            j[2][0] = 0.0;
            j[2][1] = 0.0;
            j[2][2] = 1.0e20;
            true
        };

        let bounds = NewtonBounds3D::unbounded();
        let mut options = NewtonOptions::new();
        options.f_tolerance = 1.0e-8;
        options.x_tolerance = 1.0e-16;
        options.max_iterations = 10;

        let result = solve_3d(&func, [0.0, 0.0, 0.0], &bounds, &options);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::MaxIterations);
        assert!(result.residual_norm > 1.0e-2);
    }

    #[test]
    fn solve_3d_bounded() {
        let func = |x1: f64, x2: f64, x3: f64, f: &mut [f64; 3], j: &mut [[f64; 3]; 3]| {
            f[0] = 2.0 * x1 + x2 + x3 - 4.0;
            f[1] = x1 + 3.0 * x2 + x3 - 5.0;
            f[2] = x1 + x2 + 4.0 * x3 - 6.0;

            j[0][0] = 2.0;
            j[0][1] = 1.0;
            j[0][2] = 1.0;
            j[1][0] = 1.0;
            j[1][1] = 3.0;
            j[1][2] = 1.0;
            j[2][0] = 1.0;
            j[2][1] = 1.0;
            j[2][2] = 4.0;
            true
        };

        let mut bounds = NewtonBounds3D::new();
        bounds.min = [-10.0, -10.0, -10.0];
        bounds.max = [10.0, 10.0, 10.0];

        let mut options = NewtonOptions::new();
        options.f_tolerance = THE_TOL;

        let result = solve_3d(&func, [0.0, 0.0, 0.0], &bounds, &options);

        assert!(result.is_done());
        assert!((result.x[0] - 1.0).abs() < 1.0e-8);
        assert!((result.x[1] - 1.0).abs() < 1.0e-8);
        assert!((result.x[2] - 1.0).abs() < 1.0e-8);
    }

    #[test]
    fn solve_3d_invalid_input() {
        let func = |_x1: f64, _x2: f64, _x3: f64, f: &mut [f64; 3], j: &mut [[f64; 3]; 3]| {
            f[0] = 0.0;
            f[1] = 0.0;
            f[2] = 0.0;
            j[0][0] = 1.0;
            j[0][1] = 0.0;
            j[0][2] = 0.0;
            j[1][0] = 0.0;
            j[1][1] = 1.0;
            j[1][2] = 0.0;
            j[2][0] = 0.0;
            j[2][1] = 0.0;
            j[2][2] = 1.0;
            true
        };

        let mut bounds = NewtonBounds3D::new();
        bounds.min = [1.0, 0.0, 0.0];
        bounds.max = [0.0, 1.0, 1.0];

        let mut options = NewtonOptions::new();
        options.f_tolerance = THE_TOL;

        let result = solve_3d(&func, [0.0, 0.0, 0.0], &bounds, &options);

        assert_eq!(result.status, Status::InvalidInput);
        assert!(!result.is_done());
    }
}

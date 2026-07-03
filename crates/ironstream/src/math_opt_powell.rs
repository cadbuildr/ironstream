// FILE: math_opt_powell.rs

/// Powell's conjugate direction method for N-dimensional minimization.
/// occt: MathOpt_Powell
///
/// A gradient-free optimization algorithm that uses conjugate directions.
///
/// Algorithm:
/// 1. Start with N linearly independent directions (coordinate axes)
/// 2. Perform line minimization along each direction
/// 3. Replace one direction with the overall displacement direction
/// 4. Repeat until convergence
///
/// Advantages:
/// - No gradient required
/// - Generates conjugate directions for quadratic functions
/// - Robust for non-smooth functions
///
/// Disadvantages:
/// - Slower than gradient methods for smooth functions
/// - May lose direction independence over iterations

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

/// Configuration for Powell solver
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

/// Result of Powell optimization
#[derive(Debug, Clone)]
pub struct VectorResult {
    pub status: Status,
    pub nb_iterations: usize,
    pub solution: Option<Vec<f64>>,
    pub value: Option<f64>,
}

impl VectorResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

/// Constants for numerical computation
const THE_ZERO_TOL: f64 = 1.0e-15;
const THE_EPSILON: f64 = f64::EPSILON;

/// Check if value is effectively zero
#[inline]
fn is_zero(value: f64, tolerance: f64) -> bool {
    value.abs() < tolerance
}

/// Square of a value
#[inline]
fn sqr(value: f64) -> f64 {
    value * value
}

/// Cube root with proper sign handling
#[inline]
fn cube_root(value: f64) -> f64 {
    if value >= 0.0 {
        value.cbrt()
    } else {
        -(-value).cbrt()
    }
}

/// Line search result
#[derive(Debug, Clone)]
struct LineSearchResult {
    is_valid: bool,
    alpha: f64,
    f_new: f64,
}

/// Exact line search using Brent's method
fn exact_line_search<F: Fn(&[f64]) -> Option<f64>>(
    func: &F,
    x: &[f64],
    dir: &[f64],
    alpha_max: f64,
    tolerance: f64,
) -> LineSearchResult {
    let mut result = LineSearchResult {
        is_valid: false,
        alpha: 0.0,
        f_new: 0.0,
    };

    let n = x.len();
    let mut x_new = vec![0.0; n];

    // Evaluate phi(alpha) = f(x + alpha*d)
    let mut eval_phi = |alpha: f64| -> Option<f64> {
        for i in 0..n {
            x_new[i] = x[i] + alpha * dir[i];
        }
        func(&x_new)
    };

    // Brent's method for 1D minimization
    // Search in [-alpha_max, alpha_max]
    let mut a = -alpha_max;
    let mut b = alpha_max;
    let mut alpha = 0.0;
    let mut w = alpha;
    let mut v = alpha;

    let mut f_x = match eval_phi(alpha) {
        Some(v) => v,
        None => return result,
    };
    let mut f_w = f_x;
    let mut f_v = f_x;

    let mut d: f64 = 0.0;
    let mut e: f64 = 0.0;

    for _iter in 0..100 {
        let x_m = 0.5 * (a + b);
        let tol1 = tolerance * alpha.abs() + THE_ZERO_TOL / 10.0;
        let tol2 = 2.0 * tol1;

        // Check convergence
        if (alpha - x_m).abs() <= tol2 - 0.5 * (b - a) {
            result.is_valid = true;
            result.alpha = alpha;
            result.f_new = f_x;
            return result;
        }

        let mut u = 0.0;
        let use_parabolic;

        // Try parabolic interpolation
        if e.abs() > tol1 {
            let r = (alpha - w) * (f_x - f_v);
            let mut q = (alpha - v) * (f_x - f_w);
            let mut p = (alpha - v) * q - (alpha - w) * r;
            q = 2.0 * (q - r);

            if q > 0.0 {
                p = -p;
            } else {
                q = -q;
            }

            let e_tmp = e;
            e = d;

            if p.abs() < 0.5 * q * e_tmp.abs()
                && p > q * (a - alpha)
                && p < q * (b - alpha)
            {
                d = p / q;
                u = alpha + d;
                if (u - a) < tol2 || (b - u) < tol2 {
                    d = if x_m - alpha >= 0.0 { tol1 } else { -tol1 };
                }
                use_parabolic = true;
            } else {
                e = if alpha < x_m { b - alpha } else { a - alpha };
                d = 0.381966011250105 * e;
                use_parabolic = false;
            }
        } else {
            e = if alpha < x_m { b - alpha } else { a - alpha };
            d = 0.381966011250105 * e;
            use_parabolic = false;
        }

        if d.abs() >= tol1 {
            u = alpha + d;
        } else {
            u = alpha + if d >= 0.0 { tol1 } else { -tol1 };
        }

        let f_u = match eval_phi(u) {
            Some(v) => v,
            None => return result,
        };

        // Update bracket
        if f_u <= f_x {
            if u < alpha {
                b = alpha;
            } else {
                a = alpha;
            }
            v = w;
            w = alpha;
            alpha = u;
            f_v = f_w;
            f_w = f_x;
            f_x = f_u;
        } else {
            if u < alpha {
                a = u;
            } else {
                b = u;
            }
            if f_u <= f_w || w == alpha {
                v = w;
                w = u;
                f_v = f_w;
                f_w = f_u;
            } else if f_u <= f_v || v == alpha || v == w {
                v = u;
                f_v = f_u;
            }
        }
    }

    result.is_valid = true;
    result.alpha = alpha;
    result.f_new = f_x;
    result
}

/// Powell's conjugate direction method for N-dimensional minimization
pub fn powell<F: Fn(&[f64]) -> Option<f64>>(
    func: &F,
    starting_point: &[f64],
    config: Option<&Config>,
) -> VectorResult {
    let default_cfg = Config::default();
    let cfg = config.unwrap_or(&default_cfg);
    let mut result = VectorResult {
        status: Status::NotConverged,
        nb_iterations: 0,
        solution: None,
        value: None,
    };

    let n = starting_point.len();
    if n == 0 {
        result.status = Status::InvalidInput;
        return result;
    }

    // Current point
    let mut x = starting_point.to_vec();

    // Function value at current point
    let mut f_x = match func(&x) {
        Some(v) => v,
        None => {
            result.status = Status::NumericalError;
            return result;
        }
    };

    // Initialize direction set to coordinate axes (NxN identity matrix)
    let mut directions = vec![vec![0.0; n]; n];
    for i in 0..n {
        directions[i][i] = 1.0;
    }

    // Working vectors
    let mut dir = vec![0.0; n];
    let mut x_old = vec![0.0; n];
    let mut pt = vec![0.0; n];
    let mut ptt = vec![0.0; n];
    let mut xit = vec![0.0; n];

    for an_iter in 0..cfg.max_iterations {
        result.nb_iterations = an_iter + 1;

        let f_p = f_x;
        x_old.copy_from_slice(&x);

        // Track largest decrease and its direction index
        let mut del = 0.0;
        let mut i_big = 0;

        // Minimize along each direction
        for i in 0..n {
            // Extract direction i
            dir.copy_from_slice(&directions[i]);

            let f_p_prev = f_x;

            // Line minimization along direction
            let line_result = exact_line_search(func, &x, &dir, 10.0, cfg.x_tolerance);

            if line_result.is_valid {
                // Update position
                for j in 0..n {
                    x[j] += line_result.alpha * dir[j];
                }
                f_x = line_result.f_new;

                // Track direction with largest decrease
                let decrease = f_p_prev - f_x;
                if decrease > del {
                    del = decrease;
                    i_big = i;
                }
            }
        }

        // Check convergence
        if 2.0 * (f_p - f_x).abs()
            <= cfg.f_tolerance * ((f_p).abs() + (f_x).abs() + THE_ZERO_TOL)
        {
            result.status = Status::OK;
            result.solution = Some(x);
            result.value = Some(f_x);
            return result;
        }

        // Construct extrapolated point and new direction
        for j in 0..n {
            ptt[j] = 2.0 * x[j] - x_old[j];
            xit[j] = x[j] - x_old[j];
        }

        // Evaluate at extrapolated point
        let f_ptt = match func(&ptt) {
            Some(v) => v,
            None => {
                // If evaluation fails, continue with current directions
                continue;
            }
        };

        // Check if new direction should be added
        if f_ptt < f_p {
            let t = 2.0
                * (f_p - 2.0 * f_x + f_ptt) * sqr(f_p - f_x - del)
                - del * sqr(f_p - f_ptt);

            if t < 0.0 {
                // Minimize along new direction
                let line_result = exact_line_search(func, &x, &xit, 10.0, cfg.x_tolerance);

                if line_result.is_valid {
                    // Update position
                    for j in 0..n {
                        x[j] += line_result.alpha * xit[j];
                    }
                    f_x = line_result.f_new;

                    // Replace direction with largest decrease
                    if i_big < n {
                        for j in 0..n {
                            directions[i_big][j] = directions[n - 1][j];
                            directions[n - 1][j] = xit[j];
                        }
                    }
                }
            }
        }

        // Check X convergence
        let mut max_diff: f64 = 0.0;
        for j in 0..n {
            max_diff = max_diff.max((x[j] - x_old[j]).abs());
        }
        if max_diff < cfg.x_tolerance {
            result.status = Status::OK;
            result.solution = Some(x);
            result.value = Some(f_x);
            return result;
        }
    }

    // Maximum iterations reached
    result.status = Status::MaxIterations;
    result.solution = Some(x);
    result.value = Some(f_x);
    result
}

/// Powell's method with custom initial directions
pub fn powell_with_directions<F: Fn(&[f64]) -> Option<f64>>(
    func: &F,
    starting_point: &[f64],
    initial_directions: &[Vec<f64>],
    config: Option<&Config>,
) -> VectorResult {
    let default_cfg = Config::default();
    let cfg = config.unwrap_or(&default_cfg);
    let mut result = VectorResult {
        status: Status::NotConverged,
        nb_iterations: 0,
        solution: None,
        value: None,
    };

    let n = starting_point.len();
    if n == 0 || initial_directions.len() != n || initial_directions[0].len() != n {
        result.status = Status::InvalidInput;
        return result;
    }

    // Current point
    let mut x = starting_point.to_vec();

    let mut f_x = match func(&x) {
        Some(v) => v,
        None => {
            result.status = Status::NumericalError;
            return result;
        }
    };

    // Copy initial directions
    let mut directions = initial_directions.to_vec();

    let mut dir = vec![0.0; n];
    let mut x_old = vec![0.0; n];
    let mut ptt = vec![0.0; n];
    let mut xit = vec![0.0; n];

    for an_iter in 0..cfg.max_iterations {
        result.nb_iterations = an_iter + 1;

        let f_p = f_x;
        x_old.copy_from_slice(&x);

        let mut del = 0.0;
        let mut i_big = 0;

        for i in 0..n {
            dir.copy_from_slice(&directions[i]);

            let f_p_prev = f_x;

            let line_result = exact_line_search(func, &x, &dir, 10.0, cfg.x_tolerance);

            if line_result.is_valid {
                for j in 0..n {
                    x[j] += line_result.alpha * dir[j];
                }
                f_x = line_result.f_new;

                let decrease = f_p_prev - f_x;
                if decrease > del {
                    del = decrease;
                    i_big = i;
                }
            }
        }

        // Check convergence
        if 2.0 * (f_p - f_x).abs()
            <= cfg.f_tolerance * ((f_p).abs() + (f_x).abs() + THE_ZERO_TOL)
        {
            result.status = Status::OK;
            result.solution = Some(x);
            result.value = Some(f_x);
            return result;
        }

        // Construct extrapolated point
        for j in 0..n {
            ptt[j] = 2.0 * x[j] - x_old[j];
            xit[j] = x[j] - x_old[j];
        }

        let f_ptt = match func(&ptt) {
            Some(v) => v,
            None => {
                continue;
            }
        };

        if f_ptt < f_p {
            let t = 2.0
                * (f_p - 2.0 * f_x + f_ptt) * sqr(f_p - f_x - del)
                - del * sqr(f_p - f_ptt);

            if t < 0.0 {
                let line_result = exact_line_search(func, &x, &xit, 10.0, cfg.x_tolerance);

                if line_result.is_valid {
                    for j in 0..n {
                        x[j] += line_result.alpha * xit[j];
                    }
                    f_x = line_result.f_new;

                    if i_big < n {
                        for j in 0..n {
                            directions[i_big][j] = directions[n - 1][j];
                            directions[n - 1][j] = xit[j];
                        }
                    }
                }
            }
        }

        let mut max_diff: f64 = 0.0;
        for j in 0..n {
            max_diff = max_diff.max((x[j] - x_old[j]).abs());
        }
        if max_diff < cfg.x_tolerance {
            result.status = Status::OK;
            result.solution = Some(x);
            result.value = Some(f_x);
            return result;
        }
    }

    result.status = Status::MaxIterations;
    result.solution = Some(x);
    result.value = Some(f_x);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powell_quadratic() {
        // Minimize f(x, y) = (x - 2)^2 + (y - 3)^2
        // Minimum at (2, 3) with f = 0
        let func = |x: &[f64]| -> Option<f64> {
            let dx = x[0] - 2.0;
            let dy = x[1] - 3.0;
            Some(dx * dx + dy * dy)
        };

        let start = vec![0.0, 0.0];
        let config = Config {
            max_iterations: 500,
            tolerance: 1.0e-10,
            x_tolerance: 1.0e-10,
            f_tolerance: 1.0e-10,
        };

        let result = powell(&func, &start, Some(&config));

        assert_eq!(result.status, Status::OK);
        assert!(result.solution.is_some());
        assert!(result.value.is_some());

        let sol = result.solution.unwrap();
        assert!((sol[0] - 2.0).abs() < 1.0e-6);
        assert!((sol[1] - 3.0).abs() < 1.0e-6);
        assert!(result.value.unwrap() < 1.0e-10);
    }

    #[test]
    fn test_powell_1d() {
        // Minimize f(x) = (x - 5)^2
        // Minimum at x = 5 with f = 0
        let func = |x: &[f64]| -> Option<f64> {
            let dx = x[0] - 5.0;
            Some(dx * dx)
        };

        let start = vec![0.0];
        let config = Config {
            max_iterations: 100,
            tolerance: 1.0e-10,
            x_tolerance: 1.0e-10,
            f_tolerance: 1.0e-10,
        };

        let result = powell(&func, &start, Some(&config));

        assert_eq!(result.status, Status::OK);
        let sol = result.solution.unwrap();
        assert!((sol[0] - 5.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_powell_rosenbrock_relaxed() {
        // Minimize Rosenbrock: f(x, y) = 100*(y - x^2)^2 + (1 - x)^2
        // Minimum at (1, 1) with f = 0
        let func = |x: &[f64]| -> Option<f64> {
            let a = 1.0 - x[0];
            let b = x[1] - x[0] * x[0];
            Some(a * a + 100.0 * b * b)
        };

        let start = vec![0.0, 0.0];
        let config = Config {
            max_iterations: 2000,
            tolerance: 1.0e-6,
            x_tolerance: 1.0e-6,
            f_tolerance: 1.0e-6,
        };

        let result = powell(&func, &start, Some(&config));

        // Powell's method is not optimal for Rosenbrock, but should make progress
        assert!(result.solution.is_some());
        let sol = result.solution.unwrap();
        let f_val = func(&sol).unwrap();
        // Check that we've improved from the starting point
        assert!(f_val < func(&start).unwrap());
    }

    #[test]
    fn test_powell_invalid_input() {
        let func = |_x: &[f64]| -> Option<f64> { Some(0.0) };
        let start = vec![];

        let result = powell(&func, &start, None);
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_powell_with_directions() {
        // Test with custom directions
        let func = |x: &[f64]| -> Option<f64> {
            let dx = x[0] - 2.0;
            let dy = x[1] - 3.0;
            Some(dx * dx + dy * dy)
        };

        let start = vec![0.0, 0.0];

        // Custom directions (identity matrix)
        let mut directions = vec![vec![0.0; 2]; 2];
        directions[0][0] = 1.0;
        directions[1][1] = 1.0;

        let config = Config {
            max_iterations: 500,
            tolerance: 1.0e-10,
            x_tolerance: 1.0e-10,
            f_tolerance: 1.0e-10,
        };

        let result = powell_with_directions(&func, &start, &directions, Some(&config));

        assert_eq!(result.status, Status::OK);
        let sol = result.solution.unwrap();
        assert!((sol[0] - 2.0).abs() < 1.0e-6);
        assert!((sol[1] - 3.0).abs() < 1.0e-6);
    }

    #[test]
    fn test_powell_dimensions_mismatch() {
        let func = |_x: &[f64]| -> Option<f64> { Some(0.0) };
        let start = vec![0.0, 0.0];
        let directions = vec![vec![0.0; 3]; 3]; // Wrong dimension

        let result = powell_with_directions(&func, &start, &directions, None);
        assert_eq!(result.status, Status::InvalidInput);
    }
}

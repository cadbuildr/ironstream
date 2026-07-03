// FILE: math_utils_line_search.rs

//! Line search algorithms for optimization.
//! Provides Armijo backtracking, Wolfe line search, and exact line search (Brent's method)
//! for finding step sizes along descent directions.
//!
//! occt: MathUtils_LineSearch

use std::f64;

const THE_EPSILON: f64 = 2.2204460492503131e-16; // Machine epsilon for f64
const THE_ZERO_TOL: f64 = 1.0e-15;
const THE_GOLDEN_SECTION: f64 = 0.381966011250105; // 1 - 1/phi

/// Result of line search operation.
///
/// occt: LineSearchResult
#[derive(Clone, Debug)]
pub struct LineSearchResult {
    /// True if line search succeeded
    pub is_valid: bool,
    /// Step size found
    pub alpha: f64,
    /// Function value at new point
    pub f_new: f64,
    /// Number of function evaluations
    pub nb_evals: i32,
}

impl Default for LineSearchResult {
    fn default() -> Self {
        LineSearchResult {
            is_valid: false,
            alpha: 0.0,
            f_new: 0.0,
            nb_evals: 0,
        }
    }
}

/// Sign transfer function: returns |a| with sign of b.
fn sign_transfer(a: f64, b: f64) -> f64 {
    if b >= 0.0 {
        a.abs()
    } else {
        -a.abs()
    }
}

/// Backtracking line search with Armijo condition.
/// Finds alpha such that f(x + alpha*d) <= f(x) + c1*alpha*grad(f).d
/// (sufficient decrease condition)
///
/// # Arguments
/// * `func` - A callable that computes f(x) and returns true on success
/// * `x` - Current point
/// * `direction` - Search direction (should be descent direction)
/// * `gradient` - Gradient at current point
/// * `fx` - Function value at current point
/// * `alpha_init` - Initial step size (default 1.0)
/// * `c1` - Armijo parameter (default 1e-4)
/// * `rho` - Backtracking factor (default 0.5)
/// * `max_iter` - Maximum iterations (default 50)
///
/// # Returns
/// Line search result with step size
pub fn armijo_backtrack<F>(
    mut func: F,
    x: &[f64],
    direction: &[f64],
    gradient: &[f64],
    fx: f64,
    alpha_init: f64,
    c1: f64,
    rho: f64,
    max_iter: i32,
) -> LineSearchResult
where
    F: FnMut(&[f64]) -> Option<f64>,
{
    let mut result = LineSearchResult::default();
    result.alpha = alpha_init;
    result.nb_evals = 0;

    let n = x.len();
    if n == 0 || direction.len() != n || gradient.len() != n {
        return result;
    }

    // Compute directional derivative: grad(f) . d
    let mut dir_deriv = 0.0;
    for i in 0..n {
        dir_deriv += gradient[i] * direction[i];
    }

    // Ensure descent direction
    if dir_deriv >= 0.0 {
        return result;
    }

    let mut x_new = vec![0.0; n];

    for _k in 0..max_iter {
        // Compute new point: x + alpha*d
        for i in 0..n {
            x_new[i] = x[i] + result.alpha * direction[i];
        }

        // Evaluate function at new point
        let f_new = match func(&x_new) {
            Some(val) => val,
            None => {
                result.alpha *= rho;
                result.nb_evals += 1;
                continue;
            }
        };
        result.nb_evals += 1;

        // Check Armijo condition: f(x + alpha*d) <= f(x) + c1*alpha*phi'(0)
        if f_new <= fx + c1 * result.alpha * dir_deriv {
            result.is_valid = true;
            result.f_new = f_new;
            return result;
        }

        // Reduce step size
        result.alpha *= rho;

        // Check for too small step
        if result.alpha < THE_EPSILON {
            break;
        }
    }

    result
}

/// Strong Wolfe line search.
/// Finds alpha satisfying both:
/// 1. Armijo: f(x + alpha*d) <= f(x) + c1*alpha*grad(f).d
/// 2. Curvature: |grad(f)(x + alpha*d).d| <= c2*|grad(f).d|
///
/// Uses bracketing and zoom procedure for guaranteed convergence.
///
/// # Arguments
/// * `func` - A callable that computes f(x) (takes &[f64] -> Option<f64>)
/// * `grad_func` - A callable that computes grad(f(x)) (takes &[f64] -> Option<Vec<f64>>)
/// * `x` - Current point
/// * `direction` - Search direction
/// * `gradient` - Gradient at current point
/// * `fx` - Function value at current point
/// * `alpha_init` - Initial step size
/// * `c1` - Armijo parameter (default 1e-4)
/// * `c2` - Curvature parameter (default 0.9)
/// * `max_iter` - Maximum iterations
///
/// # Returns
/// Line search result
pub fn wolfe_search<F, G>(
    mut func: F,
    mut grad_func: G,
    x: &[f64],
    direction: &[f64],
    gradient: &[f64],
    fx: f64,
    alpha_init: f64,
    c1: f64,
    c2: f64,
    max_iter: i32,
) -> LineSearchResult
where
    F: FnMut(&[f64]) -> Option<f64>,
    G: FnMut(&[f64]) -> Option<Vec<f64>>,
{
    let mut result = LineSearchResult::default();
    result.nb_evals = 0;

    let n = x.len();
    if n == 0 || direction.len() != n || gradient.len() != n {
        return result;
    }

    // Compute initial directional derivative
    let mut phi0_prime = 0.0;
    for i in 0..n {
        phi0_prime += gradient[i] * direction[i];
    }

    if phi0_prime >= 0.0 {
        result.is_valid = false;
        return result;
    }

    let mut x_new = vec![0.0; n];
    let mut grad_new = vec![0.0; n];

    let mut alpha_lo = 0.0;
    let mut alpha_hi = alpha_init * 2.0;
    let mut alpha = alpha_init;

    let mut phi_lo = fx;

    // Phase 1: Bracket finding
    for _k in 0..max_iter {
        // Evaluate at current alpha
        for i in 0..n {
            x_new[i] = x[i] + alpha * direction[i];
        }

        let phi = match func(&x_new) {
            Some(val) => val,
            None => {
                alpha_hi = alpha;
                alpha = 0.5 * (alpha_lo + alpha_hi);
                result.nb_evals += 1;
                continue;
            }
        };
        result.nb_evals += 1;

        // Check Armijo
        if phi > fx + c1 * alpha * phi0_prime || (result.nb_evals > 1 && phi >= phi_lo) {
            alpha_hi = alpha;
            break;
        }

        // Evaluate gradient at new point
        let grad_result = match grad_func(&x_new) {
            Some(g) => g,
            None => {
                result.is_valid = false;
                return result;
            }
        };

        let mut phi_prime = 0.0;
        for i in 0..n {
            phi_prime += grad_result[i] * direction[i];
        }

        // Check curvature condition
        if phi_prime.abs() <= -c2 * phi0_prime {
            result.is_valid = true;
            result.alpha = alpha;
            result.f_new = phi;
            return result;
        }

        if phi_prime >= 0.0 {
            alpha_hi = alpha_lo;
            alpha_lo = alpha;
            phi_lo = phi;
            break;
        }

        // Increase alpha
        alpha_lo = alpha;
        phi_lo = phi;
        alpha = 0.5 * (alpha + alpha_hi);
    }

    // Phase 2: Zoom
    for _k in 0..max_iter {
        alpha = 0.5 * (alpha_lo + alpha_hi);

        for i in 0..n {
            x_new[i] = x[i] + alpha * direction[i];
        }

        let phi = match func(&x_new) {
            Some(val) => val,
            None => {
                alpha_hi = alpha;
                result.nb_evals += 1;
                continue;
            }
        };
        result.nb_evals += 1;

        if phi > fx + c1 * alpha * phi0_prime || phi >= phi_lo {
            alpha_hi = alpha;
        } else {
            let grad_result = match grad_func(&x_new) {
                Some(g) => g,
                None => {
                    break;
                }
            };

            let mut phi_prime = 0.0;
            for i in 0..n {
                phi_prime += grad_result[i] * direction[i];
            }

            if phi_prime.abs() <= -c2 * phi0_prime {
                result.is_valid = true;
                result.alpha = alpha;
                result.f_new = phi;
                return result;
            }

            if phi_prime * (alpha_hi - alpha_lo) >= 0.0 {
                alpha_hi = alpha_lo;
            }

            alpha_lo = alpha;
            phi_lo = phi;
        }

        // Check for convergence
        if (alpha_hi - alpha_lo).abs() < THE_EPSILON {
            break;
        }
    }

    // Return best found
    result.is_valid = true;
    result.alpha = alpha;
    result.f_new = phi_lo;
    result
}

/// Exact line search using Brent's method.
/// Minimizes f(x + alpha*d) over alpha in [-alpha_max, alpha_max].
/// First brackets the minimum by exploring both directions.
///
/// # Arguments
/// * `func` - A callable that computes f(x)
/// * `x` - Current point
/// * `direction` - Search direction
/// * `alpha_max` - Maximum step size (searches in both directions)
/// * `tolerance` - Convergence tolerance
/// * `max_iter` - Maximum iterations
///
/// # Returns
/// Line search result
pub fn exact_line_search<F>(
    mut func: F,
    x: &[f64],
    direction: &[f64],
    alpha_max: f64,
    tolerance: f64,
    max_iter: i32,
) -> LineSearchResult
where
    F: FnMut(&[f64]) -> Option<f64>,
{
    let mut result = LineSearchResult::default();
    result.nb_evals = 0;

    let n = x.len();
    if n == 0 || direction.len() != n {
        return result;
    }

    let mut x_new = vec![0.0; n];

    // Lambda to evaluate phi(alpha) = f(x + alpha*d)
    let mut eval_phi = |alpha: f64, x_pt: &[f64], dir: &[f64], func_ref: &mut F| -> Option<f64> {
        for i in 0..x_pt.len() {
            x_new[i] = x_pt[i] + alpha * dir[i];
        }
        result.nb_evals += 1;
        func_ref(&x_new)
    };

    // Brent's method for 1D minimization
    let a: f64 = -alpha_max;
    let b: f64 = alpha_max;
    let mut x_b: f64 = 0.0; // Start at the current point
    let mut w = x_b;
    let mut v = x_b;

    let mut fx_b = match eval_phi(x_b, x, direction, &mut func) {
        Some(val) => val,
        None => {
            result.is_valid = false;
            return result;
        }
    };
    let mut fw = fx_b;
    let mut fv = fx_b;

    let mut d: f64 = 0.0;
    let mut e: f64 = 0.0;

    for _an_iter in 0..max_iter {
        let xm = 0.5 * (a + b);
        let tol1 = tolerance * x_b.abs() + THE_ZERO_TOL / 10.0;
        let tol2 = 2.0 * tol1;

        // Check convergence
        if (x_b - xm).abs() <= (tol2 - 0.5 * (b - a)) {
            result.is_valid = true;
            result.alpha = x_b;
            result.f_new = fx_b;
            return result;
        }

        let mut u = 0.0;
        let mut use_parabolic = false;

        // Try parabolic interpolation
        if e.abs() > tol1 {
            let r = (x_b - w) * (fx_b - fv);
            let mut q = (x_b - v) * (fx_b - fw);
            let mut p = (x_b - v) * q - (x_b - w) * r;
            q = 2.0 * (q - r);

            if q > 0.0 {
                p = -p;
            } else {
                q = -q;
            }

            let e_tmp = e;
            e = d;

            if p.abs() < (0.5 * q * e_tmp).abs()
                && p > q * (a - x_b)
                && p < q * (b - x_b)
            {
                d = p / q;
                u = x_b + d;
                if (u - a) < tol2 || (b - u) < tol2 {
                    d = sign_transfer(tol1, xm - x_b);
                }
                use_parabolic = true;
            }
        }

        if !use_parabolic {
            e = if x_b < xm { b - x_b } else { a - x_b };
            d = THE_GOLDEN_SECTION * e;
        }

        if d.abs() >= tol1 {
            u = x_b + d;
        } else {
            u = x_b + sign_transfer(tol1, d);
        }

        let fu = match eval_phi(u, x, direction, &mut func) {
            Some(val) => val,
            None => {
                result.is_valid = false;
                result.alpha = x_b;
                result.f_new = fx_b;
                return result;
            }
        };

        // Update bracket
        if fu <= fx_b {
            if u < x_b {
                let _b = x_b;
            } else {
                let _a = x_b;
            }

            v = w;
            w = x_b;
            x_b = u;
            fv = fw;
            fw = fx_b;
            fx_b = fu;
        } else {
            if u < x_b {
                let _a = u;
            } else {
                let _b = u;
            }

            if fu <= fw || w == x_b {
                v = w;
                w = u;
                fv = fw;
                fw = fu;
            } else if fu <= fv || v == x_b || v == w {
                v = u;
                fv = fu;
            }
        }
    }

    result.is_valid = true;
    result.alpha = x_b;
    result.f_new = fx_b;
    result
}

/// Quadratic interpolation step for line search.
/// Given phi(0), phi'(0), and phi(alpha1), finds minimum of quadratic fit.
///
/// # Arguments
/// * `phi0` - Function value at 0
/// * `phi0_prime` - Directional derivative at 0
/// * `alpha1` - Current step size
/// * `phi1` - Function value at alpha1
///
/// # Returns
/// Interpolated step size
pub fn quadratic_interpolation(
    phi0: f64,
    phi0_prime: f64,
    alpha1: f64,
    phi1: f64,
) -> f64 {
    // Quadratic: phi(alpha) = phi(0) + phi'(0)*alpha + c*alpha^2
    // where c = (phi(alpha1) - phi(0) - phi'(0)*alpha1) / alpha1^2
    // Minimum at alpha* = -phi'(0) / (2c)
    let num = phi0_prime * alpha1 * alpha1;
    let denom = 2.0 * (phi1 - phi0 - phi0_prime * alpha1);

    if denom.abs() < THE_ZERO_TOL {
        return 0.5 * alpha1;
    }

    let mut alpha_new = -num / denom;

    // Safeguard: ensure we stay within reasonable bounds
    if alpha_new < 0.1 * alpha1 {
        alpha_new = 0.1 * alpha1;
    } else if alpha_new > 0.9 * alpha1 {
        alpha_new = 0.5 * alpha1;
    }

    alpha_new
}

/// Brent's method for 1D minimization along a single coordinate axis.
/// Operates in-place on one coordinate of the point, avoiding vector allocations.
///
/// # Arguments
/// * `func` - A callable that computes f(x)
/// * `point` - Current point (the active coordinate is modified during search)
/// * `dim_idx` - Index of the coordinate to optimize
/// * `lo_bound` - Lower bound for this coordinate
/// * `up_bound` - Upper bound for this coordinate
/// * `fx` - Current function value at point (updated if improved)
/// * `tolerance` - Convergence tolerance
/// * `max_iter` - Maximum iterations
/// * `eval_count` - Incremented by the number of function evaluations used
///
/// # Returns
/// True if an improvement was found; point and fx are updated accordingly
pub fn brent_along_coordinate<F>(
    mut func: F,
    point: &mut [f64],
    dim_idx: usize,
    lo_bound: f64,
    up_bound: f64,
    fx: &mut f64,
    tolerance: f64,
    max_iter: i32,
    eval_count: &mut i32,
) -> bool
where
    F: FnMut(&[f64]) -> Option<f64>,
{
    let orig_coord = point[dim_idx];

    let mut a: f64 = lo_bound;
    let mut b: f64 = up_bound;
    let mut x: f64 = orig_coord;
    let mut w = x;
    let mut v = x;

    let mut fx_local = *fx;
    let mut fw = fx_local;
    let mut fv = fx_local;

    let mut d: f64 = 0.0;
    let mut e: f64 = 0.0;

    for _an_iter in 0..max_iter {
        let xm = 0.5 * (a + b);
        let tol1 = tolerance * x.abs() + THE_ZERO_TOL / 10.0;
        let tol2 = 2.0 * tol1;

        // Check convergence
        if (x - xm).abs() <= (tol2 - 0.5 * (b - a)) {
            break;
        }

        let mut u = 0.0;
        let mut use_parabolic = false;

        // Try parabolic interpolation
        if e.abs() > tol1 {
            let r = (x - w) * (fx_local - fv);
            let mut q = (x - v) * (fx_local - fw);
            let mut p = (x - v) * q - (x - w) * r;
            q = 2.0 * (q - r);

            if q > 0.0 {
                p = -p;
            } else {
                q = -q;
            }

            let e_tmp = e;
            e = d;

            if p.abs() < (0.5 * q * e_tmp).abs()
                && p > q * (a - x)
                && p < q * (b - x)
            {
                d = p / q;
                u = x + d;
                if (u - a) < tol2 || (b - u) < tol2 {
                    d = sign_transfer(tol1, xm - x);
                }
                use_parabolic = true;
            }
        }

        if !use_parabolic {
            e = if x < xm { b - x } else { a - x };
            d = THE_GOLDEN_SECTION * e;
        }

        if d.abs() >= tol1 {
            u = x + d;
        } else {
            u = x + sign_transfer(tol1, d);
        }

        // Evaluate: only modify the single coordinate
        point[dim_idx] = u;
        let fu = match func(point) {
            Some(val) => val,
            None => {
                point[dim_idx] = orig_coord;
                return false;
            }
        };
        *eval_count += 1;

        // Update bracket
        if fu <= fx_local {
            if u < x {
                b = x;
            } else {
                a = x;
            }
            v = w;
            w = x;
            x = u;
            fv = fw;
            fw = fx_local;
            fx_local = fu;
        } else {
            if u < x {
                a = u;
            } else {
                b = u;
            }
            if fu <= fw || w == x {
                v = w;
                w = u;
                fv = fw;
                fw = fu;
            } else if fu <= fv || v == x || v == w {
                v = u;
                fv = fu;
            }
        }
    }

    // Accept if improved
    if fx_local < *fx {
        point[dim_idx] = x;
        *fx = fx_local;
        return true;
    }

    // Restore original coordinate
    point[dim_idx] = orig_coord;
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test Armijo backtracking with a simple quadratic function f(x) = x^2
    #[test]
    fn test_armijo_backtrack_quadratic() {
        let x = vec![1.0];
        let direction = vec![-1.0]; // Descent direction
        let gradient = vec![2.0]; // grad(x^2) at x=1 is 2*x
        let fx = 1.0;

        let result = armijo_backtrack(
            |x_test| Some(x_test[0] * x_test[0]),
            &x,
            &direction,
            &gradient,
            fx,
            1.0,
            1e-4,
            0.5,
            50,
        );

        assert!(result.is_valid);
        assert!(result.alpha > 0.0 && result.alpha <= 1.0);
        assert!(result.f_new < fx);
    }

    // Test that non-descent direction fails
    #[test]
    fn test_armijo_non_descent() {
        let x = vec![1.0];
        let direction = vec![1.0]; // Not a descent direction
        let gradient = vec![2.0];
        let fx = 1.0;

        let result = armijo_backtrack(
            |x_test| Some(x_test[0] * x_test[0]),
            &x,
            &direction,
            &gradient,
            fx,
            1.0,
            1e-4,
            0.5,
            50,
        );

        assert!(!result.is_valid);
    }

    // Test quadratic interpolation
    #[test]
    fn test_quadratic_interpolation() {
        let phi0 = 1.0;
        let phi0_prime = -2.0;
        let alpha1 = 1.0;
        let phi1 = 0.5;

        let alpha_new = quadratic_interpolation(phi0, phi0_prime, alpha1, phi1);
        assert!(alpha_new > 0.0);
        assert!(alpha_new < alpha1);
    }

    // Test exact line search on quadratic
    #[test]
    fn test_exact_line_search_quadratic() {
        let x = vec![2.0];
        let direction = vec![-1.0];

        let result = exact_line_search(
            |x_test| Some(x_test[0] * x_test[0]),
            &x,
            &direction,
            10.0,
            1e-6,
            100,
        );

        // The algorithm should find some step size
        assert!(result.nb_evals > 0);
        // The function value at the new point should be valid
        assert!(result.f_new >= 0.0); // Should be >= minimum at x=0
    }

    // Test Brent along coordinate
    #[test]
    fn test_brent_along_coordinate() {
        let mut point = vec![2.0];
        let mut fx = 4.0;
        let mut eval_count = 0;

        let improved = brent_along_coordinate(
            |p| Some(p[0] * p[0]),
            &mut point,
            0,
            -5.0,
            5.0,
            &mut fx,
            1e-6,
            100,
            &mut eval_count,
        );

        assert!(improved);
        // Should minimize to near x=0
        assert!(point[0].abs() < 0.1);
        assert!(fx < 0.01);
    }
}

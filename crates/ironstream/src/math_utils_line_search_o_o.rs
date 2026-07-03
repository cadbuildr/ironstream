// FILE: math_utils_line_search_o_o.rs
// occt: MathUtils_LineSearch

//! Line search algorithms for optimization.
//! Backtracking and Wolfe line search implementations.

const EPSILON: f64 = 1e-15;

/// Result of line search operation.
#[derive(Clone, Debug)]
pub struct LineSearchResult {
    pub is_valid: bool,   // True if line search succeeded
    pub alpha: f64,       // Step size found
    pub f_new: f64,       // Function value at new point
    pub nb_evals: i32,    // Number of function evaluations
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

/// Backtracking line search with Armijo condition.
/// Finds alpha such that f(x + alpha*d) <= f(x) + c1*alpha*grad(f).d
pub fn armijo_backtrack<F>(
    func: &F,
    x: &[f64],
    dir: &[f64],
    grad: &[f64],
    fx: f64,
    alpha_init: f64,
    c1: f64,
    rho: f64,
    max_iter: i32,
) -> LineSearchResult
where
    F: Fn(&[f64]) -> Option<f64>,
{
    let mut result = LineSearchResult::default();
    result.alpha = alpha_init;

    // Compute directional derivative: grad(f) . d
    let mut dir_deriv = 0.0;
    for i in 0..x.len() {
        dir_deriv += grad[i] * dir[i];
    }

    // Ensure descent direction
    if dir_deriv >= 0.0 {
        result.is_valid = false;
        return result;
    }

    for _k in 0..max_iter {
        // Compute new point: x + alpha*d
        let mut x_new = vec![0.0; x.len()];
        for i in 0..x.len() {
            x_new[i] = x[i] + result.alpha * dir[i];
        }

        // Evaluate function at new point
        if let Some(f_new) = func(&x_new) {
            result.nb_evals += 1;

            // Check Armijo condition: f(x + alpha*d) <= f(x) + c1*alpha*phi'(0)
            if f_new <= fx + c1 * result.alpha * dir_deriv {
                result.is_valid = true;
                result.f_new = f_new;
                return result;
            }
        } else {
            result.nb_evals += 1;
        }

        // Reduce step size
        result.alpha *= rho;

        // Check for too small step
        if result.alpha < EPSILON {
            break;
        }
    }

    result.is_valid = false;
    result
}

/// Strong Wolfe line search.
/// Finds alpha satisfying both Armijo and curvature conditions.
pub fn wolfe_search<F, G>(
    func: &F,
    grad_func: &G,
    x: &[f64],
    dir: &[f64],
    grad: &[f64],
    fx: f64,
    alpha_init: f64,
    c1: f64,
    c2: f64,
    max_iter: i32,
) -> LineSearchResult
where
    F: Fn(&[f64]) -> Option<f64>,
    G: Fn(&[f64], &mut [f64]) -> bool,
{
    let mut result = LineSearchResult::default();

    // Compute initial directional derivative
    let mut phi0_prime = 0.0;
    for i in 0..x.len() {
        phi0_prime += grad[i] * dir[i];
    }

    if phi0_prime >= 0.0 {
        result.is_valid = false;
        return result;
    }

    let mut alpha_prev = 0.0;
    let mut alpha = alpha_init;
    let mut f_prev = fx;

    for _k in 0..max_iter {
        // Evaluate function at current alpha
        let mut x_new = vec![0.0; x.len()];
        for i in 0..x.len() {
            x_new[i] = x[i] + alpha * dir[i];
        }

        if let Some(f_curr) = func(&x_new) {
            result.nb_evals += 1;

            // Check Armijo condition
            if f_curr > fx + c1 * alpha * phi0_prime || (result.nb_evals > 1 && f_curr >= f_prev)
            {
                // Zoom phase: alpha is too large
                let _ = zoom_phase(
                    func, grad_func, x, dir, grad, fx, alpha_prev, alpha, phi0_prime, c1, c2,
                    &mut result,
                );
                return result;
            }

            // Compute gradient at new point
            let mut grad_new = vec![0.0; x.len()];
            if grad_func(&x_new, &mut grad_new) {
                // Compute phi'(alpha)
                let mut phi_alpha_prime = 0.0;
                for i in 0..x.len() {
                    phi_alpha_prime += grad_new[i] * dir[i];
                }

                // Check curvature condition
                if phi_alpha_prime.abs() <= -c2 * phi0_prime {
                    result.is_valid = true;
                    result.alpha = alpha;
                    result.f_new = f_curr;
                    return result;
                }

                // If phi'(alpha) >= 0, zoom to [alpha_prev, alpha]
                if phi_alpha_prime >= 0.0 {
                    let _ = zoom_phase(
                        func, grad_func, x, dir, grad, fx, alpha, alpha_prev, phi0_prime, c1, c2,
                        &mut result,
                    );
                    return result;
                }
            }

            alpha_prev = alpha;
            f_prev = f_curr;
            alpha *= 2.0; // Increase alpha
        } else {
            result.nb_evals += 1;
            break;
        }
    }

    result.is_valid = false;
    result
}

/// Zoom phase for Wolfe line search.
fn zoom_phase<F, G>(
    func: &F,
    grad_func: &G,
    x: &[f64],
    dir: &[f64],
    grad: &[f64],
    fx: f64,
    mut alpha_lo: f64,
    mut alpha_hi: f64,
    phi0_prime: f64,
    c1: f64,
    c2: f64,
    result: &mut LineSearchResult,
) -> bool
where
    F: Fn(&[f64]) -> Option<f64>,
    G: Fn(&[f64], &mut [f64]) -> bool,
{
    for _ in 0..10 {
        let alpha = (alpha_lo + alpha_hi) / 2.0;

        let mut x_new = vec![0.0; x.len()];
        for i in 0..x.len() {
            x_new[i] = x[i] + alpha * dir[i];
        }

        if let Some(f_curr) = func(&x_new) {
            result.nb_evals += 1;

            if f_curr > fx + c1 * alpha * phi0_prime {
                alpha_hi = alpha;
                continue;
            }

            let mut grad_new = vec![0.0; x.len()];
            if grad_func(&x_new, &mut grad_new) {
                let mut phi_alpha_prime = 0.0;
                for i in 0..x.len() {
                    phi_alpha_prime += grad_new[i] * dir[i];
                }

                if phi_alpha_prime.abs() <= -c2 * phi0_prime {
                    result.is_valid = true;
                    result.alpha = alpha;
                    result.f_new = f_curr;
                    return true;
                }

                if phi_alpha_prime * (alpha_hi - alpha_lo) >= 0.0 {
                    alpha_hi = alpha_lo;
                }
                alpha_lo = alpha;
            }
        } else {
            result.nb_evals += 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_armijo_backtrack() {
        // Simple quadratic: f(x) = x^2
        let func = |x: &[f64]| Some(x[0] * x[0]);
        let x = vec![2.0];
        let dir = vec![-1.0]; // descent direction
        let grad = vec![4.0]; // gradient at x=2
        let fx = 4.0;

        let result = armijo_backtrack(&func, &x, &dir, &grad, fx, 1.0, 1e-4, 0.5, 50);
        assert!(result.is_valid);
        assert!(result.alpha > 0.0);
        assert!(result.f_new < fx);
    }

    #[test]
    fn test_armijo_no_descent() {
        let func = |_x: &[f64]| Some(0.0);
        let x = vec![1.0];
        let dir = vec![1.0]; // NOT a descent direction
        let grad = vec![1.0]; // positive gradient
        let fx = 1.0;

        let result = armijo_backtrack(&func, &x, &dir, &grad, fx, 1.0, 1e-4, 0.5, 50);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_lineserarch_sphere() {
        let func = |x: &[f64]| Some(x[0] * x[0] + x[1] * x[1]);
        let grad_func = |x: &[f64], g: &mut [f64]| {
            g[0] = 2.0 * x[0];
            g[1] = 2.0 * x[1];
            true
        };

        let x = vec![3.0, 4.0];
        let mut grad = vec![0.0; 2];
        let _ = grad_func(&x, &mut grad);
        let fx = 25.0;

        let dir = vec![-grad[0], -grad[1]]; // negative gradient
        let result = wolfe_search(&func, &grad_func, &x, &dir, &grad, fx, 1.0, 1e-4, 0.9, 20);
        assert!(result.is_valid);
    }
}

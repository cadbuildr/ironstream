// FILE: math_utils_line_search_o.rs
// occt-note: MathUtils::LineSearchResult, MathUtils::ArmijoBacktrack, MathUtils::WolfeSearch, MathUtils::ExactLineSearch, MathUtils::QuadraticInterpolation, MathUtils::BrentAlongCoordinate

/// Result of line search operation.
#[derive(Debug, Clone)]
pub struct LineSearchResult {
    pub is_valid: bool,   // True if line search succeeded
    pub alpha: f64,       // Step size found
    pub f_new: f64,       // Function value at new point
    pub nb_evals: usize,  // Number of function evaluations
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

/// Quadratic interpolation for step size.
pub fn quadratic_interpolation(
    phi0: f64,
    phi0_prime: f64,
    alpha1: f64,
    phi1: f64,
) -> f64 {
    let num = phi0_prime * alpha1 * alpha1;
    let denom = 2.0 * (phi1 - phi0 - phi0_prime * alpha1);

    if denom.abs() < 1e-15 {
        return 0.5 * alpha1;
    }

    let mut alpha_new = -num / denom;

    if alpha_new < 0.1 * alpha1 {
        alpha_new = 0.1 * alpha1;
    } else if alpha_new > 0.9 * alpha1 {
        alpha_new = 0.5 * alpha1;
    }

    alpha_new
}

/// Backtracking line search with Armijo condition.
pub fn armijo_backtrack<F>(
    func: &F,
    x: &[f64],
    dir: &[f64],
    grad: &[f64],
    fx: f64,
    alpha_init: f64,
    c1: f64,
    rho: f64,
    max_iter: usize,
) -> LineSearchResult
where
    F: Fn(&[f64]) -> Option<f64>,
{
    let mut result = LineSearchResult::default();
    result.alpha = alpha_init;

    if x.len() != dir.len() || x.len() != grad.len() {
        return result;
    }

    // Compute directional derivative: grad . dir
    let mut dir_deriv = 0.0;
    for i in 0..grad.len() {
        dir_deriv += grad[i] * dir[i];
    }

    // Ensure descent direction
    if dir_deriv >= 0.0 {
        return result;
    }

    let mut x_new = x.to_vec();
    let n = x.len();

    for _ in 0..max_iter {
        for i in 0..n {
            x_new[i] = x[i] + result.alpha * dir[i];
        }

        let f_new = match func(&x_new) {
            Some(v) => v,
            None => {
                result.alpha *= rho;
                result.nb_evals += 1;
                continue;
            }
        };
        result.nb_evals += 1;

        // Check Armijo condition
        if f_new <= fx + c1 * result.alpha * dir_deriv {
            result.is_valid = true;
            result.f_new = f_new;
            return result;
        }

        result.alpha *= rho;

        if result.alpha < 1e-15 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic_interpolation() {
        let result = quadratic_interpolation(1.0, -1.0, 1.0, 2.0);
        assert!(result > 0.0 && result < 1.0);
    }

    #[test]
    fn test_armijo_simple() {
        let func = |x: &[f64]| -> Option<f64> { Some(x[0] * x[0]) };
        let x = vec![1.0];
        let dir = vec![-1.0];
        let grad = vec![2.0];
        let fx = 1.0;

        let result = armijo_backtrack(&func, &x, &dir, &grad, fx, 1.0, 1e-4, 0.5, 50);
        assert!(result.is_valid);
        assert!(result.alpha > 0.0);
    }

    #[test]
    fn test_line_search_result_default() {
        let result = LineSearchResult::default();
        assert!(!result.is_valid);
        assert_eq!(result.nb_evals, 0);
    }
}

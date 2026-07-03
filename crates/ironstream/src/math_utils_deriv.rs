// FILE: math_utils_deriv.rs
// occt: MathUtils_Deriv

//! Numerical differentiation utilities.
//! Provides templates for computing derivatives, gradients, Jacobians, and Hessians
//! using finite difference approximations.

/// Central difference derivative approximation for scalar functions.
/// f'(x) ~= (f(x+h) - f(x-h)) / (2h)
/// Accuracy: O(h^2)
///
/// # Arguments
/// * `func` - function providing `value(&mut self, x: f64) -> Option<f64>` method
/// * `x` - point at which to evaluate derivative
/// * `step` - step size (default 1e-8)
///
/// # Returns
/// Some(derivative) if successful, None otherwise
pub fn central_difference<F>(func: &mut F, x: f64, step: f64) -> Option<f64>
where
    F: FnMut(f64) -> Option<f64>,
{
    let f_plus = func(x + step)?;
    let f_minus = func(x - step)?;
    Some((f_plus - f_minus) / (2.0 * step))
}

/// Forward difference derivative (one-sided).
/// f'(x) ~= (f(x+h) - f(x)) / h
/// Accuracy: O(h)
/// Useful when central difference is not possible (e.g., at boundaries).
///
/// # Arguments
/// * `func` - function providing `value(&mut self, x: f64) -> Option<f64>` method
/// * `x` - point at which to evaluate derivative
/// * `fx` - function value at x (if already computed)
/// * `step` - step size (default 1e-8)
///
/// # Returns
/// Some(derivative) if successful, None otherwise
pub fn forward_difference<F>(func: &mut F, x: f64, fx: f64, step: f64) -> Option<f64>
where
    F: FnMut(f64) -> Option<f64>,
{
    let f_plus = func(x + step)?;
    Some((f_plus - fx) / step)
}

/// Numerical gradient using central differences for N-D functions.
/// df/dx[i] ~= (f(x + h[i]*e[i]) - f(x - h[i]*e[i])) / (2*h[i])
///
/// # Arguments
/// * `func` - function with signature `(&mut [f64]) -> Option<f64>`
/// * `x` - point at which to evaluate gradient (will be modified then restored)
/// * `grad` - output gradient vector (same dimension as x)
/// * `step` - step size (default 1e-8)
///
/// # Returns
/// true if successful, false otherwise
pub fn numerical_gradient<F>(
    func: &mut F,
    x: &mut [f64],
    grad: &mut [f64],
    step: f64,
) -> bool
where
    F: FnMut(&[f64]) -> Option<f64>,
{
    for i in 0..x.len() {
        let xi = x[i];

        // Forward perturbation
        x[i] = xi + step;
        let f_plus = match func(x) {
            Some(v) => v,
            None => {
                x[i] = xi;
                return false;
            }
        };

        // Backward perturbation
        x[i] = xi - step;
        let f_minus = match func(x) {
            Some(v) => v,
            None => {
                x[i] = xi;
                return false;
            }
        };

        // Restore original value
        x[i] = xi;

        // Central difference
        if i < grad.len() {
            grad[i] = (f_plus - f_minus) / (2.0 * step);
        }
    }

    true
}

/// Numerical gradient with adaptive step size.
/// Uses step proportional to |x[i]| for better conditioning.
///
/// # Arguments
/// * `func` - function with signature `(&mut [f64]) -> Option<f64>`
/// * `x` - point at which to evaluate gradient (will be modified then restored)
/// * `grad` - output gradient vector
/// * `rel_step` - relative step size (default 1e-8)
///
/// # Returns
/// true if successful, false otherwise
pub fn numerical_gradient_adaptive<F>(
    func: &mut F,
    x: &mut [f64],
    grad: &mut [f64],
    rel_step: f64,
) -> bool
where
    F: FnMut(&[f64]) -> Option<f64>,
{
    for i in 0..x.len() {
        let xi = x[i];
        // Adaptive step: larger for larger |x|, with minimum floor
        let step = rel_step * 1.0_f64.max(xi.abs());

        // Forward perturbation
        x[i] = xi + step;
        let f_plus = match func(x) {
            Some(v) => v,
            None => {
                x[i] = xi;
                return false;
            }
        };

        // Backward perturbation
        x[i] = xi - step;
        let f_minus = match func(x) {
            Some(v) => v,
            None => {
                x[i] = xi;
                return false;
            }
        };

        // Restore and store
        x[i] = xi;

        if i < grad.len() {
            grad[i] = (f_plus - f_minus) / (2.0 * step);
        }
    }

    true
}

/// Numerical Jacobian matrix for vector-valued functions.
/// J[i,j] = dF[i]/dx[j] ~= (F[i](x + h[j]*e[j]) - F[i](x - h[j]*e[j])) / (2*h[j])
///
/// # Arguments
/// * `func` - function with signature `(&[f64], &mut [f64]) -> bool`
///   that computes output vector from input vector
/// * `x` - input point (n-dimensional, will be modified then restored)
/// * `jac` - output Jacobian matrix (m rows x n columns)
///   where m = output dimension, n = input dimension
/// * `step` - step size (default 1e-8)
///
/// # Returns
/// true if successful, false otherwise
pub fn numerical_jacobian<F>(
    func: &mut F,
    x: &mut [f64],
    jac: &mut [Vec<f64>],
    step: f64,
) -> bool
where
    F: FnMut(&[f64], &mut Vec<f64>) -> bool,
{
    let n_cols = x.len();
    let n_rows = if jac.is_empty() { 0 } else { jac[0].len() };

    if jac.is_empty() || jac.iter().any(|row| row.len() != n_rows) {
        return false;
    }

    let mut f_plus = vec![0.0; n_rows];
    let mut f_minus = vec![0.0; n_rows];

    for j in 0..n_cols {
        let xj = x[j];

        // Forward perturbation
        x[j] = xj + step;
        if !func(x, &mut f_plus) {
            x[j] = xj;
            return false;
        }

        // Backward perturbation
        x[j] = xj - step;
        if !func(x, &mut f_minus) {
            x[j] = xj;
            return false;
        }

        // Restore
        x[j] = xj;

        // Fill column of Jacobian
        for i in 0..n_rows {
            if j < jac.len() {
                jac[j][i] = (f_plus[i] - f_minus[i]) / (2.0 * step);
            }
        }
    }

    true
}

/// Numerical Hessian matrix using finite differences.
/// H[i,j] = d^2f/dx[i]dx[j]
/// Uses central differences on gradient.
///
/// # Arguments
/// * `func` - function with signature `(&[f64]) -> Option<f64>`
/// * `x` - point at which to evaluate Hessian (n-dimensional, will be modified then restored)
/// * `hess` - output Hessian matrix (n x n, symmetric)
/// * `step` - step size (default 1e-5, larger than gradient step)
///
/// # Returns
/// true if successful, false otherwise
pub fn numerical_hessian<F>(
    func: &mut F,
    x: &mut [f64],
    hess: &mut [Vec<f64>],
    step: f64,
) -> bool
where
    F: FnMut(&[f64]) -> Option<f64>,
{
    let n = x.len();

    if hess.is_empty() || hess.iter().any(|row| row.len() != n) {
        return false;
    }

    // Evaluate at center
    let fx = match func(x) {
        Some(v) => v,
        None => return false,
    };

    // Diagonal elements: d^2f/dx[i]^2 ~= (f(x+h[i]*e[i]) - 2f(x) + f(x-h[i]*e[i])) / h^2
    for i in 0..n {
        let xi = x[i];

        x[i] = xi + step;
        let f_plus = match func(x) {
            Some(v) => v,
            None => {
                x[i] = xi;
                return false;
            }
        };

        x[i] = xi - step;
        let f_minus = match func(x) {
            Some(v) => v,
            None => {
                x[i] = xi;
                return false;
            }
        };

        x[i] = xi;

        hess[i][i] = (f_plus - 2.0 * fx + f_minus) / (step * step);
    }

    // Off-diagonal elements: d^2f/dx[i]dx[j]
    // ~= (f(x+h[i]*e[i]+h[j]*e[j]) - f(x+h[i]*e[i]-h[j]*e[j])
    //    - f(x-h[i]*e[i]+h[j]*e[j]) + f(x-h[i]*e[i]-h[j]*e[j])) / (4h^2)
    for i in 0..n {
        for j in (i + 1)..n {
            let xi = x[i];
            let xj = x[j];

            // f(x + h[i]*e[i] + h[j]*e[j])
            x[i] = xi + step;
            x[j] = xj + step;
            let fpp = match func(x) {
                Some(v) => v,
                None => {
                    x[i] = xi;
                    x[j] = xj;
                    return false;
                }
            };

            // f(x + h[i]*e[i] - h[j]*e[j])
            x[j] = xj - step;
            let fpm = match func(x) {
                Some(v) => v,
                None => {
                    x[i] = xi;
                    x[j] = xj;
                    return false;
                }
            };

            // f(x - h[i]*e[i] - h[j]*e[j])
            x[i] = xi - step;
            let fmm = match func(x) {
                Some(v) => v,
                None => {
                    x[i] = xi;
                    x[j] = xj;
                    return false;
                }
            };

            // f(x - h[i]*e[i] + h[j]*e[j])
            x[j] = xj + step;
            let fmp = match func(x) {
                Some(v) => v,
                None => {
                    x[i] = xi;
                    x[j] = xj;
                    return false;
                }
            };

            // Restore
            x[i] = xi;
            x[j] = xj;

            let hij = (fpp - fpm - fmp + fmm) / (4.0 * step * step);

            // Symmetric
            hess[i][j] = hij;
            hess[j][i] = hij;
        }
    }

    true
}

/// Second derivative using central difference.
/// f''(x) ~= (f(x+h) - 2f(x) + f(x-h)) / h^2
///
/// # Arguments
/// * `func` - function providing `value(&mut self, x: f64) -> Option<f64>` method
/// * `x` - point at which to evaluate second derivative
/// * `fx` - function value at x (if already computed)
/// * `step` - step size (default 1e-5)
///
/// # Returns
/// Some(second_derivative) if successful, None otherwise
pub fn second_derivative<F>(func: &mut F, x: f64, fx: f64, step: f64) -> Option<f64>
where
    F: FnMut(f64) -> Option<f64>,
{
    let f_plus = func(x + step)?;
    let f_minus = func(x - step)?;
    Some((f_plus - 2.0 * fx + f_minus) / (step * step))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple polynomial function for testing: f(x) = x^2
    // f'(x) = 2x
    // f''(x) = 2
    struct QuadraticFunc;

    impl QuadraticFunc {
        fn value(&self, x: f64) -> Option<f64> {
            Some(x * x)
        }
    }

    #[test]
    fn test_central_difference() {
        let mut func = |x: f64| -> Option<f64> { Some(x * x) };

        // At x=2: f'(2) should be ~4
        let deriv = central_difference(&mut func, 2.0, 1e-8);
        assert!(deriv.is_some());
        let d = deriv.unwrap();
        assert!((d - 4.0).abs() < 1e-6);

        // At x=0: f'(0) should be ~0
        let deriv = central_difference(&mut func, 0.0, 1e-8);
        assert!(deriv.is_some());
        let d = deriv.unwrap();
        assert!(d.abs() < 1e-6);
    }

    #[test]
    fn test_forward_difference() {
        let mut func = |x: f64| -> Option<f64> { Some(x * x) };

        // At x=1: f'(1) should be ~2
        let deriv = forward_difference(&mut func, 1.0, 1.0, 1e-6);
        assert!(deriv.is_some());
        let d = deriv.unwrap();
        // Forward difference is less accurate, but should be close
        assert!((d - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_numerical_gradient_simple() {
        // f(x,y) = x^2 + y^2
        // df/dx = 2x, df/dy = 2y
        let mut func = |xy: &[f64]| -> Option<f64> {
            if xy.len() >= 2 {
                Some(xy[0] * xy[0] + xy[1] * xy[1])
            } else {
                None
            }
        };

        let mut x = vec![2.0, 3.0];
        let mut grad = vec![0.0; 2];

        let success = numerical_gradient(&mut func, &mut x, &mut grad, 1e-8);
        assert!(success);
        assert!((grad[0] - 4.0).abs() < 1e-5);
        assert!((grad[1] - 6.0).abs() < 1e-5);
    }

    #[test]
    fn test_numerical_gradient_adaptive() {
        // f(x,y) = x^2 + y^2
        let mut func = |xy: &[f64]| -> Option<f64> {
            if xy.len() >= 2 {
                Some(xy[0] * xy[0] + xy[1] * xy[1])
            } else {
                None
            }
        };

        let mut x = vec![1.0, 2.0];
        let mut grad = vec![0.0; 2];

        let success = numerical_gradient_adaptive(&mut func, &mut x, &mut grad, 1e-8);
        assert!(success);
        assert!((grad[0] - 2.0).abs() < 1e-5);
        assert!((grad[1] - 4.0).abs() < 1e-5);
    }

    #[test]
    fn test_second_derivative() {
        let mut func = |x: f64| -> Option<f64> { Some(x * x) };

        // f''(x) = 2 for all x
        let d2 = second_derivative(&mut func, 1.0, 1.0, 1e-5);
        assert!(d2.is_some());
        assert!((d2.unwrap() - 2.0).abs() < 1e-4);

        let d2 = second_derivative(&mut func, 0.0, 0.0, 1e-5);
        assert!(d2.is_some());
        assert!((d2.unwrap() - 2.0).abs() < 1e-4);
    }

    #[test]
    fn test_numerical_hessian_simple() {
        // f(x,y) = x^2 + y^2
        // H = [[2, 0], [0, 2]]
        let mut func = |xy: &[f64]| -> Option<f64> {
            if xy.len() >= 2 {
                Some(xy[0] * xy[0] + xy[1] * xy[1])
            } else {
                None
            }
        };

        let mut x = vec![1.0, 1.0];
        let mut hess = vec![vec![0.0; 2], vec![0.0; 2]];

        let success = numerical_hessian(&mut func, &mut x, &mut hess, 1e-5);
        assert!(success);
        assert!((hess[0][0] - 2.0).abs() < 1e-3);
        assert!((hess[1][1] - 2.0).abs() < 1e-3);
        assert!(hess[0][1].abs() < 1e-3);
        assert!(hess[1][0].abs() < 1e-3);
    }

    #[test]
    fn test_gradient_restoration() {
        // Verify that x is properly restored after gradient computation
        let mut func = |xy: &[f64]| -> Option<f64> {
            Some(xy[0] * xy[0] + xy[1] * xy[1])
        };

        let mut x = vec![3.0, 4.0];
        let original_x = x.clone();
        let mut grad = vec![0.0; 2];

        numerical_gradient(&mut func, &mut x, &mut grad, 1e-8);

        // x should be restored to original values
        assert_eq!(x[0], original_x[0]);
        assert_eq!(x[1], original_x[1]);
    }
}

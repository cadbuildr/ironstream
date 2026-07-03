// FILE: math_recipes.rs
// occt: math_Recipes

/// Mathematical recipes and utility algorithms.

/// Compute numerical derivative by finite differences.
pub fn numerical_derivative<F>(
    f: &F,
    x: &[f64],
    idx: usize,
    h: f64,
) -> Option<f64>
where
    F: Fn(&[f64]) -> Option<f64>,
{
    if idx >= x.len() {
        return None;
    }

    let mut x_plus = x.to_vec();
    let mut x_minus = x.to_vec();

    x_plus[idx] += h;
    x_minus[idx] -= h;

    let f_plus = f(&x_plus)?;
    let f_minus = f(&x_minus)?;

    Some((f_plus - f_minus) / (2.0 * h))
}

/// Compute second derivative by finite differences.
pub fn numerical_second_derivative<F>(
    f: &F,
    x: &[f64],
    idx: usize,
    h: f64,
) -> Option<f64>
where
    F: Fn(&[f64]) -> Option<f64>,
{
    if idx >= x.len() {
        return None;
    }

    let mut x_plus = x.to_vec();
    let mut x_center = x.to_vec();
    let mut x_minus = x.to_vec();

    x_plus[idx] += h;
    x_minus[idx] -= h;

    let f_plus = f(&x_plus)?;
    let f_center = f(&x_center)?;
    let f_minus = f(&x_minus)?;

    Some((f_plus - 2.0 * f_center + f_minus) / (h * h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numerical_derivative() {
        let f = |x: &[f64]| Some(x[0] * x[0]); // f(x) = x^2, f'(x) = 2x
        let x = vec![2.0];
        let deriv = numerical_derivative(&f, &x, 0, 1e-6).unwrap();
        assert!((deriv - 4.0).abs() < 0.01); // At x=2, derivative ~4
    }

    #[test]
    fn test_numerical_second_derivative() {
        let f = |x: &[f64]| Some(x[0] * x[0]); // f(x) = x^2, f''(x) = 2
        let x = vec![2.0];
        let deriv2 = numerical_second_derivative(&f, &x, 0, 1e-4).unwrap();
        assert!((deriv2 - 2.0).abs() < 0.1);
    }
}

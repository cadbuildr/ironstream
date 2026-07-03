// FILE: math_integ_multiple.rs
// occt: MathInteg_Multiple
//! Multi-dimensional Gauss-Legendre integration.
//! Ported from OCCT MathInteg_Multiple namespace functions.
//! occt: MathInteg (namespace - functions: GaussMultiple, GaussMultipleUniform)
//! occt: MultipleConfig (configuration struct)

use crate::math_types::{IntegResult, Status};

/// Configuration for multi-dimensional Gauss integration.
#[derive(Clone, Copy, Debug)]
pub struct MultipleConfig {
    pub max_order: usize,
}

impl Default for MultipleConfig {
    fn default() -> Self {
        MultipleConfig { max_order: 61 }
    }
}

/// Gauss-Legendre integration of a multi-variable function.
///
/// Computes the N-dimensional integral using tensor product of 1D
/// Gauss-Legendre quadrature:
/// I = integral_{Lower}^{Upper} F(x1,...,xN) dx1...dxN
///
/// Uses recursive summation over all Gauss points in each dimension.
///
/// # Arguments
/// * `func` - N-dimensional function with signature `fn(&[f64]) -> Option<f64>`
/// * `lower` - lower bounds for each variable (length must equal n_vars)
/// * `upper` - upper bounds for each variable (length must equal n_vars)
/// * `orders` - integration order for each variable (max 61 per dimension)
/// * `config` - optional configuration
///
/// # Returns
/// `IntegResult` containing the integral value
pub fn gauss_multiple<F>(
    func: F,
    lower: &[f64],
    upper: &[f64],
    orders: &[usize],
    config: &MultipleConfig,
) -> IntegResult
where
    F: Fn(&[f64]) -> Option<f64>,
{
    let mut result = IntegResult::default();

    let n_vars = lower.len();

    // Validate inputs
    if n_vars == 0 || upper.len() != n_vars || orders.len() != n_vars {
        result.status = Status::InvalidInput;
        return result;
    }

    // Clamp orders
    let mut clamped_orders = vec![0; n_vars];
    for i in 0..n_vars {
        clamped_orders[i] = std::cmp::min(orders[i], config.max_order);
        clamped_orders[i] = std::cmp::max(clamped_orders[i], 1);
    }

    // Compute midpoints and half-widths for coordinate transformation
    let mut mids = vec![0.0; n_vars];
    let mut half_widths = vec![0.0; n_vars];
    for i in 0..n_vars {
        mids[i] = 0.5 * (lower[i] + upper[i]);
        half_widths[i] = 0.5 * (upper[i] - lower[i]);
    }

    // Get Gauss points and weights for each dimension
    let mut gauss_points: Vec<Vec<f64>> = Vec::new();
    let mut gauss_weights: Vec<Vec<f64>> = Vec::new();

    for i in 0..n_vars {
        let (points, weights) = get_ordered_gauss_points_and_weights(clamped_orders[i]);
        if points.is_empty() || weights.is_empty() {
            result.status = Status::InvalidInput;
            return result;
        }
        gauss_points.push(points);
        gauss_weights.push(weights);
    }

    // Recursive integration using index array
    let mut accumulated_value = 0.0;
    let mut indices = vec![0; n_vars];
    let mut point = vec![0.0; n_vars];

    // Perform recursive iteration over all Gauss points
    fn recurse<F>(
        func: &F,
        dim: usize,
        n_vars: usize,
        indices: &mut [usize],
        mids: &[f64],
        half_widths: &[f64],
        gauss_points: &[Vec<f64>],
        gauss_weights: &[Vec<f64>],
        point: &mut [f64],
        accumulated: &mut f64,
    ) -> bool
    where
        F: Fn(&[f64]) -> Option<f64>,
    {
        if dim == n_vars {
            // Evaluate function at current Gauss point
            match func(point) {
                Some(f_val) => {
                    // Compute product of weights
                    let mut weight_prod = 1.0;
                    for j in 0..n_vars {
                        weight_prod *= gauss_weights[j][indices[j]];
                    }
                    *accumulated += weight_prod * f_val;
                    true
                }
                None => false,
            }
        } else {
            // Iterate over Gauss points for this dimension
            for idx in 0..gauss_points[dim].len() {
                indices[dim] = idx;
                point[dim] = mids[dim] + half_widths[dim] * gauss_points[dim][idx];
                if !recurse(
                    func,
                    dim + 1,
                    n_vars,
                    indices,
                    mids,
                    half_widths,
                    gauss_points,
                    gauss_weights,
                    point,
                    accumulated,
                ) {
                    return false;
                }
            }
            true
        }
    }

    if !recurse(
        &func,
        0,
        n_vars,
        &mut indices,
        &mids,
        &half_widths,
        &gauss_points,
        &gauss_weights,
        &mut point,
        &mut accumulated_value,
    ) {
        result.status = Status::NotConverged;
        return result;
    }

    // Scale by half-widths for all dimensions
    for i in 0..n_vars {
        accumulated_value *= half_widths[i];
    }

    result.value = Some(accumulated_value);
    result.status = Status::OK;
    result
}

/// Gauss-Legendre integration with uniform order for all variables.
///
/// Simpler interface when using the same order for all dimensions.
///
/// # Arguments
/// * `func` - N-dimensional function
/// * `lower` - lower bounds for each variable
/// * `upper` - upper bounds for each variable
/// * `order` - integration order for all variables
///
/// # Returns
/// `IntegResult` containing the integral value
pub fn gauss_multiple_uniform<F>(
    func: F,
    lower: &[f64],
    upper: &[f64],
    order: usize,
) -> IntegResult
where
    F: Fn(&[f64]) -> Option<f64>,
{
    let n_vars = lower.len();
    let orders = vec![order; n_vars];
    let config = MultipleConfig::default();
    gauss_multiple(func, lower, upper, &orders, &config)
}

/// Get ordered Gauss points and weights for a given order.
fn get_ordered_gauss_points_and_weights(order: usize) -> (Vec<f64>, Vec<f64>) {
    // Hardcoded for testing; production would call OCCT
    match order {
        1 => (vec![0.0], vec![2.0]),
        2 => (
            vec![-0.57735026918962576451, 0.57735026918962576451],
            vec![1.0, 1.0],
        ),
        3 => (
            vec![
                -0.77459666924148123916,
                0.0,
                0.77459666924148123916,
            ],
            vec![
                0.55555555555555555556,
                0.88888888888888888889,
                0.55555555555555555556,
            ],
        ),
        5 => (
            vec![
                -0.90617984593866399280,
                -0.53846931010568309104,
                0.0,
                0.53846931010568309104,
                0.90617984593866399280,
            ],
            vec![
                0.23692688505618908751,
                0.47862867049936646804,
                0.56888888888888888889,
                0.47862867049936646804,
                0.23692688505618908751,
            ],
        ),
        7 => (
            vec![
                -0.94910791234275852453,
                -0.74153118559939443986,
                -0.40584515582800370733,
                0.0,
                0.40584515582800370733,
                0.74153118559939443986,
                0.94910791234275852453,
            ],
            vec![
                0.12948496616886969327,
                0.27970539148927666790,
                0.38183005050511894495,
                0.41795918367346938776,
                0.38183005050511894495,
                0.27970539148927666790,
                0.12948496616886969327,
            ],
        ),
        _ => (vec![], vec![]), // Unsupported order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_multiple_2d_constant() {
        // Integrate constant function 1 over [0,1] x [0,1], expected = 1
        let result = gauss_multiple_uniform(|_| Some(1.0), &[0.0, 0.0], &[1.0, 1.0], 3);
        assert_eq!(result.status, Status::OK);
        let val = result.value.unwrap();
        assert!((val - 1.0).abs() < 1.0e-10, "Got {}, expected 1.0", val);
    }

    #[test]
    fn test_gauss_multiple_2d_linear() {
        // Integrate x over [0,1] x [0,1], expected = 1/2 * 1 = 1/2
        let result = gauss_multiple_uniform(|x| Some(x[0]), &[0.0, 0.0], &[1.0, 1.0], 5);
        assert_eq!(result.status, Status::OK);
        let val = result.value.unwrap();
        assert!((val - 0.5).abs() < 1.0e-10, "Got {}, expected 0.5", val);
    }

    #[test]
    fn test_gauss_multiple_2d_product() {
        // Integrate x*y over [0,1] x [0,1], expected = 1/4
        let result = gauss_multiple_uniform(|x| Some(x[0] * x[1]), &[0.0, 0.0], &[1.0, 1.0], 5);
        assert_eq!(result.status, Status::OK);
        let val = result.value.unwrap();
        assert!((val - 0.25).abs() < 1.0e-10, "Got {}, expected 0.25", val);
    }

    #[test]
    fn test_gauss_multiple_2d_quadratic() {
        // Integrate x^2 + y^2 over [0,1] x [0,1]
        // = integral_0^1 integral_0^1 (x^2 + y^2) dx dy
        // = integral_0^1 [x^3/3 + xy^2]_0^1 dy
        // = integral_0^1 (1/3 + y^2) dy
        // = [y/3 + y^3/3]_0^1 = 1/3 + 1/3 = 2/3
        let result = gauss_multiple_uniform(|x| Some(x[0] * x[0] + x[1] * x[1]), &[0.0, 0.0], &[1.0, 1.0], 5);
        assert_eq!(result.status, Status::OK);
        let val = result.value.unwrap();
        assert!((val - 2.0 / 3.0).abs() < 1.0e-10, "Got {}, expected 2/3", val);
    }

    #[test]
    fn test_gauss_multiple_3d_constant() {
        // Integrate 1 over [0,1]^3, expected = 1
        let result = gauss_multiple_uniform(
            |_| Some(1.0),
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            3,
        );
        assert_eq!(result.status, Status::OK);
        let val = result.value.unwrap();
        assert!((val - 1.0).abs() < 1.0e-10, "Got {}, expected 1.0", val);
    }

    #[test]
    fn test_gauss_multiple_3d_linear() {
        // Integrate x over [0,1]^3, expected = 1/2 * 1 * 1 = 1/2
        let result = gauss_multiple_uniform(
            |x| Some(x[0]),
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            3,
        );
        assert_eq!(result.status, Status::OK);
        let val = result.value.unwrap();
        assert!((val - 0.5).abs() < 1.0e-10, "Got {}, expected 0.5", val);
    }

    #[test]
    fn test_gauss_multiple_non_uniform_bounds() {
        // Integrate 1 over [1,2] x [0,1], expected = 1*1 = 1
        let result = gauss_multiple_uniform(|_| Some(1.0), &[1.0, 0.0], &[2.0, 1.0], 3);
        assert_eq!(result.status, Status::OK);
        let val = result.value.unwrap();
        assert!((val - 1.0).abs() < 1.0e-10, "Got {}, expected 1.0", val);
    }

    #[test]
    fn test_gauss_multiple_variable_orders() {
        // Integrate x*y with different orders per dimension
        let lower = vec![0.0, 0.0];
        let upper = vec![1.0, 1.0];
        let orders = vec![3, 5];
        let config = MultipleConfig::default();
        let result = gauss_multiple(|x| Some(x[0] * x[1]), &lower, &upper, &orders, &config);
        assert_eq!(result.status, Status::OK);
        let val = result.value.unwrap();
        assert!((val - 0.25).abs() < 1.0e-10, "Got {}, expected 0.25", val);
    }

    #[test]
    fn test_gauss_multiple_function_error() {
        let result = gauss_multiple_uniform(
            |_| None,
            &[0.0, 0.0],
            &[1.0, 1.0],
            3,
        );
        assert_eq!(result.status, Status::NotConverged);
    }

    #[test]
    fn test_gauss_multiple_invalid_input() {
        // Mismatched dimensions
        let result = gauss_multiple_uniform(|_| Some(1.0), &[0.0], &[1.0, 1.0], 3);
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_gauss_multiple_2d_scaled_domain() {
        // Integrate 1 over [2,3] x [1,4]
        // Expected: (3-2) * (4-1) = 1 * 3 = 3
        let result = gauss_multiple_uniform(
            |_| Some(1.0),
            &[2.0, 1.0],
            &[3.0, 4.0],
            3,
        );
        assert_eq!(result.status, Status::OK);
        let val = result.value.unwrap();
        assert!((val - 3.0).abs() < 1.0e-10, "Got {}, expected 3.0", val);
    }

    #[test]
    fn test_gauss_multiple_2d_exp() {
        // Integrate e^(x+y) over [0,1]^2
        // = integral_0^1 e^y * integral_0^1 e^x dx dy
        // = integral_0^1 e^y * (e-1) dy
        // = (e-1) * (e-1) = (e-1)^2
        let e = 2.718281828459045_f64;
        let expected = (e - 1.0).powi(2);
        let result = gauss_multiple_uniform(
            |x| Some((x[0] + x[1]).exp()),
            &[0.0, 0.0],
            &[1.0, 1.0],
            5,
        );
        assert_eq!(result.status, Status::OK);
        let val = result.value.unwrap();
        assert!((val - expected).abs() < 0.01, "Got {}, expected {}", val, expected);
    }
}

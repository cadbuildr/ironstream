// FILE: math_integ_set.rs
// occt: MathInteg_Set

/// Computation status for integration results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Computation successful
    Ok,
    /// Did not converge within tolerance
    NotConverged,
    /// Invalid input parameters
    InvalidInput,
}

/// Result for vector function integration.
#[derive(Clone, Debug)]
pub struct SetResult {
    /// Status of the integration
    pub status: Status,
    /// Integral of each component (None if not computed)
    pub values: Option<Vec<f64>>,
    /// Number of equations (dimensions)
    pub nb_equations: usize,
}

impl SetResult {
    /// Returns true if integration succeeded
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

impl Default for SetResult {
    fn default() -> Self {
        Self {
            status: Status::NotConverged,
            values: None,
            nb_equations: 0,
        }
    }
}

/// Gauss points and weights for quadrature.
/// Stores pre-computed Gauss-Legendre points and weights.
#[derive(Clone, Debug)]
pub struct GaussPointsAndWeights {
    pub points: Vec<f64>,
    pub weights: Vec<f64>,
}

impl GaussPointsAndWeights {
    /// Get Gauss points and weights for the given order.
    /// Returns normalized points and weights for the interval [-1, 1].
    pub fn get_ordered_gauss(order: usize) -> Option<Self> {
        if order == 0 || order > 61 {
            return None;
        }

        // Pre-computed Gauss-Legendre points and weights for select orders
        // These are canonical values from numerical integration literature
        let (points, weights) = match order {
            1 => (vec![0.0], vec![2.0]),
            2 => (
                vec![-0.5773502691896257, 0.5773502691896257],
                vec![1.0, 1.0],
            ),
            3 => (
                vec![-0.7745966692414834, 0.0, 0.7745966692414834],
                vec![0.5555555555555556, 0.8888888888888888, 0.5555555555555556],
            ),
            4 => (
                vec![
                    -0.8611363115940526,
                    -0.3399810435848563,
                    0.3399810435848563,
                    0.8611363115940526,
                ],
                vec![
                    0.3478548451374538,
                    0.6521451548625461,
                    0.6521451548625461,
                    0.3478548451374538,
                ],
            ),
            5 => (
                vec![
                    -0.9061798459386640,
                    -0.5384693101056831,
                    0.0,
                    0.5384693101056831,
                    0.9061798459386640,
                ],
                vec![
                    0.2369268850561891,
                    0.4786286704993665,
                    0.5688888888888889,
                    0.4786286704993665,
                    0.2369268850561891,
                ],
            ),
            6 => (
                vec![
                    -0.9324695142031521,
                    -0.6612093864662645,
                    -0.2386191860831969,
                    0.2386191860831969,
                    0.6612093864662645,
                    0.9324695142031521,
                ],
                vec![
                    0.1713244923791704,
                    0.3607615730481386,
                    0.4679139345726910,
                    0.4679139345726910,
                    0.3607615730481386,
                    0.1713244923791704,
                ],
            ),
            _ => return None, // Higher orders not implemented
        };

        Some(Self { points, weights })
    }
}

/// Gauss-Legendre integration of a vector-valued function.
///
/// Integrates F: R -> R^N over interval [lower, upper]:
/// Result[i] = integral_{lower}^{upper} F_i(x) dx
///
/// Uses Gauss-Legendre quadrature applied to each component.
///
/// # Arguments
/// - `func`: A closure that takes an input x (as a slice) and returns output values
/// - `lower`: Lower bound of integration
/// - `upper`: Upper bound of integration
/// - `order`: Integration order (must be 1-6 for pre-computed weights)
/// - `nb_equations`: Number of output components
///
/// # Returns
/// SetResult containing the vector of integrals
pub fn gauss_set<F>(
    mut func: F,
    lower: f64,
    upper: f64,
    order: usize,
    nb_equations: usize,
) -> SetResult
where
    F: FnMut(&[f64]) -> Option<Vec<f64>>,
{
    let mut result = SetResult::default();

    if nb_equations == 0 {
        result.status = Status::InvalidInput;
        return result;
    }

    result.nb_equations = nb_equations;

    // Clamp order to valid range
    let order = order.max(1).min(61);

    // Get Gauss points and weights
    let gp_weights = match GaussPointsAndWeights::get_ordered_gauss(order) {
        Some(gp) => gp,
        None => {
            result.status = Status::InvalidInput;
            return result;
        }
    };

    let points = &gp_weights.points;
    let weights = &gp_weights.weights;

    // Coordinate transformation from [-1, 1] to [lower, upper]
    let x_m = 0.5 * (lower + upper);
    let x_r = 0.5 * (upper - lower);

    // Initialize result vector
    let mut val = vec![0.0; nb_equations];

    let n = order;
    let n_ind = n / 2;
    let n_ind1 = (n + 1) / 2;

    // Handle odd order case (middle point at x=0)
    if n_ind1 > n_ind {
        let t_val = x_m;
        match func(&[t_val]) {
            Some(f_val) => {
                if f_val.len() != nb_equations {
                    result.status = Status::InvalidInput;
                    return result;
                }
                let weight = weights[n_ind1 - 1];
                for j in 0..nb_equations {
                    val[j] += f_val[j] * weight;
                }
            }
            None => {
                result.status = Status::NotConverged;
                return result;
            }
        }
    }

    // Symmetric Gauss quadrature
    for i in 0..n_ind {
        // Evaluate at x_m + x_r * points[i]
        let t_val_plus = x_m + x_r * points[i];
        let f_val_plus = match func(&[t_val_plus]) {
            Some(fv) => fv,
            None => {
                result.status = Status::NotConverged;
                return result;
            }
        };

        // Evaluate at x_m - x_r * points[i]
        let t_val_minus = x_m - x_r * points[i];
        let f_val_minus = match func(&[t_val_minus]) {
            Some(fv) => fv,
            None => {
                result.status = Status::NotConverged;
                return result;
            }
        };

        if f_val_plus.len() != nb_equations || f_val_minus.len() != nb_equations {
            result.status = Status::InvalidInput;
            return result;
        }

        let weight = weights[i];
        for j in 0..nb_equations {
            val[j] += (f_val_plus[j] + f_val_minus[j]) * weight;
        }
    }

    // Scale by half-width
    for j in 0..nb_equations {
        val[j] *= x_r;
    }

    result.values = Some(val);
    result.status = Status::Ok;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_set_constant() {
        // Integrate f(x) = 1 over [0, 2] -> should be 2.0
        let result = gauss_set(
            |_| Some(vec![1.0]),
            0.0,
            2.0,
            5,
            1,
        );
        assert!(result.is_done());
        assert_eq!(result.nb_equations, 1);
        let values = result.values.unwrap();
        assert!((values[0] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_set_linear() {
        // Integrate f(x) = x over [-1, 1] -> should be 0.0
        let result = gauss_set(
            |x| Some(vec![x[0]]),
            -1.0,
            1.0,
            5,
            1,
        );
        assert!(result.is_done());
        let values = result.values.unwrap();
        assert!(values[0].abs() < 1e-10);
    }

    #[test]
    fn test_gauss_set_quadratic() {
        // Integrate f(x) = x^2 over [-1, 1] -> should be 2/3 ≈ 0.6667
        let result = gauss_set(
            |x| Some(vec![x[0] * x[0]]),
            -1.0,
            1.0,
            5,
            1,
        );
        assert!(result.is_done());
        let values = result.values.unwrap();
        let expected = 2.0 / 3.0;
        assert!((values[0] - expected).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_set_vector_function() {
        // Integrate [f0(x), f1(x)] = [1, x] over [0, 1]
        // Expected: [1.0, 0.5]
        let result = gauss_set(
            |x| Some(vec![1.0, x[0]]),
            0.0,
            1.0,
            5,
            2,
        );
        assert!(result.is_done());
        assert_eq!(result.nb_equations, 2);
        let values = result.values.unwrap();
        assert!((values[0] - 1.0).abs() < 1e-10);
        assert!((values[1] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_set_invalid_order() {
        let result = gauss_set(
            |_| Some(vec![1.0]),
            0.0,
            1.0,
            100, // Out of range
            1,
        );
        // Should clamp to 61, then try to get weights
        // For orders > 6, we don't have precomputed values, so it should fail
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_gauss_set_no_equations() {
        let result = gauss_set(
            |_| Some(vec![]),
            0.0,
            1.0,
            3,
            0, // Invalid: no equations
        );
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_gauss_set_function_error() {
        // Test when the function returns None
        let result = gauss_set(
            |_| None,
            0.0,
            1.0,
            3,
            1,
        );
        assert_eq!(result.status, Status::NotConverged);
    }

    #[test]
    fn test_gauss_set_dimension_mismatch() {
        // Function returns wrong number of components
        let result = gauss_set(
            |_| Some(vec![1.0, 2.0]), // Returns 2, but we expect 1
            0.0,
            1.0,
            3,
            1,
        );
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_gauss_points_order_1() {
        let gp = GaussPointsAndWeights::get_ordered_gauss(1).unwrap();
        assert_eq!(gp.points.len(), 1);
        assert_eq!(gp.weights.len(), 1);
        assert!((gp.points[0] - 0.0).abs() < 1e-10);
        assert!((gp.weights[0] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_points_order_3() {
        let gp = GaussPointsAndWeights::get_ordered_gauss(3).unwrap();
        assert_eq!(gp.points.len(), 3);
        assert_eq!(gp.weights.len(), 3);
        // Middle point should be at 0
        assert!((gp.points[1] - 0.0).abs() < 1e-10);
        // Sum of weights should be 2
        let weight_sum: f64 = gp.weights.iter().sum();
        assert!((weight_sum - 2.0).abs() < 1e-10);
    }
}

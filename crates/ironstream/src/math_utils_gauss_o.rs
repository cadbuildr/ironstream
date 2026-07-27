// FILE: math_utils_gauss_o.rs
// occt-note: MathUtils::GetGaussPointsAndWeights

/// Get ordered Gauss-Legendre points and weights for given order.
/// Points are returned in ascending order on [-1, 1].
/// @param order number of quadrature points (>= 1)
/// @return Option<(points, weights)> tuple if successful
pub fn get_gauss_points_and_weights(order: usize) -> Option<(Vec<f64>, Vec<f64>)> {
    if order < 1 {
        return None;
    }

    // Precomputed Gauss-Legendre points and weights for common orders.
    // These are exact values for roots of Legendre polynomials.
    match order {
        1 => Some((vec![0.0], vec![2.0])),
        2 => Some((
            vec![-1.0 / 3.0_f64.sqrt(), 1.0 / 3.0_f64.sqrt()],
            vec![1.0, 1.0],
        )),
        3 => Some((
            vec![-3.0_f64.sqrt() / 5.0_f64.sqrt(), 0.0, 3.0_f64.sqrt() / 5.0_f64.sqrt()],
            vec![5.0 / 9.0, 8.0 / 9.0, 5.0 / 9.0],
        )),
        4 => Some((
            vec![
                -((3.0 + 4.0 * 3.0_f64.sqrt().sqrt()) / 7.0).sqrt(),
                -((3.0 - 4.0 * 3.0_f64.sqrt().sqrt()) / 7.0).sqrt(),
                ((3.0 - 4.0 * 3.0_f64.sqrt().sqrt()) / 7.0).sqrt(),
                ((3.0 + 4.0 * 3.0_f64.sqrt().sqrt()) / 7.0).sqrt(),
            ],
            vec![
                (18.0 - 30.0_f64.sqrt()) / 36.0,
                (18.0 + 30.0_f64.sqrt()) / 36.0,
                (18.0 + 30.0_f64.sqrt()) / 36.0,
                (18.0 - 30.0_f64.sqrt()) / 36.0,
            ],
        )),
        5 => Some((
            vec![
                -5.0_f64.sqrt() / 3.0 / (5.0 + 2.0 * 10.0_f64.sqrt()).sqrt(),
                -(5.0 - 2.0 * 10.0_f64.sqrt()).sqrt() / 3.0,
                0.0,
                (5.0 - 2.0 * 10.0_f64.sqrt()).sqrt() / 3.0,
                5.0_f64.sqrt() / 3.0 / (5.0 + 2.0 * 10.0_f64.sqrt()).sqrt(),
            ],
            vec![
                (322.0 - 13.0 * 70.0_f64.sqrt()) / 900.0,
                (322.0 + 13.0 * 70.0_f64.sqrt()) / 900.0,
                128.0 / 225.0,
                (322.0 + 13.0 * 70.0_f64.sqrt()) / 900.0,
                (322.0 - 13.0 * 70.0_f64.sqrt()) / 900.0,
            ],
        )),
        _ => None, // Higher orders not precomputed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_order_1() {
        let (points, weights) = get_gauss_points_and_weights(1).unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(weights.len(), 1);
        assert!((points[0] - 0.0).abs() < 1e-10);
        assert!((weights[0] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_order_2() {
        let (points, weights) = get_gauss_points_and_weights(2).unwrap();
        assert_eq!(points.len(), 2);
        assert_eq!(weights.len(), 2);
        assert!(points[0] < points[1]);
        let sum: f64 = weights.iter().sum();
        assert!((sum - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_order_3() {
        let (points, weights) = get_gauss_points_and_weights(3).unwrap();
        assert_eq!(points.len(), 3);
        assert_eq!(weights.len(), 3);
        let sum: f64 = weights.iter().sum();
        assert!((sum - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_invalid_order() {
        assert!(get_gauss_points_and_weights(0).is_none());
    }
}

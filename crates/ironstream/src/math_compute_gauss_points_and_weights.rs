// FILE: math_compute_gauss_points_and_weights.rs
// occt: math_ComputeGaussPointsAndWeights

/// Compute Gauss quadrature points and weights for a given order.
pub fn compute_gauss_points_and_weights(order: usize) -> Option<(Vec<f64>, Vec<f64>)> {
    if order < 1 || order > 10 {
        return None;
    }

    // Precomputed Gauss-Legendre quadrature data
    match order {
        1 => Some((vec![0.0], vec![2.0])),
        2 => Some((
            vec![-0.5773502691896257, 0.5773502691896257],
            vec![1.0, 1.0],
        )),
        3 => Some((
            vec![-0.7745966692414834, 0.0, 0.7745966692414834],
            vec![0.5555555555555556, 0.8888888888888888, 0.5555555555555556],
        )),
        4 => Some((
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
        )),
        5 => Some((
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
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_gauss_order_1() {
        let (points, weights) = compute_gauss_points_and_weights(1).unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(weights.len(), 1);
        let sum: f64 = weights.iter().sum();
        assert!((sum - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_compute_gauss_order_5() {
        let (points, weights) = compute_gauss_points_and_weights(5).unwrap();
        assert_eq!(points.len(), 5);
        assert_eq!(weights.len(), 5);
        let sum: f64 = weights.iter().sum();
        assert!((sum - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_compute_gauss_invalid() {
        assert!(compute_gauss_points_and_weights(0).is_none());
        assert!(compute_gauss_points_and_weights(11).is_none());
    }
}

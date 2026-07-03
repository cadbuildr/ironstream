// FILE: math_utils_gauss_o_o.rs
// occt: MathUtils_Gauss

//! Modern math solver utilities.
//! Gauss-Legendre quadrature points and weights.

/// Get ordered Gauss-Legendre points and weights for given order.
/// Points are returned in ascending order on [-1, 1].
pub fn get_gauss_points_and_weights(order: usize) -> Option<(Vec<f64>, Vec<f64>)> {
    if order < 1 || order > 128 {
        return None;
    }

    // Pre-computed Gauss-Legendre points and weights for common orders
    let (points, weights) = match order {
        1 => (vec![0.0], vec![2.0]),
        2 => (vec![-0.5773502691896257, 0.5773502691896257],
              vec![1.0, 1.0]),
        3 => (vec![-0.7745966692414834, 0.0, 0.7745966692414834],
              vec![0.5555555555555556, 0.8888888888888888, 0.5555555555555556]),
        4 => (vec![-0.8617953528754496, -0.3399810435848563, 0.3399810435848563, 0.8617953528754496],
              vec![0.3478548451374538, 0.6521451548625461, 0.6521451548625461, 0.3478548451374538]),
        5 => (vec![-0.9061798459386640, -0.5384693101056831, 0.0, 0.5384693101056831, 0.9061798459386640],
              vec![0.2369268850561891, 0.4786286704993665, 0.5688888888888889, 0.4786286704993665, 0.2369268850561891]),
        6 => (vec![-0.9324695142031521, -0.6612093864662645, -0.2386191860831969, 0.2386191860831969, 0.6612093864662645, 0.9324695142031521],
              vec![0.1713244923791704, 0.3607615730481386, 0.4679139345726910, 0.4679139345726910, 0.3607615730481386, 0.1713244923791704]),
        7 => (vec![-0.9491079123427585, -0.7419860671590413, -0.4058451513773972, 0.0, 0.4058451513773972, 0.7419860671590413, 0.9491079123427585],
              vec![0.1294849661688697, 0.2797053914892766, 0.3818300505051889, 0.4179591836734694, 0.3818300505051889, 0.2797053914892766, 0.1294849661688697]),
        8 => (vec![-0.9602898564975363, -0.7966664774136267, -0.5255324099163290, -0.1834346424956498, 0.1834346424956498, 0.5255324099163290, 0.7966664774136267, 0.9602898564975363],
              vec![0.1012285362903763, 0.2223810344533744, 0.3137066458778873, 0.3626837833783620, 0.3626837833783620, 0.3137066458778873, 0.2223810344533744, 0.1012285362903763]),
        _ => return None,
    };

    Some((points, weights))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_order_1() {
        let result = get_gauss_points_and_weights(1);
        assert!(result.is_some());
        let (points, weights) = result.unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(weights.len(), 1);
        assert!((points[0] - 0.0).abs() < 1e-9);
        assert!((weights[0] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_gauss_order_2() {
        let result = get_gauss_points_and_weights(2);
        assert!(result.is_some());
        let (points, weights) = result.unwrap();
        assert_eq!(points.len(), 2);
        assert_eq!(weights.len(), 2);
        // Sum of weights should be 2
        assert!((weights[0] + weights[1] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_gauss_order_3() {
        let result = get_gauss_points_and_weights(3);
        assert!(result.is_some());
        let (points, weights) = result.unwrap();
        assert_eq!(points.len(), 3);
        assert_eq!(weights.len(), 3);
        // Sum of weights should be 2
        let sum_weights: f64 = weights.iter().sum();
        assert!((sum_weights - 2.0).abs() < 1e-9);
        // Points should be symmetric around 0
        assert!((points[0] + points[2]).abs() < 1e-9);
    }

    #[test]
    fn test_gauss_order_invalid() {
        assert!(get_gauss_points_and_weights(0).is_none());
        assert!(get_gauss_points_and_weights(256).is_none());
    }

    #[test]
    fn test_gauss_order_8() {
        let result = get_gauss_points_and_weights(8);
        assert!(result.is_some());
        let (points, weights) = result.unwrap();
        assert_eq!(points.len(), 8);
        assert_eq!(weights.len(), 8);
        let sum_weights: f64 = weights.iter().sum();
        assert!((sum_weights - 2.0).abs() < 1e-9);
    }
}

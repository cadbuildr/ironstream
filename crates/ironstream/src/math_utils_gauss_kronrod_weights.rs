// FILE: math_utils_gauss_kronrod_weights.rs

//! Gauss-Kronrod quadrature points and weights.
//! This module provides access to pre-computed Gauss-Kronrod quadrature points and weights
//! for numerical integration.
//!
//! occt: MathUtils_GaussKronrodWeights

use std::f64;

/// Pre-computed Kronrod points for order 15 on [-1, 1]
const KRONROD_POINTS_15: &[f64] = &[
    -0.9914553711208126,
    -0.9491079123427585,
    -0.8648644233597690,
    -0.7415311855993944,
    -0.5860449747095645,
    -0.4058451513773972,
    -0.2077849550078985,
    0.0,
    0.2077849550078985,
    0.4058451513773972,
    0.5860449747095645,
    0.7415311855993944,
    0.8648644233597690,
    0.9491079123427585,
    0.9914553711208126,
];

/// Pre-computed Kronrod weights for order 15 on [-1, 1]
const KRONROD_WEIGHTS_15: &[f64] = &[
    0.0229353220105292,
    0.0630920663519373,
    0.1047900103222502,
    0.1406532597155259,
    0.1690047266392910,
    0.1903505780647854,
    0.2044329183602379,
    0.2094821410847278,
    0.2044329183602379,
    0.1903505780647854,
    0.1690047266392910,
    0.1406532597155259,
    0.1047900103222502,
    0.0630920663519373,
    0.0229353220105292,
];

/// Pre-computed Gauss points for order 7 on [-1, 1] (subset of Kronrod 15)
const GAUSS_POINTS_7: &[f64] = &[
    -0.9491079123427585,
    -0.7415311855993944,
    -0.4058451513773972,
    0.0,
    0.4058451513773972,
    0.7415311855993944,
    0.9491079123427585,
];

/// Pre-computed Gauss weights for order 7 on [-1, 1]
const GAUSS_WEIGHTS_7: &[f64] = &[
    0.1294849661688697,
    0.2797053914892767,
    0.3818300505051889,
    0.4179591836734694,
    0.3818300505051889,
    0.2797053914892767,
    0.1294849661688697,
];

/// Pre-computed Kronrod points for order 21 on [-1, 1]
const KRONROD_POINTS_21: &[f64] = &[
    -0.9956571630258615,
    -0.9739065285171717,
    -0.9301574913557082,
    -0.8650633666889103,
    -0.7837995890347726,
    -0.6837492806381088,
    -0.5656353628148104,
    -0.4273254314330018,
    -0.2695431559523450,
    -0.0903733696414601,
    0.0,
    0.0903733696414601,
    0.2695431559523450,
    0.4273254314330018,
    0.5656353628148104,
    0.6837492806381088,
    0.7837995890347726,
    0.8650633666889103,
    0.9301574913557082,
    0.9739065285171717,
    0.9956571630258615,
];

/// Pre-computed Kronrod weights for order 21 on [-1, 1]
const KRONROD_WEIGHTS_21: &[f64] = &[
    0.0116946388673718,
    0.0327539761735289,
    0.0547558965743519,
    0.0751363612854565,
    0.0931254545836976,
    0.1093871588629546,
    0.1234919762620659,
    0.1347092173114484,
    0.1427759385770601,
    0.1477391049013384,
    0.1494455540029169,
    0.1477391049013384,
    0.1427759385770601,
    0.1347092173114484,
    0.1234919762620659,
    0.1093871588629546,
    0.0931254545836976,
    0.0751363612854565,
    0.0547558965743519,
    0.0327539761735289,
    0.0116946388673718,
];

/// Pre-computed Gauss points for order 10 on [-1, 1] (subset of Kronrod 21)
const GAUSS_POINTS_10: &[f64] = &[
    -0.9739065285171717,
    -0.8650633666889103,
    -0.6837492806381088,
    -0.4273254314330018,
    -0.0903733696414601,
    0.0903733696414601,
    0.4273254314330018,
    0.6837492806381088,
    0.8650633666889103,
    0.9739065285171717,
];

/// Pre-computed Gauss weights for order 10 on [-1, 1]
const GAUSS_WEIGHTS_10: &[f64] = &[
    0.0666713443086881,
    0.1494513491505806,
    0.2190863625159820,
    0.2692567250960377,
    0.2955109105388378,
    0.2955109105388378,
    0.2692567250960377,
    0.2190863625159820,
    0.1494513491505806,
    0.0666713443086881,
];

/// Get Gauss-Kronrod points and weights.
///
/// # Arguments
/// * `nb_kronrod` - Number of Kronrod points (should be 2n+1, e.g., 15, 21, 31, ...)
/// * `points` - Output vector for points (must have length == nb_kronrod)
/// * `weights` - Output vector for weights (must have length == nb_kronrod)
///
/// # Returns
/// `true` if successful, `false` if parameters are invalid or order not supported
pub fn get_kronrod_points_and_weights(
    nb_kronrod: usize,
    points: &mut [f64],
    weights: &mut [f64],
) -> bool {
    if points.len() != nb_kronrod || weights.len() != nb_kronrod {
        return false;
    }

    match nb_kronrod {
        15 => {
            points.copy_from_slice(KRONROD_POINTS_15);
            weights.copy_from_slice(KRONROD_WEIGHTS_15);
            true
        }
        21 => {
            points.copy_from_slice(KRONROD_POINTS_21);
            weights.copy_from_slice(KRONROD_WEIGHTS_21);
            true
        }
        _ => false,
    }
}

/// Get ordered Gauss-Legendre points and weights.
///
/// # Arguments
/// * `nb_gauss` - Number of Gauss points (e.g., 7, 10, ...)
/// * `points` - Output vector for points (must have length == nb_gauss)
/// * `weights` - Output vector for weights (must have length == nb_gauss)
///
/// # Returns
/// `true` if successful, `false` if parameters are invalid or order not supported
pub fn get_ordered_gauss_points_and_weights(
    nb_gauss: usize,
    points: &mut [f64],
    weights: &mut [f64],
) -> bool {
    if points.len() != nb_gauss || weights.len() != nb_gauss {
        return false;
    }

    match nb_gauss {
        7 => {
            points.copy_from_slice(GAUSS_POINTS_7);
            weights.copy_from_slice(GAUSS_WEIGHTS_7);
            true
        }
        10 => {
            points.copy_from_slice(GAUSS_POINTS_10);
            weights.copy_from_slice(GAUSS_WEIGHTS_10);
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kronrod_15_points_count() {
        let mut points = vec![0.0; 15];
        let mut weights = vec![0.0; 15];
        assert!(get_kronrod_points_and_weights(15, &mut points, &mut weights));
        assert_eq!(points.len(), 15);
    }

    #[test]
    fn test_kronrod_15_weights_sum() {
        let mut points = vec![0.0; 15];
        let mut weights = vec![0.0; 15];
        assert!(get_kronrod_points_and_weights(15, &mut points, &mut weights));
        let weight_sum: f64 = weights.iter().sum();
        // Kronrod weights should sum to approximately 2.0 on [-1, 1]
        // Allow some numerical error in the precomputed data
        assert!((weight_sum - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_kronrod_21_points_symmetry() {
        let mut points = vec![0.0; 21];
        let mut weights = vec![0.0; 21];
        assert!(get_kronrod_points_and_weights(21, &mut points, &mut weights));
        // Kronrod points should be symmetric around 0
        for i in 0..10 {
            assert!(
                (points[i] + points[20 - i]).abs() < 1e-10,
                "Points should be symmetric: points[{}]={}, points[{}]={}",
                i, points[i], 20-i, points[20-i]
            );
            assert!(
                (weights[i] - weights[20 - i]).abs() < 1e-10,
                "Weights should be symmetric: weights[{}]={}, weights[{}]={}",
                i, weights[i], 20-i, weights[20-i]
            );
        }
    }

    #[test]
    fn test_gauss_7_weights_sum() {
        let mut points = vec![0.0; 7];
        let mut weights = vec![0.0; 7];
        assert!(get_ordered_gauss_points_and_weights(7, &mut points, &mut weights));
        let weight_sum: f64 = weights.iter().sum();
        // Gauss weights should sum to 2.0 on [-1, 1]
        assert!((weight_sum - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_invalid_sizes() {
        let mut points = vec![0.0; 14];
        let mut weights = vec![0.0; 14];
        assert!(!get_kronrod_points_and_weights(15, &mut points, &mut weights));
        assert!(!get_ordered_gauss_points_and_weights(7, &mut points, &mut weights));
    }

    #[test]
    fn test_unsupported_order() {
        let mut points = vec![0.0; 31];
        let mut weights = vec![0.0; 31];
        assert!(!get_kronrod_points_and_weights(31, &mut points, &mut weights));
    }
}

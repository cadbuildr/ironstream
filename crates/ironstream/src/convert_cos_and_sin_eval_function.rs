// FILE: convert_cos_and_sin_eval_function.rs

// occt: Convert_CosAndSinEvalFunction

/// Represents a 2D point with x and y coordinates
#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
}

/// Function type for evaluating cos and sin representations used in rational curve parameterization.
///
/// # Parameters
/// - `param`: The parameter value (typically in [0, 1])
/// - `degree`: The degree of the curve
/// - `poles`: Array of control points (2D points, typically from a rational curve)
/// - `knots`: Array of knot values
/// - `mults`: Optional array of knot multiplicities (None if no multiplicities needed)
/// - `result`: Output array of length 2 containing [cos_value, sin_value]
///
/// # Notes
/// This is a function type definition that evaluates cos and sin basis functions
/// for rational Bezier/BSpline curves. The evaluation uses the control points and
/// knot structure to compute trigonometric basis evaluations.
pub type CosAndSinEvalFunction = fn(
    param: f64,
    degree: i32,
    poles: &[Point2D],
    knots: &[f64],
    mults: Option<&[i32]>,
    result: &mut [f64; 2],
);

/// A concrete implementation of the cos-sin evaluation function for rational curves.
///
/// This function evaluates the cosine and sine basis functions for a rational
/// curve parameterization, which is used in converting elementary surfaces
/// (cylinders, cones, spheres, tori) to BSpline surfaces.
///
/// # Algorithm
/// The function computes the trigonometric basis evaluation using:
/// - Bernstein basis polynomials (for Bezier representation)
/// - B-spline basis functions (when knots and multiplicities are provided)
/// - Rational weighting through the 2D control points
pub fn evaluate_cos_and_sin_basis(
    param: f64,
    degree: i32,
    poles: &[Point2D],
    knots: &[f64],
    mults: Option<&[i32]>,
    result: &mut [f64; 2],
) {
    // Initialize result
    result[0] = 0.0;
    result[1] = 0.0;

    if poles.is_empty() || degree < 0 {
        return;
    }

    let n = degree as usize;
    let num_poles = poles.len();

    // Check if we should use B-spline basis or Bernstein basis
    if knots.len() > n + 1 {
        // B-spline case with full knot vector
        evaluate_bspline_cos_sin(param, degree, poles, knots, mults, result);
    } else {
        // Bernstein basis case (Bezier curve)
        evaluate_bernstein_cos_sin(param, degree, poles, result);
    }
}

/// Evaluates using Bernstein basis (Bezier curve representation)
fn evaluate_bernstein_cos_sin(
    t: f64,
    degree: i32,
    poles: &[Point2D],
    result: &mut [f64; 2],
) {
    let n = degree as usize;
    let t_clamped = t.clamp(0.0, 1.0);
    let one_minus_t = 1.0 - t_clamped;

    // Compute Bernstein basis values
    let mut basis = vec![0.0; n + 1];
    basis[0] = 1.0;

    // Use recursion formula for Bernstein polynomials
    for i in 1..=n {
        let mut new_basis = vec![0.0; i + 1];
        for j in 0..=i {
            let left = if j > 0 {
                basis[j - 1] * t_clamped
            } else {
                0.0
            };
            let right = if j < i {
                basis[j] * one_minus_t
            } else {
                0.0
            };
            new_basis[j] = left + right;
        }
        basis = new_basis;
    }

    // Evaluate the weighted sum of control points
    let mut cos_sum = 0.0;
    let mut sin_sum = 0.0;
    let mut weight_sum = 0.0;

    for i in 0..=n {
        if i < poles.len() {
            // Poles are 2D points where x = cos(angle) and y = sin(angle)
            let b_i = basis[i];
            cos_sum += b_i * poles[i].x;
            sin_sum += b_i * poles[i].y;
            weight_sum += b_i;
        }
    }

    // Normalize by weight sum
    if weight_sum.abs() > 1e-15 {
        result[0] = cos_sum / weight_sum;
        result[1] = sin_sum / weight_sum;
    }
}

/// Evaluates using B-spline basis functions
fn evaluate_bspline_cos_sin(
    t: f64,
    degree: i32,
    poles: &[Point2D],
    knots: &[f64],
    mults: Option<&[i32]>,
    result: &mut [f64; 2],
) {
    let n = degree as usize;
    let t_clamped = t.clamp(0.0, 1.0);

    // Find the knot span containing t
    let span = find_knot_span(t_clamped, n, knots);

    // Compute B-spline basis functions
    let mut basis = vec![0.0; n + 1];
    compute_bspline_basis(span, t_clamped, n, knots, &mut basis);

    // Evaluate the weighted sum of control points
    let mut cos_sum = 0.0;
    let mut sin_sum = 0.0;
    let mut weight_sum = 0.0;

    let first_pole = if span >= n { span - n } else { 0 };
    let last_pole = (first_pole + n + 1).min(poles.len());

    for i in first_pole..last_pole {
        let basis_idx = i - first_pole;
        if basis_idx < basis.len() && i < poles.len() {
            let b_i = basis[basis_idx];
            cos_sum += b_i * poles[i].x;
            sin_sum += b_i * poles[i].y;
            weight_sum += b_i;
        }
    }

    // Normalize by weight sum
    if weight_sum.abs() > 1e-15 {
        result[0] = cos_sum / weight_sum;
        result[1] = sin_sum / weight_sum;
    }
}

/// Finds the knot span for parameter t
fn find_knot_span(t: f64, degree: usize, knots: &[f64]) -> usize {
    let n = knots.len() - degree - 2;

    if t >= knots[knots.len() - 1] {
        return n;
    }

    let mut low = degree;
    let mut high = knots.len() - 1;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if t < knots[mid] {
            high = mid;
        } else {
            low = mid;
        }
    }

    low
}

/// Computes B-spline basis functions for a given knot span
fn compute_bspline_basis(
    span: usize,
    t: f64,
    degree: usize,
    knots: &[f64],
    basis: &mut [f64],
) {
    if basis.len() < degree + 1 {
        return;
    }

    basis[0] = 1.0;

    for j in 1..=degree {
        let mut saved = 0.0;

        for r in 0..j {
            let den = knots[span + 1 + r] - knots[span + 1 + r - j];
            if den.abs() < 1e-15 {
                continue;
            }

            let coeff = basis[r] / den;
            basis[r] = saved + (knots[span + 1 + r] - t) * coeff;
            saved = (t - knots[span + 1 + r - j]) * coeff;
        }

        if j <= basis.len() - 1 {
            basis[j] = saved;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_creation() {
        let p = Point2D::new(0.6, 0.8);
        assert!((p.x - 0.6).abs() < 1e-10);
        assert!((p.y - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_bernstein_evaluation_linear() {
        // Linear Bezier (degree 1)
        let poles = vec![Point2D::new(1.0, 0.0), Point2D::new(0.0, 1.0)];

        let mut result = [0.0; 2];
        evaluate_bernstein_cos_sin(0.0, 1, &poles, &mut result);
        // At t=0, should be first pole
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[1] - 0.0).abs() < 1e-10);

        let mut result = [0.0; 2];
        evaluate_bernstein_cos_sin(1.0, 1, &poles, &mut result);
        // At t=1, should be second pole
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 1.0).abs() < 1e-10);

        let mut result = [0.0; 2];
        evaluate_bernstein_cos_sin(0.5, 1, &poles, &mut result);
        // At t=0.5, should be midpoint
        assert!((result[0] - 0.5).abs() < 1e-10);
        assert!((result[1] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_bernstein_evaluation_quadratic() {
        // Quadratic Bezier (degree 2)
        let poles = vec![
            Point2D::new(1.0, 0.0),
            Point2D::new(0.6, 0.8),
            Point2D::new(0.0, 1.0),
        ];

        let mut result = [0.0; 2];
        evaluate_bernstein_cos_sin(0.0, 2, &poles, &mut result);
        // At t=0, should be first pole
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[1] - 0.0).abs() < 1e-10);

        let mut result = [0.0; 2];
        evaluate_bernstein_cos_sin(1.0, 2, &poles, &mut result);
        // At t=1, should be third pole
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_bernstein_evaluation_clamping() {
        let poles = vec![Point2D::new(1.0, 0.0), Point2D::new(0.0, 1.0)];

        // Test with parameter outside [0, 1]
        let mut result = [0.0; 2];
        evaluate_bernstein_cos_sin(-0.5, 1, &poles, &mut result);
        // Should be clamped to 0, so first pole
        assert!((result[0] - 1.0).abs() < 1e-10);

        let mut result = [0.0; 2];
        evaluate_bernstein_cos_sin(1.5, 1, &poles, &mut result);
        // Should be clamped to 1, so second pole
        assert!((result[0] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_public_function_type() {
        // Test that the function type can be called
        let poles = vec![Point2D::new(1.0, 0.0), Point2D::new(0.0, 1.0)];
        let knots = vec![];
        let mut result = [0.0; 2];

        evaluate_cos_and_sin_basis(0.5, 1, &poles, &knots, None, &mut result);
        assert!((result[0] - 0.5).abs() < 1e-10);
        assert!((result[1] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_empty_poles() {
        let poles: Vec<Point2D> = vec![];
        let knots = vec![];
        let mut result = [0.0; 2];

        evaluate_cos_and_sin_basis(0.5, 1, &poles, &knots, None, &mut result);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
    }

    #[test]
    fn test_negative_degree() {
        let poles = vec![Point2D::new(1.0, 0.0)];
        let knots = vec![];
        let mut result = [0.0; 2];

        evaluate_cos_and_sin_basis(0.5, -1, &poles, &knots, None, &mut result);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
    }

    #[test]
    fn test_unit_circle_representation() {
        // Test with control points representing quarter circle
        let poles = vec![
            Point2D::new(1.0, 0.0),     // cos(0°), sin(0°)
            Point2D::new(0.707, 0.707), // cos(45°), sin(45°)
            Point2D::new(0.0, 1.0),     // cos(90°), sin(90°)
        ];

        let mut result = [0.0; 2];
        evaluate_bernstein_cos_sin(0.0, 2, &poles, &mut result);
        // At t=0, should be first point
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[1] - 0.0).abs() < 1e-10);

        let mut result = [0.0; 2];
        evaluate_bernstein_cos_sin(1.0, 2, &poles, &mut result);
        // At t=1, should be third point
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 1.0).abs() < 1e-10);
    }
}

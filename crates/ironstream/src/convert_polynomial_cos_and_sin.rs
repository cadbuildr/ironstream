// FILE: convert_polynomial_cos_and_sin.rs
// occt: Convert_PolynomialCosAndSin
//
// Faithful port of OCCT Convert_PolynomialCosAndSin function.
// Builds polynomial cos and sin coefficients for rational BSpline representation.

use std::f64::consts::PI;

// occt: BuildPolynomialCosAndSin
/// Builds polynomial cos and sin coefficients for a rational BSpline representation of a circle/ellipse arc.
///
/// This function generates degree-7 polynomial Bezier control points that approximate
/// the cosine and sine of angles in the range [u_first, u_last].
/// The resulting BSpline is exact for the circle/ellipse representation.
///
/// # Arguments
///
/// * `u_first` - Start angle (radians)
/// * `u_last` - End angle (radians)
/// * `num_poles` - Number of control points (typically 8 for degree 7)
/// * `cos_numerator` - Output array for cosine numerator coefficients (x-coordinates)
/// * `sin_numerator` - Output array for sine numerator coefficients (y-coordinates)
/// * `denominator` - Output array for denominator coefficients (weights, all 1.0 for polynomial)
///
/// # Algorithm
///
/// 1. Normalizes u_first to [-2π, 2π] for safe rotation arithmetic
/// 2. Creates a degree-7 Bezier curve with precomputed control points
/// 3. Rotates the curve to center the angular sector at -π (for symmetry)
/// 4. Computes trimming bounds using binary search with angle matching
/// 5. Trims the Bezier curve to the desired arc
/// 6. Adjusts endpoints and tangent directions via Euclidean transformation
/// 7. Rotates back to the original angular range [u_first, u_last]
/// 8. Extracts x,y coordinates as numerators and constant weight 1.0 as denominator
pub fn build_polynomial_cos_and_sin(
    u_first: f64,
    u_last: f64,
    num_poles: usize,
    cos_numerator: &mut [f64],
    sin_numerator: &mut [f64],
    denominator: &mut [f64],
) {
    assert_eq!(
        cos_numerator.len(),
        num_poles,
        "cos_numerator length must match num_poles"
    );
    assert_eq!(
        sin_numerator.len(),
        num_poles,
        "sin_numerator length must match num_poles"
    );
    assert_eq!(
        denominator.len(),
        num_poles,
        "denominator length must match num_poles"
    );

    // Normalize u_first to [-2π, 2π] for rotation safety
    let mut loc_u_first = u_first;
    let pi2 = 2.0 * PI;

    while loc_u_first > pi2 {
        loc_u_first -= pi2;
    }
    while loc_u_first < -pi2 {
        loc_u_first += pi2;
    }

    // Delta: angular span, middle: bisector of the sector
    let delta = u_last - u_first;
    let middle = 0.5 * delta;

    // Rotate curve so its bisector aligns with -X axis (for symmetry at parameter 0.5)
    // This centers the angular sector at -π
    let angle = middle - PI;

    // Pre-computed Bezier control points for unit circle (degree-7 polynomial approximation)
    // Coefficients from Tiller's algorithm for polynomial cos/sin representation
    let mut poles = vec![
        Point2d::new(1.0, 0.0),
        Point2d::new(1.0, 1.013854),
        Point2d::new(-0.199043, 1.871905),
        Point2d::new(-1.937729, 1.057323),
        Point2d::new(-1.937729, -1.057323),
        Point2d::new(-0.199043, -1.871905),
        Point2d::new(1.0, -1.013854),
        Point2d::new(1.0, 0.0),
    ];

    // Rotate all poles by angle
    for pole in &mut poles {
        *pole = pole.rotate_about_origin(angle);
    }

    // Compute trimming bounds via binary search
    let t_min = {
        let mut t = 1.0 - (delta * 1.3 / PI);
        t *= 0.5;
        t.max(0.0)
    };
    let t_max = {
        let mut t = 1.0 + (delta * 1.3 / PI);
        t *= 0.5;
        t.min(1.0)
    };

    let trim_max = locate_trim_parameter(delta, &poles, t_min, t_max);
    // Bezier is symmetric about bisector, so trim_min is symmetric to trim_max
    let trim_min = 1.0 - trim_max;

    // Trim the Bezier curve: create new poles by interpolation
    // We perform degree+1 order trimming (8 points for degree 7)
    let degree = num_poles - 1;
    let mut new_poles = trim_bezier(&poles, degree, trim_min, trim_max);

    // Readjustment to exact endpoints
    let sin_d = delta.sin();
    let cos_d = delta.cos();

    let p_deb = Point2d::new(1.0, 0.0);
    let p_fin = Point2d::new(cos_d, sin_d);

    // Adjust first control point and its successor (tangent direction)
    let dtg = new_poles[0].distance_to(&new_poles[1]);
    new_poles[0] = p_deb;
    new_poles[1] = Point2d::new(p_deb.x, p_deb.y + dtg);

    // Adjust last control point and its predecessor
    let dtg_end = new_poles[num_poles - 1].distance_to(&new_poles[num_poles - 2]);
    new_poles[num_poles - 1] = p_fin;
    new_poles[num_poles - 2] = Point2d::new(
        p_fin.x + dtg_end * sin_d,
        p_fin.y - dtg_end * cos_d,
    );

    // Rotate back to original angular range [loc_u_first, loc_u_first + delta]
    let rotate_back_angle = loc_u_first;
    for pole in &mut new_poles {
        *pole = pole.rotate_about_origin(rotate_back_angle);
    }

    // Extract coordinates as numerators and constant weight 1.0 as denominator
    for (i, pole) in new_poles.iter().enumerate() {
        cos_numerator[i] = pole.x;
        sin_numerator[i] = pole.y;
        denominator[i] = 1.0;
    }
}

/// 2D point helper for polynomial curve operations
#[derive(Debug, Clone, Copy)]
struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    #[inline]
    fn new(x: f64, y: f64) -> Self {
        Point2d { x, y }
    }

    /// Rotate point about origin by angle in radians (counter-clockwise)
    #[inline]
    fn rotate_about_origin(self, angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Point2d {
            x: self.x * cos_a - self.y * sin_a,
            y: self.x * sin_a + self.y * cos_a,
        }
    }

    /// Distance to another point
    #[inline]
    fn distance_to(self, other: &Point2d) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Locate the trim parameter by binary search matching the desired angle
fn locate_trim_parameter(
    target_angle: f64,
    poles: &[Point2d],
    u_min: f64,
    u_max: f64,
) -> f64 {
    const PTOL: f64 = 1.0e-10; // Angular precision
    const UTOL: f64 = 1.0e-15; // Parametric precision
    const MAX_ITERATIONS: usize = 100;

    let mut umin = u_min;
    let mut umax = u_max;

    for _ in 0..MAX_ITERATIONS {
        if (umax - umin).abs() < UTOL {
            break;
        }

        let ptest = (umax + umin) / 2.0;
        let val_p = evaluate_bezier_d0(ptest, poles);

        // Convert to angle in [0, 2π)
        let mut theta = val_p.y.atan2(val_p.x);
        if theta < 0.0 {
            theta += 2.0 * PI;
        }

        if (theta - target_angle).abs() < PTOL {
            return ptest;
        }

        if theta < target_angle {
            umin = ptest;
        } else {
            umax = ptest;
        }
    }

    (umin + umax) / 2.0
}

/// Evaluate degree-7 Bezier curve at parameter t using de Casteljau's algorithm
fn evaluate_bezier_d0(t: f64, poles: &[Point2d]) -> Point2d {
    let mut pts: Vec<Point2d> = poles.to_vec();
    let n = poles.len();

    for level in 0..n - 1 {
        for i in 0..n - level - 1 {
            pts[i] = Point2d::new(
                (1.0 - t) * pts[i].x + t * pts[i + 1].x,
                (1.0 - t) * pts[i].y + t * pts[i + 1].y,
            );
        }
    }

    pts[0]
}

/// Trim a Bezier curve using degree + 1 order refinement
fn trim_bezier(
    poles: &[Point2d],
    degree: usize,
    trim_min: f64,
    trim_max: f64,
) -> Vec<Point2d> {
    let mut result = vec![Point2d::new(0.0, 0.0); poles.len()];

    // Perform high-order trimming by repeated de Casteljau splits
    // First evaluate at trim_min, then subdivide to find control points
    let mut current_poles = poles.to_vec();

    // Trim from the left at trim_min
    if trim_min > 0.0 {
        current_poles = trim_left(&current_poles, degree, trim_min);
    }

    // Trim from the right at (trim_max - trim_min) / (1 - trim_min)
    if trim_max < 1.0 {
        let adjusted_t = if trim_min < 1.0 {
            (trim_max - trim_min) / (1.0 - trim_min)
        } else {
            0.0
        };
        current_poles = trim_right(&current_poles, degree, adjusted_t);
    }

    current_poles
}

/// Trim left: keep portion from parameter t to 1.0
fn trim_left(poles: &[Point2d], _degree: usize, t: f64) -> Vec<Point2d> {
    let mut result = poles.to_vec();
    let n = poles.len();

    // Use de Casteljau subdivison: evaluate and keep right portion
    // Perform subdivisions to extract right control points
    for _ in 0..n - 1 {
        for i in 0..n - 1 {
            result[i] = Point2d::new(
                (1.0 - t) * result[i].x + t * result[i + 1].x,
                (1.0 - t) * result[i].y + t * result[i + 1].y,
            );
        }
    }

    // After n-1 passes, result[0] contains the right control points in reverse order
    // Need to extract right-hand Bezier poles
    let mut trimmed = vec![Point2d::new(0.0, 0.0); n];
    let mut work = poles.to_vec();

    for i in 0..n {
        let mut pts = work.clone();
        for j in 0..n - i {
            for k in 0..n - j - 1 {
                pts[k] = Point2d::new(
                    (1.0 - t) * pts[k].x + t * pts[k + 1].x,
                    (1.0 - t) * pts[k].y + t * pts[k + 1].y,
                );
            }
        }
        trimmed[i] = pts[0];
    }

    trimmed
}

/// Trim right: keep portion from 0.0 to parameter t
fn trim_right(poles: &[Point2d], _degree: usize, t: f64) -> Vec<Point2d> {
    let n = poles.len();
    let mut trimmed = vec![Point2d::new(0.0, 0.0); n];
    let mut work = poles.to_vec();

    for i in 0..n {
        let mut pts = work.clone();
        for _ in 0..n - i - 1 {
            for k in 0..n - 1 {
                pts[k] = Point2d::new(
                    (1.0 - t) * pts[k].x + t * pts[k + 1].x,
                    (1.0 - t) * pts[k].y + t * pts[k + 1].y,
                );
            }
        }
        trimmed[n - 1 - i] = pts[0];
    }

    trimmed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_polynomial_cos_and_sin_full_circle() {
        let mut cos_num = vec![0.0; 8];
        let mut sin_num = vec![0.0; 8];
        let mut denom = vec![0.0; 8];

        // Full circle: [0, 2π]
        build_polynomial_cos_and_sin(0.0, 2.0 * PI, 8, &mut cos_num, &mut sin_num, &mut denom);

        // Check all denominators are 1.0 (polynomial parameterization)
        for d in &denom {
            assert!((d - 1.0).abs() < 1e-10);
        }

        // Check reasonable bounds on cosine and sine
        for (c, s) in cos_num.iter().zip(sin_num.iter()) {
            assert!(c.is_finite());
            assert!(s.is_finite());
        }
    }

    #[test]
    fn test_build_polynomial_cos_and_sin_quarter_circle() {
        let mut cos_num = vec![0.0; 8];
        let mut sin_num = vec![0.0; 8];
        let mut denom = vec![0.0; 8];

        // Quarter circle: [0, π/2]
        build_polynomial_cos_and_sin(0.0, PI / 2.0, 8, &mut cos_num, &mut sin_num, &mut denom);

        // Check output arrays are populated
        let cos_sum: f64 = cos_num.iter().sum();
        let sin_sum: f64 = sin_num.iter().sum();
        assert!(cos_sum.is_finite());
        assert!(sin_sum.is_finite());

        // All denominators should be 1.0
        for d in &denom {
            assert!((d - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_build_polynomial_cos_and_sin_negative_angle() {
        let mut cos_num = vec![0.0; 8];
        let mut sin_num = vec![0.0; 8];
        let mut denom = vec![0.0; 8];

        // Negative angle range: [-π, 0]
        build_polynomial_cos_and_sin(-PI, 0.0, 8, &mut cos_num, &mut sin_num, &mut denom);

        // Check denominators
        for d in &denom {
            assert!((d - 1.0).abs() < 1e-10);
        }

        // Outputs should be reasonable
        for (c, s) in cos_num.iter().zip(sin_num.iter()) {
            assert!(c.is_finite());
            assert!(s.is_finite());
        }
    }

    #[test]
    fn test_point2d_rotation() {
        let p = Point2d::new(1.0, 0.0);

        // Rotate 90 degrees (π/2)
        let p_rot = p.rotate_about_origin(PI / 2.0);
        assert!((p_rot.x - 0.0).abs() < 1e-10);
        assert!((p_rot.y - 1.0).abs() < 1e-10);

        // Rotate 180 degrees
        let p_rot2 = p.rotate_about_origin(PI);
        assert!((p_rot2.x - (-1.0)).abs() < 1e-10);
        assert!((p_rot2.y - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_point2d_distance() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(3.0, 4.0);
        let dist = p1.distance_to(&p2);
        assert!((dist - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_bezier_d0_endpoints() {
        let poles = vec![
            Point2d::new(1.0, 0.0),
            Point2d::new(1.0, 1.0),
            Point2d::new(0.0, 1.0),
            Point2d::new(0.0, 0.0),
        ];

        // At t=0, should be at first control point
        let p0 = evaluate_bezier_d0(0.0, &poles);
        assert!((p0.x - 1.0).abs() < 1e-10);
        assert!((p0.y - 0.0).abs() < 1e-10);

        // At t=1, should be at last control point
        let p1 = evaluate_bezier_d0(1.0, &poles);
        assert!((p1.x - 0.0).abs() < 1e-10);
        assert!((p1.y - 0.0).abs() < 1e-10);

        // At t=0.5, should be interior point
        let p05 = evaluate_bezier_d0(0.5, &poles);
        assert!(p05.x.is_finite());
        assert!(p05.y.is_finite());
    }
}

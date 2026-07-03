// FILE: cs_lib_class2d.rs

// occt: CSLib_Class2d
//
// Low-level algorithm for 2D point-in-polygon classification using ray-casting.
// Determines whether a 2D point lies inside, outside, or on the boundary of a closed polygon.
// The polygon is internally normalized to [0,1] x [0,1] domain for numerical stability.

/// Minimum range threshold for normalization.
const MINIMUM_RANGE: f64 = 1e-10;

/// Standard precision confusion for numerical comparisons.
const PRECISION_CONFUSION: f64 = 1e-7;

/// Classification result for point-in-polygon tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassificationResult {
    /// Point is strictly inside the polygon
    Inside = 1,
    /// Point is strictly outside the polygon
    Outside = -1,
    /// Point is on boundary or classification is uncertain
    Uncertain = 0,
}

impl ClassificationResult {
    pub fn as_i32(self) -> i32 {
        match self {
            ClassificationResult::Inside => 1,
            ClassificationResult::Outside => -1,
            ClassificationResult::Uncertain => 0,
        }
    }
}

/// Represents a 2D point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pnt2d {
    pub x: f64,
    pub y: f64,
}

impl Pnt2d {
    pub fn new(x: f64, y: f64) -> Self {
        Pnt2d { x, y }
    }
}

/// Low-level algorithm for 2D point-in-polygon classification.
///
/// This class determines whether a 2D point lies inside, outside, or on the boundary
/// of a closed polygon. It uses a ray-casting algorithm where a horizontal ray
/// from the test point is extended to infinity, and the number of polygon edge
/// crossings determines the classification.
///
/// The polygon is internally normalized to [0,1] x [0,1] domain for numerical stability.
#[derive(Debug)]
pub struct CslibClass2d {
    /// X coordinates (normalized)
    pnts_2d_x: Vec<f64>,
    /// Y coordinates (normalized)
    pnts_2d_y: Vec<f64>,
    /// Tolerance in U direction (normalized)
    tol_u: f64,
    /// Tolerance in V direction (normalized)
    tol_v: f64,
    /// Number of polygon vertices
    points_count: usize,
    /// Original minimum U bound
    u_min: f64,
    /// Original minimum V bound
    v_min: f64,
    /// Original maximum U bound
    u_max: f64,
    /// Original maximum V bound
    v_max: f64,
}

impl CslibClass2d {
    /// Default constructor. Creates an empty classifier.
    pub fn new() -> Self {
        CslibClass2d {
            pnts_2d_x: Vec::new(),
            pnts_2d_y: Vec::new(),
            tol_u: 0.0,
            tol_v: 0.0,
            points_count: 0,
            u_min: 0.0,
            v_min: 0.0,
            u_max: 0.0,
            v_max: 0.0,
        }
    }

    /// Constructs a 2D classifier from a slice of polygon vertices.
    ///
    /// The polygon is automatically closed (no need to repeat the first point at the end).
    /// Points are normalized internally to the UV bounds for numerical stability.
    ///
    /// # Arguments
    /// * `the_pnts_2d` - Slice of polygon vertices (minimum 3 points required)
    /// * `the_tol_u` - Tolerance in U direction for boundary detection
    /// * `the_tol_v` - Tolerance in V direction for boundary detection
    /// * `the_u_min` - Minimum U bound of the polygon domain
    /// * `the_v_min` - Minimum V bound of the polygon domain
    /// * `the_u_max` - Maximum U bound of the polygon domain
    /// * `the_v_max` - Maximum V bound of the polygon domain
    pub fn from_slice(
        the_pnts_2d: &[Pnt2d],
        the_tol_u: f64,
        the_tol_v: f64,
        the_u_min: f64,
        the_v_min: f64,
        the_u_max: f64,
        the_v_max: f64,
    ) -> Self {
        let mut classifier = CslibClass2d::new();
        classifier.init(
            the_pnts_2d,
            the_tol_u,
            the_tol_v,
            the_u_min,
            the_v_min,
            the_u_max,
            the_v_max,
        );
        classifier
    }

    /// Internal initialization.
    fn init(
        &mut self,
        the_pnts_2d: &[Pnt2d],
        the_tol_u: f64,
        the_tol_v: f64,
        the_u_min: f64,
        the_v_min: f64,
        the_u_max: f64,
        the_v_max: f64,
    ) {
        self.u_min = the_u_min;
        self.v_min = the_v_min;
        self.u_max = the_u_max;
        self.v_max = the_v_max;

        // Validate input parameters.
        if the_u_max <= the_u_min || the_v_max <= the_v_min || the_pnts_2d.len() < 3 {
            self.points_count = 0;
            return;
        }

        self.points_count = the_pnts_2d.len();
        self.tol_u = the_tol_u;
        self.tol_v = the_tol_v;

        // Allocate arrays with one extra element for closing the polygon.
        self.pnts_2d_x.resize(self.points_count + 1, 0.0);
        self.pnts_2d_y.resize(self.points_count + 1, 0.0);

        let a_du = the_u_max - the_u_min;
        let a_dv = the_v_max - the_v_min;

        // Transform points to normalized coordinates.
        for i in 0..self.points_count {
            let p2d = &the_pnts_2d[i];
            self.pnts_2d_x[i] = transform_to_normalized(p2d.x, the_u_min, a_du);
            self.pnts_2d_y[i] = transform_to_normalized(p2d.y, the_v_min, a_dv);
        }

        // Close the polygon by copying first point to last position.
        self.pnts_2d_x[self.points_count] = self.pnts_2d_x[0];
        self.pnts_2d_y[self.points_count] = self.pnts_2d_y[0];

        // Normalize tolerances.
        if a_du > MINIMUM_RANGE {
            self.tol_u /= a_du;
        }
        if a_dv > MINIMUM_RANGE {
            self.tol_v /= a_dv;
        }
    }

    /// Classifies a point relative to the polygon.
    ///
    /// # Arguments
    /// * `the_point` - The 2D point to classify
    ///
    /// # Returns
    /// Classification result (Inside, Outside, or Uncertain)
    pub fn si_dans(&self, the_point: &Pnt2d) -> ClassificationResult {
        if self.points_count == 0 {
            return ClassificationResult::Uncertain;
        }

        let mut a_x = the_point.x;
        let mut a_y = the_point.y;

        // Compute tolerance in original coordinate space.
        let a_tol_u = self.tol_u * (self.u_max - self.u_min);
        let a_tol_v = self.tol_v * (self.v_max - self.v_min);

        // Quick rejection test for points clearly outside the bounding box.
        if a_x < (self.u_min - a_tol_u)
            || a_x > (self.u_max + a_tol_u)
            || a_y < (self.v_min - a_tol_v)
            || a_y > (self.v_max + a_tol_v)
        {
            return ClassificationResult::Outside;
        }

        // Transform to normalized coordinates.
        a_x = transform_to_normalized(a_x, self.u_min, self.u_max - self.u_min);
        a_y = transform_to_normalized(a_y, self.v_min, self.v_max - self.v_min);

        // Perform classification with ON detection.
        let a_result = self.internal_si_dans_ou_on(a_x, a_y);
        if a_result == ClassificationResult::Uncertain {
            return ClassificationResult::Uncertain; // ON boundary
        }

        // Check corner points with tolerance for boundary detection.
        if self.tol_u > 0.0 || self.tol_v > 0.0 {
            let is_inside = a_result == ClassificationResult::Inside;
            if is_inside != self.internal_si_dans(a_x - self.tol_u, a_y - self.tol_v)
                || is_inside != self.internal_si_dans(a_x + self.tol_u, a_y - self.tol_v)
                || is_inside != self.internal_si_dans(a_x - self.tol_u, a_y + self.tol_v)
                || is_inside != self.internal_si_dans(a_x + self.tol_u, a_y + self.tol_v)
            {
                return ClassificationResult::Uncertain; // Near boundary
            }
        }

        a_result
    }

    /// Classifies a point with explicit ON tolerance.
    ///
    /// Similar to si_dans() but uses the specified tolerance for boundary detection
    /// instead of the tolerances specified at construction.
    ///
    /// # Arguments
    /// * `the_point` - The 2D point to classify
    /// * `the_tol` - Tolerance for boundary detection
    ///
    /// # Returns
    /// Classification result (Inside, Outside, or Uncertain)
    pub fn si_dans_on_mode(&self, the_point: &Pnt2d, the_tol: f64) -> ClassificationResult {
        if self.points_count == 0 {
            return ClassificationResult::Uncertain;
        }

        let mut a_x = the_point.x;
        let mut a_y = the_point.y;

        // Quick rejection test.
        if a_x < (self.u_min - the_tol)
            || a_x > (self.u_max + the_tol)
            || a_y < (self.v_min - the_tol)
            || a_y > (self.v_max + the_tol)
        {
            return ClassificationResult::Outside;
        }

        // Transform to normalized coordinates.
        a_x = transform_to_normalized(a_x, self.u_min, self.u_max - self.u_min);
        a_y = transform_to_normalized(a_y, self.v_min, self.v_max - self.v_min);

        // Perform classification with ON detection.
        let a_result = self.internal_si_dans_ou_on(a_x, a_y);

        // Check corner points with tolerance.
        if the_tol > 0.0 {
            let is_inside = a_result == ClassificationResult::Inside;
            if is_inside != self.internal_si_dans(a_x - the_tol, a_y - the_tol)
                || is_inside != self.internal_si_dans(a_x + the_tol, a_y - the_tol)
                || is_inside != self.internal_si_dans(a_x - the_tol, a_y + the_tol)
                || is_inside != self.internal_si_dans(a_x + the_tol, a_y + the_tol)
            {
                return ClassificationResult::Uncertain;
            }
        }

        a_result
    }

    /// Internal classification in normalized coordinates using ray-casting algorithm.
    ///
    /// The point coordinates should be in the normalized [0,1] domain.
    /// Returns true if inside, false if outside.
    fn internal_si_dans(&self, the_px: f64, the_py: f64) -> bool {
        // Ray-casting algorithm: count edge crossings with a horizontal ray from (Px, Py) to +infinity.
        let mut a_nb_crossings = 0;

        let mut a_prev_dx = self.pnts_2d_x[0] - the_px;
        let mut a_prev_dy = self.pnts_2d_y[0] - the_py;
        let mut a_prev_y_is_negative = a_prev_dy < 0.0;

        for a_next_idx in 1..=self.points_count {
            let a_curr_dx = self.pnts_2d_x[a_next_idx] - the_px;
            let a_curr_dy = self.pnts_2d_y[a_next_idx] - the_py;
            let a_curr_y_is_negative = a_curr_dy < 0.0;

            // Check for edge crossing when Y changes sign.
            if a_curr_y_is_negative != a_prev_y_is_negative {
                if a_prev_dx > 0.0 && a_curr_dx > 0.0 {
                    // Both endpoints are to the right of the test point.
                    a_nb_crossings += 1;
                } else if a_prev_dx > 0.0 || a_curr_dx > 0.0 {
                    // Compute X intersection with horizontal line Y = 0.
                    let a_x_intersect =
                        a_prev_dx - a_prev_dy * (a_curr_dx - a_prev_dx) / (a_curr_dy - a_prev_dy);
                    if a_x_intersect > 0.0 {
                        a_nb_crossings += 1;
                    }
                }
                a_prev_y_is_negative = a_curr_y_is_negative;
            }

            a_prev_dx = a_curr_dx;
            a_prev_dy = a_curr_dy;
        }

        // Odd number of crossings means inside.
        (a_nb_crossings & 1) != 0
    }

    /// Internal classification with ON detection using ray-casting algorithm.
    ///
    /// Same as internal_si_dans() but also detects if the point lies on the boundary.
    fn internal_si_dans_ou_on(&self, the_px: f64, the_py: f64) -> ClassificationResult {
        // Ray-casting algorithm with ON detection.
        let mut a_nb_crossings = 0;

        let mut a_prev_dx = self.pnts_2d_x[0] - the_px;
        let mut a_prev_dy = self.pnts_2d_y[0] - the_py;
        let mut a_prev_y_is_negative = a_prev_dy < 0.0;

        for a_next_idx in 1..=self.points_count {
            let a_prev_idx = a_next_idx - 1;
            let a_curr_dx = self.pnts_2d_x[a_next_idx] - the_px;
            let a_curr_dy = self.pnts_2d_y[a_next_idx] - the_py;

            // Check if point is very close to current vertex.
            if a_curr_dx < self.tol_u
                && a_curr_dx > -self.tol_u
                && a_curr_dy < self.tol_v
                && a_curr_dy > -self.tol_v
            {
                return ClassificationResult::Uncertain; // ON boundary (at vertex)
            }

            // Check if point is ON the edge by computing Y at the test point's X.
            // Skip interpolation for nearly vertical edges to avoid division instability.
            // For vertical edges, the ON detection is handled by the tolerance check above.
            let a_edge_dx = self.pnts_2d_x[a_next_idx] - self.pnts_2d_x[a_prev_idx];
            if (self.pnts_2d_x[a_prev_idx] - the_px) * a_curr_dx < 0.0
                && a_edge_dx.abs() > PRECISION_CONFUSION
            {
                let a_interp_y = self.pnts_2d_y[a_next_idx]
                    - (self.pnts_2d_y[a_next_idx] - self.pnts_2d_y[a_prev_idx]) / a_edge_dx
                        * a_curr_dx;
                let a_delta_y = a_interp_y - the_py;
                if a_delta_y >= -self.tol_v && a_delta_y <= self.tol_v {
                    return ClassificationResult::Uncertain; // ON boundary (on edge)
                }
            }

            let a_curr_y_is_negative = a_curr_dy < 0.0;
            if a_curr_y_is_negative != a_prev_y_is_negative {
                if a_prev_dx > 0.0 && a_curr_dx > 0.0 {
                    a_nb_crossings += 1;
                } else if a_prev_dx > 0.0 || a_curr_dx > 0.0 {
                    let a_x_intersect =
                        a_prev_dx - a_prev_dy * (a_curr_dx - a_prev_dx) / (a_curr_dy - a_prev_dy);
                    if a_x_intersect > 0.0 {
                        a_nb_crossings += 1;
                    }
                }
                a_prev_y_is_negative = a_curr_y_is_negative;
            }

            a_prev_dx = a_curr_dx;
            a_prev_dy = a_curr_dy;
        }

        // Odd number of crossings means inside.
        if (a_nb_crossings & 1) != 0 {
            ClassificationResult::Inside
        } else {
            ClassificationResult::Outside
        }
    }
}

impl Default for CslibClass2d {
    fn default() -> Self {
        CslibClass2d::new()
    }
}

/// Transforms a coordinate from original space to normalized [0,1] space.
fn transform_to_normalized(the_u: f64, the_u_min: f64, the_u_range: f64) -> f64 {
    if the_u_range > MINIMUM_RANGE {
        (the_u - the_u_min) / the_u_range
    } else {
        the_u
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_classifier() {
        let classifier = CslibClass2d::new();
        let point = Pnt2d::new(0.5, 0.5);
        assert_eq!(classifier.si_dans(&point), ClassificationResult::Uncertain);
    }

    #[test]
    fn test_simple_square_inside() {
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(10.0, 0.0),
            Pnt2d::new(10.0, 10.0),
            Pnt2d::new(0.0, 10.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.01, 0.01, 0.0, 0.0, 10.0, 10.0);

        // Test point inside the square
        let inside_point = Pnt2d::new(5.0, 5.0);
        assert_eq!(
            classifier.si_dans(&inside_point),
            ClassificationResult::Inside
        );
    }

    #[test]
    fn test_simple_square_outside() {
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(10.0, 0.0),
            Pnt2d::new(10.0, 10.0),
            Pnt2d::new(0.0, 10.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.01, 0.01, 0.0, 0.0, 10.0, 10.0);

        // Test point outside the square
        let outside_point = Pnt2d::new(15.0, 15.0);
        assert_eq!(
            classifier.si_dans(&outside_point),
            ClassificationResult::Outside
        );
    }

    #[test]
    fn test_simple_square_far_outside() {
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(10.0, 0.0),
            Pnt2d::new(10.0, 10.0),
            Pnt2d::new(0.0, 10.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.0, 0.0, 0.0, 0.0, 10.0, 10.0);

        // Test point far outside
        let far_outside = Pnt2d::new(-50.0, -50.0);
        assert_eq!(
            classifier.si_dans(&far_outside),
            ClassificationResult::Outside
        );
    }

    #[test]
    fn test_triangle_inside() {
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(10.0, 0.0),
            Pnt2d::new(5.0, 10.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.0, 0.0, 0.0, 0.0, 10.0, 10.0);

        // Test point inside the triangle
        let inside_point = Pnt2d::new(5.0, 3.0);
        assert_eq!(
            classifier.si_dans(&inside_point),
            ClassificationResult::Inside
        );
    }

    #[test]
    fn test_triangle_outside() {
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(10.0, 0.0),
            Pnt2d::new(5.0, 10.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.0, 0.0, 0.0, 0.0, 10.0, 10.0);

        // Test point outside the triangle
        let outside_point = Pnt2d::new(1.0, 5.0);
        assert_eq!(
            classifier.si_dans(&outside_point),
            ClassificationResult::Outside
        );
    }

    #[test]
    fn test_quick_rejection_above_bbox() {
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(10.0, 0.0),
            Pnt2d::new(10.0, 10.0),
            Pnt2d::new(0.0, 10.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.0, 0.0, 0.0, 0.0, 10.0, 10.0);

        // Test point far above bounding box (should be quickly rejected)
        let above = Pnt2d::new(5.0, 100.0);
        assert_eq!(classifier.si_dans(&above), ClassificationResult::Outside);
    }

    #[test]
    fn test_quick_rejection_left_of_bbox() {
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(10.0, 0.0),
            Pnt2d::new(10.0, 10.0),
            Pnt2d::new(0.0, 10.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.0, 0.0, 0.0, 0.0, 10.0, 10.0);

        // Test point far left of bounding box
        let left = Pnt2d::new(-100.0, 5.0);
        assert_eq!(classifier.si_dans(&left), ClassificationResult::Outside);
    }

    #[test]
    fn test_si_dans_on_mode_inside() {
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(10.0, 0.0),
            Pnt2d::new(10.0, 10.0),
            Pnt2d::new(0.0, 10.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.01, 0.01, 0.0, 0.0, 10.0, 10.0);

        let inside = Pnt2d::new(5.0, 5.0);
        assert_eq!(
            classifier.si_dans_on_mode(&inside, 0.1),
            ClassificationResult::Inside
        );
    }

    #[test]
    fn test_si_dans_on_mode_outside() {
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(10.0, 0.0),
            Pnt2d::new(10.0, 10.0),
            Pnt2d::new(0.0, 10.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.01, 0.01, 0.0, 0.0, 10.0, 10.0);

        let outside = Pnt2d::new(20.0, 20.0);
        assert_eq!(
            classifier.si_dans_on_mode(&outside, 0.1),
            ClassificationResult::Outside
        );
    }

    #[test]
    fn test_pnt2d_creation() {
        let p1 = Pnt2d::new(3.5, 4.2);
        assert_eq!(p1.x, 3.5);
        assert_eq!(p1.y, 4.2);
    }

    #[test]
    fn test_pnt2d_equality() {
        let p1 = Pnt2d::new(1.0, 2.0);
        let p2 = Pnt2d::new(1.0, 2.0);
        let p3 = Pnt2d::new(1.0, 3.0);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_classification_result_inside_value() {
        assert_eq!(ClassificationResult::Inside.as_i32(), 1);
    }

    #[test]
    fn test_classification_result_outside_value() {
        assert_eq!(ClassificationResult::Outside.as_i32(), -1);
    }

    #[test]
    fn test_classification_result_uncertain_value() {
        assert_eq!(ClassificationResult::Uncertain.as_i32(), 0);
    }

    #[test]
    fn test_complex_polygon() {
        // Pentagon
        let vertices = vec![
            Pnt2d::new(5.0, 0.0),
            Pnt2d::new(10.0, 3.0),
            Pnt2d::new(8.0, 8.0),
            Pnt2d::new(2.0, 8.0),
            Pnt2d::new(0.0, 3.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.0, 0.0, 0.0, 0.0, 10.0, 10.0);

        // Test point at center (should be inside)
        let center = Pnt2d::new(5.0, 4.5);
        assert_eq!(classifier.si_dans(&center), ClassificationResult::Inside);
    }

    #[test]
    fn test_invalid_bounds_returns_uncertain() {
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(10.0, 0.0),
            Pnt2d::new(10.0, 10.0),
        ];

        // Create classifier with invalid bounds (u_max <= u_min)
        let classifier = CslibClass2d::from_slice(&vertices, 0.0, 0.0, 10.0, 0.0, 5.0, 10.0); // invalid

        let point = Pnt2d::new(5.0, 5.0);
        assert_eq!(classifier.si_dans(&point), ClassificationResult::Uncertain);
    }

    #[test]
    fn test_too_few_points_returns_uncertain() {
        let vertices = vec![Pnt2d::new(0.0, 0.0), Pnt2d::new(10.0, 0.0)]; // Only 2 points

        let classifier = CslibClass2d::from_slice(&vertices, 0.0, 0.0, 0.0, 0.0, 10.0, 10.0);

        let point = Pnt2d::new(5.0, 5.0);
        assert_eq!(classifier.si_dans(&point), ClassificationResult::Uncertain);
    }

    #[test]
    fn test_transform_to_normalized() {
        let normalized = transform_to_normalized(5.0, 0.0, 10.0);
        assert!((normalized - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_transform_to_normalized_zero_range() {
        let normalized = transform_to_normalized(5.0, 5.0, 1e-11); // Range too small
        assert_eq!(normalized, 5.0); // Should return original value
    }

    #[test]
    fn test_ray_casting_simple_case() {
        // Unit square
        let vertices = vec![
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(0.0, 1.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0);

        // Multiple test points inside
        for x in [0.1, 0.25, 0.5, 0.75, 0.9] {
            for y in [0.1, 0.25, 0.5, 0.75, 0.9] {
                let p = Pnt2d::new(x, y);
                assert_eq!(
                    classifier.si_dans(&p),
                    ClassificationResult::Inside,
                    "Point ({}, {}) should be inside",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_rectangle_asymmetric() {
        // Rectangle from (1, 2) to (5, 8)
        let vertices = vec![
            Pnt2d::new(1.0, 2.0),
            Pnt2d::new(5.0, 2.0),
            Pnt2d::new(5.0, 8.0),
            Pnt2d::new(1.0, 8.0),
        ];

        let classifier = CslibClass2d::from_slice(&vertices, 0.0, 0.0, 1.0, 2.0, 5.0, 8.0);

        let inside = Pnt2d::new(3.0, 5.0);
        let outside_left = Pnt2d::new(0.0, 5.0);
        let outside_right = Pnt2d::new(6.0, 5.0);

        assert_eq!(classifier.si_dans(&inside), ClassificationResult::Inside);
        assert_eq!(
            classifier.si_dans(&outside_left),
            ClassificationResult::Outside
        );
        assert_eq!(
            classifier.si_dans(&outside_right),
            ClassificationResult::Outside
        );
    }
}

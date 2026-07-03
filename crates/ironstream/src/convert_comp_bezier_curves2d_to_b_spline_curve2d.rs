// FILE: convert_comp_bezier_curves2d_to_b_spline_curve2d.rs
// occt: Convert_CompBezierCurves2dToBSplineCurve2d

//! Converts a list of connecting Bezier Curves 2d to a BSplineCurve 2d.
//! If possible, the continuity of the BSpline will be increased to more than C0.

/// Represents a 2D point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Point2d { x, y }
    }
}

/// Represents a 2D vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    /// Creates a vector from point p1 to point p2
    pub fn from_points(p1: Point2d, p2: Point2d) -> Self {
        Vec2d {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
        }
    }

    /// Returns the squared magnitude of the vector
    pub fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        self.square_magnitude().sqrt()
    }

    /// Checks if this vector is parallel to another within an angular tolerance
    /// angular_tolerance in radians
    pub fn is_parallel(&self, other: &Vec2d, angular_tolerance: f64) -> bool {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();

        if mag1 <= 1e-20 || mag2 <= 1e-20 {
            return false;
        }

        // Compute cross product (in 2D, returns the z-component)
        let cross = self.x * other.y - self.y * other.x;

        // Compute dot product
        let dot = self.x * other.x + self.y * other.y;

        // For parallel vectors: |cross| / (mag1 * mag2) should be near 0
        // Use arcsin: asin(cross / (mag1*mag2)) < angular_tolerance
        // At small angles: sin(theta) ≈ theta, so |cross| / (mag1*mag2) < sin(angular_tolerance)
        let sin_angular_tol = angular_tolerance.sin();

        (cross.abs() / (mag1 * mag2)).abs() < sin_angular_tol && dot > 0.0
    }
}

/// Template base class for converting a sequence of adjacent non-rational
/// Bezier curves into a BSpline curve.
pub struct CompBezierCurves2dToBSplineCurve2d {
    angular_tolerance: f64,
    degree: i32,
    sequence: Vec<Vec<Point2d>>,
    curve_poles: Vec<Point2d>,
    curve_knots: Vec<f64>,
    knots_mults: Vec<i32>,
}

impl CompBezierCurves2dToBSplineCurve2d {
    /// Constructs a framework for converting a sequence of
    /// adjacent non-rational Bezier curves into a BSpline curve.
    ///
    /// # Arguments
    /// * `angular_tolerance` - angular tolerance in radians for checking tangent parallelism at junction points
    pub fn new(angular_tolerance: f64) -> Self {
        CompBezierCurves2dToBSplineCurve2d {
            angular_tolerance,
            degree: 0,
            sequence: Vec::new(),
            curve_poles: Vec::new(),
            curve_knots: Vec::new(),
            knots_mults: Vec::new(),
        }
    }

    /// Adds the Bezier curve defined by the table of poles to
    /// the sequence of adjacent Bezier curves to be converted.
    pub fn add_curve(&mut self, poles: Vec<Point2d>) {
        self.sequence.push(poles);
    }

    /// Computes all the data needed to build a BSpline curve
    /// equivalent to the adjacent Bezier curve sequence.
    pub fn perform(&mut self) {
        self.curve_poles.clear();
        self.curve_knots.clear();
        self.knots_mults.clear();

        if self.sequence.is_empty() {
            return;
        }

        let lower_i = 0;
        let upper_i = self.sequence.len() - 1;
        let nbr_curv = upper_i - lower_i + 1;
        let mut curve_knots_vals = vec![0.0; nbr_curv];

        // Find maximum degree
        self.degree = 0;
        for curve in &self.sequence {
            self.degree = self.degree.max((curve.len() - 1) as i32);
        }

        let max_degree = self.degree;
        let mut det = 0.0;
        let mut p1 = Point2d::new(0.0, 0.0);

        for i in lower_i..=upper_i {
            let curve_i = &self.sequence[i];
            let deg = (curve_i.len() - 1) as i32;
            let inc = max_degree - deg;

            let mut points = curve_i.clone();

            // 1- Raise the Bezier curve to the maximum degree if needed.
            if inc > 0 {
                points = self.increase_degree_bezier(&points, max_degree);
            }

            // 2- Process the node of junction between 2 Bezier curves.
            if i == lower_i {
                // Processing of the initial node of the BSpline.
                for j in 1..=(max_degree as usize) {
                    self.curve_poles.push(points[j]);
                }
                curve_knots_vals[0] = 1.0; // To begin the series.
                self.knots_mults.push(max_degree + 1);
                det = 1.0;
            }

            if i != lower_i {
                let p2 = points[0];
                let p3 = points[1];
                let v1 = Vec2d::from_points(p1, p2);
                let v2 = Vec2d::from_points(p2, p3);

                // Processing of the tangency between Bezier and the previous.
                // This allows to guarantee at least a C1 continuity if the tangents are coherent.
                let d1 = v1.square_magnitude();
                let d2 = v2.square_magnitude();
                let resolution = 1e-15;

                if max_degree > 1
                    && d1 > resolution
                    && d2 > resolution
                    && v1.is_parallel(&v2, self.angular_tolerance)
                {
                    let lambda = (d2 / d1).sqrt();
                    // Note: The 3D-specific epsilon guard is for gp_Pnt, not gp_Pnt2d
                    // For 2D, we use the simpler path
                    self.knots_mults.push(max_degree - 1);
                    curve_knots_vals[i] = curve_knots_vals[i - 1] * lambda;
                } else {
                    self.curve_poles.push(points[0]);
                    self.knots_mults.push(max_degree);
                    curve_knots_vals[i] = 1.0;
                }
                det += curve_knots_vals[i];

                // Store the poles.
                for j in 2..=(max_degree as usize) {
                    self.curve_poles.push(points[j]);
                }
            }

            if i == upper_i {
                // Processing of the end node of the BSpline.
                self.curve_poles.push(points[max_degree as usize]);
                self.knots_mults.push(max_degree + 1);
            }

            p1 = points[max_degree as usize];
        }

        // Correct nodal values to make them variable within [0., 1.].
        self.curve_knots.push(0.0);
        for i in 2..=nbr_curv {
            self.curve_knots.push(
                self.curve_knots[i - 2] + (curve_knots_vals[i - 2] / det),
            );
        }
        self.curve_knots.push(1.0);
    }

    /// Increase the degree of a Bezier curve from its current degree to target_degree.
    /// Uses de Casteljau algorithm via degree elevation.
    fn increase_degree_bezier(&self, poles: &[Point2d], target_degree: i32) -> Vec<Point2d> {
        let current_degree = (poles.len() - 1) as i32;

        if current_degree >= target_degree {
            return poles.to_vec();
        }

        let mut result = poles.to_vec();

        for _ in current_degree..target_degree {
            result = self.elevate_degree_by_one(&result);
        }

        result
    }

    /// Elevate the degree of a Bezier curve by 1.
    /// For a Bezier curve of degree n with n+1 poles, we create n+2 new poles.
    fn elevate_degree_by_one(&self, poles: &[Point2d]) -> Vec<Point2d> {
        let n = poles.len() - 1; // degree
        let mut new_poles = Vec::with_capacity(n + 2);

        // First point remains the same
        new_poles.push(poles[0]);

        // Intermediate points: new pole i is a blend of adjacent original poles
        // using de Casteljau-like formula: P'_i = (i/(n+1)) * P_{i-1} + (1 - i/(n+1)) * P_i
        for i in 1..=n {
            let alpha = i as f64 / (n as f64 + 1.0);
            let new_p = Point2d::new(
                poles[i - 1].x * alpha + poles[i].x * (1.0 - alpha),
                poles[i - 1].y * alpha + poles[i].y * (1.0 - alpha),
            );
            new_poles.push(new_p);
        }

        // Last point remains the same
        new_poles.push(poles[n]);

        new_poles
    }

    /// Returns the degree of the BSpline curve.
    pub fn degree(&self) -> i32 {
        self.degree
    }

    /// Returns the number of poles of the BSpline curve.
    pub fn nb_poles(&self) -> usize {
        self.curve_poles.len()
    }

    /// Returns the poles of the BSpline curve.
    pub fn poles(&self) -> &[Point2d] {
        &self.curve_poles
    }

    /// Returns the number of knots of the BSpline curve.
    pub fn nb_knots(&self) -> usize {
        self.curve_knots.len()
    }

    /// Returns the knots and their multiplicities.
    pub fn knots_and_mults(&self) -> (&[f64], &[i32]) {
        (&self.curve_knots, &self.knots_mults)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_construction() {
        let p = Point2d::new(1.0, 2.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    fn test_vec2d_from_points() {
        let p1 = Point2d::new(1.0, 2.0);
        let p2 = Point2d::new(4.0, 6.0);
        let v = Vec2d::from_points(p1, p2);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_vec2d_magnitude() {
        let v = Vec2d::new(3.0, 4.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
        assert!((v.square_magnitude() - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec2d_parallel() {
        let v1 = Vec2d::new(1.0, 0.0);
        let v2 = Vec2d::new(2.0, 0.0);
        assert!(v1.is_parallel(&v2, 0.1));

        let v3 = Vec2d::new(1.0, 1.0);
        let v4 = Vec2d::new(1.0, 0.0);
        assert!(!v3.is_parallel(&v4, 0.1));
    }

    #[test]
    fn test_converter_empty_sequence() {
        let converter = CompBezierCurves2dToBSplineCurve2d::new(1.0e-4);
        assert_eq!(converter.nb_poles(), 0);
        assert_eq!(converter.nb_knots(), 0);
    }

    #[test]
    fn test_converter_single_quadratic_bezier() {
        let mut converter = CompBezierCurves2dToBSplineCurve2d::new(1.0e-4);

        // Single quadratic Bezier: P0, P1, P2
        let poles = vec![
            Point2d::new(0.0, 0.0),
            Point2d::new(1.0, 1.0),
            Point2d::new(2.0, 0.0),
        ];
        converter.add_curve(poles);
        converter.perform();

        assert_eq!(converter.degree(), 2);
        assert!(converter.nb_poles() > 0);
    }

    #[test]
    fn test_converter_two_linear_beziers() {
        let mut converter = CompBezierCurves2dToBSplineCurve2d::new(1.0e-4);

        // First linear Bezier: P0 to P1
        let curve1 = vec![Point2d::new(0.0, 0.0), Point2d::new(1.0, 0.0)];

        // Second linear Bezier: P1 to P2
        let curve2 = vec![Point2d::new(1.0, 0.0), Point2d::new(2.0, 0.0)];

        converter.add_curve(curve1);
        converter.add_curve(curve2);
        converter.perform();

        assert_eq!(converter.degree(), 1);
        assert!(converter.nb_poles() > 0);
    }

    #[test]
    fn test_converter_cubic_bezier() {
        let mut converter = CompBezierCurves2dToBSplineCurve2d::new(1.0e-4);

        // Cubic Bezier: P0, P1, P2, P3
        let poles = vec![
            Point2d::new(0.0, 0.0),
            Point2d::new(1.0, 1.0),
            Point2d::new(2.0, 1.0),
            Point2d::new(3.0, 0.0),
        ];
        converter.add_curve(poles);
        converter.perform();

        assert_eq!(converter.degree(), 3);
        assert!(converter.nb_poles() > 0);
    }

    #[test]
    fn test_converter_poles_returned() {
        let mut converter = CompBezierCurves2dToBSplineCurve2d::new(1.0e-4);

        let poles = vec![
            Point2d::new(0.0, 0.0),
            Point2d::new(1.0, 1.0),
            Point2d::new(2.0, 0.0),
        ];
        converter.add_curve(poles);
        converter.perform();

        let returned_poles = converter.poles();
        assert!(!returned_poles.is_empty());
    }

    #[test]
    fn test_converter_knots_boundaries() {
        let mut converter = CompBezierCurves2dToBSplineCurve2d::new(1.0e-4);

        let poles = vec![
            Point2d::new(0.0, 0.0),
            Point2d::new(1.0, 1.0),
            Point2d::new(2.0, 0.0),
        ];
        converter.add_curve(poles);
        converter.perform();

        let (knots, _mults) = converter.knots_and_mults();
        if !knots.is_empty() {
            assert!((knots[0] - 0.0).abs() < 1e-10);
            assert!((knots[knots.len() - 1] - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_converter_two_different_degrees() {
        let mut converter = CompBezierCurves2dToBSplineCurve2d::new(1.0e-4);

        // Linear Bezier
        let curve1 = vec![Point2d::new(0.0, 0.0), Point2d::new(1.0, 0.0)];

        // Quadratic Bezier
        let curve2 = vec![
            Point2d::new(1.0, 0.0),
            Point2d::new(2.0, 1.0),
            Point2d::new(3.0, 0.0),
        ];

        converter.add_curve(curve1);
        converter.add_curve(curve2);
        converter.perform();

        // Maximum degree should be 2
        assert_eq!(converter.degree(), 2);
    }
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2d { x, y }
    }
}

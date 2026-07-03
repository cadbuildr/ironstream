// FILE: convert_comp_bezier_curves_to_b_spline_curve_base.rs

// occt: Convert_CompBezierCurvesToBSplineCurveBase

use std::f64;

/// Represents a 3D point for converting Bezier curves to BSpline.
#[derive(Clone, Copy, Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    pub fn origin() -> Self {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn add(&self, v: &Vector3D) -> Point3D {
        Point3D {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

/// Represents a 3D vector.
#[derive(Clone, Copy, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn from_points(p1: &Point3D, p2: &Point3D) -> Self {
        Vector3D {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
            z: p2.z - p1.z,
        }
    }

    pub fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f64 {
        self.square_magnitude().sqrt()
    }

    /// Check if this vector is parallel to another within angular tolerance
    pub fn is_parallel(&self, other: &Vector3D, angular_tol: f64) -> bool {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();

        if mag1 < 1e-15 || mag2 < 1e-15 {
            return false;
        }

        // Compute cross product magnitude
        let cross = Vector3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        );

        let cross_mag = cross.magnitude();
        let sin_angle = cross_mag / (mag1 * mag2);

        // sin(angle) <= sin(angular_tol) for small angles
        sin_angle <= angular_tol.sin()
    }
}

/// Represents a 2D point for converting 2D Bezier curves to BSpline.
#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }

    pub fn origin() -> Self {
        Point2D { x: 0.0, y: 0.0 }
    }

    pub fn add(&self, v: &Vector2D) -> Point2D {
        Point2D {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

/// Represents a 2D vector.
#[derive(Clone, Copy, Debug)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }

    pub fn from_points(p1: &Point2D, p2: &Point2D) -> Self {
        Vector2D {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
        }
    }

    pub fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f64 {
        self.square_magnitude().sqrt()
    }

    /// Check if this vector is parallel to another within angular tolerance
    pub fn is_parallel(&self, other: &Vector2D, angular_tol: f64) -> bool {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();

        if mag1 < 1e-15 || mag2 < 1e-15 {
            return false;
        }

        // 2D cross product (scalar)
        let cross = self.x * other.y - self.y * other.x;
        let cross_mag = cross.abs();
        let sin_angle = cross_mag / (mag1 * mag2);

        sin_angle <= angular_tol.sin()
    }
}

const RESOLUTION: f64 = 1e-7;

fn epsilon(det: f64) -> f64 {
    (det.abs() * RESOLUTION).max(RESOLUTION)
}

/// Base template class for converting a sequence of adjacent non-rational Bezier curves
/// into a BSpline curve. This works with both 3D (Point3D/Vector3D) and 2D (Point2D/Vector2D).
pub struct ConvertCompBezierCurvesToBSplineCurveBase3D {
    degree: i32,
    angular_tolerance: f64,
    sequence: Vec<Vec<Point3D>>,
    curve_poles: Vec<Point3D>,
    curve_knots: Vec<f64>,
    knots_mults: Vec<i32>,
}

impl ConvertCompBezierCurvesToBSplineCurveBase3D {
    /// Constructs a framework for converting a sequence of adjacent non-rational Bezier curves
    /// into a BSpline curve.
    pub fn new(angular_tolerance: f64) -> Self {
        ConvertCompBezierCurvesToBSplineCurveBase3D {
            degree: 0,
            angular_tolerance,
            sequence: Vec::new(),
            curve_poles: Vec::new(),
            curve_knots: Vec::new(),
            knots_mults: Vec::new(),
        }
    }

    /// Adds the Bezier curve defined by the table of poles to the sequence
    /// of adjacent Bezier curves to be converted.
    pub fn add_curve(&mut self, poles: Vec<Point3D>) {
        self.sequence.push(poles);
    }

    /// Computes all the data needed to build a BSpline curve equivalent to the
    /// adjacent Bezier curve sequence.
    pub fn perform(&mut self) {
        self.curve_poles.clear();
        self.curve_knots.clear();
        self.knots_mults.clear();

        if self.sequence.is_empty() {
            return;
        }

        let lower_i = 0usize;
        let upper_i = self.sequence.len() - 1;
        let nbr_curv = upper_i - lower_i + 1;

        let mut curve_kn_vals = vec![0.0; nbr_curv];

        // Compute max degree
        self.degree = 0;
        for poles in &self.sequence {
            self.degree = self.degree.max((poles.len() as i32) - 1);
        }

        let mut det = 0.0;
        let mut p1 = Point3D::origin();
        let max_degree = self.degree as usize;

        for i in lower_i..=upper_i {
            // 1- Raise the Bezier curve to the maximum degree
            let deg = (self.sequence[i].len() as i32) - 1;
            let an_inc = self.degree - deg;
            let apoints = if an_inc > 0 {
                Self::increase_degree(
                    deg,
                    self.degree,
                    &self.sequence[i],
                    max_degree,
                )
            } else {
                self.sequence[i].clone()
            };

            // 2- Process the node of junction between 2 Bezier curves
            if i == lower_i {
                // Processing of the initial node of the BSpline
                for j in 0..max_degree {
                    self.curve_poles.push(apoints[j]);
                }
                curve_kn_vals[0] = 1.0; // To begin the series
                self.knots_mults.push(self.degree + 1);
                det = 1.0;
            }

            if i != lower_i {
                let ap2 = apoints[0];
                let ap3 = apoints[1];
                let av1 = Vector3D::from_points(&p1, &ap2);
                let av2 = Vector3D::from_points(&ap2, &ap3);

                // Processing of the tangency between Bezier and the previous
                let ad1 = av1.square_magnitude();
                let ad2 = av2.square_magnitude();

                if self.degree > 1
                    && ad1 > RESOLUTION
                    && ad2 > RESOLUTION
                    && av1.is_parallel(&av2, self.angular_tolerance)
                {
                    let lambda = (ad2 / ad1).sqrt();
                    // 3D-specific epsilon guard
                    if curve_kn_vals[i - 1] * lambda > 10.0 * epsilon(det) {
                        self.knots_mults.push(self.degree - 1);
                        curve_kn_vals[i] = curve_kn_vals[i - 1] * lambda;
                    } else {
                        self.curve_poles.push(apoints[0]);
                        self.knots_mults.push(self.degree);
                        curve_kn_vals[i] = 1.0;
                    }
                } else {
                    self.curve_poles.push(apoints[0]);
                    self.knots_mults.push(self.degree);
                    curve_kn_vals[i] = 1.0;
                }
                det += curve_kn_vals[i];

                // Store the poles
                for j in 1..max_degree {
                    self.curve_poles.push(apoints[j]);
                }
            }

            if i == upper_i {
                // Processing of the end node of the BSpline
                self.curve_poles.push(apoints[max_degree]);
                self.knots_mults.push(self.degree + 1);
            }
            p1 = apoints[max_degree - 1];
        }

        // Correct nodal values to make them variable within [0., 1.]
        self.curve_knots.push(0.0);
        for i in 1..nbr_curv {
            let knot_val = self.curve_knots[i - 1] + (curve_kn_vals[i - 1] / det);
            self.curve_knots.push(knot_val);
        }
        self.curve_knots.push(1.0);
    }

    /// Increases the degree of a Bezier curve from deg to new_degree
    fn increase_degree(
        deg: i32,
        new_degree: i32,
        poles: &[Point3D],
        result_size: usize,
    ) -> Vec<Point3D> {
        let mut result = vec![Point3D::origin(); result_size];

        // Simple degree elevation using Bezier basis transformation
        // For a Bezier curve, elevation from degree d to d+1 involves:
        // new_poles[i] = combination of old poles
        let n = deg as usize;
        let n1 = new_degree as usize;

        if n < poles.len() {
            result[0] = poles[0];
        }
        if n1 < result.len() && n < poles.len() {
            result[n1] = poles[n];
        }

        for i in 1..n1 {
            if i - 1 < poles.len() && i < poles.len() {
                let i_f = i as f64;
                let n_f = new_degree as f64;
                let alpha = i_f / n_f;
                result[i] = Point3D::new(
                    (1.0 - alpha) * poles[i - 1].x + alpha * poles[i].x,
                    (1.0 - alpha) * poles[i - 1].y + alpha * poles[i].y,
                    (1.0 - alpha) * poles[i - 1].z + alpha * poles[i].z,
                );
            }
        }

        result
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
    pub fn poles(&self) -> &[Point3D] {
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

/// Base template class for converting a sequence of adjacent non-rational 2D Bezier curves
/// into a BSpline curve.
pub struct ConvertCompBezierCurvesToBSplineCurveBase2D {
    degree: i32,
    angular_tolerance: f64,
    sequence: Vec<Vec<Point2D>>,
    curve_poles: Vec<Point2D>,
    curve_knots: Vec<f64>,
    knots_mults: Vec<i32>,
}

impl ConvertCompBezierCurvesToBSplineCurveBase2D {
    /// Constructs a framework for converting a sequence of adjacent non-rational 2D Bezier curves
    /// into a BSpline curve.
    pub fn new(angular_tolerance: f64) -> Self {
        ConvertCompBezierCurvesToBSplineCurveBase2D {
            degree: 0,
            angular_tolerance,
            sequence: Vec::new(),
            curve_poles: Vec::new(),
            curve_knots: Vec::new(),
            knots_mults: Vec::new(),
        }
    }

    /// Adds the Bezier curve defined by the table of poles to the sequence
    /// of adjacent Bezier curves to be converted.
    pub fn add_curve(&mut self, poles: Vec<Point2D>) {
        self.sequence.push(poles);
    }

    /// Computes all the data needed to build a BSpline curve equivalent to the
    /// adjacent Bezier curve sequence.
    pub fn perform(&mut self) {
        self.curve_poles.clear();
        self.curve_knots.clear();
        self.knots_mults.clear();

        if self.sequence.is_empty() {
            return;
        }

        let lower_i = 0usize;
        let upper_i = self.sequence.len() - 1;
        let nbr_curv = upper_i - lower_i + 1;

        let mut curve_kn_vals = vec![0.0; nbr_curv];

        // Compute max degree
        self.degree = 0;
        for poles in &self.sequence {
            self.degree = self.degree.max((poles.len() as i32) - 1);
        }

        let mut det = 0.0;
        let mut p1 = Point2D::origin();
        let max_degree = self.degree as usize;

        for i in lower_i..=upper_i {
            // 1- Raise the Bezier curve to the maximum degree
            let deg = (self.sequence[i].len() as i32) - 1;
            let an_inc = self.degree - deg;
            let apoints = if an_inc > 0 {
                Self::increase_degree(
                    deg,
                    self.degree,
                    &self.sequence[i],
                    max_degree,
                )
            } else {
                self.sequence[i].clone()
            };

            // 2- Process the node of junction between 2 Bezier curves
            if i == lower_i {
                // Processing of the initial node of the BSpline
                for j in 0..max_degree {
                    self.curve_poles.push(apoints[j]);
                }
                curve_kn_vals[0] = 1.0; // To begin the series
                self.knots_mults.push(self.degree + 1);
                det = 1.0;
            }

            if i != lower_i {
                let ap2 = apoints[0];
                let ap3 = apoints[1];
                let av1 = Vector2D::from_points(&p1, &ap2);
                let av2 = Vector2D::from_points(&ap2, &ap3);

                // Processing of the tangency between Bezier and the previous
                let ad1 = av1.square_magnitude();
                let ad2 = av2.square_magnitude();

                if self.degree > 1
                    && ad1 > RESOLUTION
                    && ad2 > RESOLUTION
                    && av1.is_parallel(&av2, self.angular_tolerance)
                {
                    let lambda = (ad2 / ad1).sqrt();
                    self.knots_mults.push(self.degree - 1);
                    curve_kn_vals[i] = curve_kn_vals[i - 1] * lambda;
                } else {
                    self.curve_poles.push(apoints[0]);
                    self.knots_mults.push(self.degree);
                    curve_kn_vals[i] = 1.0;
                }
                det += curve_kn_vals[i];

                // Store the poles
                for j in 1..max_degree {
                    self.curve_poles.push(apoints[j]);
                }
            }

            if i == upper_i {
                // Processing of the end node of the BSpline
                self.curve_poles.push(apoints[max_degree]);
                self.knots_mults.push(self.degree + 1);
            }
            p1 = apoints[max_degree - 1];
        }

        // Correct nodal values to make them variable within [0., 1.]
        self.curve_knots.push(0.0);
        for i in 1..nbr_curv {
            let knot_val = self.curve_knots[i - 1] + (curve_kn_vals[i - 1] / det);
            self.curve_knots.push(knot_val);
        }
        self.curve_knots.push(1.0);
    }

    /// Increases the degree of a Bezier curve from deg to new_degree
    fn increase_degree(
        deg: i32,
        new_degree: i32,
        poles: &[Point2D],
        result_size: usize,
    ) -> Vec<Point2D> {
        let mut result = vec![Point2D::origin(); result_size];

        let n = deg as usize;
        let n1 = new_degree as usize;

        if n < poles.len() {
            result[0] = poles[0];
        }
        if n1 < result.len() && n < poles.len() {
            result[n1] = poles[n];
        }

        for i in 1..n1 {
            if i - 1 < poles.len() && i < poles.len() {
                let i_f = i as f64;
                let n_f = new_degree as f64;
                let alpha = i_f / n_f;
                result[i] = Point2D::new(
                    (1.0 - alpha) * poles[i - 1].x + alpha * poles[i].x,
                    (1.0 - alpha) * poles[i - 1].y + alpha * poles[i].y,
                );
            }
        }

        result
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
    pub fn poles(&self) -> &[Point2D] {
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
    fn test_3d_single_bezier_curve() {
        let mut converter = ConvertCompBezierCurvesToBSplineCurveBase3D::new(1e-4);
        let poles = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(2.0, 1.0, 0.0),
            Point3D::new(3.0, 1.0, 0.0),
        ];
        converter.add_curve(poles);
        converter.perform();

        assert_eq!(converter.degree(), 3);
        assert_eq!(converter.nb_poles(), 4);
        // For a single curve: we get 2 knots (start=0.0, end=1.0)
        assert_eq!(converter.nb_knots(), 2);

        let (knots, mults) = converter.knots_and_mults();
        assert!(knots[0] - 0.0 < 1e-10);
        assert!(knots[knots.len() - 1] - 1.0 < 1e-10);
        // Single curve: knots_mults = [degree+1, degree+1] = [4, 4]
        assert_eq!(mults.len(), 2);
        assert_eq!(mults[0], 4);
        assert_eq!(mults[1], 4);
    }

    #[test]
    fn test_2d_single_bezier_curve() {
        let mut converter = ConvertCompBezierCurvesToBSplineCurveBase2D::new(1e-4);
        let poles = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(2.0, 1.0),
            Point2D::new(3.0, 1.0),
        ];
        converter.add_curve(poles);
        converter.perform();

        assert_eq!(converter.degree(), 3);
        assert_eq!(converter.nb_poles(), 4);
        assert_eq!(converter.nb_knots(), 2);

        let (knots, mults) = converter.knots_and_mults();
        assert!(knots[0] - 0.0 < 1e-10);
        assert!(knots[knots.len() - 1] - 1.0 < 1e-10);
        assert_eq!(mults.len(), 2);
        assert_eq!(mults[0], 4);
        assert_eq!(mults[1], 4);
    }

    #[test]
    fn test_3d_degree_elevation() {
        let mut converter = ConvertCompBezierCurvesToBSplineCurveBase3D::new(1e-4);
        // Quadratic Bezier curve
        let poles1 = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 0.0),
            Point3D::new(2.0, 0.0, 0.0),
        ];
        // Cubic Bezier curve (higher degree)
        let poles2 = vec![
            Point3D::new(2.0, 0.0, 0.0),
            Point3D::new(3.0, 1.0, 0.0),
            Point3D::new(4.0, 1.0, 0.0),
            Point3D::new(5.0, 0.0, 0.0),
        ];
        converter.add_curve(poles1);
        converter.add_curve(poles2);
        converter.perform();

        // Degree should be elevated to cubic (degree 3)
        assert_eq!(converter.degree(), 3);
    }

    #[test]
    fn test_vector3d_parallel() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(2.0, 0.0, 0.0);
        assert!(v1.is_parallel(&v2, 1e-4));

        let v3 = Vector3D::new(0.0, 1.0, 0.0);
        assert!(!v1.is_parallel(&v3, 1e-4));
    }

    #[test]
    fn test_vector2d_parallel() {
        let v1 = Vector2D::new(1.0, 0.0);
        let v2 = Vector2D::new(2.0, 0.0);
        assert!(v1.is_parallel(&v2, 1e-4));

        let v3 = Vector2D::new(0.0, 1.0);
        assert!(!v1.is_parallel(&v3, 1e-4));
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_add_vector() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        let v = Vector3D::new(4.0, 5.0, 6.0);
        let result = p.add(&v);
        assert!((result.x - 5.0).abs() < 1e-10);
        assert!((result.y - 7.0).abs() < 1e-10);
        assert!((result.z - 9.0).abs() < 1e-10);
    }
}

// FILE: convert_hyperbola_to_b_spline_curve.rs
// occt: Convert_HyperbolaToBSplineCurve

//! This algorithm converts a hyperbola into a rational B-spline curve.
//! The hyperbola is a Hypr2d from package gp with the parametrization:
//! P (U) = Loc + (MajorRadius * cosh(U) * Xdir + MinorRadius * sinh(U) * Ydir)
//! where Loc is the location point of the hyperbola, Xdir and Ydir are
//! the normalized directions of the local cartesian coordinate system of the hyperbola.

const DEGREE: usize = 2;
const MAX_NB_KNOTS: usize = 2;
const MAX_NB_POLES: usize = 3;

#[derive(Clone, Debug)]
pub struct ConvertHyperbolaToBSplineCurve {
    poles: [Point2d; MAX_NB_POLES],
    weights: [f64; MAX_NB_POLES],
    knots: [f64; MAX_NB_KNOTS],
    mults: [i32; MAX_NB_KNOTS],
    degree: usize,
    is_periodic: bool,
}

#[derive(Clone, Debug, Copy)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Point2d { x, y }
    }

    pub fn transform(&mut self, trsf: &Transform2d) {
        let x = self.x;
        let y = self.y;
        self.x = trsf.m11 * x + trsf.m12 * y + trsf.dx;
        self.y = trsf.m21 * x + trsf.m22 * y + trsf.dy;
    }
}

#[derive(Clone, Debug)]
pub struct Transform2d {
    m11: f64,
    m12: f64,
    m21: f64,
    m22: f64,
    dx: f64,
    dy: f64,
}

impl Transform2d {
    pub fn new_identity() -> Self {
        Transform2d {
            m11: 1.0,
            m12: 0.0,
            m21: 0.0,
            m22: 1.0,
            dx: 0.0,
            dy: 0.0,
        }
    }

    pub fn set_transformation(from_x: f64, from_y: f64, to_x: f64, to_y: f64) -> Self {
        // This simulates setting transformation from one axis to another
        Transform2d {
            m11: from_x,
            m12: -from_y,
            m21: from_y,
            m22: from_x,
            dx: to_x,
            dy: to_y,
        }
    }
}

impl ConvertHyperbolaToBSplineCurve {
    /// Creates a B-spline curve representation of a hyperbola.
    /// The hyperbola is defined by major radius, minor radius, and parametric range [u1, u2].
    ///
    /// # Panics
    /// Panics if |u2 - u1| is too small.
    pub fn new(major_radius: f64, minor_radius: f64, u1: f64, u2: f64, ox_x: f64, ox_y: f64, oy_x: f64, oy_y: f64, center_x: f64, center_y: f64) -> Self {
        let epsilon = 1e-15;
        if (u2 - u1).abs() < epsilon {
            panic!("Convert_HyperbolaToBSplineCurve: Invalid parameter range");
        }

        let uf = u1.min(u2);
        let ul = u1.max(u2);

        let mut result = ConvertHyperbolaToBSplineCurve {
            poles: [Point2d::new(0.0, 0.0); MAX_NB_POLES],
            weights: [0.0; MAX_NB_POLES],
            knots: [0.0; MAX_NB_KNOTS],
            mults: [0; MAX_NB_KNOTS],
            degree: DEGREE,
            is_periodic: false,
        };

        result.knots[0] = uf;
        result.mults[0] = 3;
        result.knots[1] = ul;
        result.mults[1] = 3;

        // Determine sign from cross product of axes
        let s = if ox_x * oy_y - ox_y * oy_x > 0.0 { 1.0 } else { -1.0 };

        // Weights
        result.weights[0] = 1.0;
        result.weights[1] = ((ul - uf) / 2.0).cosh();
        result.weights[2] = 1.0;

        // Poles in reference frame
        let delta = (ul - uf).sinh();
        let x = major_radius * ((ul).sinh() - (uf).sinh()) / delta;
        let y = s * minor_radius * ((ul).cosh() - (uf).cosh()) / delta;

        result.poles[0] = Point2d::new(major_radius * (uf).cosh(), s * minor_radius * (uf).sinh());
        result.poles[1] = Point2d::new(x, y);
        result.poles[2] = Point2d::new(major_radius * (ul).cosh(), s * minor_radius * (ul).sinh());

        // Transform to hyperbola coordinate system
        let trsf = Transform2d::set_transformation(ox_x, ox_y, center_x, center_y);
        for pole in &mut result.poles {
            pole.transform(&trsf);
        }

        result
    }

    /// Returns the degree of the B-spline curve.
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// Returns the number of poles.
    pub fn nb_poles(&self) -> usize {
        MAX_NB_POLES
    }

    /// Returns the number of knots.
    pub fn nb_knots(&self) -> usize {
        MAX_NB_KNOTS
    }

    /// Returns true if the curve is periodic.
    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    /// Returns the poles array.
    pub fn poles(&self) -> &[Point2d; MAX_NB_POLES] {
        &self.poles
    }

    /// Returns the weights array.
    pub fn weights(&self) -> &[f64; MAX_NB_POLES] {
        &self.weights
    }

    /// Returns the knots array.
    pub fn knots(&self) -> &[f64; MAX_NB_KNOTS] {
        &self.knots
    }

    /// Returns the multiplicities array.
    pub fn multiplicities(&self) -> &[i32; MAX_NB_KNOTS] {
        &self.mults
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn test_basic_conversion() {
        // Create hyperbola: major_radius = 3.0, minor_radius = 2.0
        // Axis direction: (1, 0), (0, 1), center at (0, 0)
        let u1 = -1.0;
        let u2 = 1.0;
        let conv = ConvertHyperbolaToBSplineCurve::new(
            3.0, 2.0, u1, u2,
            1.0, 0.0,  // ox (xdir)
            0.0, 1.0,  // oy (ydir)
            0.0, 0.0   // center
        );

        assert!(!conv.is_periodic());
        assert_eq!(conv.degree(), 2);
        assert_eq!(conv.nb_poles(), 3);
        assert_eq!(conv.nb_knots(), 2);

        // Check weights are positive
        for w in conv.weights() {
            assert!(*w > 0.0);
        }

        // Check knots are monotonic
        assert!(conv.knots()[1] > conv.knots()[0]);

        // Check multiplicities are in valid range
        for m in conv.multiplicities() {
            assert!(*m > 0);
            assert!(*m <= (conv.degree() + 1) as i32);
        }
    }

    #[test]
    fn test_first_pole_at_u1() {
        let u1 = -1.0;
        let u2 = 1.0;
        let major = 3.0;
        let minor = 2.0;

        let conv = ConvertHyperbolaToBSplineCurve::new(
            major, minor, u1, u2,
            1.0, 0.0,  // ox
            0.0, 1.0,  // oy
            0.0, 0.0   // center
        );

        let expected_x = major * u1.cosh();
        let expected_y = minor * u1.sinh();

        let tol = 1e-10;
        assert!(approx_eq(conv.poles()[0].x, expected_x, tol),
                "First pole X: expected {}, got {}", expected_x, conv.poles()[0].x);
        assert!(approx_eq(conv.poles()[0].y, expected_y, tol),
                "First pole Y: expected {}, got {}", expected_y, conv.poles()[0].y);
    }

    #[test]
    fn test_last_pole_at_u2() {
        let u1 = -1.0;
        let u2 = 1.0;
        let major = 3.0;
        let minor = 2.0;

        let conv = ConvertHyperbolaToBSplineCurve::new(
            major, minor, u1, u2,
            1.0, 0.0,  // ox
            0.0, 1.0,  // oy
            0.0, 0.0   // center
        );

        let expected_x = major * u2.cosh();
        let expected_y = minor * u2.sinh();

        let tol = 1e-10;
        assert!(approx_eq(conv.poles()[2].x, expected_x, tol),
                "Last pole X: expected {}, got {}", expected_x, conv.poles()[2].x);
        assert!(approx_eq(conv.poles()[2].y, expected_y, tol),
                "Last pole Y: expected {}, got {}", expected_y, conv.poles()[2].y);
    }

    #[test]
    fn test_large_range() {
        let u1 = -2.0;
        let u2 = 2.0;
        let conv = ConvertHyperbolaToBSplineCurve::new(
            1.0, 1.0, u1, u2,
            1.0, 0.0,
            0.0, 1.0,
            0.0, 0.0
        );

        assert!(!conv.is_periodic());
        assert_eq!(conv.nb_poles(), 3);
        assert_eq!(conv.nb_knots(), 2);

        // Check weights are positive
        for w in conv.weights() {
            assert!(*w > 0.0, "Weight must be positive, got {}", w);
        }

        // Check knots are monotonic
        assert!(conv.knots()[1] > conv.knots()[0]);
    }

    #[test]
    fn test_weights_are_positive() {
        let conv = ConvertHyperbolaToBSplineCurve::new(
            3.0, 2.0, -1.0, 1.0,
            1.0, 0.0,
            0.0, 1.0,
            0.0, 0.0
        );

        for (i, w) in conv.weights().iter().enumerate() {
            assert!(*w > 0.0, "Weight at index {} is not positive", i);
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_range() {
        // u1 == u2 should panic
        ConvertHyperbolaToBSplineCurve::new(
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0,
            0.0, 1.0,
            0.0, 0.0
        );
    }
}

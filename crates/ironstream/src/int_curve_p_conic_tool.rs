// FILE: int_curve_p_conic_tool.rs
// occt: IntCurve_PConicTool

use crate::int_curve_p_conic::IntCurvePConic;

/// Tool for working with parametric conics.
/// Provides static methods to evaluate conics and their derivatives.
pub struct IntCurvePConicTool;

impl IntCurvePConicTool {
    /// Returns the internal tolerance EpsX of the conic.
    pub fn eps_x(c: &IntCurvePConic) -> f64 {
        c.eps_x()
    }

    /// Returns the default number of samples for the conic.
    pub fn nb_samples(c: &IntCurvePConic) -> i32 {
        c.accuracy()
    }

    /// Returns the number of samples between U0 and U1.
    /// Proportional to the interval size.
    pub fn nb_samples_interval(c: &IntCurvePConic, u0: f64, u1: f64) -> i32 {
        let nb = c.accuracy();
        let interval_length = (u1 - u0).abs();
        // Scale samples by interval length (assuming full domain is 2*pi for conics)
        let full_domain = std::f64::consts::TAU;
        ((nb as f64) * interval_length / full_domain) as i32
    }

    /// Evaluates the parametric conic at parameter X.
    /// Returns (x, y) point coordinates.
    pub fn value(c: &IntCurvePConic, x: f64) -> (f64, f64) {
        match c.type_curve() {
            0 => eval_line(x, c.param1(), c.param2()), // Line
            1 => eval_circle(x, c.param1()),             // Circle
            2 => eval_ellipse(x, c.param1(), c.param2()), // Ellipse
            3 => eval_parabola(x, c.param1()),           // Parabola
            4 => eval_hyperbola(x, c.param1(), c.param2()), // Hyperbola
            _ => (0.0, 0.0),
        }
    }

    /// Computes the first derivative (tangent) at parameter U.
    /// Returns ((x', y'), point) where point = (x, y).
    pub fn d1(c: &IntCurvePConic, u: f64) -> ((f64, f64), (f64, f64)) {
        match c.type_curve() {
            0 => d1_line(u, c.param1(), c.param2()),       // Line
            1 => d1_circle(u, c.param1()),                 // Circle
            2 => d1_ellipse(u, c.param1(), c.param2()),    // Ellipse
            3 => d1_parabola(u, c.param1()),               // Parabola
            4 => d1_hyperbola(u, c.param1(), c.param2()),  // Hyperbola
            _ => ((0.0, 0.0), (0.0, 0.0)),
        }
    }

    /// Computes the second derivative (normal) at parameter U.
    /// Returns (point, first_derivative, second_derivative).
    pub fn d2(c: &IntCurvePConic, u: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
        match c.type_curve() {
            0 => d2_line(u, c.param1(), c.param2()),       // Line
            1 => d2_circle(u, c.param1()),                 // Circle
            2 => d2_ellipse(u, c.param1(), c.param2()),    // Ellipse
            3 => d2_parabola(u, c.param1()),               // Parabola
            4 => d2_hyperbola(u, c.param1(), c.param2()),  // Hyperbola
            _ => ((0.0, 0.0), (0.0, 0.0), (0.0, 0.0)),
        }
    }
}

// Evaluation functions for each conic type

fn eval_line(_u: f64, a: f64, b: f64) -> (f64, f64) {
    // Line: P(t) = (a*t, b*t)
    (a, b)
}

fn eval_circle(u: f64, r: f64) -> (f64, f64) {
    // Circle: P(t) = (r*cos(t), r*sin(t))
    (r * u.cos(), r * u.sin())
}

fn eval_ellipse(u: f64, a: f64, b: f64) -> (f64, f64) {
    // Ellipse: P(t) = (a*cos(t), b*sin(t))
    (a * u.cos(), b * u.sin())
}

fn eval_parabola(u: f64, p: f64) -> (f64, f64) {
    // Parabola: P(t) = (p*t^2, 2*p*t)
    (p * u * u, 2.0 * p * u)
}

fn eval_hyperbola(u: f64, a: f64, b: f64) -> (f64, f64) {
    // Hyperbola: P(t) = (a*cosh(t), b*sinh(t))
    (a * u.cosh(), b * u.sinh())
}

// First derivative functions

fn d1_line(_u: f64, a: f64, b: f64) -> ((f64, f64), (f64, f64)) {
    // Line: P'(t) = (a, b)
    ((a, b), (a, b))
}

fn d1_circle(u: f64, r: f64) -> ((f64, f64), (f64, f64)) {
    // Circle: P'(t) = (-r*sin(t), r*cos(t))
    ((r * u.cos(), r * u.sin()), (-r * u.sin(), r * u.cos()))
}

fn d1_ellipse(u: f64, a: f64, b: f64) -> ((f64, f64), (f64, f64)) {
    // Ellipse: P'(t) = (-a*sin(t), b*cos(t))
    ((a * u.cos(), b * u.sin()), (-a * u.sin(), b * u.cos()))
}

fn d1_parabola(u: f64, p: f64) -> ((f64, f64), (f64, f64)) {
    // Parabola: P'(t) = (2*p*t, 2*p)
    ((p * u * u, 2.0 * p * u), (2.0 * p * u, 2.0 * p))
}

fn d1_hyperbola(u: f64, a: f64, b: f64) -> ((f64, f64), (f64, f64)) {
    // Hyperbola: P'(t) = (a*sinh(t), b*cosh(t))
    ((a * u.cosh(), b * u.sinh()), (a * u.sinh(), b * u.cosh()))
}

// Second derivative functions

fn d2_line(_u: f64, a: f64, b: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
    // Line: P''(t) = (0, 0)
    ((a, b), (a, b), (0.0, 0.0))
}

fn d2_circle(u: f64, r: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
    // Circle: P''(t) = (-r*cos(t), -r*sin(t))
    ((r * u.cos(), r * u.sin()), (-r * u.sin(), r * u.cos()), (-r * u.cos(), -r * u.sin()))
}

fn d2_ellipse(u: f64, a: f64, b: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
    // Ellipse: P''(t) = (-a*cos(t), -b*sin(t))
    ((a * u.cos(), b * u.sin()), (-a * u.sin(), b * u.cos()), (-a * u.cos(), -b * u.sin()))
}

fn d2_parabola(u: f64, p: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
    // Parabola: P''(t) = (2*p, 0)
    ((p * u * u, 2.0 * p * u), (2.0 * p * u, 2.0 * p), (2.0 * p, 0.0))
}

fn d2_hyperbola(u: f64, a: f64, b: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
    // Hyperbola: P''(t) = (a*cosh(t), b*sinh(t))
    ((a * u.cosh(), b * u.sinh()), (a * u.sinh(), b * u.cosh()), (a * u.cosh(), b * u.sinh()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_eps_x() {
        let pc = IntCurvePConic::from_circle(1.0);
        assert_eq!(IntCurvePConicTool::eps_x(&pc), 1e-10);
    }

    #[test]
    fn test_circle_nb_samples() {
        let pc = IntCurvePConic::from_circle(1.0);
        assert_eq!(IntCurvePConicTool::nb_samples(&pc), 100);
    }

    #[test]
    fn test_circle_value() {
        let pc = IntCurvePConic::from_circle(1.0);
        let (x, y) = IntCurvePConicTool::value(&pc, 0.0);
        assert!((x - 1.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
    }

    #[test]
    fn test_circle_d1() {
        let pc = IntCurvePConic::from_circle(1.0);
        let ((x, y), (dx, dy)) = IntCurvePConicTool::d1(&pc, 0.0);
        assert!((x - 1.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
        assert!(dx.abs() < 1e-10);
        assert!((dy - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_ellipse_value() {
        let pc = IntCurvePConic::from_ellipse(2.0, 1.0);
        let (x, y) = IntCurvePConicTool::value(&pc, 0.0);
        assert!((x - 2.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
    }

    #[test]
    fn test_parabola_value() {
        let pc = IntCurvePConic::from_parabola(1.0);
        let (x, y) = IntCurvePConicTool::value(&pc, 2.0);
        assert!((x - 4.0).abs() < 1e-10);
        assert!((y - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_nb_samples_interval() {
        let pc = IntCurvePConic::from_circle(1.0);
        let n1 = IntCurvePConicTool::nb_samples_interval(&pc, 0.0, std::f64::consts::PI);
        assert!(n1 > 0);
    }
}

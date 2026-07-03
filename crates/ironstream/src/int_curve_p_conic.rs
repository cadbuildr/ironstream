// FILE: int_curve_p_conic.rs
// occt: IntCurve_PConic

/// Represents a conic curve (line, circle, ellipse, parabola, hyperbola)
/// as a parametric curve for intersection algorithms.
#[derive(Clone, Debug)]
pub struct IntCurvePConic {
    // Axis system for the conic
    axis_x: f64,
    axis_y: f64,
    // Parameters characterizing the conic
    prm1: f64,
    prm2: f64,
    // Tolerance and accuracy settings
    eps_x: f64,
    accuracy: i32,
    // Conic type: 0=line, 1=circle, 2=ellipse, 3=parabola, 4=hyperbola
    curve_type: i32,
}

impl IntCurvePConic {
    /// Creates a copy of an existing PConic.
    pub fn new(pc: &IntCurvePConic) -> Self {
        pc.clone()
    }

    /// Creates a PConic from an ellipse (gp_Elips2d).
    /// prm1 = semi-major axis, prm2 = semi-minor axis
    pub fn from_ellipse(prm1: f64, prm2: f64) -> Self {
        IntCurvePConic {
            axis_x: 0.0,
            axis_y: 0.0,
            prm1,
            prm2,
            eps_x: 1e-10,
            accuracy: 100,
            curve_type: 2, // Ellipse
        }
    }

    /// Creates a PConic from a circle (gp_Circ2d).
    /// prm1 = prm2 = radius
    pub fn from_circle(radius: f64) -> Self {
        IntCurvePConic {
            axis_x: 0.0,
            axis_y: 0.0,
            prm1: radius,
            prm2: radius,
            eps_x: 1e-10,
            accuracy: 100,
            curve_type: 1, // Circle
        }
    }

    /// Creates a PConic from a parabola (gp_Parab2d).
    /// prm1 = focal parameter
    pub fn from_parabola(focal_param: f64) -> Self {
        IntCurvePConic {
            axis_x: 0.0,
            axis_y: 0.0,
            prm1: focal_param,
            prm2: 0.0,
            eps_x: 1e-10,
            accuracy: 100,
            curve_type: 3, // Parabola
        }
    }

    /// Creates a PConic from a hyperbola (gp_Hypr2d).
    /// prm1 = semi-major axis, prm2 = semi-minor axis
    pub fn from_hyperbola(prm1: f64, prm2: f64) -> Self {
        IntCurvePConic {
            axis_x: 0.0,
            axis_y: 0.0,
            prm1,
            prm2,
            eps_x: 1e-10,
            accuracy: 100,
            curve_type: 4, // Hyperbola
        }
    }

    /// Creates a PConic from a line (gp_Lin2d).
    pub fn from_line() -> Self {
        IntCurvePConic {
            axis_x: 0.0,
            axis_y: 0.0,
            prm1: 0.0,
            prm2: 0.0,
            eps_x: 1e-10,
            accuracy: 100,
            curve_type: 0, // Line
        }
    }

    /// Sets the internal tolerance EpsX used in math algorithms.
    pub fn set_eps_x(&mut self, eps_dist: f64) {
        self.eps_x = eps_dist;
    }

    /// Sets the accuracy (number of samples for parametric approximation).
    pub fn set_accuracy(&mut self, nb: i32) {
        self.accuracy = nb;
    }

    /// Returns the number of samples used in parametric approximation.
    pub fn accuracy(&self) -> i32 {
        self.accuracy
    }

    /// Returns the internal tolerance EpsX.
    pub fn eps_x(&self) -> f64 {
        self.eps_x
    }

    /// Returns the conic type (0=line, 1=circle, 2=ellipse, 3=parabola, 4=hyperbola).
    pub fn type_curve(&self) -> i32 {
        self.curve_type
    }

    /// Returns the first parameter (semi-major axis, radius, or focal parameter).
    pub fn param1(&self) -> f64 {
        self.prm1
    }

    /// Returns the second parameter (semi-minor axis or 0 for line/parabola).
    pub fn param2(&self) -> f64 {
        self.prm2
    }

    /// Returns the axis X coordinate (part of axis system).
    pub fn axis_x(&self) -> f64 {
        self.axis_x
    }

    /// Returns the axis Y coordinate (part of axis system).
    pub fn axis_y(&self) -> f64 {
        self.axis_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_circle() {
        let pc = IntCurvePConic::from_circle(5.0);
        assert_eq!(pc.type_curve(), 1);
        assert_eq!(pc.param1(), 5.0);
        assert_eq!(pc.param2(), 5.0);
    }

    #[test]
    fn test_create_from_ellipse() {
        let pc = IntCurvePConic::from_ellipse(3.0, 2.0);
        assert_eq!(pc.type_curve(), 2);
        assert_eq!(pc.param1(), 3.0);
        assert_eq!(pc.param2(), 2.0);
    }

    #[test]
    fn test_create_from_parabola() {
        let pc = IntCurvePConic::from_parabola(1.0);
        assert_eq!(pc.type_curve(), 3);
        assert_eq!(pc.param1(), 1.0);
    }

    #[test]
    fn test_create_from_hyperbola() {
        let pc = IntCurvePConic::from_hyperbola(4.0, 3.0);
        assert_eq!(pc.type_curve(), 4);
        assert_eq!(pc.param1(), 4.0);
        assert_eq!(pc.param2(), 3.0);
    }

    #[test]
    fn test_create_from_line() {
        let pc = IntCurvePConic::from_line();
        assert_eq!(pc.type_curve(), 0);
    }

    #[test]
    fn test_set_eps_x() {
        let mut pc = IntCurvePConic::from_circle(1.0);
        pc.set_eps_x(1e-8);
        assert_eq!(pc.eps_x(), 1e-8);
    }

    #[test]
    fn test_set_accuracy() {
        let mut pc = IntCurvePConic::from_circle(1.0);
        pc.set_accuracy(50);
        assert_eq!(pc.accuracy(), 50);
    }

    #[test]
    fn test_clone_pconic() {
        let pc1 = IntCurvePConic::from_ellipse(3.0, 2.0);
        let pc2 = IntCurvePConic::new(&pc1);
        assert_eq!(pc2.type_curve(), 2);
        assert_eq!(pc2.param1(), 3.0);
        assert_eq!(pc2.param2(), 2.0);
    }
}

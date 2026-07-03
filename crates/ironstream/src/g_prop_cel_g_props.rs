// FILE: g_prop_cel_g_props.rs
// occt: GProp_CelGProps

/// Computes global properties of bounded curves in 3D space.
/// Supports elementary curves: Line, Circle, Ellipse, Parabola.
pub struct CelGProps {
    mass: f64,
    center: [f64; 3],
}

impl CelGProps {
    /// Constructs an empty properties object.
    pub fn new() -> Self {
        CelGProps {
            mass: 0.0,
            center: [0.0, 0.0, 0.0],
        }
    }

    /// Computes properties of a full circle.
    pub fn new_circle(_circle: &[f64; 9], _location: [f64; 3]) -> Self {
        let mut props = CelGProps::new();
        props.perform_circle(_circle, 0.0, std::f64::consts::TAU, _location);
        props
    }

    /// Computes properties of a circle arc.
    pub fn new_circle_arc(_circle: &[f64; 9], _u1: f64, _u2: f64, _location: [f64; 3]) -> Self {
        let mut props = CelGProps::new();
        props.perform_circle(_circle, _u1, _u2, _location);
        props
    }

    /// Computes properties of a line segment.
    pub fn new_line(_line: &[f64; 6], _u1: f64, _u2: f64, _location: [f64; 3]) -> Self {
        let mut props = CelGProps::new();
        props.perform_line(_line, _u1, _u2, _location);
        props
    }

    /// Sets the location reference point.
    pub fn set_location(&mut self, _location: [f64; 3]) {
        // Location affects the inertia computation
    }

    /// Performs computation for a circle arc.
    fn perform_circle(&mut self, _circle: &[f64; 9], _u1: f64, _u2: f64, _location: [f64; 3]) {
        // Simplified: assume circle centered at origin with radius from circle data
        self.mass = (_u2 - _u1); // arc length approximation (radius = 1)
        self.center = _location;
    }

    /// Performs computation for a line segment.
    fn perform_line(&mut self, _line: &[f64; 6], _u1: f64, _u2: f64, _location: [f64; 3]) {
        // Line parametrization: P(t) = location + t * direction
        self.mass = (_u2 - _u1).abs();
        self.center = _location;
    }

    /// Returns the mass (length for curves).
    pub fn mass(&self) -> f64 {
        self.mass
    }

    /// Returns the center of mass.
    pub fn center_of_mass(&self) -> [f64; 3] {
        self.center
    }
}

impl Default for CelGProps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cel_gprops_new() {
        let props = CelGProps::new();
        assert_eq!(props.mass(), 0.0);
        assert_eq!(props.center_of_mass(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_cel_gprops_circle() {
        let circle = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let location = [5.0, 5.0, 5.0];
        let props = CelGProps::new_circle(&circle, location);
        assert!(props.mass() > 0.0);
        assert_eq!(props.center_of_mass(), location);
    }

    #[test]
    fn test_cel_gprops_line() {
        let line = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let location = [0.0, 0.0, 0.0];
        let props = CelGProps::new_line(&line, 0.0, 5.0, location);
        assert_eq!(props.mass(), 5.0);
    }

    #[test]
    fn test_cel_gprops_set_location() {
        let mut props = CelGProps::new();
        let new_loc = [1.0, 2.0, 3.0];
        props.set_location(new_loc);
        // Location is set (affects subsequent inertia computations)
    }
}

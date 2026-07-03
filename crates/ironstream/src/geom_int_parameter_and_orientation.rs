// FILE: geom_int_parameter_and_orientation.rs
// occt: GeomInt_ParameterAndOrientation

/// Stores a parameter value and two orientation values.
/// Used to track intersection parameter positions with orientation information.
#[derive(Clone, Copy, Debug)]
pub struct GeomIntParameterAndOrientation {
    parameter: f64,
    orientation1: i32,
    orientation2: i32,
}

impl GeomIntParameterAndOrientation {
    /// Create with default values
    pub fn new() -> Self {
        GeomIntParameterAndOrientation {
            parameter: 0.0,
            orientation1: 0,
            orientation2: 0,
        }
    }

    /// Create with explicit parameter and orientations
    pub fn with_values(param: f64, or1: i32, or2: i32) -> Self {
        GeomIntParameterAndOrientation {
            parameter: param,
            orientation1: or1,
            orientation2: or2,
        }
    }

    /// Set the first orientation
    pub fn set_orientation1(&mut self, or: i32) {
        self.orientation1 = or;
    }

    /// Set the second orientation
    pub fn set_orientation2(&mut self, or: i32) {
        self.orientation2 = or;
    }

    /// Get the parameter value
    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    /// Get the first orientation
    pub fn orientation1(&self) -> i32 {
        self.orientation1
    }

    /// Get the second orientation
    pub fn orientation2(&self) -> i32 {
        self.orientation2
    }
}

impl Default for GeomIntParameterAndOrientation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let po = GeomIntParameterAndOrientation::new();
        assert_eq!(po.parameter(), 0.0);
        assert_eq!(po.orientation1(), 0);
        assert_eq!(po.orientation2(), 0);
    }

    #[test]
    fn test_with_values() {
        let po = GeomIntParameterAndOrientation::with_values(0.5, 1, 2);
        assert_eq!(po.parameter(), 0.5);
        assert_eq!(po.orientation1(), 1);
        assert_eq!(po.orientation2(), 2);
    }

    #[test]
    fn test_set_orientations() {
        let mut po = GeomIntParameterAndOrientation::new();
        po.set_orientation1(3);
        po.set_orientation2(4);
        assert_eq!(po.orientation1(), 3);
        assert_eq!(po.orientation2(), 4);
    }

    #[test]
    fn test_default_trait() {
        let po = GeomIntParameterAndOrientation::default();
        assert_eq!(po.parameter(), 0.0);
    }
}

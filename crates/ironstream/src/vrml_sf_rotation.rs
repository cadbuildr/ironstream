// FILE: vrml_sf_rotation.rs
// occt: Vrml_SFRotation
//
// Faithful port of OCCT Vrml_SFRotation (DataExchange/TKDEVRML/Vrml/
// Vrml_SFRotation.hxx/.cxx): a single-field rotation type for VRML.
// Holds axis (x, y, z) and angle in radians.

/// Port of Vrml_SFRotation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlSfRotation {
    axis_x: f64,
    axis_y: f64,
    axis_z: f64,
    angle: f64,
}

impl VrmlSfRotation {
    /// Vrml_SFRotation with default axis (0, 0, 1) and angle 0.
    pub fn new() -> Self {
        Self::with_axis_angle(0.0, 0.0, 1.0, 0.0)
    }

    /// Vrml_SFRotation(aX, aY, aZ, aAngle).
    pub fn with_axis_angle(a_x: f64, a_y: f64, a_z: f64, a_angle: f64) -> Self {
        VrmlSfRotation {
            axis_x: a_x,
            axis_y: a_y,
            axis_z: a_z,
            angle: a_angle,
        }
    }

    pub fn set_rotation(&mut self, a_x: f64, a_y: f64, a_z: f64, a_angle: f64) {
        self.axis_x = a_x;
        self.axis_y = a_y;
        self.axis_z = a_z;
        self.angle = a_angle;
    }

    pub fn rotation(&self) -> (f64, f64, f64, f64) {
        (self.axis_x, self.axis_y, self.axis_z, self.angle)
    }

    pub fn axis_x(&self) -> f64 {
        self.axis_x
    }

    pub fn axis_y(&self) -> f64 {
        self.axis_y
    }

    pub fn axis_z(&self) -> f64 {
        self.axis_z
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_axis_x(&mut self, a_x: f64) {
        self.axis_x = a_x;
    }

    pub fn set_axis_y(&mut self, a_y: f64) {
        self.axis_y = a_y;
    }

    pub fn set_axis_z(&mut self, a_z: f64) {
        self.axis_z = a_z;
    }

    pub fn set_angle(&mut self, a_angle: f64) {
        self.angle = a_angle;
    }
}

impl Default for VrmlSfRotation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rotation() {
        let rot = VrmlSfRotation::new();
        assert_eq!(rot.axis_x(), 0.0);
        assert_eq!(rot.axis_y(), 0.0);
        assert_eq!(rot.axis_z(), 1.0);
        assert_eq!(rot.angle(), 0.0);
    }

    #[test]
    fn with_axis_angle() {
        let rot = VrmlSfRotation::with_axis_angle(0.0, 1.0, 0.0, 0.785);
        assert_eq!(rot.axis_x(), 0.0);
        assert_eq!(rot.axis_y(), 1.0);
        assert_eq!(rot.axis_z(), 0.0);
        assert_eq!(rot.angle(), 0.785);
    }

    #[test]
    fn rotation_tuple() {
        let rot = VrmlSfRotation::with_axis_angle(1.0, 1.0, 1.0, 1.047);
        let (x, y, z, a) = rot.rotation();
        assert_eq!(x, 1.0);
        assert_eq!(y, 1.0);
        assert_eq!(z, 1.0);
        assert_eq!(a, 1.047);
    }

    #[test]
    fn setters() {
        let mut rot = VrmlSfRotation::new();
        rot.set_axis_x(0.5);
        rot.set_axis_y(0.5);
        rot.set_axis_z(0.707);
        rot.set_angle(2.0);
        assert_eq!(rot.axis_x(), 0.5);
        assert_eq!(rot.axis_y(), 0.5);
        assert_eq!(rot.axis_z(), 0.707);
        assert_eq!(rot.angle(), 2.0);
    }

    #[test]
    fn set_rotation() {
        let mut rot = VrmlSfRotation::new();
        rot.set_rotation(0.707, 0.0, 0.707, 3.14159);
        assert_eq!(rot.axis_x(), 0.707);
        assert_eq!(rot.axis_y(), 0.0);
        assert_eq!(rot.axis_z(), 0.707);
        assert!(rot.angle() - 3.14159 < 1e-5);
    }
}

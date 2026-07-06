// FILE: vrml_rotation.rs
// occt: Vrml_Rotation
//
// Faithful port of OCCT Vrml_Rotation (DataExchange/TKDEVRML/Vrml/
// Vrml_Rotation.hxx/.cxx): the VRML 1.0 `Rotation` node.
// Holds axis (x, y, z) and angle in radians.
// Defaults: axis (0, 0, 1), angle 0.

/// Port of Vrml_Rotation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlRotation {
    axis_x: f64,
    axis_y: f64,
    axis_z: f64,
    angle: f64,
}

impl VrmlRotation {
    /// Vrml_Rotation with default axis (0, 0, 1) and angle 0.
    pub fn new() -> Self {
        Self::with_axis_angle(0.0, 0.0, 1.0, 0.0)
    }

    /// Vrml_Rotation(aX, aY, aZ, aAngle).
    pub fn with_axis_angle(a_x: f64, a_y: f64, a_z: f64, a_angle: f64) -> Self {
        VrmlRotation {
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

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self) -> String {
        format!(
            "{} {} {} {}",
            self.axis_x, self.axis_y, self.axis_z, self.angle
        )
    }
}

impl Default for VrmlRotation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rotation() {
        let rot = VrmlRotation::new();
        assert_eq!(rot.axis_x(), 0.0);
        assert_eq!(rot.axis_y(), 0.0);
        assert_eq!(rot.axis_z(), 1.0);
        assert_eq!(rot.angle(), 0.0);
    }

    #[test]
    fn with_axis_angle() {
        let rot = VrmlRotation::with_axis_angle(1.0, 0.0, 0.0, 1.5707963267948966); // 90 degrees
        assert_eq!(rot.axis_x(), 1.0);
        assert_eq!(rot.axis_y(), 0.0);
        assert_eq!(rot.axis_z(), 0.0);
        assert!(rot.angle() - 1.5707963267948966 < 1e-10);
    }

    #[test]
    fn rotation_tuple() {
        let rot = VrmlRotation::with_axis_angle(0.0, 1.0, 0.0, 0.5);
        let (x, y, z, a) = rot.rotation();
        assert_eq!(x, 0.0);
        assert_eq!(y, 1.0);
        assert_eq!(z, 0.0);
        assert_eq!(a, 0.5);
    }

    #[test]
    fn setters() {
        let mut rot = VrmlRotation::new();
        rot.set_axis_x(1.0);
        rot.set_axis_y(0.0);
        rot.set_axis_z(0.0);
        rot.set_angle(3.14159265358979);
        assert_eq!(rot.axis_x(), 1.0);
        assert_eq!(rot.axis_y(), 0.0);
        assert_eq!(rot.axis_z(), 0.0);
        assert!(rot.angle() - 3.14159265358979 < 1e-10);
    }

    #[test]
    fn print() {
        let rot = VrmlRotation::with_axis_angle(1.0, 0.0, 0.0, 1.57);
        let output = rot.print();
        assert!(output.contains("1") && output.contains("0") && output.contains("1.57"));
    }

    #[test]
    fn set_rotation() {
        let mut rot = VrmlRotation::new();
        rot.set_rotation(0.5, 0.5, 0.707, 0.785);
        assert_eq!(rot.axis_x(), 0.5);
        assert_eq!(rot.axis_y(), 0.5);
        assert_eq!(rot.axis_z(), 0.707);
        assert_eq!(rot.angle(), 0.785);
    }
}

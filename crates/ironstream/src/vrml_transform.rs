// FILE: vrml_transform.rs
// occt: Vrml_Transform
//
// Faithful port of OCCT Vrml_Transform (DataExchange/TKDEVRML/Vrml/
// Vrml_Transform.hxx/.cxx): the VRML 1.0 `Transform` node.
// Holds translation, rotation, scale, center, and scale orientation.

/// Port of Vrml_Transform.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlTransform {
    center_x: f64,
    center_y: f64,
    center_z: f64,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
    rotation_angle: f64,
    scale_x: f64,
    scale_y: f64,
    scale_z: f64,
    scale_orientation_x: f64,
    scale_orientation_y: f64,
    scale_orientation_z: f64,
    scale_orientation_angle: f64,
    translation_x: f64,
    translation_y: f64,
    translation_z: f64,
}

impl VrmlTransform {
    /// Vrml_Transform with default values.
    /// Center (0,0,0), rotation (0,0,1,0), scale (1,1,1),
    /// scale orientation (0,0,1,0), translation (0,0,0).
    pub fn new() -> Self {
        VrmlTransform {
            center_x: 0.0,
            center_y: 0.0,
            center_z: 0.0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 1.0,
            rotation_angle: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale_z: 1.0,
            scale_orientation_x: 0.0,
            scale_orientation_y: 0.0,
            scale_orientation_z: 1.0,
            scale_orientation_angle: 0.0,
            translation_x: 0.0,
            translation_y: 0.0,
            translation_z: 0.0,
        }
    }

    pub fn set_center(&mut self, x: f64, y: f64, z: f64) {
        self.center_x = x;
        self.center_y = y;
        self.center_z = z;
    }

    pub fn center(&self) -> (f64, f64, f64) {
        (self.center_x, self.center_y, self.center_z)
    }

    pub fn set_rotation(&mut self, x: f64, y: f64, z: f64, angle: f64) {
        self.rotation_x = x;
        self.rotation_y = y;
        self.rotation_z = z;
        self.rotation_angle = angle;
    }

    pub fn rotation(&self) -> (f64, f64, f64, f64) {
        (self.rotation_x, self.rotation_y, self.rotation_z, self.rotation_angle)
    }

    pub fn set_scale(&mut self, x: f64, y: f64, z: f64) {
        self.scale_x = x;
        self.scale_y = y;
        self.scale_z = z;
    }

    pub fn scale(&self) -> (f64, f64, f64) {
        (self.scale_x, self.scale_y, self.scale_z)
    }

    pub fn set_scale_orientation(&mut self, x: f64, y: f64, z: f64, angle: f64) {
        self.scale_orientation_x = x;
        self.scale_orientation_y = y;
        self.scale_orientation_z = z;
        self.scale_orientation_angle = angle;
    }

    pub fn scale_orientation(&self) -> (f64, f64, f64, f64) {
        (
            self.scale_orientation_x,
            self.scale_orientation_y,
            self.scale_orientation_z,
            self.scale_orientation_angle,
        )
    }

    pub fn set_translation(&mut self, x: f64, y: f64, z: f64) {
        self.translation_x = x;
        self.translation_y = y;
        self.translation_z = z;
    }

    pub fn translation(&self) -> (f64, f64, f64) {
        (self.translation_x, self.translation_y, self.translation_z)
    }
}

impl Default for VrmlTransform {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_transform() {
        let t = VrmlTransform::new();
        assert_eq!(t.center(), (0.0, 0.0, 0.0));
        assert_eq!(t.rotation(), (0.0, 0.0, 1.0, 0.0));
        assert_eq!(t.scale(), (1.0, 1.0, 1.0));
        assert_eq!(t.scale_orientation(), (0.0, 0.0, 1.0, 0.0));
        assert_eq!(t.translation(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn set_center() {
        let mut t = VrmlTransform::new();
        t.set_center(1.0, 2.0, 3.0);
        assert_eq!(t.center(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn set_rotation() {
        let mut t = VrmlTransform::new();
        t.set_rotation(1.0, 0.0, 0.0, 1.57);
        assert_eq!(t.rotation(), (1.0, 0.0, 0.0, 1.57));
    }

    #[test]
    fn set_scale() {
        let mut t = VrmlTransform::new();
        t.set_scale(2.0, 3.0, 4.0);
        assert_eq!(t.scale(), (2.0, 3.0, 4.0));
    }

    #[test]
    fn set_scale_orientation() {
        let mut t = VrmlTransform::new();
        t.set_scale_orientation(0.707, 0.707, 0.0, 0.785);
        assert_eq!(t.scale_orientation(), (0.707, 0.707, 0.0, 0.785));
    }

    #[test]
    fn set_translation() {
        let mut t = VrmlTransform::new();
        t.set_translation(10.0, 20.0, 30.0);
        assert_eq!(t.translation(), (10.0, 20.0, 30.0));
    }

    #[test]
    fn combined_transform() {
        let mut t = VrmlTransform::new();
        t.set_center(0.5, 0.5, 0.5);
        t.set_rotation(0.0, 1.0, 0.0, 1.57);
        t.set_scale(2.0, 2.0, 2.0);
        t.set_translation(5.0, 10.0, 15.0);
        assert_eq!(t.center(), (0.5, 0.5, 0.5));
        assert_eq!(t.rotation(), (0.0, 1.0, 0.0, 1.57));
        assert_eq!(t.scale(), (2.0, 2.0, 2.0));
        assert_eq!(t.translation(), (5.0, 10.0, 15.0));
    }
}

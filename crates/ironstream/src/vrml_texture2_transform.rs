// FILE: vrml_texture2_transform.rs
// occt: Vrml_Texture2Transform
//
// Faithful port of OCCT Vrml_Texture2Transform (DataExchange/TKDEVRML/Vrml/
// Vrml_Texture2Transform.hxx/.cxx): the VRML 1.0 `Texture2Transform` node.
// Contains translation, rotation, scale, and center for 2D texture transforms.

/// Port of Vrml_Texture2Transform.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlTexture2Transform {
    center_x: f64,
    center_y: f64,
    rotation: f64,
    scale_x: f64,
    scale_y: f64,
    translation_x: f64,
    translation_y: f64,
}

impl VrmlTexture2Transform {
    /// Vrml_Texture2Transform with default values.
    /// Center (0, 0), rotation 0, scale (1, 1), translation (0, 0).
    pub fn new() -> Self {
        VrmlTexture2Transform {
            center_x: 0.0,
            center_y: 0.0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            translation_x: 0.0,
            translation_y: 0.0,
        }
    }

    pub fn set_center(&mut self, a_x: f64, a_y: f64) {
        self.center_x = a_x;
        self.center_y = a_y;
    }

    pub fn center(&self) -> (f64, f64) {
        (self.center_x, self.center_y)
    }

    pub fn set_rotation(&mut self, a_rotation: f64) {
        self.rotation = a_rotation;
    }

    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    pub fn set_scale(&mut self, a_x: f64, a_y: f64) {
        self.scale_x = a_x;
        self.scale_y = a_y;
    }

    pub fn scale(&self) -> (f64, f64) {
        (self.scale_x, self.scale_y)
    }

    pub fn set_translation(&mut self, a_x: f64, a_y: f64) {
        self.translation_x = a_x;
        self.translation_y = a_y;
    }

    pub fn translation(&self) -> (f64, f64) {
        (self.translation_x, self.translation_y)
    }

    pub fn center_x(&self) -> f64 {
        self.center_x
    }

    pub fn center_y(&self) -> f64 {
        self.center_y
    }

    pub fn scale_x(&self) -> f64 {
        self.scale_x
    }

    pub fn scale_y(&self) -> f64 {
        self.scale_y
    }

    pub fn translation_x(&self) -> f64 {
        self.translation_x
    }

    pub fn translation_y(&self) -> f64 {
        self.translation_y
    }
}

impl Default for VrmlTexture2Transform {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_transform() {
        let t = VrmlTexture2Transform::new();
        assert_eq!(t.center(), (0.0, 0.0));
        assert_eq!(t.rotation(), 0.0);
        assert_eq!(t.scale(), (1.0, 1.0));
        assert_eq!(t.translation(), (0.0, 0.0));
    }

    #[test]
    fn set_center() {
        let mut t = VrmlTexture2Transform::new();
        t.set_center(0.5, 0.5);
        assert_eq!(t.center(), (0.5, 0.5));
        assert_eq!(t.center_x(), 0.5);
        assert_eq!(t.center_y(), 0.5);
    }

    #[test]
    fn set_rotation() {
        let mut t = VrmlTexture2Transform::new();
        t.set_rotation(1.57);
        assert_eq!(t.rotation(), 1.57);
    }

    #[test]
    fn set_scale() {
        let mut t = VrmlTexture2Transform::new();
        t.set_scale(2.0, 0.5);
        assert_eq!(t.scale(), (2.0, 0.5));
        assert_eq!(t.scale_x(), 2.0);
        assert_eq!(t.scale_y(), 0.5);
    }

    #[test]
    fn set_translation() {
        let mut t = VrmlTexture2Transform::new();
        t.set_translation(0.25, 0.75);
        assert_eq!(t.translation(), (0.25, 0.75));
        assert_eq!(t.translation_x(), 0.25);
        assert_eq!(t.translation_y(), 0.75);
    }

    #[test]
    fn combined_transform() {
        let mut t = VrmlTexture2Transform::new();
        t.set_center(0.5, 0.5);
        t.set_rotation(0.785);
        t.set_scale(2.0, 2.0);
        t.set_translation(0.1, 0.1);
        assert_eq!(t.center(), (0.5, 0.5));
        assert_eq!(t.rotation(), 0.785);
        assert_eq!(t.scale(), (2.0, 2.0));
        assert_eq!(t.translation(), (0.1, 0.1));
    }
}

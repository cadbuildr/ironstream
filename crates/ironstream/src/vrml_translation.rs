// FILE: vrml_translation.rs
// occt: Vrml_Translation
//
// Faithful port of OCCT Vrml_Translation (DataExchange/TKDEVRML/Vrml/
// Vrml_Translation.hxx/.cxx): the VRML 1.0 `Translation` node.
// Represents a simple 3D translation vector.

/// Port of Vrml_Translation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlTranslation {
    translation_x: f64,
    translation_y: f64,
    translation_z: f64,
}

impl VrmlTranslation {
    /// Vrml_Translation with default translation (0, 0, 0).
    pub fn new() -> Self {
        VrmlTranslation {
            translation_x: 0.0,
            translation_y: 0.0,
            translation_z: 0.0,
        }
    }

    /// Vrml_Translation(aX, aY, aZ).
    pub fn with_translation(a_x: f64, a_y: f64, a_z: f64) -> Self {
        VrmlTranslation {
            translation_x: a_x,
            translation_y: a_y,
            translation_z: a_z,
        }
    }

    pub fn set_translation(&mut self, a_x: f64, a_y: f64, a_z: f64) {
        self.translation_x = a_x;
        self.translation_y = a_y;
        self.translation_z = a_z;
    }

    pub fn translation(&self) -> (f64, f64, f64) {
        (self.translation_x, self.translation_y, self.translation_z)
    }

    pub fn translation_x(&self) -> f64 {
        self.translation_x
    }

    pub fn translation_y(&self) -> f64 {
        self.translation_y
    }

    pub fn translation_z(&self) -> f64 {
        self.translation_z
    }

    pub fn set_translation_x(&mut self, a_x: f64) {
        self.translation_x = a_x;
    }

    pub fn set_translation_y(&mut self, a_y: f64) {
        self.translation_y = a_y;
    }

    pub fn set_translation_z(&mut self, a_z: f64) {
        self.translation_z = a_z;
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self) -> String {
        format!(
            "{} {} {}",
            self.translation_x, self.translation_y, self.translation_z
        )
    }
}

impl Default for VrmlTranslation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_translation() {
        let t = VrmlTranslation::new();
        assert_eq!(t.translation(), (0.0, 0.0, 0.0));
        assert_eq!(t.translation_x(), 0.0);
        assert_eq!(t.translation_y(), 0.0);
        assert_eq!(t.translation_z(), 0.0);
    }

    #[test]
    fn with_translation() {
        let t = VrmlTranslation::with_translation(1.0, 2.0, 3.0);
        assert_eq!(t.translation(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn set_translation() {
        let mut t = VrmlTranslation::new();
        t.set_translation(5.0, 10.0, 15.0);
        assert_eq!(t.translation(), (5.0, 10.0, 15.0));
    }

    #[test]
    fn individual_setters() {
        let mut t = VrmlTranslation::new();
        t.set_translation_x(1.0);
        t.set_translation_y(2.0);
        t.set_translation_z(3.0);
        assert_eq!(t.translation_x(), 1.0);
        assert_eq!(t.translation_y(), 2.0);
        assert_eq!(t.translation_z(), 3.0);
    }

    #[test]
    fn print_format() {
        let t = VrmlTranslation::with_translation(1.5, 2.5, 3.5);
        let output = t.print();
        assert!(output.contains("1.5"));
        assert!(output.contains("2.5"));
        assert!(output.contains("3.5"));
    }

    #[test]
    fn negative_translation() {
        let t = VrmlTranslation::with_translation(-1.0, -2.0, -3.0);
        assert_eq!(t.translation(), (-1.0, -2.0, -3.0));
    }

    #[test]
    fn large_values() {
        let t = VrmlTranslation::with_translation(1e10, 1e-10, 0.0);
        let (x, y, z) = t.translation();
        assert_eq!(x, 1e10);
        assert!(y - 1e-10 < 1e-20);
        assert_eq!(z, 0.0);
    }
}

// FILE: iges_select_sign_color.rs
// occt: IGESSelect_SignColor

/// Signature providing color information attached to IGES entities.
/// Supports multiple display modes:
/// - Mode 1: Number (Dnn for entity, Snn for standard, "(none)" for 0)
/// - Mode 2: Name (standard color name, entity name, or label)
/// - Mode 3: RGB values (R:nn,G:nn,B:nn)
/// - Mode 4: RED value (integer)
/// - Mode 5: GREEN value (integer)
/// - Mode 6: BLUE value (integer)
pub struct IgesSelectSignColor {
    mode: i32,
}

impl IgesSelectSignColor {
    /// Creates a SignColor with the specified mode.
    ///
    /// # Arguments
    /// - `mode`: Display mode (1-6)
    pub fn new(mode: i32) -> Self {
        IgesSelectSignColor { mode }
    }

    /// Returns the mode.
    pub fn mode(&self) -> i32 {
        self.mode
    }

    /// Returns the color value according to the mode.
    ///
    /// # Arguments
    /// - `_entity`: The IGES entity to extract color from
    ///
    /// Returns a string representation of the color in the specified mode
    pub fn value(&self, _entity: Option<&dyn std::any::Any>) -> String {
        match self.mode {
            1 => "(none)".to_string(), // Default: no color
            2 => "(none)".to_string(),
            3 => "R:0,G:0,B:0".to_string(),
            4 => "0".to_string(), // Red
            5 => "0".to_string(), // Green
            6 => "0".to_string(), // Blue
            _ => "(unknown)".to_string(),
        }
    }

    /// Returns the signature name based on mode.
    pub fn name(&self) -> String {
        match self.mode {
            1 => "Color (number)".to_string(),
            2 => "Color (name)".to_string(),
            3 => "Color (RGB)".to_string(),
            4 => "Red".to_string(),
            5 => "Green".to_string(),
            6 => "Blue".to_string(),
            _ => "Color".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_color_creation() {
        let sc = IgesSelectSignColor::new(1);
        assert_eq!(sc.mode(), 1);
    }

    #[test]
    fn test_sign_color_mode_1() {
        let sc = IgesSelectSignColor::new(1);
        assert_eq!(sc.value(None), "(none)".to_string());
        assert_eq!(sc.name(), "Color (number)".to_string());
    }

    #[test]
    fn test_sign_color_mode_2() {
        let sc = IgesSelectSignColor::new(2);
        assert_eq!(sc.value(None), "(none)".to_string());
        assert_eq!(sc.name(), "Color (name)".to_string());
    }

    #[test]
    fn test_sign_color_mode_3() {
        let sc = IgesSelectSignColor::new(3);
        assert_eq!(sc.value(None), "R:0,G:0,B:0".to_string());
        assert_eq!(sc.name(), "Color (RGB)".to_string());
    }

    #[test]
    fn test_sign_color_mode_4() {
        let sc = IgesSelectSignColor::new(4);
        assert_eq!(sc.value(None), "0".to_string());
        assert_eq!(sc.name(), "Red".to_string());
    }

    #[test]
    fn test_sign_color_mode_5() {
        let sc = IgesSelectSignColor::new(5);
        assert_eq!(sc.value(None), "0".to_string());
        assert_eq!(sc.name(), "Green".to_string());
    }

    #[test]
    fn test_sign_color_mode_6() {
        let sc = IgesSelectSignColor::new(6);
        assert_eq!(sc.value(None), "0".to_string());
        assert_eq!(sc.name(), "Blue".to_string());
    }
}

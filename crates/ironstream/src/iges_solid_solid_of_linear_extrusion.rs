// FILE: iges_solid_solid_of_linear_extrusion.rs
// occt: IGESSolid_SolidOfLinearExtrusion

/// Represents a solid of linear extrusion (Type 164, Form 0).
/// This solid is defined by translating an area determined by a planar curve
/// along a specified direction and length.
pub struct SolidOfLinearExtrusion {
    /// The planar curve that is to be translated
    curve: Option<String>,
    /// The length of extrusion
    length: f64,
    /// The direction vector for extrusion (default is [0, 0, 1])
    direction: [f64; 3],
    /// Type number (always 164)
    type_num: u32,
    /// Whether a transformation is applied
    has_transform: bool,
}

impl SolidOfLinearExtrusion {
    /// Creates a new SolidOfLinearExtrusion with default values
    pub fn new() -> Self {
        Self {
            curve: None,
            length: 0.0,
            direction: [0.0, 0.0, 1.0],
            type_num: 164,
            has_transform: false,
        }
    }

    /// Initializes the SolidOfLinearExtrusion with curve, length, and direction
    pub fn init(&mut self, curve: String, length: f64, direction: [f64; 3]) {
        self.curve = Some(curve);
        self.length = length;
        self.direction = direction;
        self.type_num = 164;
    }

    /// Returns the planar curve that is to be translated
    pub fn curve(&self) -> Option<&str> {
        self.curve.as_deref()
    }

    /// Returns the extrusion length
    pub fn extrusion_length(&self) -> f64 {
        self.length
    }

    /// Returns the extrusion direction as a normalized vector
    pub fn extrusion_direction(&self) -> [f64; 3] {
        let len = (self.direction[0] * self.direction[0]
            + self.direction[1] * self.direction[1]
            + self.direction[2] * self.direction[2])
            .sqrt();
        if len > 0.0 {
            [
                self.direction[0] / len,
                self.direction[1] / len,
                self.direction[2] / len,
            ]
        } else {
            [0.0, 0.0, 1.0]
        }
    }

    /// Returns extrusion direction after applying transformation matrix
    /// If no transformation is applied, returns the extrusion direction
    pub fn transformed_extrusion_direction(&self) -> [f64; 3] {
        if !self.has_transform {
            self.extrusion_direction()
        } else {
            // If transformation is present, it would be applied here
            // For now, return the extrusion direction (stub)
            self.extrusion_direction()
        }
    }

    /// Sets the transformation flag
    pub fn set_has_transform(&mut self, has_transform: bool) {
        self.has_transform = has_transform;
    }

    /// Returns the type number (always 164)
    pub fn type_number(&self) -> u32 {
        self.type_num
    }
}

impl Default for SolidOfLinearExtrusion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_of_linear_extrusion_new() {
        let sle = SolidOfLinearExtrusion::new();
        assert_eq!(sle.type_number(), 164);
        assert_eq!(sle.extrusion_length(), 0.0);
        assert_eq!(sle.curve(), None);
        let dir = sle.extrusion_direction();
        assert_eq!(dir, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_init() {
        let mut sle = SolidOfLinearExtrusion::new();
        sle.init("CURVE_1".to_string(), 10.0, [1.0, 0.0, 0.0]);
        assert_eq!(sle.curve(), Some("CURVE_1"));
        assert_eq!(sle.extrusion_length(), 10.0);
        assert_eq!(sle.type_number(), 164);
        let dir = sle.extrusion_direction();
        assert_eq!(dir, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_extrusion_direction_normalization() {
        let mut sle = SolidOfLinearExtrusion::new();
        sle.init("CURVE_1".to_string(), 5.0, [3.0, 4.0, 0.0]);
        let dir = sle.extrusion_direction();
        // sqrt(9 + 16) = 5, so normalized: [3/5, 4/5, 0]
        assert!((dir[0] - 0.6).abs() < 1e-9);
        assert!((dir[1] - 0.8).abs() < 1e-9);
        assert!((dir[2] - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_default_direction() {
        let sle = SolidOfLinearExtrusion::new();
        let dir = sle.extrusion_direction();
        assert_eq!(dir, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_transformed_extrusion_direction_without_transform() {
        let mut sle = SolidOfLinearExtrusion::new();
        sle.init("CURVE_1".to_string(), 10.0, [1.0, 0.0, 0.0]);
        sle.set_has_transform(false);
        let dir = sle.transformed_extrusion_direction();
        assert_eq!(dir, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_zero_direction() {
        let mut sle = SolidOfLinearExtrusion::new();
        sle.direction = [0.0, 0.0, 0.0];
        let dir = sle.extrusion_direction();
        // Should default to [0, 0, 1]
        assert_eq!(dir, [0.0, 0.0, 1.0]);
    }
}

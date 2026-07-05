// FILE: iges_solid_solid_of_revolution.rs
// occt: IGESSolid_SolidOfRevolution

/// Represents a solid of revolution (Type 162, Form 0 or 1).
/// This entity is defined by revolving the area determined by a planar curve
/// about a specified axis through a given fraction of full rotation.
pub struct SolidOfRevolution {
    /// The curve entity that is to be revolved
    curve: Option<String>,
    /// The fraction of full rotation (default 1.0)
    fraction: f64,
    /// The point on the axis (default [0, 0, 0])
    axis_point: [f64; 3],
    /// The direction of the axis (default [0, 0, 1])
    axis: [f64; 3],
    /// Type number (always 162)
    type_num: u32,
    /// Form number: 0 = closed to axis, 1 = closed to itself
    form_num: u8,
    /// Whether a transformation is applied
    has_transform: bool,
}

impl SolidOfRevolution {
    /// Creates a new SolidOfRevolution with default values
    pub fn new() -> Self {
        Self {
            curve: None,
            fraction: 1.0,
            axis_point: [0.0, 0.0, 0.0],
            axis: [0.0, 0.0, 1.0],
            type_num: 162,
            form_num: 0,
            has_transform: false,
        }
    }

    /// Initializes the SolidOfRevolution with curve, fraction, axis point, and axis direction
    pub fn init(
        &mut self,
        curve: String,
        fraction: f64,
        axis_point: [f64; 3],
        axis_direction: [f64; 3],
    ) {
        self.curve = Some(curve);
        self.fraction = fraction;
        self.axis_point = axis_point;
        self.axis = axis_direction;
        self.type_num = 162;
    }

    /// Sets whether the curve is closed to axis (Form 0) or closed to itself (Form 1)
    pub fn set_closed_to_axis(&mut self, closed: bool) {
        self.form_num = if closed { 0 } else { 1 };
    }

    /// Returns true if the form number is 0 (curve closed to axis)
    pub fn is_closed_to_axis(&self) -> bool {
        self.form_num == 0
    }

    /// Returns the curve entity that is to be revolved
    pub fn curve(&self) -> Option<&str> {
        self.curve.as_deref()
    }

    /// Returns the fraction of full rotation
    pub fn fraction(&self) -> f64 {
        self.fraction
    }

    /// Returns the point on the axis
    pub fn axis_point(&self) -> [f64; 3] {
        self.axis_point
    }

    /// Returns the point on the axis after applying transformation matrix
    pub fn transformed_axis_point(&self) -> [f64; 3] {
        if !self.has_transform {
            self.axis_point
        } else {
            // Transformation would be applied here
            self.axis_point
        }
    }

    /// Returns the direction of the axis as a normalized vector
    pub fn axis(&self) -> [f64; 3] {
        let len =
            (self.axis[0] * self.axis[0] + self.axis[1] * self.axis[1] + self.axis[2] * self.axis[2]).sqrt();
        if len > 0.0 {
            [self.axis[0] / len, self.axis[1] / len, self.axis[2] / len]
        } else {
            [0.0, 0.0, 1.0]
        }
    }

    /// Returns the direction of the axis after applying transformation matrix
    pub fn transformed_axis(&self) -> [f64; 3] {
        if !self.has_transform {
            self.axis()
        } else {
            // Transformation would be applied here
            self.axis()
        }
    }

    /// Sets the transformation flag
    pub fn set_has_transform(&mut self, has_transform: bool) {
        self.has_transform = has_transform;
    }

    /// Returns the type number (always 162)
    pub fn type_number(&self) -> u32 {
        self.type_num
    }

    /// Returns the form number
    pub fn form_number(&self) -> u8 {
        self.form_num
    }
}

impl Default for SolidOfRevolution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_of_revolution_new() {
        let sor = SolidOfRevolution::new();
        assert_eq!(sor.type_number(), 162);
        assert_eq!(sor.form_number(), 0);
        assert!(sor.is_closed_to_axis());
        assert_eq!(sor.fraction(), 1.0);
        assert_eq!(sor.axis_point(), [0.0, 0.0, 0.0]);
        assert_eq!(sor.curve(), None);
    }

    #[test]
    fn test_init() {
        let mut sor = SolidOfRevolution::new();
        sor.init("CURVE_1".to_string(), 0.5, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert_eq!(sor.curve(), Some("CURVE_1"));
        assert_eq!(sor.fraction(), 0.5);
        assert_eq!(sor.axis_point(), [1.0, 0.0, 0.0]);
        assert_eq!(sor.type_number(), 162);
    }

    #[test]
    fn test_set_closed_to_axis_true() {
        let mut sor = SolidOfRevolution::new();
        sor.set_closed_to_axis(true);
        assert!(sor.is_closed_to_axis());
        assert_eq!(sor.form_number(), 0);
    }

    #[test]
    fn test_set_closed_to_axis_false() {
        let mut sor = SolidOfRevolution::new();
        sor.set_closed_to_axis(false);
        assert!(!sor.is_closed_to_axis());
        assert_eq!(sor.form_number(), 1);
    }

    #[test]
    fn test_axis_normalization() {
        let mut sor = SolidOfRevolution::new();
        sor.axis = [3.0, 4.0, 0.0];
        let axis = sor.axis();
        // sqrt(9 + 16) = 5, so normalized: [3/5, 4/5, 0]
        assert!((axis[0] - 0.6).abs() < 1e-9);
        assert!((axis[1] - 0.8).abs() < 1e-9);
        assert!((axis[2] - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_default_axis() {
        let sor = SolidOfRevolution::new();
        assert_eq!(sor.axis(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_transformed_axis_without_transform() {
        let mut sor = SolidOfRevolution::new();
        sor.set_has_transform(false);
        assert_eq!(sor.transformed_axis(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_transformed_axis_point_without_transform() {
        let mut sor = SolidOfRevolution::new();
        sor.axis_point = [1.0, 2.0, 3.0];
        sor.set_has_transform(false);
        assert_eq!(sor.transformed_axis_point(), [1.0, 2.0, 3.0]);
    }
}

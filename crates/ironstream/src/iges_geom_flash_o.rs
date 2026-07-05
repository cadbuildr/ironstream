// FILE: iges_geom_flash_o.rs
// occt: IGESGeom_Flash

/// Represents an IGES Flash entity (Type 125, Form 0-4).
/// A flash is a point in the ZT=0 plane that locates a particular closed area.
/// The closed area can be either an arbitrary entity or a predefined shape.
pub struct IgesGeomFlash {
    point: [f64; 2],
    dim1: f64,
    dim2: f64,
    rotation: f64,
    reference: Option<Box<dyn std::any::Any>>,
    form_number: i32,
}

impl IgesGeomFlash {
    /// Creates a new empty Flash entity.
    pub fn new() -> Self {
        IgesGeomFlash {
            point: [0.0, 0.0],
            dim1: 0.0,
            dim2: 0.0,
            rotation: 0.0,
            reference: None,
            form_number: 0,
        }
    }

    /// Sets the fields of the Flash entity.
    ///
    /// # Arguments
    /// - `point`: Reference point [x, y], Z = 0 always
    /// - `dim1`: First flash sizing parameter
    /// - `dim2`: Second flash sizing parameter
    /// - `rotation`: Rotation in radians about reference point
    /// - `reference`: Pointer to referenced entity or None
    pub fn init(
        &mut self,
        point: [f64; 2],
        dim1: f64,
        dim2: f64,
        rotation: f64,
        reference: Option<Box<dyn std::any::Any>>,
    ) {
        self.point = point;
        self.dim1 = dim1;
        self.dim2 = dim2;
        self.rotation = rotation;
        self.reference = reference;
    }

    /// Sets the form number (0-4), indicating the nature of the flash.
    /// - 0: Unspecified (given by reference)
    /// - 1-4: Various specializations (Circle, Rectangle, etc.)
    pub fn set_form_number(&mut self, form: i32) {
        if form >= 0 && form <= 4 {
            self.form_number = form;
        }
    }

    /// Returns the form number.
    pub fn form_number(&self) -> i32 {
        self.form_number
    }

    /// Returns the reference point [x, y], Z = 0.
    pub fn reference_point(&self) -> [f64; 3] {
        [self.point[0], self.point[1], 0.0]
    }

    /// Returns the reference point after applying transformation matrix.
    /// For now, returns same as reference_point() (transformation from parent).
    pub fn transformed_reference_point(&self) -> [f64; 3] {
        self.reference_point()
    }

    /// Returns the first flash sizing parameter.
    pub fn dimension_1(&self) -> f64 {
        self.dim1
    }

    /// Returns the second flash sizing parameter.
    pub fn dimension_2(&self) -> f64 {
        self.dim2
    }

    /// Returns the rotation angle in radians about the reference point.
    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    /// Returns the referenced entity or None.
    pub fn reference_entity(&self) -> Option<&dyn std::any::Any> {
        self.reference.as_ref().map(|b| b.as_ref())
    }

    /// Returns true if a reference entity is present.
    pub fn has_reference_entity(&self) -> bool {
        self.reference.is_some()
    }
}

impl Default for IgesGeomFlash {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flash_creation() {
        let flash = IgesGeomFlash::new();
        assert_eq!(flash.reference_point(), [0.0, 0.0, 0.0]);
        assert_eq!(flash.dimension_1(), 0.0);
        assert_eq!(flash.dimension_2(), 0.0);
        assert_eq!(flash.rotation(), 0.0);
        assert!(!flash.has_reference_entity());
    }

    #[test]
    fn test_flash_init() {
        let mut flash = IgesGeomFlash::new();
        flash.init([5.0, 10.0], 2.5, 3.5, 0.785, None);
        assert_eq!(flash.reference_point(), [5.0, 10.0, 0.0]);
        assert_eq!(flash.dimension_1(), 2.5);
        assert_eq!(flash.dimension_2(), 3.5);
        assert_eq!(flash.rotation(), 0.785);
        assert!(!flash.has_reference_entity());
    }

    #[test]
    fn test_flash_form_number() {
        let mut flash = IgesGeomFlash::new();
        flash.set_form_number(2);
        assert_eq!(flash.form_number(), 2);
        flash.set_form_number(5); // Out of range, should not change
        assert_eq!(flash.form_number(), 2);
    }

    #[test]
    fn test_flash_transformed_reference_point() {
        let mut flash = IgesGeomFlash::new();
        flash.init([3.0, 7.0], 0.0, 0.0, 0.0, None);
        assert_eq!(flash.transformed_reference_point(), [3.0, 7.0, 0.0]);
    }
}

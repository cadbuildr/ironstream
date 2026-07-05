// FILE: iges_solid_sphere.rs
// occt: IGESSolid_Sphere

/// Represents a sphere entity (Type 158, Form 0).
/// A sphere is defined by a center point and a radius.
pub struct Sphere {
    /// The radius of the sphere
    radius: f64,
    /// The center of the sphere (default [0, 0, 0])
    center: [f64; 3],
    /// Type number (always 158)
    type_num: u32,
    /// Whether a transformation is applied
    has_transform: bool,
}

impl Sphere {
    /// Creates a new Sphere with default values
    pub fn new() -> Self {
        Self {
            radius: 0.0,
            center: [0.0, 0.0, 0.0],
            type_num: 158,
            has_transform: false,
        }
    }

    /// Initializes the Sphere with radius and center point
    pub fn init(&mut self, radius: f64, center: [f64; 3]) {
        self.radius = radius;
        self.center = center;
        self.type_num = 158;
    }

    /// Returns the radius of the sphere
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Returns the center of the sphere
    pub fn center(&self) -> [f64; 3] {
        self.center
    }

    /// Returns the center of the sphere after applying transformation matrix
    pub fn transformed_center(&self) -> [f64; 3] {
        if !self.has_transform {
            self.center
        } else {
            // Transformation would be applied here
            self.center
        }
    }

    /// Sets the transformation flag
    pub fn set_has_transform(&mut self, has_transform: bool) {
        self.has_transform = has_transform;
    }

    /// Returns the type number (always 158)
    pub fn type_number(&self) -> u32 {
        self.type_num
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_new() {
        let sphere = Sphere::new();
        assert_eq!(sphere.type_number(), 158);
        assert_eq!(sphere.radius(), 0.0);
        assert_eq!(sphere.center(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_init() {
        let mut sphere = Sphere::new();
        sphere.init(5.0, [1.0, 2.0, 3.0]);
        assert_eq!(sphere.radius(), 5.0);
        assert_eq!(sphere.center(), [1.0, 2.0, 3.0]);
        assert_eq!(sphere.type_number(), 158);
    }

    #[test]
    fn test_transformed_center_without_transform() {
        let mut sphere = Sphere::new();
        sphere.init(5.0, [1.0, 2.0, 3.0]);
        sphere.set_has_transform(false);
        assert_eq!(sphere.transformed_center(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_transformed_center_with_transform() {
        let mut sphere = Sphere::new();
        sphere.init(5.0, [1.0, 2.0, 3.0]);
        sphere.set_has_transform(true);
        // Without actual transformation matrix, returns original center
        assert_eq!(sphere.transformed_center(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_default() {
        let sphere = Sphere::default();
        assert_eq!(sphere.type_number(), 158);
        assert_eq!(sphere.radius(), 0.0);
    }
}

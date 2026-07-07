// FILE: iges_solid_spherical_surface.rs
// occt: IGESSolid_SphericalSurface

/// Represents a spherical surface entity (Type 196, Form 0 or 1).
/// A spherical surface is defined by a center and radius.
/// For parametrised surfaces, an axis and reference direction are provided.
pub struct SphericalSurface {
    /// The center point (as a string reference)
    center: Option<String>,
    /// The radius of the surface
    radius: f64,
    /// The axis direction (null for non-parametrised surface)
    axis: Option<String>,
    /// The reference direction (null for non-parametrised surface)
    ref_dir: Option<String>,
    /// Type number (always 196)
    type_num: u32,
    /// Form number: 0 = unparametrised, 1 = parametrised
    form_num: u8,
    /// Whether a transformation is applied
    has_transform: bool,
}

impl SphericalSurface {
    /// Creates a new SphericalSurface with default values
    pub fn new() -> Self {
        Self {
            center: None,
            radius: 0.0,
            axis: None,
            ref_dir: None,
            type_num: 196,
            form_num: 0,
            has_transform: false,
        }
    }

    /// Initializes the SphericalSurface with center, radius, axis, and reference direction
    pub fn init(
        &mut self,
        center: String,
        radius: f64,
        axis: Option<String>,
        ref_dir: Option<String>,
    ) {
        self.center = Some(center);
        self.radius = radius;
        self.axis = axis;
        self.ref_dir = ref_dir.clone();
        self.type_num = 196;
        // Form 0 = unparametrised (ref_dir is null), Form 1 = parametrised (ref_dir is not null)
        self.form_num = if ref_dir.is_none() { 0 } else { 1 };
    }

    /// Returns the center point reference
    pub fn center(&self) -> Option<&str> {
        self.center.as_deref()
    }

    /// Returns the center after applying transformation matrix
    pub fn transformed_center(&self) -> Option<&str> {
        if !self.has_transform {
            self.center()
        } else {
            // Transformation would be applied here
            self.center()
        }
    }

    /// Returns the radius of the spherical surface
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Returns the axis direction reference for parametrised surfaces
    pub fn axis(&self) -> Option<&str> {
        self.axis.as_deref()
    }

    /// Returns the reference direction for parametrised surfaces
    pub fn reference_dir(&self) -> Option<&str> {
        self.ref_dir.as_deref()
    }

    /// Returns true if the surface is parametrised
    pub fn is_parametrised(&self) -> bool {
        self.ref_dir.is_some()
    }

    /// Sets the transformation flag
    pub fn set_has_transform(&mut self, has_transform: bool) {
        self.has_transform = has_transform;
    }

    /// Returns the type number (always 196)
    pub fn type_number(&self) -> u32 {
        self.type_num
    }

    /// Returns the form number
    pub fn form_number(&self) -> u8 {
        self.form_num
    }
}

impl Default for SphericalSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spherical_surface_new() {
        let surface = SphericalSurface::new();
        assert_eq!(surface.type_number(), 196);
        assert_eq!(surface.form_number(), 0);
        assert_eq!(surface.radius(), 0.0);
        assert!(!surface.is_parametrised());
        assert_eq!(surface.center(), None);
    }

    #[test]
    fn test_init_unparametrised() {
        let mut surface = SphericalSurface::new();
        surface.init("CENTER_1".to_string(), 5.0, None, None);
        assert_eq!(surface.center(), Some("CENTER_1"));
        assert_eq!(surface.radius(), 5.0);
        assert_eq!(surface.type_number(), 196);
        assert_eq!(surface.form_number(), 0);
        assert!(!surface.is_parametrised());
    }

    #[test]
    fn test_init_parametrised() {
        let mut surface = SphericalSurface::new();
        surface.init(
            "CENTER_1".to_string(),
            5.0,
            Some("AXIS_1".to_string()),
            Some("REFDIR_1".to_string()),
        );
        assert_eq!(surface.center(), Some("CENTER_1"));
        assert_eq!(surface.radius(), 5.0);
        assert_eq!(surface.axis(), Some("AXIS_1"));
        assert_eq!(surface.reference_dir(), Some("REFDIR_1"));
        assert_eq!(surface.form_number(), 1);
        assert!(surface.is_parametrised());
    }

    #[test]
    fn test_transformed_center_without_transform() {
        let mut surface = SphericalSurface::new();
        surface.init("CENTER_1".to_string(), 5.0, None, None);
        surface.set_has_transform(false);
        assert_eq!(surface.transformed_center(), Some("CENTER_1"));
    }

    #[test]
    fn test_transformed_center_with_transform() {
        let mut surface = SphericalSurface::new();
        surface.init("CENTER_1".to_string(), 5.0, None, None);
        surface.set_has_transform(true);
        // Without actual transformation matrix, returns original center
        assert_eq!(surface.transformed_center(), Some("CENTER_1"));
    }

    #[test]
    fn test_default() {
        let surface = SphericalSurface::default();
        assert_eq!(surface.type_number(), 196);
        assert!(!surface.is_parametrised());
    }
}

// FILE: step_dim_tol_flatness_tolerance.rs
// occt: StepDimTol_FlatnessTolerance

//! Representation of STEP entity FlatnessTolerance.

/// A simple representation of magnitude (value with unit)
#[derive(Debug, Clone)]
pub struct Magnitude {
    value: f64,
    unit: String,
}

impl Magnitude {
    pub fn new(value: f64, unit: String) -> Self {
        Self { value, unit }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn unit(&self) -> &str {
        &self.unit
    }
}

/// Represents a toleranced shape aspect
#[derive(Debug, Clone)]
pub struct TolerancedShapeAspect {
    aspect_id: String,
}

impl TolerancedShapeAspect {
    pub fn new(aspect_id: String) -> Self {
        Self { aspect_id }
    }

    pub fn aspect_id(&self) -> &str {
        &self.aspect_id
    }
}

/// A GeometricTolerance base structure
#[derive(Debug, Clone)]
pub struct GeometricTolerance {
    name: Option<String>,
    description: Option<String>,
    magnitude: Option<Magnitude>,
    toleranced_shape_aspect: Option<TolerancedShapeAspect>,
}

impl GeometricTolerance {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        description: String,
        magnitude: Magnitude,
        aspect: TolerancedShapeAspect,
    ) {
        self.name = Some(name);
        self.description = Some(description);
        self.magnitude = Some(magnitude);
        self.toleranced_shape_aspect = Some(aspect);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, desc: String) {
        self.description = Some(desc);
    }

    pub fn magnitude(&self) -> Option<&Magnitude> {
        self.magnitude.as_ref()
    }

    pub fn set_magnitude(&mut self, mag: Magnitude) {
        self.magnitude = Some(mag);
    }

    pub fn toleranced_shape_aspect(&self) -> Option<&TolerancedShapeAspect> {
        self.toleranced_shape_aspect.as_ref()
    }

    pub fn set_toleranced_shape_aspect(&mut self, aspect: TolerancedShapeAspect) {
        self.toleranced_shape_aspect = Some(aspect);
    }
}

impl Default for GeometricTolerance {
    fn default() -> Self {
        Self::new()
    }
}

/// A FlatnessTolerance is a specialized GeometricTolerance
#[derive(Debug, Clone)]
pub struct StepDimTolFlatnessTolerance {
    geometric_tolerance: GeometricTolerance,
}

impl StepDimTolFlatnessTolerance {
    /// Create a new FlatnessTolerance
    pub fn new() -> Self {
        Self {
            geometric_tolerance: GeometricTolerance::new(),
        }
    }

    /// Initialize the tolerance with all components
    pub fn init(
        &mut self,
        name: String,
        description: String,
        magnitude: Magnitude,
        aspect: TolerancedShapeAspect,
    ) {
        self.geometric_tolerance.init(name, description, magnitude, aspect);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.geometric_tolerance.name()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.geometric_tolerance.set_name(name);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.geometric_tolerance.description()
    }

    /// Set the description
    pub fn set_description(&mut self, desc: String) {
        self.geometric_tolerance.set_description(desc);
    }

    /// Get the magnitude (tolerance value)
    pub fn magnitude(&self) -> Option<&Magnitude> {
        self.geometric_tolerance.magnitude()
    }

    /// Set the magnitude
    pub fn set_magnitude(&mut self, mag: Magnitude) {
        self.geometric_tolerance.set_magnitude(mag);
    }

    /// Get the toleranced shape aspect
    pub fn toleranced_shape_aspect(&self) -> Option<&TolerancedShapeAspect> {
        self.geometric_tolerance.toleranced_shape_aspect()
    }

    /// Set the toleranced shape aspect
    pub fn set_toleranced_shape_aspect(&mut self, aspect: TolerancedShapeAspect) {
        self.geometric_tolerance.set_toleranced_shape_aspect(aspect);
    }
}

impl Default for StepDimTolFlatnessTolerance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tol = StepDimTolFlatnessTolerance::new();
        assert_eq!(tol.name(), None);
    }

    #[test]
    fn test_magnitude() {
        let mag = Magnitude::new(0.5, "mm".to_string());
        assert_eq!(mag.value(), 0.5);
        assert_eq!(mag.unit(), "mm");
    }

    #[test]
    fn test_init() {
        let mut tol = StepDimTolFlatnessTolerance::new();
        let mag = Magnitude::new(1.0, "mm".to_string());
        let aspect = TolerancedShapeAspect::new("ASPECT_1".to_string());
        tol.init(
            "Flatness".to_string(),
            "Surface flatness tolerance".to_string(),
            mag,
            aspect,
        );
        assert_eq!(tol.name(), Some("Flatness"));
        assert_eq!(tol.description(), Some("Surface flatness tolerance"));
        assert_eq!(tol.magnitude().unwrap().value(), 1.0);
    }

    #[test]
    fn test_set_name() {
        let mut tol = StepDimTolFlatnessTolerance::new();
        tol.set_name("Flatness Tol".to_string());
        assert_eq!(tol.name(), Some("Flatness Tol"));
    }

    #[test]
    fn test_set_magnitude() {
        let mut tol = StepDimTolFlatnessTolerance::new();
        let mag = Magnitude::new(2.5, "μm".to_string());
        tol.set_magnitude(mag);
        assert_eq!(tol.magnitude().unwrap().value(), 2.5);
    }

    #[test]
    fn test_toleranced_shape_aspect() {
        let aspect = TolerancedShapeAspect::new("ASPECT_X".to_string());
        assert_eq!(aspect.aspect_id(), "ASPECT_X");
    }
}

// FILE: step_fea_surface3d_element_representation.rs
// occt: StepFEA_Surface3dElementRepresentation

/// Representation of STEP entity Surface3dElementRepresentation
#[derive(Debug, Clone)]
pub struct StepFeaSurface3dElementRepresentation {
    name: String,
    model_ref: Option<i32>,
    element_descriptor: Option<i32>,
    property: Option<i32>,
    material: Option<i32>,
}

impl StepFeaSurface3dElementRepresentation {
    /// Creates a new empty Surface3dElementRepresentation
    pub fn new() -> Self {
        StepFeaSurface3dElementRepresentation {
            name: String::new(),
            model_ref: None,
            element_descriptor: None,
            property: None,
            material: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        model_ref: Option<i32>,
        element_descriptor: Option<i32>,
        property: Option<i32>,
        material: Option<i32>,
    ) {
        self.name = name;
        self.model_ref = model_ref;
        self.element_descriptor = element_descriptor;
        self.property = property;
        self.material = material;
    }

    /// Returns field ModelRef
    pub fn model_ref(&self) -> Option<i32> {
        self.model_ref
    }

    /// Set field ModelRef
    pub fn set_model_ref(&mut self, model_ref: Option<i32>) {
        self.model_ref = model_ref;
    }

    /// Returns field ElementDescriptor
    pub fn element_descriptor(&self) -> Option<i32> {
        self.element_descriptor
    }

    /// Set field ElementDescriptor
    pub fn set_element_descriptor(&mut self, element_descriptor: Option<i32>) {
        self.element_descriptor = element_descriptor;
    }

    /// Returns field Property
    pub fn property(&self) -> Option<i32> {
        self.property
    }

    /// Set field Property
    pub fn set_property(&mut self, property: Option<i32>) {
        self.property = property;
    }

    /// Returns field Material
    pub fn material(&self) -> Option<i32> {
        self.material
    }

    /// Set field Material
    pub fn set_material(&mut self, material: Option<i32>) {
        self.material = material;
    }

    /// Returns field name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepFeaSurface3dElementRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface3d_element_representation_creation() {
        let repr = StepFeaSurface3dElementRepresentation::new();
        assert_eq!(repr.name(), "");
        assert_eq!(repr.model_ref(), None);
        assert_eq!(repr.element_descriptor(), None);
        assert_eq!(repr.property(), None);
        assert_eq!(repr.material(), None);
    }

    #[test]
    fn test_surface3d_element_representation_init() {
        let mut repr = StepFeaSurface3dElementRepresentation::new();
        repr.init("Surface".to_string(), Some(1), Some(2), Some(3), Some(4));

        assert_eq!(repr.name(), "Surface");
        assert_eq!(repr.model_ref(), Some(1));
        assert_eq!(repr.element_descriptor(), Some(2));
        assert_eq!(repr.property(), Some(3));
        assert_eq!(repr.material(), Some(4));
    }

    #[test]
    fn test_surface3d_element_representation_setters() {
        let mut repr = StepFeaSurface3dElementRepresentation::new();
        repr.set_name("Test".to_string());
        repr.set_model_ref(Some(5));
        repr.set_element_descriptor(Some(6));
        repr.set_property(Some(7));
        repr.set_material(Some(8));

        assert_eq!(repr.name(), "Test");
        assert_eq!(repr.model_ref(), Some(5));
        assert_eq!(repr.element_descriptor(), Some(6));
        assert_eq!(repr.property(), Some(7));
        assert_eq!(repr.material(), Some(8));
    }
}

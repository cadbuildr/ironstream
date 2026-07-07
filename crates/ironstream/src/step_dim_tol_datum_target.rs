// FILE: step_dim_tol_datum_target.rs
// occt: StepDimTol_DatumTarget

//! Representation of STEP entity DatumTarget in dimensional and tolerancing.

/// Represents the product definition shape
#[derive(Debug, Clone)]
pub struct ProductDefinitionShape {
    id: String,
}

impl ProductDefinitionShape {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Logical value for STEP data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Logical {
    True,
    False,
    Unknown,
}

/// A ShapeAspect base for DatumTarget
#[derive(Debug, Clone)]
pub struct ShapeAspect {
    name: Option<String>,
    description: Option<String>,
    of_shape: Option<ProductDefinitionShape>,
    product_definitional: Logical,
}

impl ShapeAspect {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            of_shape: None,
            product_definitional: Logical::Unknown,
        }
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

    pub fn of_shape(&self) -> Option<&ProductDefinitionShape> {
        self.of_shape.as_ref()
    }

    pub fn set_of_shape(&mut self, shape: ProductDefinitionShape) {
        self.of_shape = Some(shape);
    }

    pub fn product_definitional(&self) -> Logical {
        self.product_definitional
    }

    pub fn set_product_definitional(&mut self, value: Logical) {
        self.product_definitional = value;
    }
}

impl Default for ShapeAspect {
    fn default() -> Self {
        Self::new()
    }
}

/// A DatumTarget is a ShapeAspect that identifies a datum target
#[derive(Debug, Clone)]
pub struct StepDimTolDatumTarget {
    shape_aspect: ShapeAspect,
    target_id: Option<String>,
}

impl StepDimTolDatumTarget {
    /// Create a new DatumTarget
    pub fn new() -> Self {
        Self {
            shape_aspect: ShapeAspect::new(),
            target_id: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        description: String,
        of_shape: ProductDefinitionShape,
        product_definitional: Logical,
        target_id: String,
    ) {
        self.shape_aspect.set_name(name);
        self.shape_aspect.set_description(description);
        self.shape_aspect.set_of_shape(of_shape);
        self.shape_aspect.set_product_definitional(product_definitional);
        self.target_id = Some(target_id);
    }

    /// Get the name from the shape aspect
    pub fn name(&self) -> Option<&str> {
        self.shape_aspect.name()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.shape_aspect.set_name(name);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.shape_aspect.description()
    }

    /// Set the description
    pub fn set_description(&mut self, desc: String) {
        self.shape_aspect.set_description(desc);
    }

    /// Get the target ID
    pub fn target_id(&self) -> Option<&str> {
        self.target_id.as_deref()
    }

    /// Set the target ID
    pub fn set_target_id(&mut self, id: String) {
        self.target_id = Some(id);
    }
}

impl Default for StepDimTolDatumTarget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dt = StepDimTolDatumTarget::new();
        assert_eq!(dt.target_id(), None);
    }

    #[test]
    fn test_init() {
        let mut dt = StepDimTolDatumTarget::new();
        let shape = ProductDefinitionShape::new("SHAPE_1".to_string());
        dt.init(
            "aspect".to_string(),
            "datum target".to_string(),
            shape,
            Logical::True,
            "TARGET_A".to_string(),
        );
        assert_eq!(dt.name(), Some("aspect"));
        assert_eq!(dt.description(), Some("datum target"));
        assert_eq!(dt.target_id(), Some("TARGET_A"));
    }

    #[test]
    fn test_set_target_id() {
        let mut dt = StepDimTolDatumTarget::new();
        dt.set_target_id("TID_X".to_string());
        assert_eq!(dt.target_id(), Some("TID_X"));
    }

    #[test]
    fn test_set_name() {
        let mut dt = StepDimTolDatumTarget::new();
        dt.set_name("My Target".to_string());
        assert_eq!(dt.name(), Some("My Target"));
    }

    #[test]
    fn test_set_description() {
        let mut dt = StepDimTolDatumTarget::new();
        dt.set_description("A test datum".to_string());
        assert_eq!(dt.description(), Some("A test datum"));
    }

    #[test]
    fn test_product_definition_shape() {
        let shape = ProductDefinitionShape::new("SHAPE_ID".to_string());
        assert_eq!(shape.id(), "SHAPE_ID");
    }

    #[test]
    fn test_logical() {
        assert_eq!(Logical::True, Logical::True);
        assert_ne!(Logical::True, Logical::False);
        assert_eq!(Logical::Unknown, Logical::Unknown);
    }
}

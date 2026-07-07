// FILE: step_repr_shape_aspect.rs
// occt: StepRepr_ShapeAspect

/// Placeholder for ProductDefinitionShape
#[derive(Clone, Debug, PartialEq)]
pub struct ProductDefinitionShape {
    name: String,
}

/// Logical type for STEP (can be True, False, or Unknown)
#[derive(Clone, Debug, PartialEq)]
pub enum StepLogical {
    True,
    False,
    Unknown,
}

/// Represents a shape aspect in STEP - a facet of a shape with properties.
pub struct ShapeAspect {
    name: Option<String>,
    description: Option<String>,
    of_shape: Option<ProductDefinitionShape>,
    product_definitional: Option<StepLogical>,
}

impl ShapeAspect {
    /// Create a new ShapeAspect
    pub fn new() -> Self {
        ShapeAspect {
            name: None,
            description: None,
            of_shape: None,
            product_definitional: None,
        }
    }

    /// Initialize shape aspect with all attributes
    pub fn init(
        &mut self,
        name: String,
        description: String,
        of_shape: ProductDefinitionShape,
        product_definitional: StepLogical,
    ) {
        self.name = Some(name);
        self.description = Some(description);
        self.of_shape = Some(of_shape);
        self.product_definitional = Some(product_definitional);
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the of shape reference
    pub fn set_of_shape(&mut self, of_shape: ProductDefinitionShape) {
        self.of_shape = Some(of_shape);
    }

    /// Get the of shape reference
    pub fn of_shape(&self) -> Option<&ProductDefinitionShape> {
        self.of_shape.as_ref()
    }

    /// Set the product definitional flag
    pub fn set_product_definitional(&mut self, value: StepLogical) {
        self.product_definitional = Some(value);
    }

    /// Get the product definitional flag
    pub fn product_definitional(&self) -> Option<&StepLogical> {
        self.product_definitional.as_ref()
    }
}

impl Default for ShapeAspect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let aspect = ShapeAspect::new();
        assert_eq!(aspect.name(), None);
        assert_eq!(aspect.description(), None);
        assert_eq!(aspect.of_shape(), None);
        assert_eq!(aspect.product_definitional(), None);
    }

    #[test]
    fn test_init() {
        let mut aspect = ShapeAspect::new();
        let pds = ProductDefinitionShape {
            name: "pds".to_string(),
        };
        aspect.init(
            "aspect1".to_string(),
            "description".to_string(),
            pds.clone(),
            StepLogical::True,
        );
        assert_eq!(aspect.name(), Some("aspect1"));
        assert_eq!(aspect.description(), Some("description"));
        assert_eq!(aspect.of_shape(), Some(&pds));
        assert_eq!(aspect.product_definitional(), Some(&StepLogical::True));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut aspect = ShapeAspect::new();
        aspect.set_name("TestAspect".to_string());
        assert_eq!(aspect.name(), Some("TestAspect"));
    }

    #[test]
    fn test_set_product_definitional() {
        let mut aspect = ShapeAspect::new();
        aspect.set_product_definitional(StepLogical::False);
        assert_eq!(aspect.product_definitional(), Some(&StepLogical::False));
    }
}

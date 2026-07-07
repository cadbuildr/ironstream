// FILE: step_shape_context_dependent_shape_representation.rs
// occt: StepShape_ContextDependentShapeRepresentation

//! Representation of STEP entity ContextDependentShapeRepresentation

#[derive(Clone, Debug)]
pub struct ContextDependentShapeRepresentation {
    rep_rel: Option<String>, // Placeholder for ShapeRepresentationRelationship handle
    pro_rel: Option<String>, // Placeholder for ProductDefinitionShape handle
}

impl ContextDependentShapeRepresentation {
    /// Constructor
    pub fn new() -> Self {
        ContextDependentShapeRepresentation {
            rep_rel: None,
            pro_rel: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, rep_rel: Option<String>, pro_rel: Option<String>) {
        self.rep_rel = rep_rel;
        self.pro_rel = pro_rel;
    }

    /// Returns RepresentationRelation
    pub fn representation_relation(&self) -> &Option<String> {
        &self.rep_rel
    }

    /// Set RepresentationRelation
    pub fn set_representation_relation(&mut self, rep_rel: Option<String>) {
        self.rep_rel = rep_rel;
    }

    /// Returns RepresentedProductRelation
    pub fn represented_product_relation(&self) -> &Option<String> {
        &self.pro_rel
    }

    /// Set RepresentedProductRelation
    pub fn set_represented_product_relation(&mut self, pro_rel: Option<String>) {
        self.pro_rel = pro_rel;
    }
}

impl Default for ContextDependentShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cdsr = ContextDependentShapeRepresentation::new();
        assert!(cdsr.representation_relation().is_none());
        assert!(cdsr.represented_product_relation().is_none());
    }

    #[test]
    fn test_init() {
        let mut cdsr = ContextDependentShapeRepresentation::new();
        cdsr.init(
            Some("rel1".to_string()),
            Some("prod1".to_string()),
        );
        assert_eq!(cdsr.representation_relation(), &Some("rel1".to_string()));
        assert_eq!(cdsr.represented_product_relation(), &Some("prod1".to_string()));
    }

    #[test]
    fn test_set_representation_relation() {
        let mut cdsr = ContextDependentShapeRepresentation::new();
        cdsr.set_representation_relation(Some("new_rel".to_string()));
        assert_eq!(cdsr.representation_relation(), &Some("new_rel".to_string()));
    }

    #[test]
    fn test_set_represented_product_relation() {
        let mut cdsr = ContextDependentShapeRepresentation::new();
        cdsr.set_represented_product_relation(Some("new_prod".to_string()));
        assert_eq!(cdsr.represented_product_relation(), &Some("new_prod".to_string()));
    }
}

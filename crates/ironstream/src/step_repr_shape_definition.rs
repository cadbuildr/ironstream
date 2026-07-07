// FILE: step_repr_shape_definition.rs
// occt: StepRepr_ShapeDefinition

/// Placeholder for ProductDefinitionShape
#[derive(Clone, Debug, PartialEq)]
pub struct ProductDefinitionShape {
    id: String,
}

/// Placeholder for ShapeAspect
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeAspect {
    id: String,
}

/// Placeholder for ShapeAspectRelationship
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeAspectRelationship {
    id: String,
}

/// SELECT type that can contain various shape definition types
#[derive(Clone, Debug, PartialEq)]
pub enum ShapeDefinition {
    ProductDefinitionShape(ProductDefinitionShape),
    ShapeAspect(ShapeAspect),
    ShapeAspectRelationship(ShapeAspectRelationship),
}

impl ShapeDefinition {
    /// Create a new shape definition
    pub fn new() -> Self {
        ShapeDefinition::ProductDefinitionShape(ProductDefinitionShape {
            id: String::new(),
        })
    }

    /// Get the case number
    pub fn case_num(&self) -> i32 {
        match self {
            ShapeDefinition::ProductDefinitionShape(_) => 1,
            ShapeDefinition::ShapeAspect(_) => 2,
            ShapeDefinition::ShapeAspectRelationship(_) => 3,
        }
    }

    /// Get as ProductDefinitionShape if applicable
    pub fn product_definition_shape(&self) -> Option<&ProductDefinitionShape> {
        match self {
            ShapeDefinition::ProductDefinitionShape(p) => Some(p),
            _ => None,
        }
    }

    /// Get as ShapeAspect if applicable
    pub fn shape_aspect(&self) -> Option<&ShapeAspect> {
        match self {
            ShapeDefinition::ShapeAspect(s) => Some(s),
            _ => None,
        }
    }

    /// Get as ShapeAspectRelationship if applicable
    pub fn shape_aspect_relationship(&self) -> Option<&ShapeAspectRelationship> {
        match self {
            ShapeDefinition::ShapeAspectRelationship(s) => Some(s),
            _ => None,
        }
    }
}

impl Default for ShapeDefinition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_definition_shape() {
        let pds = ProductDefinitionShape {
            id: "pds1".to_string(),
        };
        let def = ShapeDefinition::ProductDefinitionShape(pds.clone());
        assert_eq!(def.case_num(), 1);
        assert_eq!(def.product_definition_shape(), Some(&pds));
        assert_eq!(def.shape_aspect(), None);
    }

    #[test]
    fn test_shape_aspect() {
        let aspect = ShapeAspect {
            id: "sa1".to_string(),
        };
        let def = ShapeDefinition::ShapeAspect(aspect.clone());
        assert_eq!(def.case_num(), 2);
        assert_eq!(def.shape_aspect(), Some(&aspect));
    }

    #[test]
    fn test_shape_aspect_relationship() {
        let rel = ShapeAspectRelationship {
            id: "sar1".to_string(),
        };
        let def = ShapeDefinition::ShapeAspectRelationship(rel.clone());
        assert_eq!(def.case_num(), 3);
        assert_eq!(def.shape_aspect_relationship(), Some(&rel));
    }

    #[test]
    fn test_default_is_pds() {
        let def = ShapeDefinition::default();
        assert_eq!(def.case_num(), 1);
        assert!(def.product_definition_shape().is_some());
    }
}

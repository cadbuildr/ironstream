// FILE: step_repr_represented_definition.rs
// occt: StepRepr_RepresentedDefinition

/// Placeholder for GeneralProperty
#[derive(Clone, Debug, PartialEq)]
pub struct GeneralProperty {
    id: String,
}

/// Placeholder for PropertyDefinition
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyDefinition {
    id: String,
}

/// Placeholder for PropertyDefinitionRelationship
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyDefinitionRelationship {
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

/// SELECT type that can contain various definition types
#[derive(Clone, Debug, PartialEq)]
pub enum RepresentedDefinition {
    GeneralProperty(GeneralProperty),
    PropertyDefinition(PropertyDefinition),
    PropertyDefinitionRelationship(PropertyDefinitionRelationship),
    ShapeAspect(ShapeAspect),
    ShapeAspectRelationship(ShapeAspectRelationship),
}

impl RepresentedDefinition {
    /// Create a new represented definition
    pub fn new() -> Self {
        RepresentedDefinition::GeneralProperty(GeneralProperty {
            id: String::new(),
        })
    }

    /// Get the case number
    pub fn case_num(&self) -> i32 {
        match self {
            RepresentedDefinition::GeneralProperty(_) => 1,
            RepresentedDefinition::PropertyDefinition(_) => 2,
            RepresentedDefinition::PropertyDefinitionRelationship(_) => 3,
            RepresentedDefinition::ShapeAspect(_) => 4,
            RepresentedDefinition::ShapeAspectRelationship(_) => 5,
        }
    }

    /// Get as GeneralProperty if applicable
    pub fn general_property(&self) -> Option<&GeneralProperty> {
        match self {
            RepresentedDefinition::GeneralProperty(p) => Some(p),
            _ => None,
        }
    }

    /// Get as PropertyDefinition if applicable
    pub fn property_definition(&self) -> Option<&PropertyDefinition> {
        match self {
            RepresentedDefinition::PropertyDefinition(p) => Some(p),
            _ => None,
        }
    }

    /// Get as PropertyDefinitionRelationship if applicable
    pub fn property_definition_relationship(&self) -> Option<&PropertyDefinitionRelationship> {
        match self {
            RepresentedDefinition::PropertyDefinitionRelationship(p) => Some(p),
            _ => None,
        }
    }

    /// Get as ShapeAspect if applicable
    pub fn shape_aspect(&self) -> Option<&ShapeAspect> {
        match self {
            RepresentedDefinition::ShapeAspect(s) => Some(s),
            _ => None,
        }
    }

    /// Get as ShapeAspectRelationship if applicable
    pub fn shape_aspect_relationship(&self) -> Option<&ShapeAspectRelationship> {
        match self {
            RepresentedDefinition::ShapeAspectRelationship(s) => Some(s),
            _ => None,
        }
    }
}

impl Default for RepresentedDefinition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_property() {
        let prop = GeneralProperty {
            id: "prop1".to_string(),
        };
        let def = RepresentedDefinition::GeneralProperty(prop.clone());
        assert_eq!(def.case_num(), 1);
        assert_eq!(def.general_property(), Some(&prop));
    }

    #[test]
    fn test_property_definition() {
        let prop = PropertyDefinition {
            id: "pdef1".to_string(),
        };
        let def = RepresentedDefinition::PropertyDefinition(prop.clone());
        assert_eq!(def.case_num(), 2);
        assert_eq!(def.property_definition(), Some(&prop));
    }

    #[test]
    fn test_shape_aspect() {
        let aspect = ShapeAspect {
            id: "aspect1".to_string(),
        };
        let def = RepresentedDefinition::ShapeAspect(aspect.clone());
        assert_eq!(def.case_num(), 4);
        assert_eq!(def.shape_aspect(), Some(&aspect));
    }

    #[test]
    fn test_case_nums() {
        let def1 = RepresentedDefinition::GeneralProperty(GeneralProperty {
            id: "1".to_string(),
        });
        let def2 = RepresentedDefinition::PropertyDefinition(PropertyDefinition {
            id: "2".to_string(),
        });
        let def3 = RepresentedDefinition::PropertyDefinitionRelationship(
            PropertyDefinitionRelationship {
                id: "3".to_string(),
            },
        );
        let def4 = RepresentedDefinition::ShapeAspect(ShapeAspect {
            id: "4".to_string(),
        });
        let def5 = RepresentedDefinition::ShapeAspectRelationship(ShapeAspectRelationship {
            id: "5".to_string(),
        });
        assert_eq!(def1.case_num(), 1);
        assert_eq!(def2.case_num(), 2);
        assert_eq!(def3.case_num(), 3);
        assert_eq!(def4.case_num(), 4);
        assert_eq!(def5.case_num(), 5);
    }
}

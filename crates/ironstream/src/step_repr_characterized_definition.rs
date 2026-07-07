// FILE: step_repr_characterized_definition.rs
// occt: StepRepr_CharacterizedDefinition

use std::fmt;

/// StepRepr_CharacterizedDefinition: A SELECT type that can hold multiple different types
/// 1 -> CharacterizedObject from StepBasic
/// 2 -> ProductDefinition from StepBasic
/// 3 -> ProductDefinitionRelationship from StepBasic
/// 4 -> ProductDefinitionShape from StepRepr
/// 5 -> ShapeAspect from StepRepr
/// 6 -> ShapeAspectRelationship from StepRepr
/// 7 -> DocumentFile from StepBasic
#[derive(Clone, Debug)]
pub enum StepReprCharacterizedDefinition {
    CharacterizedObject,
    ProductDefinition,
    ProductDefinitionRelationship,
    ProductDefinitionShape,
    ShapeAspect,
    ShapeAspectRelationship,
    DocumentFile,
    Unknown,
}

impl StepReprCharacterizedDefinition {
    /// Create a new empty CharacterizedDefinition
    pub fn new() -> Self {
        StepReprCharacterizedDefinition::Unknown
    }

    /// Recognize a kind of CharacterizedDefinition select type
    /// Returns the case number (1-7) or 0 for unknown
    pub fn case_num(&self) -> i32 {
        match self {
            StepReprCharacterizedDefinition::CharacterizedObject => 1,
            StepReprCharacterizedDefinition::ProductDefinition => 2,
            StepReprCharacterizedDefinition::ProductDefinitionRelationship => 3,
            StepReprCharacterizedDefinition::ProductDefinitionShape => 4,
            StepReprCharacterizedDefinition::ShapeAspect => 5,
            StepReprCharacterizedDefinition::ShapeAspectRelationship => 6,
            StepReprCharacterizedDefinition::DocumentFile => 7,
            StepReprCharacterizedDefinition::Unknown => 0,
        }
    }
}

impl Default for StepReprCharacterizedDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StepReprCharacterizedDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepReprCharacterizedDefinition::CharacterizedObject => write!(f, "CharacterizedObject"),
            StepReprCharacterizedDefinition::ProductDefinition => write!(f, "ProductDefinition"),
            StepReprCharacterizedDefinition::ProductDefinitionRelationship => {
                write!(f, "ProductDefinitionRelationship")
            }
            StepReprCharacterizedDefinition::ProductDefinitionShape => {
                write!(f, "ProductDefinitionShape")
            }
            StepReprCharacterizedDefinition::ShapeAspect => write!(f, "ShapeAspect"),
            StepReprCharacterizedDefinition::ShapeAspectRelationship => {
                write!(f, "ShapeAspectRelationship")
            }
            StepReprCharacterizedDefinition::DocumentFile => write!(f, "DocumentFile"),
            StepReprCharacterizedDefinition::Unknown => write!(f, "Unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_numbers() {
        assert_eq!(StepReprCharacterizedDefinition::CharacterizedObject.case_num(), 1);
        assert_eq!(StepReprCharacterizedDefinition::ProductDefinition.case_num(), 2);
        assert_eq!(
            StepReprCharacterizedDefinition::ProductDefinitionRelationship.case_num(),
            3
        );
        assert_eq!(StepReprCharacterizedDefinition::ProductDefinitionShape.case_num(), 4);
        assert_eq!(StepReprCharacterizedDefinition::ShapeAspect.case_num(), 5);
        assert_eq!(StepReprCharacterizedDefinition::ShapeAspectRelationship.case_num(), 6);
        assert_eq!(StepReprCharacterizedDefinition::DocumentFile.case_num(), 7);
        assert_eq!(StepReprCharacterizedDefinition::Unknown.case_num(), 0);
    }

    #[test]
    fn test_default() {
        let def = StepReprCharacterizedDefinition::default();
        assert_eq!(def.case_num(), 0);
    }
}

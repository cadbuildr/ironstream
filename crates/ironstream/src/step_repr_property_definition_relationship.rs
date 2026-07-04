// FILE: step_repr_property_definition_relationship.rs
// occt: StepRepr_PropertyDefinitionRelationship

/// StepRepr_PropertyDefinitionRelationship: Representation of STEP entity PropertyDefinitionRelationship
#[derive(Clone, Debug)]
pub struct StepReprPropertyDefinitionRelationship {
    name: String,
    description: String,
    relating_property_definition: String,  // Simplified: storing identifier
    related_property_definition: String,   // Simplified: storing identifier
}

impl StepReprPropertyDefinitionRelationship {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprPropertyDefinitionRelationship {
            name: String::new(),
            description: String::new(),
            relating_property_definition: String::new(),
            related_property_definition: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        description: String,
        relating: String,
        related: String,
    ) {
        self.name = name;
        self.description = description;
        self.relating_property_definition = relating;
        self.related_property_definition = related;
    }

    /// Returns field Name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field Name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns field Description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set field Description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Returns field RelatingPropertyDefinition
    pub fn relating_property_definition(&self) -> &str {
        &self.relating_property_definition
    }

    /// Set field RelatingPropertyDefinition
    pub fn set_relating_property_definition(&mut self, definition: String) {
        self.relating_property_definition = definition;
    }

    /// Returns field RelatedPropertyDefinition
    pub fn related_property_definition(&self) -> &str {
        &self.related_property_definition
    }

    /// Set field RelatedPropertyDefinition
    pub fn set_related_property_definition(&mut self, definition: String) {
        self.related_property_definition = definition;
    }
}

impl Default for StepReprPropertyDefinitionRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let pdr = StepReprPropertyDefinitionRelationship::new();
        assert_eq!(pdr.name(), "");
        assert_eq!(pdr.description(), "");
        assert_eq!(pdr.relating_property_definition(), "");
        assert_eq!(pdr.related_property_definition(), "");
    }

    #[test]
    fn test_init() {
        let mut pdr = StepReprPropertyDefinitionRelationship::new();
        pdr.init(
            "rel1".to_string(),
            "description".to_string(),
            "relating".to_string(),
            "related".to_string(),
        );
        assert_eq!(pdr.name(), "rel1");
        assert_eq!(pdr.description(), "description");
        assert_eq!(pdr.relating_property_definition(), "relating");
        assert_eq!(pdr.related_property_definition(), "related");
    }

    #[test]
    fn test_setters() {
        let mut pdr = StepReprPropertyDefinitionRelationship::new();
        pdr.set_name("newrel".to_string());
        pdr.set_description("newdesc".to_string());
        pdr.set_relating_property_definition("newrelating".to_string());
        pdr.set_related_property_definition("newrelated".to_string());
        assert_eq!(pdr.name(), "newrel");
        assert_eq!(pdr.description(), "newdesc");
        assert_eq!(pdr.relating_property_definition(), "newrelating");
        assert_eq!(pdr.related_property_definition(), "newrelated");
    }
}

// FILE: step_basic_general_property_association.rs
// occt: StepBasic_GeneralPropertyAssociation

/// Representation of STEP entity GeneralPropertyAssociation
#[derive(Clone, Debug)]
pub struct GeneralPropertyAssociation {
    name: Option<String>,
    description: Option<String>,
    general_property: Option<String>,
    property_definition: Option<String>,
}

impl GeneralPropertyAssociation {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            general_property: None,
            property_definition: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        description: String,
        general_property: String,
        property_definition: String,
    ) {
        self.name = Some(name);
        self.description = Some(description);
        self.general_property = Some(general_property);
        self.property_definition = Some(property_definition);
    }

    /// Get name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Get general property
    pub fn general_property(&self) -> Option<&str> {
        self.general_property.as_deref()
    }

    /// Set general property
    pub fn set_general_property(&mut self, general_property: String) {
        self.general_property = Some(general_property);
    }

    /// Get property definition
    pub fn property_definition(&self) -> Option<&str> {
        self.property_definition.as_deref()
    }

    /// Set property definition
    pub fn set_property_definition(&mut self, property_definition: String) {
        self.property_definition = Some(property_definition);
    }
}

impl Default for GeneralPropertyAssociation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assoc = GeneralPropertyAssociation::new();
        assert!(assoc.name().is_none());
        assert!(assoc.description().is_none());
        assert!(assoc.general_property().is_none());
        assert!(assoc.property_definition().is_none());
    }

    #[test]
    fn test_init() {
        let mut assoc = GeneralPropertyAssociation::new();
        assoc.init(
            "name1".to_string(),
            "desc1".to_string(),
            "prop1".to_string(),
            "propdef1".to_string(),
        );
        assert_eq!(assoc.name(), Some("name1"));
        assert_eq!(assoc.description(), Some("desc1"));
        assert_eq!(assoc.general_property(), Some("prop1"));
        assert_eq!(assoc.property_definition(), Some("propdef1"));
    }

    #[test]
    fn test_set_fields() {
        let mut assoc = GeneralPropertyAssociation::new();
        assoc.set_name("name2".to_string());
        assoc.set_description("desc2".to_string());
        assoc.set_general_property("prop2".to_string());
        assoc.set_property_definition("propdef2".to_string());

        assert_eq!(assoc.name(), Some("name2"));
        assert_eq!(assoc.description(), Some("desc2"));
        assert_eq!(assoc.general_property(), Some("prop2"));
        assert_eq!(assoc.property_definition(), Some("propdef2"));
    }
}

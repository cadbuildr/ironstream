// FILE: step_repr_configuration_item.rs
// occt: StepRepr_ConfigurationItem

/// StepRepr_ConfigurationItem: Representation of STEP entity ConfigurationItem
#[derive(Clone, Debug)]
pub struct StepReprConfigurationItem {
    id: String,
    name: String,
    description: Option<String>,
    item_concept: String,
    purpose: Option<String>,
}

impl StepReprConfigurationItem {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprConfigurationItem {
            id: String::new(),
            name: String::new(),
            description: None,
            item_concept: String::new(),
            purpose: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        id: String,
        name: String,
        has_description: bool,
        description: Option<String>,
        item_concept: String,
        has_purpose: bool,
        purpose: Option<String>,
    ) {
        self.id = id;
        self.name = name;
        self.description = if has_description { description } else { None };
        self.item_concept = item_concept;
        self.purpose = if has_purpose { purpose } else { None };
    }

    /// Returns field Id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set field Id
    pub fn set_id(&mut self, id: String) {
        self.id = id;
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
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set field Description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Returns True if optional field Description is defined
    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    /// Returns field ItemConcept
    pub fn item_concept(&self) -> &str {
        &self.item_concept
    }

    /// Set field ItemConcept
    pub fn set_item_concept(&mut self, item_concept: String) {
        self.item_concept = item_concept;
    }

    /// Returns field Purpose
    pub fn purpose(&self) -> Option<&str> {
        self.purpose.as_deref()
    }

    /// Set field Purpose
    pub fn set_purpose(&mut self, purpose: String) {
        self.purpose = Some(purpose);
    }

    /// Returns True if optional field Purpose is defined
    pub fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
}

impl Default for StepReprConfigurationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let ci = StepReprConfigurationItem::new();
        assert_eq!(ci.id(), "");
        assert_eq!(ci.name(), "");
        assert!(!ci.has_description());
        assert!(!ci.has_purpose());
    }

    #[test]
    fn test_init() {
        let mut ci = StepReprConfigurationItem::new();
        ci.init(
            "id1".to_string(),
            "name1".to_string(),
            true,
            Some("desc1".to_string()),
            "concept1".to_string(),
            true,
            Some("purpose1".to_string()),
        );
        assert_eq!(ci.id(), "id1");
        assert_eq!(ci.name(), "name1");
        assert!(ci.has_description());
        assert_eq!(ci.description(), Some("desc1"));
        assert!(ci.has_purpose());
        assert_eq!(ci.purpose(), Some("purpose1"));
    }

    #[test]
    fn test_optional_fields() {
        let mut ci = StepReprConfigurationItem::new();
        assert!(!ci.has_description());
        ci.set_description("new_desc".to_string());
        assert!(ci.has_description());
        assert_eq!(ci.description(), Some("new_desc"));
    }

    #[test]
    fn test_setters() {
        let mut ci = StepReprConfigurationItem::new();
        ci.set_id("newid".to_string());
        ci.set_name("newname".to_string());
        ci.set_item_concept("newconcept".to_string());
        assert_eq!(ci.id(), "newid");
        assert_eq!(ci.name(), "newname");
        assert_eq!(ci.item_concept(), "newconcept");
    }
}

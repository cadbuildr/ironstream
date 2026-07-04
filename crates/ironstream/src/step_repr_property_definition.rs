// FILE: step_repr_property_definition.rs
// occt: StepRepr_PropertyDefinition

/// StepRepr_PropertyDefinition: Representation of STEP entity PropertyDefinition
#[derive(Clone, Debug)]
pub struct StepReprPropertyDefinition {
    name: String,
    description: Option<String>,
    definition: i32, // Simplified: storing definition case number
}

impl StepReprPropertyDefinition {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprPropertyDefinition {
            name: String::new(),
            description: None,
            definition: 0,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        has_description: bool,
        description: Option<String>,
        definition: i32,
    ) {
        self.name = name;
        self.description = if has_description { description } else { None };
        self.definition = definition;
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

    /// Returns field Definition
    pub fn definition(&self) -> i32 {
        self.definition
    }

    /// Set field Definition
    pub fn set_definition(&mut self, definition: i32) {
        self.definition = definition;
    }
}

impl Default for StepReprPropertyDefinition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let pd = StepReprPropertyDefinition::new();
        assert_eq!(pd.name(), "");
        assert!(!pd.has_description());
        assert_eq!(pd.definition(), 0);
    }

    #[test]
    fn test_init() {
        let mut pd = StepReprPropertyDefinition::new();
        pd.init("prop1".to_string(), true, Some("desc1".to_string()), 2);
        assert_eq!(pd.name(), "prop1");
        assert!(pd.has_description());
        assert_eq!(pd.definition(), 2);
    }

    #[test]
    fn test_set_description() {
        let mut pd = StepReprPropertyDefinition::new();
        assert!(!pd.has_description());
        pd.set_description("new_desc".to_string());
        assert!(pd.has_description());
        assert_eq!(pd.description(), Some("new_desc"));
    }

    #[test]
    fn test_set_definition() {
        let mut pd = StepReprPropertyDefinition::new();
        pd.set_definition(3);
        assert_eq!(pd.definition(), 3);
    }

    #[test]
    fn test_set_name() {
        let mut pd = StepReprPropertyDefinition::new();
        pd.set_name("new_prop".to_string());
        assert_eq!(pd.name(), "new_prop");
    }
}

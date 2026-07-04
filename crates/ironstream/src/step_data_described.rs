// FILE: step_data_described.rs
// occt: StepData_Described

//! General frame to describe STEP entities with their descriptions.

use std::collections::HashMap;

/// Simplified entity description placeholder
#[derive(Debug, Clone)]
pub struct EntityDescription {
    name: String,
    fields: HashMap<String, String>,
}

impl EntityDescription {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_field(&mut self, name: String, field_type: String) {
        self.fields.insert(name, field_type);
    }

    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }

    pub fn get_field(&self, name: &str) -> Option<&str> {
        self.fields.get(name).map(|s| s.as_str())
    }
}

/// Base trait for described entities
pub trait DescribedEntity {
    /// Returns the description used to define this entity
    fn description(&self) -> Option<&EntityDescription>;

    /// Tells if a described entity is complex (composite)
    fn is_complex(&self) -> bool;

    /// Tells if a step type is matched by this entity
    fn matches(&self, step_type: &str) -> bool;

    /// Tells if a field has a given name
    fn has_field(&self, name: &str) -> bool;

    /// Get field value as string
    fn get_field(&self, name: &str) -> Option<String>;

    /// Fills a check for validation
    fn check(&self) -> bool {
        true
    }

    /// Get shared entities
    fn shared_entities(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Simple described entity (non-composite)
#[derive(Debug, Clone)]
pub struct StepDataDescribedSimple {
    description: EntityDescription,
    values: HashMap<String, String>,
}

impl StepDataDescribedSimple {
    pub fn new(description: EntityDescription) -> Self {
        Self {
            description,
            values: HashMap::new(),
        }
    }

    pub fn set_field_value(&mut self, name: String, value: String) {
        self.values.insert(name, value);
    }
}

impl DescribedEntity for StepDataDescribedSimple {
    fn description(&self) -> Option<&EntityDescription> {
        Some(&self.description)
    }

    fn is_complex(&self) -> bool {
        false
    }

    fn matches(&self, step_type: &str) -> bool {
        self.description.name() == step_type
    }

    fn has_field(&self, name: &str) -> bool {
        self.description.has_field(name)
    }

    fn get_field(&self, name: &str) -> Option<String> {
        self.values.get(name).cloned()
    }
}

/// Complex described entity (composite)
#[derive(Debug, Clone)]
pub struct StepDataDescribedComplex {
    members: Vec<StepDataDescribedSimple>,
}

impl StepDataDescribedComplex {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: StepDataDescribedSimple) {
        self.members.push(member);
    }
}

impl Default for StepDataDescribedComplex {
    fn default() -> Self {
        Self::new()
    }
}

impl DescribedEntity for StepDataDescribedComplex {
    fn description(&self) -> Option<&EntityDescription> {
        self.members.first().and_then(|m| m.description())
    }

    fn is_complex(&self) -> bool {
        true
    }

    fn matches(&self, step_type: &str) -> bool {
        self.members.iter().any(|m| m.matches(step_type))
    }

    fn has_field(&self, name: &str) -> bool {
        self.members.iter().any(|m| m.has_field(name))
    }

    fn get_field(&self, name: &str) -> Option<String> {
        self.members.iter().find_map(|m| m.get_field(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_description() {
        let mut desc = EntityDescription::new("MyEntity".to_string());
        desc.add_field("field1".to_string(), "String".to_string());
        assert!(desc.has_field("field1"));
        assert_eq!(desc.get_field("field1"), Some("String"));
    }

    #[test]
    fn test_simple_entity() {
        let desc = EntityDescription::new("Simple".to_string());
        let entity = StepDataDescribedSimple::new(desc);
        assert!(!entity.is_complex());
        assert!(entity.matches("Simple"));
    }

    #[test]
    fn test_simple_entity_field() {
        let desc = EntityDescription::new("Simple".to_string());
        let mut entity = StepDataDescribedSimple::new(desc);
        entity.set_field_value("name".to_string(), "test".to_string());
        assert_eq!(entity.get_field("name"), Some("test".to_string()));
    }

    #[test]
    fn test_complex_entity() {
        let mut complex = StepDataDescribedComplex::new();
        assert!(complex.is_complex());
        assert_eq!(complex.members.len(), 0);
    }

    #[test]
    fn test_complex_entity_add_member() {
        let mut complex = StepDataDescribedComplex::new();
        let desc = EntityDescription::new("Member".to_string());
        let member = StepDataDescribedSimple::new(desc);
        complex.add_member(member);
        assert!(complex.matches("Member"));
    }
}

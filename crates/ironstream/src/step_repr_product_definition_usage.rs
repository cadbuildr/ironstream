// FILE: step_repr_product_definition_usage.rs
// occt: StepRepr_ProductDefinitionUsage

/// StepRepr_ProductDefinitionUsage: Representation of STEP entity ProductDefinitionUsage
/// Inherits from StepBasic_ProductDefinitionRelationship
#[derive(Clone, Debug)]
pub struct StepReprProductDefinitionUsage {
    id: String,
    name: String,
}

impl StepReprProductDefinitionUsage {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprProductDefinitionUsage {
            id: String::new(),
            name: String::new(),
        }
    }

    /// Get id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set id
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepReprProductDefinitionUsage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let pdu = StepReprProductDefinitionUsage::new();
        assert_eq!(pdu.id(), "");
        assert_eq!(pdu.name(), "");
    }

    #[test]
    fn test_set_id() {
        let mut pdu = StepReprProductDefinitionUsage::new();
        pdu.set_id("usage1".to_string());
        assert_eq!(pdu.id(), "usage1");
    }

    #[test]
    fn test_set_name() {
        let mut pdu = StepReprProductDefinitionUsage::new();
        pdu.set_name("usage_name".to_string());
        assert_eq!(pdu.name(), "usage_name");
    }
}

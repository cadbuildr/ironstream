// FILE: step_repr_product_concept.rs
// occt: StepRepr_ProductConcept

/// StepRepr_ProductConcept: Representation of STEP entity ProductConcept
#[derive(Clone, Debug)]
pub struct StepReprProductConcept {
    id: String,
    name: String,
    description: Option<String>,
    market_context: String, // Simplified: storing identifier
}

impl StepReprProductConcept {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprProductConcept {
            id: String::new(),
            name: String::new(),
            description: None,
            market_context: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        id: String,
        name: String,
        has_description: bool,
        description: Option<String>,
        market_context: String,
    ) {
        self.id = id;
        self.name = name;
        self.description = if has_description { description } else { None };
        self.market_context = market_context;
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

    /// Returns field MarketContext
    pub fn market_context(&self) -> &str {
        &self.market_context
    }

    /// Set field MarketContext
    pub fn set_market_context(&mut self, market_context: String) {
        self.market_context = market_context;
    }
}

impl Default for StepReprProductConcept {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let pc = StepReprProductConcept::new();
        assert_eq!(pc.id(), "");
        assert_eq!(pc.name(), "");
        assert!(!pc.has_description());
        assert_eq!(pc.market_context(), "");
    }

    #[test]
    fn test_init() {
        let mut pc = StepReprProductConcept::new();
        pc.init(
            "id1".to_string(),
            "name1".to_string(),
            true,
            Some("desc1".to_string()),
            "context1".to_string(),
        );
        assert_eq!(pc.id(), "id1");
        assert_eq!(pc.name(), "name1");
        assert!(pc.has_description());
        assert_eq!(pc.market_context(), "context1");
    }

    #[test]
    fn test_set_description() {
        let mut pc = StepReprProductConcept::new();
        assert!(!pc.has_description());
        pc.set_description("new_desc".to_string());
        assert!(pc.has_description());
        assert_eq!(pc.description(), Some("new_desc"));
    }

    #[test]
    fn test_setters() {
        let mut pc = StepReprProductConcept::new();
        pc.set_id("newid".to_string());
        pc.set_name("newname".to_string());
        pc.set_market_context("newctx".to_string());
        assert_eq!(pc.id(), "newid");
        assert_eq!(pc.name(), "newname");
        assert_eq!(pc.market_context(), "newctx");
    }
}

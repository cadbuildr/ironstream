// FILE: step_repr_representation_relationship.rs
// occt: StepRepr_RepresentationRelationship

/// Placeholder for Representation
#[derive(Clone, Debug, PartialEq)]
pub struct Representation {
    name: String,
}

/// Represents a relationship between two representations in STEP.
pub struct RepresentationRelationship {
    name: Option<String>,
    description: Option<String>,
    rep1: Option<Representation>,
    rep2: Option<Representation>,
}

impl RepresentationRelationship {
    /// Create a new RepresentationRelationship
    pub fn new() -> Self {
        RepresentationRelationship {
            name: None,
            description: None,
            rep1: None,
            rep2: None,
        }
    }

    /// Initialize representation relationship with name, description, and two representations
    pub fn init(
        &mut self,
        name: String,
        description: String,
        rep1: Representation,
        rep2: Representation,
    ) {
        self.name = Some(name);
        self.description = Some(description);
        self.rep1 = Some(rep1);
        self.rep2 = Some(rep2);
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Check if description exists
    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    /// Set the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the first representation
    pub fn set_rep1(&mut self, rep1: Representation) {
        self.rep1 = Some(rep1);
    }

    /// Get the first representation
    pub fn rep1(&self) -> Option<&Representation> {
        self.rep1.as_ref()
    }

    /// Set the second representation
    pub fn set_rep2(&mut self, rep2: Representation) {
        self.rep2 = Some(rep2);
    }

    /// Get the second representation
    pub fn rep2(&self) -> Option<&Representation> {
        self.rep2.as_ref()
    }
}

impl Default for RepresentationRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let rel = RepresentationRelationship::new();
        assert_eq!(rel.name(), None);
        assert!(!rel.has_description());
        assert_eq!(rel.rep1(), None);
        assert_eq!(rel.rep2(), None);
    }

    #[test]
    fn test_init() {
        let mut rel = RepresentationRelationship::new();
        let rep1 = Representation {
            name: "rep1".to_string(),
        };
        let rep2 = Representation {
            name: "rep2".to_string(),
        };
        rel.init(
            "relationship".to_string(),
            "description".to_string(),
            rep1.clone(),
            rep2.clone(),
        );
        assert_eq!(rel.name(), Some("relationship"));
        assert_eq!(rel.description(), Some("description"));
        assert_eq!(rel.rep1(), Some(&rep1));
        assert_eq!(rel.rep2(), Some(&rep2));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut rel = RepresentationRelationship::new();
        rel.set_name("TestName".to_string());
        assert_eq!(rel.name(), Some("TestName"));
    }

    #[test]
    fn test_has_description() {
        let mut rel = RepresentationRelationship::new();
        assert!(!rel.has_description());
        rel.set_description("Test description".to_string());
        assert!(rel.has_description());
        assert_eq!(rel.description(), Some("Test description"));
    }

    #[test]
    fn test_set_representations() {
        let mut rel = RepresentationRelationship::new();
        let rep1 = Representation {
            name: "test1".to_string(),
        };
        let rep2 = Representation {
            name: "test2".to_string(),
        };
        rel.set_rep1(rep1.clone());
        rel.set_rep2(rep2.clone());
        assert_eq!(rel.rep1(), Some(&rep1));
        assert_eq!(rel.rep2(), Some(&rep2));
    }
}

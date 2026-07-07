// FILE: step_basic_security_classification_level.rs
// occt: StepBasic_SecurityClassificationLevel

/// Represents a SecurityClassificationLevel in the STEP AP standard.
pub struct StepBasicSecurityClassificationLevel {
    name: Option<String>,
}

impl StepBasicSecurityClassificationLevel {
    pub fn new() -> Self {
        StepBasicSecurityClassificationLevel { name: None }
    }

    pub fn init(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl Default for StepBasicSecurityClassificationLevel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let scl = StepBasicSecurityClassificationLevel::new();
        assert_eq!(scl.name(), None);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut scl = StepBasicSecurityClassificationLevel::new();
        scl.set_name("level1".to_string());
        assert_eq!(scl.name(), Some("level1"));
    }

    #[test]
    fn test_default() {
        let scl = StepBasicSecurityClassificationLevel::default();
        assert_eq!(scl.name(), None);
    }
}

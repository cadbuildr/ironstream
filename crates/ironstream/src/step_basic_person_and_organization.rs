// FILE: step_basic_person_and_organization.rs
// occt: StepBasic_PersonAndOrganization

/// Represents a STEP PersonAndOrganization entity combining a Person and Organization.
#[derive(Clone, Debug)]
pub struct StepBasicPersonAndOrganization {
    the_person_id: String,         // Simplified: using ID string
    the_organization_id: String,   // Simplified: using ID string
}

impl StepBasicPersonAndOrganization {
    /// Create a new empty StepBasicPersonAndOrganization.
    pub fn new() -> Self {
        StepBasicPersonAndOrganization {
            the_person_id: String::new(),
            the_organization_id: String::new(),
        }
    }

    /// Initialize all fields.
    pub fn init(&mut self, the_person_id: String, the_organization_id: String) {
        self.the_person_id = the_person_id;
        self.the_organization_id = the_organization_id;
    }

    /// Returns the Person ID.
    pub fn the_person(&self) -> &str {
        &self.the_person_id
    }

    /// Set the Person ID.
    pub fn set_the_person(&mut self, id: String) {
        self.the_person_id = id;
    }

    /// Returns the Organization ID.
    pub fn the_organization(&self) -> &str {
        &self.the_organization_id
    }

    /// Set the Organization ID.
    pub fn set_the_organization(&mut self, id: String) {
        self.the_organization_id = id;
    }
}

impl Default for StepBasicPersonAndOrganization {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let po = StepBasicPersonAndOrganization::new();
        assert_eq!(po.the_person(), "");
        assert_eq!(po.the_organization(), "");
    }

    #[test]
    fn test_init() {
        let mut po = StepBasicPersonAndOrganization::new();
        po.init("P-001".to_string(), "ORG-001".to_string());

        assert_eq!(po.the_person(), "P-001");
        assert_eq!(po.the_organization(), "ORG-001");
    }

    #[test]
    fn test_setters() {
        let mut po = StepBasicPersonAndOrganization::new();
        po.set_the_person("P-002".to_string());
        po.set_the_organization("ORG-002".to_string());

        assert_eq!(po.the_person(), "P-002");
        assert_eq!(po.the_organization(), "ORG-002");
    }
}

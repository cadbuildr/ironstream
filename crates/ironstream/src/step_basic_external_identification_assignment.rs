// FILE: step_basic_external_identification_assignment.rs
// occt: StepBasic_ExternalIdentificationAssignment

/// Representation of STEP entity ExternalIdentificationAssignment
/// Extends IdentificationAssignment with an ExternalSource field
#[derive(Clone, Debug)]
pub struct ExternalIdentificationAssignment {
    // Inherited fields
    assigned_id: Option<String>,
    role: Option<String>,
    // Own field
    source: Option<String>,
}

impl ExternalIdentificationAssignment {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            assigned_id: None,
            role: None,
            source: None,
        }
    }

    /// Initialize all fields (own and inherited)
    pub fn init(&mut self, assigned_id: String, role: String, source: String) {
        self.assigned_id = Some(assigned_id);
        self.role = Some(role);
        self.source = Some(source);
    }

    /// Get assigned id (inherited)
    pub fn assigned_id(&self) -> Option<&str> {
        self.assigned_id.as_deref()
    }

    /// Set assigned id (inherited)
    pub fn set_assigned_id(&mut self, assigned_id: String) {
        self.assigned_id = Some(assigned_id);
    }

    /// Get role (inherited)
    pub fn role(&self) -> Option<&str> {
        self.role.as_deref()
    }

    /// Set role (inherited)
    pub fn set_role(&mut self, role: String) {
        self.role = Some(role);
    }

    /// Get source (own field)
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Set source (own field)
    pub fn set_source(&mut self, source: String) {
        self.source = Some(source);
    }
}

impl Default for ExternalIdentificationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ext_id = ExternalIdentificationAssignment::new();
        assert!(ext_id.assigned_id().is_none());
        assert!(ext_id.role().is_none());
        assert!(ext_id.source().is_none());
    }

    #[test]
    fn test_init() {
        let mut ext_id = ExternalIdentificationAssignment::new();
        ext_id.init("id123".to_string(), "role1".to_string(), "src1".to_string());
        assert_eq!(ext_id.assigned_id(), Some("id123"));
        assert_eq!(ext_id.role(), Some("role1"));
        assert_eq!(ext_id.source(), Some("src1"));
    }

    #[test]
    fn test_set_fields() {
        let mut ext_id = ExternalIdentificationAssignment::new();
        ext_id.set_assigned_id("id456".to_string());
        ext_id.set_role("role2".to_string());
        ext_id.set_source("src2".to_string());
        assert_eq!(ext_id.assigned_id(), Some("id456"));
        assert_eq!(ext_id.role(), Some("role2"));
        assert_eq!(ext_id.source(), Some("src2"));
    }

    #[test]
    fn test_default() {
        let ext_id = ExternalIdentificationAssignment::default();
        assert!(ext_id.assigned_id().is_none());
        assert!(ext_id.source().is_none());
    }
}

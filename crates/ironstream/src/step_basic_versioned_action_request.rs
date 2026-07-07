// FILE: step_basic_versioned_action_request.rs
// occt: StepBasic_VersionedActionRequest

//! Representation of a STEP entity VersionedActionRequest.

/// Represents a versioned action request with an ID, version, purpose, and optional description.
#[derive(Debug, Clone)]
pub struct StepBasicVersionedActionRequest {
    /// The ID of the action request
    id: Option<String>,
    /// The version identifier
    version: Option<String>,
    /// The purpose of the request
    purpose: Option<String>,
    /// Optional description
    description: Option<String>,
    /// Whether description is defined
    has_description: bool,
}

impl StepBasicVersionedActionRequest {
    /// Create a new VersionedActionRequest
    pub fn new() -> Self {
        Self {
            id: None,
            version: None,
            purpose: None,
            description: None,
            has_description: false,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        id: String,
        version: String,
        purpose: String,
        has_description: bool,
        description: Option<String>,
    ) {
        self.id = Some(id);
        self.version = Some(version);
        self.purpose = Some(purpose);
        self.has_description = has_description;
        self.description = description;
    }

    /// Get the ID
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Set the ID
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Get the version
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Set the version
    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    /// Get the purpose
    pub fn purpose(&self) -> Option<&str> {
        self.purpose.as_deref()
    }

    /// Set the purpose
    pub fn set_purpose(&mut self, purpose: String) {
        self.purpose = Some(purpose);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Check if description is defined
    pub fn has_description(&self) -> bool {
        self.has_description
    }
}

impl Default for StepBasicVersionedActionRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let var = StepBasicVersionedActionRequest::new();
        assert_eq!(var.id(), None);
        assert_eq!(var.version(), None);
        assert_eq!(var.purpose(), None);
        assert!(!var.has_description());
    }

    #[test]
    fn test_init() {
        let mut var = StepBasicVersionedActionRequest::new();
        var.init(
            "req_1".to_string(),
            "1.0".to_string(),
            "testing".to_string(),
            true,
            Some("A test request".to_string()),
        );
        assert_eq!(var.id(), Some("req_1"));
        assert_eq!(var.version(), Some("1.0"));
        assert_eq!(var.purpose(), Some("testing"));
        assert_eq!(var.description(), Some("A test request"));
        assert!(var.has_description());
    }

    #[test]
    fn test_set_id() {
        let mut var = StepBasicVersionedActionRequest::new();
        var.set_id("action_123".to_string());
        assert_eq!(var.id(), Some("action_123"));
    }

    #[test]
    fn test_set_version() {
        let mut var = StepBasicVersionedActionRequest::new();
        var.set_version("2.0".to_string());
        assert_eq!(var.version(), Some("2.0"));
    }

    #[test]
    fn test_set_purpose() {
        let mut var = StepBasicVersionedActionRequest::new();
        var.set_purpose("manufacturing".to_string());
        assert_eq!(var.purpose(), Some("manufacturing"));
    }

    #[test]
    fn test_set_description() {
        let mut var = StepBasicVersionedActionRequest::new();
        var.set_description("For CAD export".to_string());
        assert_eq!(var.description(), Some("For CAD export"));
    }
}

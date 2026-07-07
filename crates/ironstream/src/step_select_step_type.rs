// FILE: step_select_step_type.rs
// occt: StepSelect_StepType

use std::sync::Mutex;

/// Represents the STEP type signature
pub struct StepType {
    protocol_name: Option<String>,
    last_value: Mutex<String>,
}

impl StepType {
    /// Create a new StepType signature
    pub fn new() -> Self {
        StepType {
            protocol_name: None,
            last_value: Mutex::new(String::new()),
        }
    }

    /// Set the protocol to work with
    pub fn set_protocol(&mut self, protocol_name: String) {
        self.protocol_name = Some(protocol_name);
    }

    /// Get the protocol name
    pub fn protocol_name(&self) -> Option<&str> {
        self.protocol_name.as_deref()
    }

    /// Get the step type value for an entity
    pub fn value(&self, entity_id: &str) -> String {
        if let Some(proto) = &self.protocol_name {
            if entity_id.is_empty() {
                format!("(..NOT FROM SCHEMA {}..)", proto)
            } else {
                entity_id.to_string()
            }
        } else {
            "(..UNKNOWN PROTOCOL..)".to_string()
        }
    }

    /// Get the last computed value
    pub fn last_value(&self) -> String {
        self.last_value.lock().unwrap().clone()
    }

    /// Set the last computed value
    pub fn set_last_value(&self, value: String) {
        *self.last_value.lock().unwrap() = value;
    }
}

impl Default for StepType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let step_type = StepType::new();
        assert_eq!(step_type.protocol_name(), None);
    }

    #[test]
    fn test_set_protocol() {
        let mut step_type = StepType::new();
        step_type.set_protocol("AP214".to_string());
        assert_eq!(step_type.protocol_name(), Some("AP214"));
    }

    #[test]
    fn test_value_with_protocol() {
        let mut step_type = StepType::new();
        step_type.set_protocol("AP203".to_string());
        let value = step_type.value("PRODUCT");
        assert_eq!(value, "PRODUCT");
    }

    #[test]
    fn test_value_empty_entity() {
        let mut step_type = StepType::new();
        step_type.set_protocol("AP203".to_string());
        let value = step_type.value("");
        assert!(value.contains("NOT FROM SCHEMA"));
        assert!(value.contains("AP203"));
    }

    #[test]
    fn test_value_no_protocol() {
        let step_type = StepType::new();
        let value = step_type.value("PRODUCT");
        assert!(value.contains("UNKNOWN PROTOCOL"));
    }

    #[test]
    fn test_last_value() {
        let step_type = StepType::new();
        step_type.set_last_value("TEST_VALUE".to_string());
        assert_eq!(step_type.last_value(), "TEST_VALUE");
    }
}

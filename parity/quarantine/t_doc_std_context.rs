// FILE: t_doc_std_context.rs
// occt: TDocStd_Context

use std::collections::HashMap;

/// A context object for managing document-level state and settings.
/// Provides key-value storage for various context parameters.
#[derive(Clone, Debug)]
pub struct TDocStd_Context {
    parameters: HashMap<String, String>,
    id: [u8; 16],
}

impl TDocStd_Context {
    /// Create a new context.
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for Context attributes.
    pub fn get_id() -> [u8; 16] {
        [
            0x19, 0x2A, 0x3B, 0x4C, 0x5D, 0x6E, 0x7F, 0x80, 0x91, 0xA2, 0xB3, 0xC4, 0x99, 0x22,
            0x22, 0x22,
        ]
    }

    /// Set a parameter value.
    pub fn set_parameter(&mut self, key: String, value: String) {
        self.parameters.insert(key, value);
    }

    /// Get a parameter value.
    pub fn get_parameter(&self, key: &str) -> Option<&str> {
        self.parameters.get(key).map(|s| s.as_str())
    }

    /// Check if a parameter exists.
    pub fn has_parameter(&self, key: &str) -> bool {
        self.parameters.contains_key(key)
    }

    /// Remove a parameter.
    pub fn remove_parameter(&mut self, key: &str) -> Option<String> {
        self.parameters.remove(key)
    }

    /// Get the ID of this context.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Clear all parameters.
    pub fn clear(&mut self) {
        self.parameters.clear();
    }
}

impl Default for TDocStd_Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_context() {
        let context = TDocStd_Context::new();
        assert_eq!(context.id(), &TDocStd_Context::get_id());
    }

    #[test]
    fn test_set_and_get_parameter() {
        let mut context = TDocStd_Context::new();
        context.set_parameter("key1".to_string(), "value1".to_string());
        assert_eq!(context.get_parameter("key1"), Some("value1"));
    }

    #[test]
    fn test_has_parameter() {
        let mut context = TDocStd_Context::new();
        context.set_parameter("key".to_string(), "value".to_string());
        assert!(context.has_parameter("key"));
        assert!(!context.has_parameter("nonexistent"));
    }

    #[test]
    fn test_remove_parameter() {
        let mut context = TDocStd_Context::new();
        context.set_parameter("key".to_string(), "value".to_string());
        assert_eq!(context.remove_parameter("key"), Some("value".to_string()));
        assert!(!context.has_parameter("key"));
    }

    #[test]
    fn test_clear() {
        let mut context = TDocStd_Context::new();
        context.set_parameter("key1".to_string(), "value1".to_string());
        context.set_parameter("key2".to_string(), "value2".to_string());
        context.clear();
        assert!(!context.has_parameter("key1"));
    }

    #[test]
    fn test_default() {
        let context = TDocStd_Context::default();
        assert!(context.get_parameter("any").is_none());
    }
}

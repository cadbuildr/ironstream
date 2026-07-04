// FILE: destep_configuration_node.rs
// occt: DESTEP_ConfigurationNode

use crate::destep_parameters::DESTEP_Parameters;

/// Configuration node for STEP format transfer.
/// Stores necessary settings for DESTEP_Provider.
/// Vendor name: "OCC"
/// Format type: "STEP"
/// Supported extensions: .stp, .step, .stpz
/// Supports import and export, as well as streams.
pub struct DESTEP_ConfigurationNode {
    pub internal_parameters: DESTEP_Parameters,
}

impl DESTEP_ConfigurationNode {
    /// Creates a new configuration node with default values
    pub fn new() -> Self {
        DESTEP_ConfigurationNode {
            internal_parameters: DESTEP_Parameters::new(),
        }
    }

    /// Copies values from another configuration node
    pub fn copy_from(&mut self, other: &DESTEP_ConfigurationNode) {
        self.internal_parameters = other.internal_parameters.clone();
    }

    /// Loads configuration from a resource context (placeholder)
    /// Returns true if loading succeeded
    pub fn load(&mut self, _context: &()) -> bool {
        true
    }

    /// Saves configuration to a string representation
    pub fn save(&self) -> String {
        format!("DESTEP_ConfigurationNode{{parameters: {:?}}}", self.internal_parameters)
    }

    /// Creates a copy of this configuration node
    pub fn copy(&self) -> Self {
        DESTEP_ConfigurationNode {
            internal_parameters: self.internal_parameters.clone(),
        }
    }

    /// Creates a new provider for STEP format
    pub fn build_provider(&self) -> Option<String> {
        Some("DESTEP_Provider".to_string())
    }

    /// Checks if import is supported (always true for STEP)
    pub fn is_import_supported(&self) -> bool {
        true
    }

    /// Checks if export is supported (always true for STEP)
    pub fn is_export_supported(&self) -> bool {
        true
    }

    /// Checks if stream support is available (always true for STEP)
    pub fn is_stream_supported(&self) -> bool {
        true
    }

    /// Returns the CAD format name
    pub fn get_format(&self) -> String {
        "STEP".to_string()
    }

    /// Returns the provider's vendor name
    pub fn get_vendor(&self) -> String {
        "OCC".to_string()
    }

    /// Returns list of supported file extensions
    pub fn get_extensions(&self) -> Vec<String> {
        vec![".stp".to_string(), ".step".to_string(), ".stpz".to_string()]
    }

    /// Checks file content to verify if it's a STEP file
    pub fn check_content(&self, buffer: &[u8]) -> bool {
        // Check for STEP file signature
        if buffer.is_empty() {
            return false;
        }

        // STEP files typically start with ISO-10303
        let content = String::from_utf8_lossy(buffer);
        content.contains("ISO-10303") || content.contains("ISO 10303") || content.contains("STEP")
    }
}

impl Default for DESTEP_ConfigurationNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let node = DESTEP_ConfigurationNode::new();
        assert_eq!(node.get_format(), "STEP");
        assert_eq!(node.get_vendor(), "OCC");
    }

    #[test]
    fn test_copy() {
        let node = DESTEP_ConfigurationNode::new();
        let copied = node.copy();
        assert_eq!(copied.get_format(), "STEP");
        assert_eq!(copied.get_vendor(), "OCC");
    }

    #[test]
    fn test_copy_from() {
        let original = DESTEP_ConfigurationNode::new();
        let mut target = DESTEP_ConfigurationNode::new();
        target.copy_from(&original);
        assert_eq!(target.get_format(), original.get_format());
    }

    #[test]
    fn test_is_import_supported() {
        let node = DESTEP_ConfigurationNode::new();
        assert!(node.is_import_supported());
    }

    #[test]
    fn test_is_export_supported() {
        let node = DESTEP_ConfigurationNode::new();
        assert!(node.is_export_supported());
    }

    #[test]
    fn test_is_stream_supported() {
        let node = DESTEP_ConfigurationNode::new();
        assert!(node.is_stream_supported());
    }

    #[test]
    fn test_get_extensions() {
        let node = DESTEP_ConfigurationNode::new();
        let exts = node.get_extensions();
        assert!(exts.contains(&".stp".to_string()));
        assert!(exts.contains(&".step".to_string()));
        assert!(exts.contains(&".stpz".to_string()));
    }

    #[test]
    fn test_check_content_valid_step() {
        let node = DESTEP_ConfigurationNode::new();
        let content = b"ISO-10303-21;";
        assert!(node.check_content(content));
    }

    #[test]
    fn test_check_content_invalid() {
        let node = DESTEP_ConfigurationNode::new();
        let content = b"<?xml version=\"1.0\"?>";
        assert!(!node.check_content(content));
    }

    #[test]
    fn test_check_content_empty() {
        let node = DESTEP_ConfigurationNode::new();
        assert!(!node.check_content(&[]));
    }

    #[test]
    fn test_build_provider() {
        let node = DESTEP_ConfigurationNode::new();
        assert!(node.build_provider().is_some());
        assert_eq!(node.build_provider(), Some("DESTEP_Provider".to_string()));
    }

    #[test]
    fn test_save() {
        let node = DESTEP_ConfigurationNode::new();
        let saved = node.save();
        assert!(!saved.is_empty());
    }
}

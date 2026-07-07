// FILE: dexcaf_configuration_node.rs
// occt: DEXCAF_ConfigurationNode

/// Configuration node for XDE Document format (XBF) file transfer.
/// Stores and configures settings for DEXCAF_Provider to handle XCAF document import/export.
///
/// Format: XCAF
/// Vendor: OCC
/// Extension: .xbf
/// Supports both import and export of binary XDE documents.
#[derive(Clone)]
pub struct DexcafConfigurationNode {
    /// Reader append mode for loading documents
    pub read_append_mode: AppendMode,
    /// Attributes to skip when reading (overwrites existing)
    pub read_skip_values: Vec<String>,
    /// Attributes or sub-tree paths to read
    pub read_values: Vec<String>,
}

/// Append mode for reading XDE documents
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppendMode {
    /// Don't allow append (default)
    Forbid = 0,
    /// Keep existing attributes, add only new ones
    Keep = 1,
    /// Overwrite existing attributes with loaded ones
    Overwrite = 2,
}

impl AppendMode {
    /// Convert from integer representation
    pub fn from_int(val: i32) -> Self {
        match val {
            0 => AppendMode::Forbid,
            1 => AppendMode::Keep,
            _ => AppendMode::Overwrite,
        }
    }

    /// Convert to integer representation
    pub fn to_int(self) -> i32 {
        self as i32
    }
}

impl Default for DexcafConfigurationNode {
    fn default() -> Self {
        DexcafConfigurationNode {
            read_append_mode: AppendMode::Forbid,
            read_skip_values: Vec::new(),
            read_values: Vec::new(),
        }
    }
}

impl DexcafConfigurationNode {
    /// Initializes all fields with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Copies values from another configuration node
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// Returns true if import is supported
    pub fn is_import_supported(&self) -> bool {
        true
    }

    /// Returns true if export is supported
    pub fn is_export_supported(&self) -> bool {
        true
    }

    /// Gets the CAD format name
    pub fn get_format(&self) -> &'static str {
        "XCAF"
    }

    /// Gets the vendor name
    pub fn get_vendor(&self) -> &'static str {
        "OCC"
    }

    /// Gets list of supported file extensions
    pub fn get_extensions(&self) -> Vec<&'static str> {
        vec!["xbf"]
    }

    /// Checks if buffer content matches XBF format
    /// XBF files start with "BINFILE" magic bytes
    pub fn check_content(&self, buffer: &[u8]) -> bool {
        if buffer.len() < 8 {
            return false;
        }
        // Check for "BINFILE" magic bytes
        buffer[0] == b'B'
            && buffer[1] == b'I'
            && buffer[2] == b'N'
            && buffer[3] == b'F'
            && buffer[4] == b'I'
            && buffer[5] == b'L'
            && buffer[6] == b'E'
    }

    /// Adds a value to skip during reading
    pub fn add_skip_value(&mut self, value: String) {
        self.read_skip_values.push(value);
    }

    /// Adds a value/attribute to read
    pub fn add_read_value(&mut self, value: String) {
        self.read_values.push(value);
    }

    /// Generates configuration string for saving
    pub fn save(&self) -> String {
        let mut result = String::new();
        result.push_str("!*****************************************************************************\n");
        result.push_str(&format!("!Configuration Node  Vendor: {} Format: {}\n", self.get_vendor(), self.get_format()));

        let scope = format!("provider.{}.{}.", self.get_format(), self.get_vendor());

        result.push_str("!\n");
        result.push_str("!Read parameters:\n");
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Don't allow append (when the value  = 0, it is the default value), ");
        result.push_str("keeps existing attributes, reads only new ones(when the value = 1), ");
        result.push_str("overwrites the existing attributes by the loaded ones(when the value = 2)\n");
        result.push_str("!Default value: 0. Available values: 0, 1, 2\n");
        result.push_str(&format!("{}read.append.mode :\t {}\n", scope, self.read_append_mode.to_int()));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Overwrites the existing attributes by the loaded ones");
        result.push_str("!Default value: empty. Available values: {sequence<string>}\n");
        result.push_str(&format!("{}read.skip.values :\t ", scope));
        for value in &self.read_skip_values {
            result.push_str(value);
            result.push(' ');
        }
        result.push_str("\n!\n");

        result.push_str("!\n");
        result.push_str("!1) Adds sub-tree path like \"0:2\"");
        result.push_str("2) Adds attribute to read by typename. Disables the skipped attributes added. (there shouldn't be '0' after -read)\n");
        result.push_str("!Default value: empty. Available values: {sequence<string>}\n");
        result.push_str(&format!("{}read.values :\t ", scope));
        for value in &self.read_values {
            result.push_str(value);
            result.push(' ');
        }
        result.push_str("\n!\n");

        result.push_str("!*****************************************************************************\n");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_initialization() {
        let node = DexcafConfigurationNode::new();
        assert_eq!(node.read_append_mode, AppendMode::Forbid);
        assert!(node.read_skip_values.is_empty());
        assert!(node.read_values.is_empty());
    }

    #[test]
    fn test_format_and_vendor() {
        let node = DexcafConfigurationNode::new();
        assert_eq!(node.get_format(), "XCAF");
        assert_eq!(node.get_vendor(), "OCC");
    }

    #[test]
    fn test_import_export_support() {
        let node = DexcafConfigurationNode::new();
        assert!(node.is_import_supported());
        assert!(node.is_export_supported());
    }

    #[test]
    fn test_extensions() {
        let node = DexcafConfigurationNode::new();
        let exts = node.get_extensions();
        assert_eq!(exts.len(), 1);
        assert_eq!(exts[0], "xbf");
    }

    #[test]
    fn test_check_content_valid() {
        let node = DexcafConfigurationNode::new();
        let buffer = b"BINFILE\0extra";
        assert!(node.check_content(buffer));

        let buffer2 = b"BINFILEXXXXXX";
        assert!(node.check_content(buffer2));
    }

    #[test]
    fn test_check_content_invalid_too_short() {
        let node = DexcafConfigurationNode::new();
        let buffer = b"BINFILE";
        assert!(!node.check_content(buffer));
    }

    #[test]
    fn test_check_content_invalid_wrong_header() {
        let node = DexcafConfigurationNode::new();
        let buffer = b"WRONGHD\0";
        assert!(!node.check_content(buffer));
    }

    #[test]
    fn test_check_content_empty_buffer() {
        let node = DexcafConfigurationNode::new();
        let buffer = b"";
        assert!(!node.check_content(buffer));
    }

    #[test]
    fn test_append_mode_conversion() {
        assert_eq!(AppendMode::from_int(0), AppendMode::Forbid);
        assert_eq!(AppendMode::from_int(1), AppendMode::Keep);
        assert_eq!(AppendMode::from_int(2), AppendMode::Overwrite);
        assert_eq!(AppendMode::from_int(99), AppendMode::Overwrite);
        assert_eq!(AppendMode::Forbid.to_int(), 0);
        assert_eq!(AppendMode::Keep.to_int(), 1);
        assert_eq!(AppendMode::Overwrite.to_int(), 2);
    }

    #[test]
    fn test_add_skip_value() {
        let mut node = DexcafConfigurationNode::new();
        node.add_skip_value("attr1".to_string());
        node.add_skip_value("attr2".to_string());

        assert_eq!(node.read_skip_values.len(), 2);
        assert_eq!(node.read_skip_values[0], "attr1");
        assert_eq!(node.read_skip_values[1], "attr2");
    }

    #[test]
    fn test_add_read_value() {
        let mut node = DexcafConfigurationNode::new();
        node.add_read_value("0:1".to_string());
        node.add_read_value("Attribute".to_string());

        assert_eq!(node.read_values.len(), 2);
        assert_eq!(node.read_values[0], "0:1");
        assert_eq!(node.read_values[1], "Attribute");
    }

    #[test]
    fn test_copy_functionality() {
        let mut node = DexcafConfigurationNode::new();
        node.read_append_mode = AppendMode::Overwrite;
        node.add_skip_value("skip_attr".to_string());
        node.add_read_value("read_attr".to_string());

        let copied = node.copy();
        assert_eq!(copied.read_append_mode, AppendMode::Overwrite);
        assert_eq!(copied.read_skip_values.len(), 1);
        assert_eq!(copied.read_values.len(), 1);
        assert_eq!(copied.read_skip_values[0], "skip_attr");
        assert_eq!(copied.read_values[0], "read_attr");
    }

    #[test]
    fn test_save_configuration_string() {
        let mut node = DexcafConfigurationNode::new();
        node.add_skip_value("test".to_string());
        let config_str = node.save();
        assert!(config_str.contains("XCAF"));
        assert!(config_str.contains("OCC"));
        assert!(config_str.contains("provider.XCAF.OCC."));
        assert!(config_str.contains("read.append.mode"));
    }

    #[test]
    fn test_mutable_fields() {
        let mut node = DexcafConfigurationNode::new();
        node.read_append_mode = AppendMode::Keep;
        node.read_skip_values.push("attr".to_string());

        assert_eq!(node.read_append_mode, AppendMode::Keep);
        assert_eq!(node.read_skip_values.len(), 1);
    }
}

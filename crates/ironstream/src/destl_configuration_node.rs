// FILE: destl_configuration_node.rs
// occt: DESTL_ConfigurationNode

/// Configuration node for STL format file transfer.
/// Stores and configures settings for DESTL_Provider to handle STL file import/export.
///
/// Format: STL
/// Vendor: OCC
/// Extension: .stl
/// Supports both import and export, as well as streaming.
#[derive(Clone)]
pub struct DestlConfigurationNode {
    /// Input merge angle value (in degrees, range 0.0-90.0)
    pub read_merge_angle: f64,
    /// Setting up Boundary Representation flag
    pub read_brep: bool,
    /// Setting up writing mode (true = ASCII, false = Binary)
    pub write_ascii: bool,
}

impl Default for DestlConfigurationNode {
    fn default() -> Self {
        DestlConfigurationNode {
            read_merge_angle: 90.0,
            read_brep: false,
            write_ascii: true,
        }
    }
}

impl DestlConfigurationNode {
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

    /// Returns true if streaming is supported
    pub fn is_stream_supported(&self) -> bool {
        true
    }

    /// Gets the CAD format name
    pub fn get_format(&self) -> &'static str {
        "STL"
    }

    /// Gets the vendor name
    pub fn get_vendor(&self) -> &'static str {
        "OCC"
    }

    /// Gets list of supported file extensions
    pub fn get_extensions(&self) -> Vec<&'static str> {
        vec!["stl"]
    }

    /// Checks if buffer content matches STL format (ASCII)
    /// ASCII STL files start with "solid" or "SOLID" followed by whitespace
    pub fn check_content(&self, buffer: &[u8]) -> bool {
        if buffer.len() < 7 {
            return false;
        }
        // Check for ASCII STL header "solid " or "SOLID "
        let matches_solid_lower = buffer.len() >= 6
            && buffer[0] == b's'
            && buffer[1] == b'o'
            && buffer[2] == b'l'
            && buffer[3] == b'i'
            && buffer[4] == b'd'
            && is_whitespace(buffer[5]);

        let matches_solid_upper = buffer.len() >= 6
            && buffer[0] == b'S'
            && buffer[1] == b'O'
            && buffer[2] == b'L'
            && buffer[3] == b'I'
            && buffer[4] == b'D'
            && is_whitespace(buffer[5]);

        matches_solid_lower || matches_solid_upper
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
        result.push_str("!Input merge angle value\n");
        result.push_str("!Default value (in degrees): 90.0. Angle should be within [0.0, 90.0] range\n");
        result.push_str(&format!("{}read.merge.angle :\t {}\n", scope, self.read_merge_angle));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Setting up Boundary Representation flag\n");
        result.push_str("!Default value: false. Available values: \"on\", \"off\"\n");
        result.push_str(&format!("{}read.brep :\t {}\n", scope, self.read_brep as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Write parameters:\n");
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Setting up writing mode (Ascii or Binary)\n");
        result.push_str("!Default value: 1(Binary). Available values: 0(Ascii), 1(Binary)\n");
        result.push_str(&format!("{}write.ascii :\t {}\n", scope, self.write_ascii as u8));
        result.push_str("!\n");

        result.push_str("!*****************************************************************************\n");
        result
    }
}

/// Helper function to check if a byte is whitespace
fn is_whitespace(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == b'\x0c'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_initialization() {
        let node = DestlConfigurationNode::new();
        assert_eq!(node.read_merge_angle, 90.0);
        assert!(!node.read_brep);
        assert!(node.write_ascii);
    }

    #[test]
    fn test_format_and_vendor() {
        let node = DestlConfigurationNode::new();
        assert_eq!(node.get_format(), "STL");
        assert_eq!(node.get_vendor(), "OCC");
    }

    #[test]
    fn test_import_export_stream_support() {
        let node = DestlConfigurationNode::new();
        assert!(node.is_import_supported());
        assert!(node.is_export_supported());
        assert!(node.is_stream_supported());
    }

    #[test]
    fn test_extensions() {
        let node = DestlConfigurationNode::new();
        let exts = node.get_extensions();
        assert_eq!(exts.len(), 1);
        assert_eq!(exts[0], "stl");
    }

    #[test]
    fn test_check_content_ascii_solid_lower() {
        let node = DestlConfigurationNode::new();
        let buffer = b"solid myobject\n";
        assert!(node.check_content(buffer));
    }

    #[test]
    fn test_check_content_ascii_solid_upper() {
        let node = DestlConfigurationNode::new();
        let buffer = b"SOLID MYOBJECT\n";
        assert!(node.check_content(buffer));
    }

    #[test]
    fn test_check_content_with_tab() {
        let node = DestlConfigurationNode::new();
        let buffer = b"solid\t\n";
        assert!(node.check_content(buffer));
    }

    #[test]
    fn test_check_content_with_space() {
        let node = DestlConfigurationNode::new();
        let buffer = b"solid \n";
        assert!(node.check_content(buffer));
    }

    #[test]
    fn test_check_content_invalid_too_short() {
        let node = DestlConfigurationNode::new();
        let buffer = b"solid";
        assert!(!node.check_content(buffer));
    }

    #[test]
    fn test_check_content_invalid_wrong_header() {
        let node = DestlConfigurationNode::new();
        let buffer = b"other \n";
        assert!(!node.check_content(buffer));
    }

    #[test]
    fn test_check_content_invalid_no_whitespace() {
        let node = DestlConfigurationNode::new();
        let buffer = b"solidX\n";
        assert!(!node.check_content(buffer));
    }

    #[test]
    fn test_check_content_empty_buffer() {
        let node = DestlConfigurationNode::new();
        let buffer = b"";
        assert!(!node.check_content(buffer));
    }

    #[test]
    fn test_copy_functionality() {
        let mut node = DestlConfigurationNode::new();
        node.read_merge_angle = 45.5;
        node.read_brep = true;
        node.write_ascii = false;

        let copied = node.copy();
        assert_eq!(copied.read_merge_angle, 45.5);
        assert!(copied.read_brep);
        assert!(!copied.write_ascii);
    }

    #[test]
    fn test_save_configuration_string() {
        let node = DestlConfigurationNode::new();
        let config_str = node.save();
        assert!(config_str.contains("STL"));
        assert!(config_str.contains("OCC"));
        assert!(config_str.contains("provider.STL.OCC."));
        assert!(config_str.contains("read.merge.angle"));
        assert!(config_str.contains("write.ascii"));
    }

    #[test]
    fn test_mutable_fields() {
        let mut node = DestlConfigurationNode::new();
        node.read_merge_angle = 30.0;
        node.read_brep = true;
        node.write_ascii = false;

        assert_eq!(node.read_merge_angle, 30.0);
        assert!(node.read_brep);
        assert!(!node.write_ascii);
    }

    #[test]
    fn test_is_whitespace() {
        assert!(is_whitespace(b' '));
        assert!(is_whitespace(b'\t'));
        assert!(is_whitespace(b'\n'));
        assert!(is_whitespace(b'\r'));
        assert!(is_whitespace(b'\x0c'));
        assert!(!is_whitespace(b'a'));
        assert!(!is_whitespace(b'0'));
    }
}

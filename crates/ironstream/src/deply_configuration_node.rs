// FILE: deply_configuration_node.rs
// occt: DEPLY_ConfigurationNode

/// Configuration node for PLY format file transfer.
/// Stores and configures settings for DEPLY_Provider to handle PLY file export.
///
/// Format: PLY
/// Vendor: OCC
/// Extension: .ply
/// Supports export only; import is not supported.
#[derive(Clone)]
pub struct DeplyConfigurationNode {
    /// File length units to convert from while reading (scale factor for meters)
    pub file_length_unit: f64,
    /// System origin coordinate system for conversion during read
    pub system_cs: CoordinateSystem,
    /// File origin coordinate system for conversion during read
    pub file_cs: CoordinateSystem,
    /// Flag for writing normals
    pub write_normals: bool,
    /// Flag for writing colors
    pub write_colors: bool,
    /// Flag for writing UV / texture coordinates
    pub write_tex_coords: bool,
    /// Flag for writing part Id as element attribute
    pub write_part_id: bool,
    /// Flag for writing face Id as element attribute (cannot be combined with write_part_id)
    pub write_face_id: bool,
    /// Export special comment
    pub write_comment: String,
    /// Author of exported file
    pub write_author: String,
}

/// Coordinate system enumeration for mesh transformations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoordinateSystem {
    /// Z-up coordinate system
    Zup = 0,
    /// Y-up coordinate system
    Yup = 1,
}

impl CoordinateSystem {
    /// Convert from integer representation
    pub fn from_int(val: i32) -> Self {
        match val % 2 {
            0 => CoordinateSystem::Zup,
            _ => CoordinateSystem::Yup,
        }
    }

    /// Convert to integer representation
    pub fn to_int(self) -> i32 {
        self as i32
    }
}

impl Default for DeplyConfigurationNode {
    fn default() -> Self {
        DeplyConfigurationNode {
            file_length_unit: 1.0,
            system_cs: CoordinateSystem::Zup,
            file_cs: CoordinateSystem::Yup,
            write_normals: true,
            write_colors: true,
            write_tex_coords: false,
            write_part_id: true,
            write_face_id: false,
            write_comment: String::new(),
            write_author: String::new(),
        }
    }
}

impl DeplyConfigurationNode {
    /// Initializes all fields with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Copies values from another configuration node
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// Returns true if import is supported (PLY does not support import)
    pub fn is_import_supported(&self) -> bool {
        false
    }

    /// Returns true if export is supported
    pub fn is_export_supported(&self) -> bool {
        true
    }

    /// Gets the CAD format name
    pub fn get_format(&self) -> &'static str {
        "PLY"
    }

    /// Gets the vendor name
    pub fn get_vendor(&self) -> &'static str {
        "OCC"
    }

    /// Gets list of supported file extensions
    pub fn get_extensions(&self) -> Vec<&'static str> {
        vec!["ply"]
    }

    /// Checks if buffer content matches PLY format
    /// PLY files start with "ply" followed by whitespace
    pub fn check_content(&self, buffer: &[u8]) -> bool {
        if buffer.len() < 4 {
            return false;
        }
        // Check for "ply" magic bytes
        if buffer[0] != b'p' || buffer[1] != b'l' || buffer[2] != b'y' {
            return false;
        }
        // Check that the 4th byte is whitespace (space, tab, newline, etc.)
        let fourth = buffer[3];
        fourth == b' ' || fourth == b'\t' || fourth == b'\n' || fourth == b'\r'
    }

    /// Generates configuration string for saving
    pub fn save(&self) -> String {
        let mut result = String::new();
        result.push_str("!*****************************************************************************\n");
        result.push_str(&format!("!Configuration Node  Vendor: {} Format: {}\n", self.get_vendor(), self.get_format()));

        let scope = format!("provider.{}.{}.", self.get_format(), self.get_vendor());

        result.push_str("!\n");
        result.push_str("!Common parameters:\n");
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!File length units to convert from while reading the file, defined as scale factor for m (meters)\n");
        result.push_str("!Default value: 1.0(MM)\n");
        result.push_str(&format!("{}file.length.unit :\t {}\n", scope, self.file_length_unit));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!System origin coordinate system to perform conversion into during read\n");
        result.push_str("!Default value: 0(Zup). Available values: 0(Zup), 1(Yup)\n");
        result.push_str(&format!("{}system.cs :\t {}\n", scope, self.system_cs.to_int()));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!File origin coordinate system to perform conversion during read\n");
        result.push_str("!Default value: 1(Yup). Available values: 0(Zup), 1(Yup)\n");
        result.push_str(&format!("{}file.cs :\t {}\n", scope, self.file_cs.to_int()));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Write parameters:\n");
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Flag for write normals\n");
        result.push_str("!Default value: 1(true). Available values: 0(false), 1(true)\n");
        result.push_str(&format!("{}write.normals :\t {}\n", scope, self.write_normals as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Flag for write colors\n");
        result.push_str("!Default value: 1(true). Available values: 0(false), 1(true)\n");
        result.push_str(&format!("{}write.colors :\t {}\n", scope, self.write_colors as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Flag for write UV / texture coordinates\n");
        result.push_str("!Default value: 0(false). Available values: 0(false), 1(true)\n");
        result.push_str(&format!("{}write.tex.coords :\t {}\n", scope, self.write_tex_coords as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Flag for write part Id as element attribute\n");
        result.push_str("!Default value: 1(true). Available values: 0(false), 1(true)\n");
        result.push_str(&format!("{}write.part.id :\t {}\n", scope, self.write_part_id as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Flag for write face Id as element attribute. Cannot be combined with HasPartId\n");
        result.push_str("!Default value: 0(false). Available values: 0(false), 1(true)\n");
        result.push_str(&format!("{}write.face.id :\t {}\n", scope, self.write_face_id as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Export special comment\n");
        result.push_str("!Default value: (empty). Available values: <string>\n");
        result.push_str(&format!("{}write.comment :\t {}\n", scope, self.write_comment));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Author of exported file name\n");
        result.push_str("!Default value: (empty). Available values: <string>\n");
        result.push_str(&format!("{}write.author :\t {}\n", scope, self.write_author));
        result.push_str("!\n");

        result.push_str("!*****************************************************************************\n");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_initialization() {
        let node = DeplyConfigurationNode::new();
        assert_eq!(node.file_length_unit, 1.0);
        assert_eq!(node.system_cs, CoordinateSystem::Zup);
        assert_eq!(node.file_cs, CoordinateSystem::Yup);
        assert!(node.write_normals);
        assert!(node.write_colors);
        assert!(!node.write_tex_coords);
        assert!(node.write_part_id);
        assert!(!node.write_face_id);
        assert_eq!(node.write_comment, "");
        assert_eq!(node.write_author, "");
    }

    #[test]
    fn test_format_and_vendor() {
        let node = DeplyConfigurationNode::new();
        assert_eq!(node.get_format(), "PLY");
        assert_eq!(node.get_vendor(), "OCC");
    }

    #[test]
    fn test_import_export_support() {
        let node = DeplyConfigurationNode::new();
        assert!(!node.is_import_supported());
        assert!(node.is_export_supported());
    }

    #[test]
    fn test_extensions() {
        let node = DeplyConfigurationNode::new();
        let exts = node.get_extensions();
        assert_eq!(exts.len(), 1);
        assert_eq!(exts[0], "ply");
    }

    #[test]
    fn test_check_content_valid() {
        let node = DeplyConfigurationNode::new();
        let buffer = b"ply \n";
        assert!(node.check_content(buffer));

        let buffer2 = b"ply\n";
        assert!(node.check_content(buffer2));

        let buffer3 = b"ply\t";
        assert!(node.check_content(buffer3));

        let buffer4 = b"ply\r";
        assert!(node.check_content(buffer4));
    }

    #[test]
    fn test_check_content_invalid() {
        let node = DeplyConfigurationNode::new();

        // Too short
        let buffer = b"ply";
        assert!(!node.check_content(buffer));

        // Wrong magic
        let buffer2 = b"obj \n";
        assert!(!node.check_content(buffer2));

        // Fourth byte is not whitespace
        let buffer3 = b"plyX";
        assert!(!node.check_content(buffer3));

        // Empty buffer
        let buffer4 = b"";
        assert!(!node.check_content(buffer4));
    }

    #[test]
    fn test_coordinate_system_conversion() {
        assert_eq!(CoordinateSystem::from_int(0), CoordinateSystem::Zup);
        assert_eq!(CoordinateSystem::from_int(1), CoordinateSystem::Yup);
        assert_eq!(CoordinateSystem::from_int(2), CoordinateSystem::Zup);
        assert_eq!(CoordinateSystem::from_int(3), CoordinateSystem::Yup);
        assert_eq!(CoordinateSystem::Zup.to_int(), 0);
        assert_eq!(CoordinateSystem::Yup.to_int(), 1);
    }

    #[test]
    fn test_copy_functionality() {
        let mut node = DeplyConfigurationNode::new();
        node.file_length_unit = 2.5;
        node.write_normals = false;
        node.write_comment = "Test comment".to_string();

        let copied = node.copy();
        assert_eq!(copied.file_length_unit, 2.5);
        assert!(!copied.write_normals);
        assert_eq!(copied.write_comment, "Test comment");
    }

    #[test]
    fn test_save_configuration_string() {
        let node = DeplyConfigurationNode::new();
        let config_str = node.save();
        assert!(config_str.contains("PLY"));
        assert!(config_str.contains("OCC"));
        assert!(config_str.contains("provider.PLY.OCC."));
        assert!(config_str.contains("write.normals"));
        assert!(config_str.contains("write.colors"));
    }

    #[test]
    fn test_mutable_fields() {
        let mut node = DeplyConfigurationNode::new();
        node.file_length_unit = 0.001;
        node.system_cs = CoordinateSystem::Yup;
        node.write_colors = false;

        assert_eq!(node.file_length_unit, 0.001);
        assert_eq!(node.system_cs, CoordinateSystem::Yup);
        assert!(!node.write_colors);
    }
}

// FILE: devrml_configuration_node.rs
// occt: DEVRML_ConfigurationNode

/// Configuration node for VRML format file transfer.
/// Stores and configures settings for DEVRML_Provider to handle VRML file import/export.
///
/// Format: VRML
/// Vendor: OCC
/// Extensions: .vrml, .wrl
/// Supports import, export, and streaming.
#[derive(Clone)]
pub struct DevrmlConfigurationNode {
    /// File length units to convert from while reading (scale factor for meters)
    pub read_file_unit: f64,
    /// Coordinate system defined by VRML file
    pub read_file_coordinate_sys: CoordinateSystem,
    /// Result coordinate system
    pub read_system_coordinate_sys: CoordinateSystem,
    /// Fill the document with partially retrieved data even if reader failed
    pub read_fill_incomplete: bool,
    /// Writer version (1 or 2)
    pub writer_version: WriterVersion,
    /// Representation type for writing
    pub write_representation_type: RepresentationType,
}

/// Coordinate system enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoordinateSystem {
    /// Z-up coordinate system
    Zup = 0,
    /// Y-up coordinate system
    Yup = 1,
}

/// Writer version enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriterVersion {
    /// VRML version 1
    Version1 = 1,
    /// VRML version 2
    Version2 = 2,
}

/// Representation type for writing VRML
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepresentationType {
    /// Shaded representation
    Shaded = 0,
    /// Wireframe representation
    Wireframe = 1,
    /// Both shaded and wireframe
    Both = 2,
}

impl CoordinateSystem {
    /// Convert from integer representation
    pub fn from_int(val: i32) -> Self {
        match val {
            0 => CoordinateSystem::Zup,
            _ => CoordinateSystem::Yup,
        }
    }

    /// Convert to integer representation
    pub fn to_int(self) -> i32 {
        self as i32
    }
}

impl WriterVersion {
    /// Convert from integer representation
    pub fn from_int(val: i32) -> Self {
        match val {
            1 => WriterVersion::Version1,
            _ => WriterVersion::Version2,
        }
    }

    /// Convert to integer representation
    pub fn to_int(self) -> i32 {
        self as i32
    }
}

impl RepresentationType {
    /// Convert from integer representation
    pub fn from_int(val: i32) -> Self {
        match val {
            0 => RepresentationType::Shaded,
            1 => RepresentationType::Wireframe,
            _ => RepresentationType::Both,
        }
    }

    /// Convert to integer representation
    pub fn to_int(self) -> i32 {
        self as i32
    }
}

impl Default for DevrmlConfigurationNode {
    fn default() -> Self {
        DevrmlConfigurationNode {
            read_file_unit: 1.0,
            read_file_coordinate_sys: CoordinateSystem::Yup,
            read_system_coordinate_sys: CoordinateSystem::Zup,
            read_fill_incomplete: true,
            writer_version: WriterVersion::Version2,
            write_representation_type: RepresentationType::Wireframe,
        }
    }
}

impl DevrmlConfigurationNode {
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
        "VRML"
    }

    /// Gets the vendor name
    pub fn get_vendor(&self) -> &'static str {
        "OCC"
    }

    /// Gets list of supported file extensions
    pub fn get_extensions(&self) -> Vec<&'static str> {
        vec!["vrml", "wrl"]
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
        result.push_str("!Set (override) file length units to convert from while reading the file, defined as scale factor for m (meters).\n");
        result.push_str("!Default value: 1. Available values: positive double\n");
        result.push_str(&format!("{}read.file.unit :\t {}\n", scope, self.read_file_unit));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Set (override) file origin coordinate system to perform conversion during read.\n");
        result.push_str("!Default value: Yup (1). { Zup (0) | Yup (1) }\n");
        result.push_str(&format!("{}read.file.coordinate.system :\t {}\n", scope, self.read_file_coordinate_sys.to_int()));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Set system origin coordinate system to perform conversion into during read.\n");
        result.push_str("!Default value: Zup (0). Available values: { Zup (0) | Yup (1) }\n");
        result.push_str(&format!("{}read.system.coordinate.system :\t {}\n", scope, self.read_system_coordinate_sys.to_int()));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Set flag allowing partially read file content to be put into the XDE document.\n");
        result.push_str("!Default value: 1(\"ON\"). Available values: 0(\"OFF\"), 1(\"ON\")\n");
        result.push_str(&format!("{}read.fill.incomplete :\t {}\n", scope, self.read_fill_incomplete as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Write parameters:\n");
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Setting up writer version.\n");
        result.push_str("!Default value: 2. Available values: 1, 2\n");
        result.push_str(&format!("{}writer.version :\t {}\n", scope, self.writer_version.to_int()));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Setting up representation\n");
        result.push_str("!Default value: 1. Available values: 0(shaded), 1(wireframe), 2(both).\n");
        result.push_str(&format!("{}write.representation.type :\t {}\n", scope, self.write_representation_type.to_int()));
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
        let node = DevrmlConfigurationNode::new();
        assert_eq!(node.read_file_unit, 1.0);
        assert_eq!(node.read_file_coordinate_sys, CoordinateSystem::Yup);
        assert_eq!(node.read_system_coordinate_sys, CoordinateSystem::Zup);
        assert!(node.read_fill_incomplete);
        assert_eq!(node.writer_version, WriterVersion::Version2);
        assert_eq!(node.write_representation_type, RepresentationType::Wireframe);
    }

    #[test]
    fn test_format_and_vendor() {
        let node = DevrmlConfigurationNode::new();
        assert_eq!(node.get_format(), "VRML");
        assert_eq!(node.get_vendor(), "OCC");
    }

    #[test]
    fn test_import_export_stream_support() {
        let node = DevrmlConfigurationNode::new();
        assert!(node.is_import_supported());
        assert!(node.is_export_supported());
        assert!(node.is_stream_supported());
    }

    #[test]
    fn test_extensions() {
        let node = DevrmlConfigurationNode::new();
        let exts = node.get_extensions();
        assert_eq!(exts.len(), 2);
        assert!(exts.contains(&"vrml"));
        assert!(exts.contains(&"wrl"));
    }

    #[test]
    fn test_coordinate_system_conversion() {
        assert_eq!(CoordinateSystem::from_int(0), CoordinateSystem::Zup);
        assert_eq!(CoordinateSystem::from_int(1), CoordinateSystem::Yup);
        assert_eq!(CoordinateSystem::from_int(99), CoordinateSystem::Yup);
        assert_eq!(CoordinateSystem::Zup.to_int(), 0);
        assert_eq!(CoordinateSystem::Yup.to_int(), 1);
    }

    #[test]
    fn test_writer_version_conversion() {
        assert_eq!(WriterVersion::from_int(1), WriterVersion::Version1);
        assert_eq!(WriterVersion::from_int(2), WriterVersion::Version2);
        assert_eq!(WriterVersion::from_int(99), WriterVersion::Version2);
        assert_eq!(WriterVersion::Version1.to_int(), 1);
        assert_eq!(WriterVersion::Version2.to_int(), 2);
    }

    #[test]
    fn test_representation_type_conversion() {
        assert_eq!(RepresentationType::from_int(0), RepresentationType::Shaded);
        assert_eq!(RepresentationType::from_int(1), RepresentationType::Wireframe);
        assert_eq!(RepresentationType::from_int(2), RepresentationType::Both);
        assert_eq!(RepresentationType::from_int(99), RepresentationType::Both);
        assert_eq!(RepresentationType::Shaded.to_int(), 0);
        assert_eq!(RepresentationType::Wireframe.to_int(), 1);
        assert_eq!(RepresentationType::Both.to_int(), 2);
    }

    #[test]
    fn test_copy_functionality() {
        let mut node = DevrmlConfigurationNode::new();
        node.read_file_unit = 2.5;
        node.read_file_coordinate_sys = CoordinateSystem::Zup;
        node.writer_version = WriterVersion::Version1;

        let copied = node.copy();
        assert_eq!(copied.read_file_unit, 2.5);
        assert_eq!(copied.read_file_coordinate_sys, CoordinateSystem::Zup);
        assert_eq!(copied.writer_version, WriterVersion::Version1);
    }

    #[test]
    fn test_save_configuration_string() {
        let node = DevrmlConfigurationNode::new();
        let config_str = node.save();
        assert!(config_str.contains("VRML"));
        assert!(config_str.contains("OCC"));
        assert!(config_str.contains("provider.VRML.OCC."));
        assert!(config_str.contains("read.file.unit"));
        assert!(config_str.contains("writer.version"));
    }

    #[test]
    fn test_mutable_fields() {
        let mut node = DevrmlConfigurationNode::new();
        node.read_file_unit = 0.001;
        node.read_file_coordinate_sys = CoordinateSystem::Zup;
        node.writer_version = WriterVersion::Version1;
        node.write_representation_type = RepresentationType::Shaded;

        assert_eq!(node.read_file_unit, 0.001);
        assert_eq!(node.read_file_coordinate_sys, CoordinateSystem::Zup);
        assert_eq!(node.writer_version, WriterVersion::Version1);
        assert_eq!(node.write_representation_type, RepresentationType::Shaded);
    }
}

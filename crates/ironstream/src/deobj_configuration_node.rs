// FILE: deobj_configuration_node.rs
// occt: DEOBJ_ConfigurationNode

/// Configuration node for OBJ format file transfer.
/// Stores and configures settings for DEOBJ_Provider to handle OBJ file import/export.
///
/// Format: OBJ
/// Vendor: OCC
/// Extension: .obj
/// Supports both import and export.
#[derive(Clone)]
pub struct DeObjConfigurationNode {
    /// File length units to convert from while reading (scale factor for meters)
    pub file_length_unit: f64,
    /// System origin coordinate system for conversion during read
    pub system_cs: CoordinateSystem,
    /// File origin coordinate system for conversion during read
    pub file_cs: CoordinateSystem,
    /// Flag for reading vertex data with single or double precision
    pub read_single_precision: bool,
    /// Flag to create a single triangulation
    pub read_create_shapes: bool,
    /// Root folder for generating root labels names
    pub read_root_prefix: String,
    /// Flag to fill document from shape sequence
    pub read_fill_doc: bool,
    /// Flag to fill document with partially retrieved data even if reader fails
    pub read_fill_incomplete: bool,
    /// Memory usage limit in MiB (-1 = no limit)
    pub read_memory_limit_mib: i32,
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

impl Default for DeObjConfigurationNode {
    fn default() -> Self {
        DeObjConfigurationNode {
            file_length_unit: 1.0,
            system_cs: CoordinateSystem::Zup,
            file_cs: CoordinateSystem::Yup,
            read_single_precision: false,
            read_create_shapes: false,
            read_root_prefix: String::new(),
            read_fill_doc: true,
            read_fill_incomplete: true,
            read_memory_limit_mib: -1,
            write_comment: String::new(),
            write_author: String::new(),
        }
    }
}

impl DeObjConfigurationNode {
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
        "OBJ"
    }

    /// Gets the vendor name
    pub fn get_vendor(&self) -> &'static str {
        "OCC"
    }

    /// Gets list of supported file extensions
    pub fn get_extensions(&self) -> Vec<&'static str> {
        vec!["obj"]
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
        result.push_str("!Default value: 1.0(M)\n");
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
        result.push_str("!Read parameters:\n");
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Flag for reading vertex data with single or double floating point precision\n");
        result.push_str("!Default value: 0(false). Available values: 0(false), 1(true)\n");
        result.push_str(&format!("{}read.single.precision :\t {}\n", scope, self.read_single_precision as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Flag for create a single triangulation\n");
        result.push_str("!Default value: 0(false). Available values: 0(false), 1(true)\n");
        result.push_str(&format!("{}read.create.shapes :\t {}\n", scope, self.read_create_shapes as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Root folder for generating root labels names\n");
        result.push_str("!Default value: (empty). Available values: <path>\n");
        result.push_str(&format!("{}read.root.prefix :\t {}\n", scope, self.read_root_prefix));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Flag for fill document from shape sequence\n");
        result.push_str("!Default value: 1(true). Available values: 0(false), 1(true)\n");
        result.push_str(&format!("{}read.fill.doc :\t {}\n", scope, self.read_fill_doc as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Flag for fill the document with partially retrieved data even if reader has failed with error\n");
        result.push_str("!Default value: 1(true). Available values: 0(false), 1(true)\n");
        result.push_str(&format!("{}read.fill.incomplete :\t {}\n", scope, self.read_fill_incomplete as u8));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Memory usage limit(MiB)\n");
        result.push_str("!Default value: -1(no limit). Available values: -1(no limit), any positive value\n");
        result.push_str(&format!("{}read.memory.limit.mib :\t {}\n", scope, self.read_memory_limit_mib));
        result.push_str("!\n");

        result.push_str("!\n");
        result.push_str("!Write parameters:\n");
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
        let node = DeObjConfigurationNode::new();
        assert_eq!(node.file_length_unit, 1.0);
        assert_eq!(node.system_cs, CoordinateSystem::Zup);
        assert_eq!(node.file_cs, CoordinateSystem::Yup);
        assert!(!node.read_single_precision);
        assert!(!node.read_create_shapes);
        assert_eq!(node.read_root_prefix, "");
        assert!(node.read_fill_doc);
        assert!(node.read_fill_incomplete);
        assert_eq!(node.read_memory_limit_mib, -1);
        assert_eq!(node.write_comment, "");
        assert_eq!(node.write_author, "");
    }

    #[test]
    fn test_format_and_vendor() {
        let node = DeObjConfigurationNode::new();
        assert_eq!(node.get_format(), "OBJ");
        assert_eq!(node.get_vendor(), "OCC");
    }

    #[test]
    fn test_import_export_support() {
        let node = DeObjConfigurationNode::new();
        assert!(node.is_import_supported());
        assert!(node.is_export_supported());
    }

    #[test]
    fn test_extensions() {
        let node = DeObjConfigurationNode::new();
        let exts = node.get_extensions();
        assert_eq!(exts.len(), 1);
        assert_eq!(exts[0], "obj");
    }

    #[test]
    fn test_coordinate_system_conversion() {
        assert_eq!(CoordinateSystem::from_int(0), CoordinateSystem::Zup);
        assert_eq!(CoordinateSystem::from_int(1), CoordinateSystem::Yup);
        assert_eq!(CoordinateSystem::from_int(2), CoordinateSystem::Zup); // 2 % 2 = 0
        assert_eq!(CoordinateSystem::from_int(3), CoordinateSystem::Yup);  // 3 % 2 = 1
        assert_eq!(CoordinateSystem::Zup.to_int(), 0);
        assert_eq!(CoordinateSystem::Yup.to_int(), 1);
    }

    #[test]
    fn test_copy_functionality() {
        let mut node = DeObjConfigurationNode::new();
        node.file_length_unit = 2.5;
        node.read_single_precision = true;
        node.write_comment = "Test comment".to_string();

        let copied = node.copy();
        assert_eq!(copied.file_length_unit, 2.5);
        assert_eq!(copied.read_single_precision, true);
        assert_eq!(copied.write_comment, "Test comment");
    }

    #[test]
    fn test_save_configuration_string() {
        let node = DeObjConfigurationNode::new();
        let config_str = node.save();
        assert!(config_str.contains("OBJ"));
        assert!(config_str.contains("OCC"));
        assert!(config_str.contains("provider.OBJ.OCC."));
        assert!(config_str.contains("file.length.unit"));
        assert!(config_str.contains("read.create.shapes"));
    }

    #[test]
    fn test_mutable_fields() {
        let mut node = DeObjConfigurationNode::new();
        node.file_length_unit = 0.001;
        node.system_cs = CoordinateSystem::Yup;
        node.read_memory_limit_mib = 512;

        assert_eq!(node.file_length_unit, 0.001);
        assert_eq!(node.system_cs, CoordinateSystem::Yup);
        assert_eq!(node.read_memory_limit_mib, 512);
    }
}

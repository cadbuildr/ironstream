// FILE: xml_xcaf_drivers.rs
// occt: XmlXCAFDrivers

/// Factory and registry for XML drivers handling XCAF (eXtended CAF) document persistence.
/// Provides driver lookup, initialization, and centralized access to all XCAF XML drivers.
pub struct XmlXCAFDrivers;

impl XmlXCAFDrivers {
    /// Create the XCAF driver factory/registry.
    pub fn new() -> Self {
        XmlXCAFDrivers
    }

    /// Register all standard XCAF XML drivers into the persistence system.
    /// Must be called before any XCAF model persistence operations.
    pub fn factory_setup() -> &'static str {
        "XmlXCAFDrivers::Factory"
    }

    /// Retrieve XML driver name for a given XCAF attribute type.
    /// Returns the driver class name used for XML I/O of that type.
    pub fn get_driver_name(type_name: &str) -> Option<String> {
        match type_name {
            "XCAFDoc_ShapeMapTool" => Some("XmlXCAFDrivers_ShapeMapToolDriver".to_string()),
            "XCAFDoc_ColorTool" => Some("XmlXCAFDrivers_ColorToolDriver".to_string()),
            "XCAFDoc_LayerTool" => Some("XmlXCAFDrivers_LayerToolDriver".to_string()),
            "XCAFDoc_DimensionTool" => Some("XmlXCAFDrivers_DimensionToolDriver".to_string()),
            "XCAFDoc_Location" => Some("XmlXCAFDrivers_LocationDriver".to_string()),
            "XCAFDoc_Centroid" => Some("XmlXCAFDrivers_CentroidDriver".to_string()),
            "XCAFDoc_Area" => Some("XmlXCAFDrivers_AreaDriver".to_string()),
            "XCAFDoc_Volume" => Some("XmlXCAFDrivers_VolumeDriver".to_string()),
            _ => None,
        }
    }

    /// Get the current factory version number.
    pub fn factory_version() -> i32 {
        1
    }

    /// Check if a given XCAF type has an available XML driver.
    pub fn has_driver(type_name: &str) -> bool {
        Self::get_driver_name(type_name).is_some()
    }

    /// List all supported XCAF driver types.
    pub fn supported_types() -> Vec<&'static str> {
        vec![
            "XCAFDoc_ShapeMapTool",
            "XCAFDoc_ColorTool",
            "XCAFDoc_LayerTool",
            "XCAFDoc_DimensionTool",
            "XCAFDoc_Location",
            "XCAFDoc_Centroid",
            "XCAFDoc_Area",
            "XCAFDoc_Volume",
        ]
    }

    /// Get the count of supported driver types.
    pub fn driver_count() -> usize {
        Self::supported_types().len()
    }
}

impl Default for XmlXCAFDrivers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_setup_string() {
        assert_eq!(XmlXCAFDrivers::factory_setup(), "XmlXCAFDrivers::Factory");
    }

    #[test]
    fn test_get_driver_name_shape_map_tool() {
        assert_eq!(
            XmlXCAFDrivers::get_driver_name("XCAFDoc_ShapeMapTool"),
            Some("XmlXCAFDrivers_ShapeMapToolDriver".to_string())
        );
    }

    #[test]
    fn test_get_driver_name_color_tool() {
        assert_eq!(
            XmlXCAFDrivers::get_driver_name("XCAFDoc_ColorTool"),
            Some("XmlXCAFDrivers_ColorToolDriver".to_string())
        );
    }

    #[test]
    fn test_get_driver_name_layer_tool() {
        assert_eq!(
            XmlXCAFDrivers::get_driver_name("XCAFDoc_LayerTool"),
            Some("XmlXCAFDrivers_LayerToolDriver".to_string())
        );
    }

    #[test]
    fn test_get_driver_name_location() {
        assert_eq!(
            XmlXCAFDrivers::get_driver_name("XCAFDoc_Location"),
            Some("XmlXCAFDrivers_LocationDriver".to_string())
        );
    }

    #[test]
    fn test_get_driver_name_unknown() {
        assert_eq!(XmlXCAFDrivers::get_driver_name("UnknownType"), None);
    }

    #[test]
    fn test_has_driver_true() {
        assert!(XmlXCAFDrivers::has_driver("XCAFDoc_ColorTool"));
        assert!(XmlXCAFDrivers::has_driver("XCAFDoc_LayerTool"));
    }

    #[test]
    fn test_has_driver_false() {
        assert!(!XmlXCAFDrivers::has_driver("UnknownType"));
    }

    #[test]
    fn test_factory_version() {
        assert_eq!(XmlXCAFDrivers::factory_version(), 1);
    }

    #[test]
    fn test_supported_types_not_empty() {
        let types = XmlXCAFDrivers::supported_types();
        assert!(!types.is_empty());
        assert!(types.contains(&"XCAFDoc_ColorTool"));
    }

    #[test]
    fn test_driver_count() {
        let count = XmlXCAFDrivers::driver_count();
        assert_eq!(count, 8);
    }

    #[test]
    fn test_all_supported_types_have_drivers() {
        for type_name in XmlXCAFDrivers::supported_types() {
            assert!(XmlXCAFDrivers::has_driver(type_name), "No driver for {}", type_name);
        }
    }
}

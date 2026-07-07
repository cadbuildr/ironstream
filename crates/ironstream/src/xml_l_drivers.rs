// FILE: xml_l_drivers.rs
// occt: XmlLDrivers

use std::collections::HashMap;

/// GUID for XML storage driver plugin (Low-level).
pub const XML_L_STORAGE_DRIVER_GUID: &str = "03a56820-8269-11d5-aab2-0050044b1af1";

/// GUID for XML retrieval driver plugin (Low-level).
pub const XML_L_RETRIEVAL_DRIVER_GUID: &str = "03a56822-8269-11d5-aab2-0050044b1af1";

/// Low-level factory for XML drivers.
/// Provides access to base storage and retrieval drivers for XML OCAF documents.
pub struct XmlLDrivers {
    driver_map: HashMap<String, String>,
}

impl XmlLDrivers {
    /// Create a new instance of the XML low-level drivers factory.
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            XML_L_STORAGE_DRIVER_GUID.to_string(),
            "DocumentStorageDriver".to_string(),
        );
        map.insert(
            XML_L_RETRIEVAL_DRIVER_GUID.to_string(),
            "DocumentRetrievalDriver".to_string(),
        );

        XmlLDrivers { driver_map: map }
    }

    /// Look up a driver by GUID.
    pub fn factory(&self, guid: &str) -> Option<&str> {
        self.driver_map.get(guid).map(|s| s.as_str())
    }

    /// Get the creation date string.
    pub fn creation_date() -> String {
        "2001-07-25".to_string()
    }

    /// Define XML LOACF format (Low-level OCAF).
    pub fn define_format() -> FormatDefinition {
        FormatDefinition {
            name: "XmlLOcaf".to_string(),
            description: "Xml Low-level OCAF Document".to_string(),
            extension: "xml".to_string(),
        }
    }

    /// Get all registered drivers.
    pub fn list_drivers(&self) -> Vec<&str> {
        self.driver_map.values().map(|s| s.as_str()).collect()
    }
}

impl Default for XmlLDrivers {
    fn default() -> Self {
        Self::new()
    }
}

/// Format definition for low-level XML documents.
pub struct FormatDefinition {
    pub name: String,
    pub description: String,
    pub extension: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = XmlLDrivers::new();
        assert_eq!(factory.list_drivers().len(), 2);
    }

    #[test]
    fn test_factory_lookup_storage() {
        let factory = XmlLDrivers::new();
        let driver = factory.factory(XML_L_STORAGE_DRIVER_GUID);
        assert_eq!(driver, Some("DocumentStorageDriver"));
    }

    #[test]
    fn test_factory_lookup_retrieval() {
        let factory = XmlLDrivers::new();
        let driver = factory.factory(XML_L_RETRIEVAL_DRIVER_GUID);
        assert_eq!(driver, Some("DocumentRetrievalDriver"));
    }

    #[test]
    fn test_factory_unknown_guid() {
        let factory = XmlLDrivers::new();
        let driver = factory.factory("unknown-guid");
        assert_eq!(driver, None);
    }

    #[test]
    fn test_creation_date() {
        let date = XmlLDrivers::creation_date();
        assert!(!date.is_empty());
        assert_eq!(date, "2001-07-25");
    }

    #[test]
    fn test_define_format() {
        let format = XmlLDrivers::define_format();
        assert_eq!(format.name, "XmlLOcaf");
        assert!(format.description.contains("Low-level"));
        assert_eq!(format.extension, "xml");
    }

    #[test]
    fn test_default_construction() {
        let factory = XmlLDrivers::default();
        assert_eq!(factory.list_drivers().len(), 2);
    }

    #[test]
    fn test_guids_are_distinct() {
        assert_ne!(XML_L_STORAGE_DRIVER_GUID, XML_L_RETRIEVAL_DRIVER_GUID);
    }

    #[test]
    fn test_format_name_differs_from_standard() {
        let lformat = XmlLDrivers::define_format();
        assert!(lformat.name.contains("L"));
    }
}

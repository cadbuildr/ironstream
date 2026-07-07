// FILE: xml_drivers.rs
// occt: XmlDrivers

use std::collections::HashMap;

/// GUID for XML storage driver plugin.
pub const XML_STORAGE_DRIVER_GUID: &str = "03a56820-8269-11d5-aab2-0050044b1af1";

/// GUID for XML retrieval driver plugin.
pub const XML_RETRIEVAL_DRIVER_GUID: &str = "03a56822-8269-11d5-aab2-0050044b1af1";

/// Central factory for XML drivers.
/// Provides access to storage and retrieval drivers for XML OCAF documents.
pub struct XmlDrivers {
    driver_map: HashMap<String, String>,
}

impl XmlDrivers {
    /// Create a new instance of the XML drivers factory.
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            XML_STORAGE_DRIVER_GUID.to_string(),
            "DocumentStorageDriver".to_string(),
        );
        map.insert(
            XML_RETRIEVAL_DRIVER_GUID.to_string(),
            "DocumentRetrievalDriver".to_string(),
        );

        XmlDrivers { driver_map: map }
    }

    /// Look up a driver by GUID.
    /// Returns the driver type name or None if GUID is unknown.
    pub fn factory(&self, guid: &str) -> Option<&str> {
        self.driver_map.get(guid).map(|s| s.as_str())
    }

    /// Define XML OCAF format.
    /// Registers the format and associates it with file extension ".xml".
    pub fn define_format() -> FormatDefinition {
        FormatDefinition {
            name: "XmlOcaf".to_string(),
            description: "Xml OCAF Document".to_string(),
            extension: "xml".to_string(),
            copyright: "Copyright: Open Cascade, 2001-2002".to_string(),
        }
    }

    /// Get all registered drivers.
    pub fn list_drivers(&self) -> Vec<&str> {
        self.driver_map.values().map(|s| s.as_str()).collect()
    }
}

impl Default for XmlDrivers {
    fn default() -> Self {
        Self::new()
    }
}

/// Format definition for XML documents.
pub struct FormatDefinition {
    pub name: String,
    pub description: String,
    pub extension: String,
    pub copyright: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = XmlDrivers::new();
        assert_eq!(factory.list_drivers().len(), 2);
    }

    #[test]
    fn test_factory_lookup_storage() {
        let factory = XmlDrivers::new();
        let driver = factory.factory(XML_STORAGE_DRIVER_GUID);
        assert_eq!(driver, Some("DocumentStorageDriver"));
    }

    #[test]
    fn test_factory_lookup_retrieval() {
        let factory = XmlDrivers::new();
        let driver = factory.factory(XML_RETRIEVAL_DRIVER_GUID);
        assert_eq!(driver, Some("DocumentRetrievalDriver"));
    }

    #[test]
    fn test_factory_unknown_guid() {
        let factory = XmlDrivers::new();
        let driver = factory.factory("unknown-guid");
        assert_eq!(driver, None);
    }

    #[test]
    fn test_define_format() {
        let format = XmlDrivers::define_format();
        assert_eq!(format.name, "XmlOcaf");
        assert_eq!(format.extension, "xml");
        assert!(!format.copyright.is_empty());
    }

    #[test]
    fn test_default_construction() {
        let factory = XmlDrivers::default();
        assert_eq!(factory.list_drivers().len(), 2);
    }

    #[test]
    fn test_guids_are_distinct() {
        assert_ne!(XML_STORAGE_DRIVER_GUID, XML_RETRIEVAL_DRIVER_GUID);
    }
}

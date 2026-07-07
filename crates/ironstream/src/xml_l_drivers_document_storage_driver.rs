// FILE: xml_l_drivers_document_storage_driver.rs
// occt: XmlLDrivers_DocumentStorageDriver

use std::collections::HashMap;

/// Low-level XML document storage driver.
/// Base class for writing and serializing XML OCAF documents.
/// Provides interface for file I/O and stream writing.
pub struct XmlLDriversDocumentStorageDriver {
    type_name: String,
    copyright: String,
    file_name: String,
    namespaces: HashMap<String, String>,
    is_initialized: bool,
}

impl XmlLDriversDocumentStorageDriver {
    /// Create a new low-level document storage driver with copyright notice.
    pub fn new(copyright: String) -> Self {
        XmlLDriversDocumentStorageDriver {
            type_name: "XmlLDrivers_DocumentStorageDriver".to_string(),
            copyright,
            file_name: String::new(),
            namespaces: HashMap::new(),
            is_initialized: false,
        }
    }

    /// Get the type name of this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Get the copyright notice.
    pub fn copyright(&self) -> &str {
        &self.copyright
    }

    /// Get the file name.
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Set the file name to write to.
    pub fn set_file_name(&mut self, name: String) {
        self.file_name = name;
    }

    /// Check if driver is initialized.
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    /// Mark driver as initialized.
    pub fn set_initialized(&mut self, initialized: bool) {
        self.is_initialized = initialized;
    }

    /// Add a namespace definition.
    pub fn add_namespace(&mut self, prefix: String, uri: String) {
        self.namespaces.insert(prefix, uri);
    }

    /// Get a namespace URI by prefix.
    pub fn get_namespace(&self, prefix: &str) -> Option<&str> {
        self.namespaces.get(prefix).map(|s| s.as_str())
    }

    /// Get all namespaces.
    pub fn namespaces(&self) -> &HashMap<String, String> {
        &self.namespaces
    }

    /// Write to DOM document.
    pub fn write_to_dom_document(&mut self) -> Result<(), String> {
        if self.copyright.is_empty() {
            return Err("Copyright not set".to_string());
        }
        Ok(())
    }

    /// Create document structure.
    pub fn make_document(&mut self) -> Result<i32, String> {
        if !self.is_initialized {
            return Err("Driver not initialized".to_string());
        }
        Ok(0)
    }

    /// Write shape section to DOM element.
    pub fn write_shape_section(&mut self) -> Result<bool, String> {
        Ok(false)
    }

    /// Clean up resources.
    pub fn cleanup(&mut self) {
        self.is_initialized = false;
        self.namespaces.clear();
    }
}

impl Default for XmlLDriversDocumentStorageDriver {
    fn default() -> Self {
        Self::new("Copyright: Open Cascade".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlLDriversDocumentStorageDriver::new("Copyright".to_string());
        assert_eq!(driver.type_name(), "XmlLDrivers_DocumentStorageDriver");
    }

    #[test]
    fn test_copyright_preserved() {
        let copyright = "Copyright: Company, 2020".to_string();
        let driver = XmlLDriversDocumentStorageDriver::new(copyright.clone());
        assert_eq!(driver.copyright(), copyright);
    }

    #[test]
    fn test_initial_file_name_empty() {
        let driver = XmlLDriversDocumentStorageDriver::new("Test".to_string());
        assert_eq!(driver.file_name(), "");
    }

    #[test]
    fn test_set_file_name() {
        let mut driver = XmlLDriversDocumentStorageDriver::new("Test".to_string());
        driver.set_file_name("output.xml".to_string());
        assert_eq!(driver.file_name(), "output.xml");
    }

    #[test]
    fn test_add_and_get_namespace() {
        let mut driver = XmlLDriversDocumentStorageDriver::new("Test".to_string());
        driver.add_namespace("xs".to_string(), "http://www.w3.org/2001/XMLSchema".to_string());
        assert_eq!(
            driver.get_namespace("xs"),
            Some("http://www.w3.org/2001/XMLSchema")
        );
    }

    #[test]
    fn test_get_nonexistent_namespace() {
        let driver = XmlLDriversDocumentStorageDriver::new("Test".to_string());
        assert_eq!(driver.get_namespace("nonexistent"), None);
    }

    #[test]
    fn test_write_to_dom_document() {
        let mut driver = XmlLDriversDocumentStorageDriver::new("Copyright".to_string());
        assert!(driver.write_to_dom_document().is_ok());
    }

    #[test]
    fn test_make_document_without_init() {
        let mut driver = XmlLDriversDocumentStorageDriver::new("Test".to_string());
        assert!(driver.make_document().is_err());
    }

    #[test]
    fn test_make_document_with_init() {
        let mut driver = XmlLDriversDocumentStorageDriver::new("Test".to_string());
        driver.set_initialized(true);
        assert!(driver.make_document().is_ok());
    }

    #[test]
    fn test_write_shape_section() {
        let mut driver = XmlLDriversDocumentStorageDriver::new("Test".to_string());
        assert!(driver.write_shape_section().is_ok());
    }

    #[test]
    fn test_cleanup() {
        let mut driver = XmlLDriversDocumentStorageDriver::new("Test".to_string());
        driver.set_initialized(true);
        driver.add_namespace("test".to_string(), "http://test.com".to_string());
        driver.cleanup();
        assert!(!driver.is_initialized());
        assert_eq!(driver.namespaces().len(), 0);
    }

    #[test]
    fn test_default_construction() {
        let driver = XmlLDriversDocumentStorageDriver::default();
        assert!(!driver.copyright().is_empty());
    }
}

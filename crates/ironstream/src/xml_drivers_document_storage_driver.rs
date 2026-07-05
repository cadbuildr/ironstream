// FILE: xml_drivers_document_storage_driver.rs
// occt: XmlDrivers_DocumentStorageDriver

/// XML document storage driver.
/// Responsible for writing XML OCAF documents and serializing their attributes.
/// Extends the low-level (L) document storage driver with high-level (standard) functionality.
pub struct XmlDriversDocumentStorageDriver {
    copyright: String,
    drivers_initialized: bool,
}

impl XmlDriversDocumentStorageDriver {
    /// Create a new document storage driver with the specified copyright notice.
    pub fn new(copyright: String) -> Self {
        XmlDriversDocumentStorageDriver {
            copyright,
            drivers_initialized: false,
        }
    }

    /// Get the copyright notice for the stored documents.
    pub fn copyright(&self) -> &str {
        &self.copyright
    }

    /// Check if the driver has been initialized with attribute drivers.
    pub fn is_initialized(&self) -> bool {
        self.drivers_initialized
    }

    /// Mark the driver as initialized.
    pub fn set_initialized(&mut self, initialized: bool) {
        self.drivers_initialized = initialized;
    }

    /// Get the type name of this driver.
    pub fn type_name(&self) -> &str {
        "XmlDrivers_DocumentStorageDriver"
    }

    /// Write shape section to XML element.
    /// Returns Ok(true) if shapes were written, Ok(false) if no shapes to write.
    pub fn write_shape_section(&mut self) -> Result<bool, String> {
        if !self.drivers_initialized {
            return Err("Drivers not initialized".to_string());
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation_with_copyright() {
        let copyright = "Copyright: Open Cascade, 2001-2002".to_string();
        let driver = XmlDriversDocumentStorageDriver::new(copyright.clone());
        assert_eq!(driver.copyright(), copyright);
    }

    #[test]
    fn test_type_name() {
        let driver = XmlDriversDocumentStorageDriver::new("Test".to_string());
        assert_eq!(driver.type_name(), "XmlDrivers_DocumentStorageDriver");
    }

    #[test]
    fn test_initialization_flag() {
        let mut driver = XmlDriversDocumentStorageDriver::new("Test".to_string());
        assert!(!driver.is_initialized());
        driver.set_initialized(true);
        assert!(driver.is_initialized());
    }

    #[test]
    fn test_write_shape_section_without_init() {
        let mut driver = XmlDriversDocumentStorageDriver::new("Test".to_string());
        let result = driver.write_shape_section();
        assert!(result.is_err());
    }

    #[test]
    fn test_write_shape_section_with_init() {
        let mut driver = XmlDriversDocumentStorageDriver::new("Test".to_string());
        driver.set_initialized(true);
        let result = driver.write_shape_section();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_copyright_preserved() {
        let copyright1 = "Copyright: Company A, 2020".to_string();
        let copyright2 = "Copyright: Company B, 2021".to_string();
        let driver1 = XmlDriversDocumentStorageDriver::new(copyright1.clone());
        let driver2 = XmlDriversDocumentStorageDriver::new(copyright2.clone());
        assert_eq!(driver1.copyright(), copyright1);
        assert_eq!(driver2.copyright(), copyright2);
    }

    #[test]
    fn test_copyright_not_empty() {
        let driver = XmlDriversDocumentStorageDriver::new("Copyright".to_string());
        assert!(!driver.copyright().is_empty());
    }
}

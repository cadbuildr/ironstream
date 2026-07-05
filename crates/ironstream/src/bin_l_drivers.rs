// FILE: bin_l_drivers.rs
// occt: BinLDrivers

use std::collections::HashMap;

/// Binary Lite OCAF drivers registry.
/// Manages document storage and retrieval drivers, and provides attribute driver table.
pub struct BinLDrivers;

impl BinLDrivers {
    /// GUID for BinL storage driver plugin.
    pub const STORAGE_DRIVER_GUID: &'static str = "13a56835-8269-11d5-aab2-0050044b1af1";

    /// GUID for BinL retrieval driver plugin.
    pub const RETRIEVAL_DRIVER_GUID: &'static str = "13a56836-8269-11d5-aab2-0050044b1af1";

    /// Get driver factory by GUID.
    pub fn factory(guid: &str) -> Result<DriverType, String> {
        match guid {
            Self::STORAGE_DRIVER_GUID => Ok(DriverType::StorageDriver),
            Self::RETRIEVAL_DRIVER_GUID => Ok(DriverType::RetrievalDriver),
            _ => Err(format!("BinLDrivers: unknown GUID {}", guid)),
        }
    }

    /// Define BinLOcaf format in the application.
    pub fn define_format(app: &mut MockApplication) {
        app.define_format(
            "BinLOcaf",
            "Binary Lite OCAF Document",
            "cbfl",
        );
    }

    /// Create and populate attribute drivers table.
    pub fn attribute_drivers(_message_driver: Option<String>) -> DriverTable {
        let mut table = DriverTable::new();

        // Add drivers from various modules
        table.add_all_drivers();

        table
    }
}

/// Available driver types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriverType {
    StorageDriver,
    RetrievalDriver,
}

/// Mock application for format definition.
#[derive(Clone, Debug)]
pub struct MockApplication {
    formats: HashMap<String, FormatInfo>,
}

impl MockApplication {
    pub fn new() -> Self {
        MockApplication {
            formats: HashMap::new(),
        }
    }

    pub fn define_format(&mut self, name: &str, description: &str, extension: &str) {
        self.formats.insert(
            name.to_string(),
            FormatInfo {
                name: name.to_string(),
                description: description.to_string(),
                extension: extension.to_string(),
            },
        );
    }

    pub fn get_format(&self, name: &str) -> Option<&FormatInfo> {
        self.formats.get(name)
    }

    pub fn format_count(&self) -> usize {
        self.formats.len()
    }
}

impl Default for MockApplication {
    fn default() -> Self {
        MockApplication::new()
    }
}

/// Format information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormatInfo {
    pub name: String,
    pub description: String,
    pub extension: String,
}

/// Attribute drivers table.
#[derive(Clone, Debug)]
pub struct DriverTable {
    drivers: Vec<String>,
}

impl DriverTable {
    pub fn new() -> Self {
        DriverTable {
            drivers: Vec::new(),
        }
    }

    pub fn add_driver(&mut self, driver_name: &str) {
        self.drivers.push(driver_name.to_string());
    }

    pub fn add_all_drivers(&mut self) {
        // Add drivers from BinMDF module
        self.drivers.push("BinMDF drivers".to_string());

        // Add drivers from BinMDataStd module
        self.drivers.push("BinMDataStd drivers".to_string());

        // Add drivers from BinMFunction module
        self.drivers.push("BinMFunction drivers".to_string());

        // Add drivers from BinMDocStd module
        self.drivers.push("BinMDocStd drivers".to_string());
    }

    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }

    pub fn has_driver(&self, name: &str) -> bool {
        self.drivers.iter().any(|d| d.contains(name))
    }
}

impl Default for DriverTable {
    fn default() -> Self {
        DriverTable::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_storage_driver() {
        let result = BinLDrivers::factory(BinLDrivers::STORAGE_DRIVER_GUID);
        assert_eq!(result.unwrap(), DriverType::StorageDriver);
    }

    #[test]
    fn test_factory_retrieval_driver() {
        let result = BinLDrivers::factory(BinLDrivers::RETRIEVAL_DRIVER_GUID);
        assert_eq!(result.unwrap(), DriverType::RetrievalDriver);
    }

    #[test]
    fn test_factory_unknown_guid() {
        let result = BinLDrivers::factory("unknown-guid");
        assert!(result.is_err());
    }

    #[test]
    fn test_define_format() {
        let mut app = MockApplication::new();
        assert_eq!(app.format_count(), 0);

        BinLDrivers::define_format(&mut app);

        assert_eq!(app.format_count(), 1);
        let fmt = app.get_format("BinLOcaf").unwrap();
        assert_eq!(fmt.name, "BinLOcaf");
        assert_eq!(fmt.extension, "cbfl");
    }

    #[test]
    fn test_attribute_drivers() {
        let table = BinLDrivers::attribute_drivers(None);
        assert!(table.driver_count() > 0);
    }

    #[test]
    fn test_driver_table_creation() {
        let table = DriverTable::new();
        assert_eq!(table.driver_count(), 0);
    }

    #[test]
    fn test_driver_table_add_driver() {
        let mut table = DriverTable::new();
        table.add_driver("TestDriver");

        assert_eq!(table.driver_count(), 1);
        assert!(table.has_driver("TestDriver"));
    }

    #[test]
    fn test_driver_table_add_all_drivers() {
        let mut table = DriverTable::new();
        table.add_all_drivers();

        assert!(table.driver_count() >= 4);
        assert!(table.has_driver("BinMDF"));
        assert!(table.has_driver("BinMDataStd"));
    }

    #[test]
    fn test_format_info() {
        let fmt = FormatInfo {
            name: "BinLOcaf".to_string(),
            description: "Binary Lite OCAF Document".to_string(),
            extension: "cbfl".to_string(),
        };

        assert_eq!(fmt.name, "BinLOcaf");
        assert_eq!(fmt.extension, "cbfl");
    }

    #[test]
    fn test_mock_application_multiple_formats() {
        let mut app = MockApplication::new();

        app.define_format("Format1", "Description 1", "fmt1");
        app.define_format("Format2", "Description 2", "fmt2");

        assert_eq!(app.format_count(), 2);
        assert!(app.get_format("Format1").is_some());
        assert!(app.get_format("Format2").is_some());
    }

    #[test]
    fn test_guids_are_valid_strings() {
        assert!(!BinLDrivers::STORAGE_DRIVER_GUID.is_empty());
        assert!(!BinLDrivers::RETRIEVAL_DRIVER_GUID.is_empty());
        assert_ne!(BinLDrivers::STORAGE_DRIVER_GUID, BinLDrivers::RETRIEVAL_DRIVER_GUID);
    }
}

// FILE: bin_l_drivers_document_retrieval_driver.rs
// occt: BinLDrivers_DocumentRetrievalDriver

use std::collections::HashMap;

/// Binary document retrieval driver for BinL format.
/// Handles reading and deserialization of binary OCAF documents.
pub struct BinLDriversDocumentRetrievalDriver {
    reader_status: ReaderStatus,
    message_driver: Option<String>,
}

impl BinLDriversDocumentRetrievalDriver {
    pub fn new() -> Self {
        BinLDriversDocumentRetrievalDriver {
            reader_status: ReaderStatus::OK,
            message_driver: None,
        }
    }

    pub fn with_message_driver(mut self, msg_driver: Option<String>) -> Self {
        self.message_driver = msg_driver;
        self
    }

    /// Get the reader status.
    pub fn reader_status(&self) -> ReaderStatus {
        self.reader_status
    }

    /// Set the reader status.
    pub fn set_reader_status(&mut self, status: ReaderStatus) {
        self.reader_status = status;
    }

    /// Check document version compatibility.
    pub fn check_document_version(&self, file_version: i32, current_version: i32) -> bool {
        // Default implementation: file version must be within acceptable range
        file_version >= 2 && file_version <= current_version
    }

    /// Check if quick part reading is enabled for the given file version.
    pub fn is_quick_part(file_version: i32) -> bool {
        file_version >= 8 // Quick part supported from version 8 onwards
    }

    /// Clear internal driver cache.
    pub fn clear(&mut self) {
        // Clear any cached data in drivers
    }

    /// Get attribute drivers table.
    pub fn attribute_drivers(&self, _message_driver: Option<String>) -> DriverTable {
        DriverTable::new()
    }
}

impl Default for BinLDriversDocumentRetrievalDriver {
    fn default() -> Self {
        BinLDriversDocumentRetrievalDriver::new()
    }
}

/// Reader status codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReaderStatus {
    OK = 0,
    OpenError = 1,
    UserBreak = 2,
    FormatError = 3,
    TypeNotFound = 4,
    UnknownFileFormat = 5,
}

impl ReaderStatus {
    pub fn is_ok(&self) -> bool {
        *self == ReaderStatus::OK
    }

    pub fn is_error(&self) -> bool {
        !self.is_ok()
    }
}

/// Mock driver table for attribute handling.
#[derive(Clone, Debug)]
pub struct DriverTable {
    drivers: HashMap<String, String>,
}

impl DriverTable {
    pub fn new() -> Self {
        DriverTable {
            drivers: HashMap::new(),
        }
    }

    pub fn add_driver(&mut self, name: &str, description: &str) {
        self.drivers.insert(name.to_string(), description.to_string());
    }

    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }

    pub fn has_driver(&self, name: &str) -> bool {
        self.drivers.contains_key(name)
    }
}

impl Default for DriverTable {
    fn default() -> Self {
        DriverTable::new()
    }
}

/// Document section for binary format.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentSection {
    pub name: String,
    pub offset: i32,
    pub size: i32,
}

impl DocumentSection {
    pub fn new(name: &str, offset: i32, size: i32) -> Self {
        DocumentSection {
            name: name.to_string(),
            offset,
            size,
        }
    }

    pub fn is_shape_section(&self) -> bool {
        self.name.contains("SHAPE_SECTION")
    }
}

/// Relocation table for object mapping.
#[derive(Clone, Debug)]
pub struct RelocationTable {
    entries: HashMap<i32, i32>,
}

impl RelocationTable {
    pub fn new() -> Self {
        RelocationTable {
            entries: HashMap::new(),
        }
    }

    pub fn add(&mut self, source_id: i32, target_id: i32) {
        self.entries.insert(source_id, target_id);
    }

    pub fn get(&self, source_id: i32) -> Option<i32> {
        self.entries.get(&source_id).copied()
    }

    pub fn is_bound(&self, source_id: i32) -> bool {
        self.entries.contains_key(&source_id)
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for RelocationTable {
    fn default() -> Self {
        RelocationTable::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinLDriversDocumentRetrievalDriver::new();
        assert_eq!(driver.reader_status(), ReaderStatus::OK);
    }

    #[test]
    fn test_reader_status_ok() {
        assert!(ReaderStatus::OK.is_ok());
        assert!(!ReaderStatus::OK.is_error());
    }

    #[test]
    fn test_reader_status_errors() {
        assert!(!ReaderStatus::OpenError.is_ok());
        assert!(ReaderStatus::OpenError.is_error());
        assert!(!ReaderStatus::UserBreak.is_ok());
        assert!(ReaderStatus::UserBreak.is_error());
    }

    #[test]
    fn test_set_reader_status() {
        let mut driver = BinLDriversDocumentRetrievalDriver::new();
        driver.set_reader_status(ReaderStatus::OpenError);
        assert_eq!(driver.reader_status(), ReaderStatus::OpenError);
    }

    #[test]
    fn test_check_document_version_compatible() {
        let driver = BinLDriversDocumentRetrievalDriver::new();
        assert!(driver.check_document_version(5, 10));
        assert!(driver.check_document_version(2, 10));
        assert!(driver.check_document_version(10, 10));
    }

    #[test]
    fn test_check_document_version_incompatible() {
        let driver = BinLDriversDocumentRetrievalDriver::new();
        assert!(!driver.check_document_version(1, 10)); // too old
        assert!(!driver.check_document_version(11, 10)); // too new
    }

    #[test]
    fn test_is_quick_part() {
        assert!(!BinLDriversDocumentRetrievalDriver::is_quick_part(7));
        assert!(BinLDriversDocumentRetrievalDriver::is_quick_part(8));
        assert!(BinLDriversDocumentRetrievalDriver::is_quick_part(9));
    }

    #[test]
    fn test_clear() {
        let mut driver = BinLDriversDocumentRetrievalDriver::new();
        driver.clear();
        // Should not panic and clear internal state
    }

    #[test]
    fn test_driver_table_creation() {
        let table = DriverTable::new();
        assert_eq!(table.driver_count(), 0);
    }

    #[test]
    fn test_driver_table_add_driver() {
        let mut table = DriverTable::new();
        table.add_driver("TestDriver", "Test Description");

        assert_eq!(table.driver_count(), 1);
        assert!(table.has_driver("TestDriver"));
    }

    #[test]
    fn test_document_section_creation() {
        let section = DocumentSection::new("SHAPES", 100, 500);
        assert_eq!(section.name, "SHAPES");
        assert_eq!(section.offset, 100);
        assert_eq!(section.size, 500);
    }

    #[test]
    fn test_document_section_is_shape_section() {
        let shape_section = DocumentSection::new("SHAPE_SECTION", 0, 100);
        assert!(shape_section.is_shape_section());

        let other_section = DocumentSection::new("DATA", 100, 200);
        assert!(!other_section.is_shape_section());
    }

    #[test]
    fn test_relocation_table_creation() {
        let table = RelocationTable::new();
        assert_eq!(table.entry_count(), 0);
    }

    #[test]
    fn test_relocation_table_add_and_get() {
        let mut table = RelocationTable::new();
        table.add(1, 100);
        table.add(2, 200);

        assert_eq!(table.get(1), Some(100));
        assert_eq!(table.get(2), Some(200));
        assert_eq!(table.get(3), None);
    }

    #[test]
    fn test_relocation_table_is_bound() {
        let mut table = RelocationTable::new();
        table.add(1, 100);

        assert!(table.is_bound(1));
        assert!(!table.is_bound(2));
    }

    #[test]
    fn test_relocation_table_clear() {
        let mut table = RelocationTable::new();
        table.add(1, 100);
        assert_eq!(table.entry_count(), 1);

        table.clear();
        assert_eq!(table.entry_count(), 0);
        assert!(!table.is_bound(1));
    }

    #[test]
    fn test_with_message_driver() {
        let driver = BinLDriversDocumentRetrievalDriver::new()
            .with_message_driver(Some("TestDriver".to_string()));
        assert_eq!(driver.message_driver, Some("TestDriver".to_string()));
    }

    #[test]
    fn test_attribute_drivers() {
        let driver = BinLDriversDocumentRetrievalDriver::new();
        let table = driver.attribute_drivers(None);
        assert_eq!(table.driver_count(), 0);
    }
}

// FILE: bin_l_drivers_document_storage_driver.rs
// occt: BinLDrivers_DocumentStorageDriver

/// Binary document storage driver for BinL format.
/// Handles writing and serialization of binary OCAF documents.
pub struct BinLDriversDocumentStorageDriver {
    storage_status: StorageStatus,
    message_driver: Option<String>,
}

impl BinLDriversDocumentStorageDriver {
    pub fn new() -> Self {
        BinLDriversDocumentStorageDriver {
            storage_status: StorageStatus::OK,
            message_driver: None,
        }
    }

    pub fn with_message_driver(mut self, msg_driver: Option<String>) -> Self {
        self.message_driver = msg_driver;
        self
    }

    pub fn storage_status(&self) -> StorageStatus {
        self.storage_status
    }

    pub fn set_storage_status(&mut self, status: StorageStatus) {
        self.storage_status = status;
    }
}

impl Default for BinLDriversDocumentStorageDriver {
    fn default() -> Self {
        BinLDriversDocumentStorageDriver::new()
    }
}

/// Storage status codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageStatus {
    OK = 0,
    SomeDataNotWritten = 1,
    Failure = 2,
}

impl StorageStatus {
    pub fn is_ok(&self) -> bool {
        *self == StorageStatus::OK
    }

    pub fn is_error(&self) -> bool {
        !self.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinLDriversDocumentStorageDriver::new();
        assert_eq!(driver.storage_status(), StorageStatus::OK);
    }

    #[test]
    fn test_storage_status_ok() {
        assert!(StorageStatus::OK.is_ok());
        assert!(!StorageStatus::OK.is_error());
    }

    #[test]
    fn test_storage_status_errors() {
        assert!(!StorageStatus::SomeDataNotWritten.is_ok());
        assert!(StorageStatus::SomeDataNotWritten.is_error());
    }

    #[test]
    fn test_with_message_driver() {
        let driver = BinLDriversDocumentStorageDriver::new()
            .with_message_driver(Some("TestDriver".to_string()));
        assert_eq!(driver.message_driver, Some("TestDriver".to_string()));
    }
}

// FILE: pcdm_store_status.rs
// occt: PCDM_StoreStatus

/// Status of storage of a document on disk
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PCDMStoreStatus {
    Ok = 0,                 // Document is saved successfully
    DriverFailure = 1,      // Storage driver is not found
    WriteFailure = 2,       // Attempt to write a file on disk failed
    Failure = 3,            // A general error occurred (unexpected)
    DocIsNull = 4,          // Attempt to save a null document
    NoObj = 5,              // Document has no objects to be saved
    InfoSectionError = 6,   // Error occurred on writing of an information-section
    UserBreak = 7,          // User interrupted the process of storage
    UnrecognizedFormat = 8, // No storage driver exist for this document format
}

impl Default for PCDMStoreStatus {
    fn default() -> Self {
        PCDMStoreStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_status_values() {
        assert_eq!(PCDMStoreStatus::Ok as i32, 0);
        assert_eq!(PCDMStoreStatus::DriverFailure as i32, 1);
        assert_eq!(PCDMStoreStatus::UnrecognizedFormat as i32, 8);
    }

    #[test]
    fn test_default_status() {
        let status: PCDMStoreStatus = Default::default();
        assert_eq!(status, PCDMStoreStatus::Ok);
    }
}

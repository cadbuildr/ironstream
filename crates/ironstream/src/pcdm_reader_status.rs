// FILE: pcdm_reader_status.rs
// occt: PCDM_ReaderStatus

/// Status of reading of a document
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PCDMReaderStatus {
    Ok = 0,                          // Success
    NoDriver = 1,                    // No driver for file format
    UnknownFileDriver = 2,           // File is bad
    OpenError = 3,                   // Can't open file
    NoVersion = 4,                   // Unknown document version
    NoSchema = 5,                    // NOT USED
    NoDocument = 6,                  // Document is empty
    ExtensionFailure = 7,            // NOT USED
    WrongStreamMode = 8,             // Open mode is mistaken
    FormatFailure = 9,               // Document data structure is wrong
    TypeFailure = 10,                // Data type is unknown
    TypeNotFoundInSchema = 11,       // Data type is not found in schema
    UnrecognizedFileFormat = 12,     // Document data structure is wrong
    MakeFailure = 13,                // Conversion of data failed
    PermissionDenied = 14,           // Permission denied to open file
    DriverFailure = 15,              // General mistake of reading
    AlreadyRetrievedAndModified = 16, // Document is already retrieved and modified
    AlreadyRetrieved = 17,           // Document is already retrieved
    UnknownDocument = 18,            // File doesn't exist
    WrongResource = 19,              // Wrong resource file
    ReaderException = 20,            // Wrong data structure
    NoModel = 21,                    // NOT USED
    UserBreak = 22,                  // User interrupted reading
}

impl Default for PCDMReaderStatus {
    fn default() -> Self {
        PCDMReaderStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_status_values() {
        assert_eq!(PCDMReaderStatus::Ok as i32, 0);
        assert_eq!(PCDMReaderStatus::NoDriver as i32, 1);
        assert_eq!(PCDMReaderStatus::UserBreak as i32, 22);
    }

    #[test]
    fn test_default_status() {
        let status: PCDMReaderStatus = Default::default();
        assert_eq!(status, PCDMReaderStatus::Ok);
    }
}

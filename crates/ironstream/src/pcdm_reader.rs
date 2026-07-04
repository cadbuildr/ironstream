// FILE: pcdm_reader.rs
// occt: PCDM_Reader

/// Reader status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReaderStatus {
    Ok = 0,
    NoDriver = 1,
    UnknownFileDriver = 2,
    OpenError = 3,
    NoVersion = 4,
    NoSchema = 5,
    NoDocument = 6,
}

/// Abstract reader for persistent documents
pub struct PCDMReader {
    status: ReaderStatus,
}

impl PCDMReader {
    /// Create a new reader
    pub fn new() -> Self {
        PCDMReader {
            status: ReaderStatus::Ok,
        }
    }

    /// Get the reader status
    pub fn get_status(&self) -> ReaderStatus {
        self.status
    }

    /// Set the reader status
    pub fn set_status(&mut self, status: ReaderStatus) {
        self.status = status;
    }
}

impl Default for PCDMReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_creation() {
        let reader = PCDMReader::new();
        assert_eq!(reader.get_status(), ReaderStatus::Ok);
    }

    #[test]
    fn test_set_status() {
        let mut reader = PCDMReader::new();
        reader.set_status(ReaderStatus::NoDriver);
        assert_eq!(reader.get_status(), ReaderStatus::NoDriver);
    }
}

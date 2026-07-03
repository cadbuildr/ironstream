// FILE: pcdm.rs
// occt: PCDM_Document, PCDM_RetrievalDriver, PCDM_StorageDriver,
//       PCDM_ReadWriter, PCDM_DriverError

/// Storage/retrieval driver error codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcdmError {
    Ok,
    OpenError,
    ReadError,
    WriteError,
    ExtensionFailure,
    AlreadyRetrieved,
    AlreadyRetrievedAndModified,
    UnknownFileDriver,
    FormatNotFound,
    VersionNotFound,
    NoModel,
    NoDriver,
    DriverFailure,
    UnknownError,
}

impl Default for PcdmError {
    fn default() -> Self { Self::Ok }
}

impl PcdmError {
    pub fn is_ok(&self) -> bool { *self == PcdmError::Ok }
    pub fn message(&self) -> &'static str {
        match self {
            Self::Ok                         => "OK",
            Self::OpenError                  => "Failed to open file",
            Self::ReadError                  => "Failed to read file",
            Self::WriteError                 => "Failed to write file",
            Self::ExtensionFailure           => "Extension not supported",
            Self::AlreadyRetrieved          => "Document already retrieved",
            Self::AlreadyRetrievedAndModified => "Document already retrieved and modified",
            Self::UnknownFileDriver         => "No file driver for format",
            Self::FormatNotFound            => "Document format not found",
            Self::VersionNotFound           => "Document version not found",
            Self::NoModel                   => "No model in document",
            Self::NoDriver                  => "No driver found",
            Self::DriverFailure             => "Driver operation failed",
            Self::UnknownError              => "Unknown error",
        }
    }
}

// occt: PCDM_Document — base class for all persistent (stored) documents
#[derive(Clone, Debug)]
pub struct PcdmDocument {
    pub name: String,
    pub format: String,
    pub version: u32,
    pub is_modified: bool,
    pub is_stored: bool,
    pub path: Option<String>,
    pub attributes: std::collections::HashMap<String, String>,
}

impl PcdmDocument {
    pub fn new(name: &str, format: &str) -> Self {
        Self {
            name: name.to_owned(),
            format: format.to_owned(),
            version: 1,
            is_modified: false,
            is_stored: false,
            path: None,
            attributes: std::collections::HashMap::new(),
        }
    }

    pub fn set_modified(&mut self) { self.is_modified = true; }
    pub fn set_stored(&mut self, path: &str) {
        self.path = Some(path.to_owned());
        self.is_stored = true;
        self.is_modified = false;
        self.version += 1;
    }

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_owned(), value.to_owned());
        self.is_modified = true;
    }

    pub fn get_attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(String::as_str)
    }

    pub fn has_reference(&self, _doc_name: &str) -> bool { false }
}

impl Default for PcdmDocument {
    fn default() -> Self { Self::new("Untitled", "XmlOcaf") }
}

// occt: PCDM_RetrievalDriver — reads document from file
#[derive(Clone, Debug, Default)]
pub struct RetrievalDriver {
    pub supported_formats: Vec<String>,
    pub nb_documents_read: usize,
}

impl RetrievalDriver {
    pub fn new(formats: &[&str]) -> Self {
        Self {
            supported_formats: formats.iter().map(|s| s.to_string()).collect(),
            nb_documents_read: 0,
        }
    }

    pub fn xml() -> Self { Self::new(&["XmlOcaf", "xml"]) }
    pub fn binary() -> Self { Self::new(&["BinOcaf", "cbf"]) }

    pub fn can_read(&self, format: &str) -> bool {
        self.supported_formats.iter().any(|f| f.eq_ignore_ascii_case(format))
    }

    pub fn read(&mut self, path: &str, format: &str) -> Result<PcdmDocument, PcdmError> {
        if !self.can_read(format) {
            return Err(PcdmError::UnknownFileDriver);
        }
        if !path.ends_with(".xml") && !path.ends_with(".cbf") && !path.ends_with(".xbf") {
            return Err(PcdmError::FormatNotFound);
        }
        self.nb_documents_read += 1;
        Ok(PcdmDocument::new(
            path.trim_end_matches(|c: char| c != '/').trim_end_matches('/'),
            format,
        ))
    }

    pub fn schema_name(&self) -> &str { "StandardSchema" }
}

// occt: PCDM_StorageDriver — writes document to file
#[derive(Clone, Debug, Default)]
pub struct StorageDriver {
    pub supported_formats: Vec<String>,
    pub nb_documents_written: usize,
    pub last_error: PcdmError,
}

impl StorageDriver {
    pub fn new(formats: &[&str]) -> Self {
        Self {
            supported_formats: formats.iter().map(|s| s.to_string()).collect(),
            nb_documents_written: 0,
            last_error: PcdmError::Ok,
        }
    }

    pub fn xml() -> Self { Self::new(&["XmlOcaf", "xml"]) }
    pub fn binary() -> Self { Self::new(&["BinOcaf", "cbf"]) }

    pub fn can_write(&self, format: &str) -> bool {
        self.supported_formats.iter().any(|f| f.eq_ignore_ascii_case(format))
    }

    pub fn write(&mut self, doc: &mut PcdmDocument, path: &str) -> PcdmError {
        if !self.can_write(&doc.format) {
            self.last_error = PcdmError::UnknownFileDriver;
            return self.last_error;
        }
        doc.set_stored(path);
        self.nb_documents_written += 1;
        self.last_error = PcdmError::Ok;
        PcdmError::Ok
    }

    pub fn schema_name(&self) -> &str { "StandardSchema" }
}

// occt: PCDM_ReadWriter — utility that delegates to reader/writer pair
#[derive(Clone, Debug, Default)]
pub struct ReadWriter {
    pub reader: RetrievalDriver,
    pub writer: StorageDriver,
}

impl ReadWriter {
    pub fn xml() -> Self {
        Self { reader: RetrievalDriver::xml(), writer: StorageDriver::xml() }
    }

    pub fn binary() -> Self {
        Self { reader: RetrievalDriver::binary(), writer: StorageDriver::binary() }
    }

    pub fn read(&mut self, path: &str, format: &str) -> Result<PcdmDocument, PcdmError> {
        self.reader.read(path, format)
    }

    pub fn write(&mut self, doc: &mut PcdmDocument, path: &str) -> PcdmError {
        self.writer.write(doc, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pcdm_error() {
        assert!(PcdmError::Ok.is_ok());
        assert!(!PcdmError::ReadError.is_ok());
        assert!(!PcdmError::WriteError.message().is_empty());
    }

    #[test]
    fn pcdm_document() {
        let mut doc = PcdmDocument::new("TestDoc", "XmlOcaf");
        doc.set_attribute("Author", "Test");
        assert_eq!(doc.get_attribute("Author"), Some("Test"));
        assert!(doc.is_modified);
        doc.set_stored("/tmp/test.xml");
        assert!(!doc.is_modified);
        assert_eq!(doc.version, 2);
    }

    #[test]
    fn retrieval_driver() {
        let mut driver = RetrievalDriver::xml();
        assert!(driver.can_read("XmlOcaf"));
        assert!(!driver.can_read("BinOcaf"));
        let result = driver.read("model.xml", "XmlOcaf");
        assert!(result.is_ok());
    }

    #[test]
    fn storage_driver() {
        let mut driver = StorageDriver::xml();
        let mut doc = PcdmDocument::new("D", "XmlOcaf");
        let err = driver.write(&mut doc, "/tmp/out.xml");
        assert!(err.is_ok());
        assert_eq!(driver.nb_documents_written, 1);
    }

    #[test]
    fn read_writer_round_trip() {
        let mut rw = ReadWriter::xml();
        let mut doc = PcdmDocument::new("X", "XmlOcaf");
        let err = rw.write(&mut doc, "/tmp/x.xml");
        assert!(err.is_ok());
    }
}

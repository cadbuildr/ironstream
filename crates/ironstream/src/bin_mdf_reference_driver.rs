// FILE: bin_mdf_reference_driver.rs
// occt: BinMDF_ReferenceDriver

/// Binary serialization driver for TDF reference attributes.
/// Handles persistence of label references within the document structure.
pub struct BinMDFReferenceDriver {
    message_driver: Option<String>,
}

impl BinMDFReferenceDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDFReferenceDriver { message_driver }
    }

    pub fn message_driver(&self) -> &Option<String> {
        &self.message_driver
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_driver_creation() {
        let driver = BinMDFReferenceDriver::new(None);
        assert_eq!(driver.message_driver(), &None);
    }

    #[test]
    fn test_reference_driver_with_message() {
        let driver = BinMDFReferenceDriver::new(Some("TestMsg".to_string()));
        assert_eq!(driver.message_driver(), &Some("TestMsg".to_string()));
    }
}

// FILE: bin_mdf_tag_source_driver.rs
// occt: BinMDF_TagSourceDriver

/// Binary serialization driver for TDF tag source attributes.
/// Manages persistent storage of label tag source information.
pub struct BinMDFTagSourceDriver {
    message_driver: Option<String>,
}

impl BinMDFTagSourceDriver {
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDFTagSourceDriver { message_driver }
    }

    pub fn message_driver(&self) -> &Option<String> {
        &self.message_driver
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_source_driver_creation() {
        let driver = BinMDFTagSourceDriver::new(None);
        assert_eq!(driver.message_driver(), &None);
    }

    #[test]
    fn test_tag_source_driver_with_message() {
        let driver = BinMDFTagSourceDriver::new(Some("TestMsg".to_string()));
        assert_eq!(driver.message_driver(), &Some("TestMsg".to_string()));
    }
}

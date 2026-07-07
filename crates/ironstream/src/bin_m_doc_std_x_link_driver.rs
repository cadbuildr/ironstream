// FILE: bin_m_doc_std_x_link_driver.rs
// occt: BinMDocStd_XLinkDriver

/// Binary serialization driver for external link attributes.
/// Handles persistent <-> transient conversion for XLink attributes.
pub struct BinMDocStdXLinkDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMDocStdXLinkDriver {
    /// Creates a new XLinkDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDocStdXLinkDriver {
            message_driver,
            type_name: "TDocStd_XLink".to_string(),
        }
    }

    /// Returns the type name of the attribute object.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Returns the current message driver of this driver.
    pub fn message_driver(&self) -> Option<&str> {
        self.message_driver.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xlink_driver_creation() {
        let driver = BinMDocStdXLinkDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TDocStd_XLink");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_xlink_driver_no_messenger() {
        let driver = BinMDocStdXLinkDriver::new(None);
        assert_eq!(driver.type_name(), "TDocStd_XLink");
        assert_eq!(driver.message_driver(), None);
    }
}

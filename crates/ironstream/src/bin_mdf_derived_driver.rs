// FILE: bin_mdf_derived_driver.rs
// occt: BinMDF_DerivedDriver

/// Universal driver for attributes that inherit from another attribute with existing persistence.
/// Reuses the base driver's persistence mechanism for derived attributes.
pub struct BinMDFDerivedDriver {
    base_driver_name: String,
    derivative_name: String,
    message_driver: Option<String>,
}

impl BinMDFDerivedDriver {
    pub fn new(derivative_name: &str, base_driver_name: &str, message_driver: Option<String>) -> Self {
        BinMDFDerivedDriver {
            base_driver_name: base_driver_name.to_string(),
            derivative_name: derivative_name.to_string(),
            message_driver,
        }
    }

    pub fn derivative_name(&self) -> &str {
        &self.derivative_name
    }

    pub fn base_driver_name(&self) -> &str {
        &self.base_driver_name
    }

    pub fn message_driver(&self) -> &Option<String> {
        &self.message_driver
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_driver_creation() {
        let driver = BinMDFDerivedDriver::new("DerivedAttr", "BaseAttr", None);
        assert_eq!(driver.derivative_name(), "DerivedAttr");
        assert_eq!(driver.base_driver_name(), "BaseAttr");
    }

    #[test]
    fn test_derived_driver_with_message() {
        let driver = BinMDFDerivedDriver::new("Derived", "Base", Some("Msg".to_string()));
        assert_eq!(driver.message_driver(), &Some("Msg".to_string()));
    }
}

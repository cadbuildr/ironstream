// FILE: bin_mxcaf_doc_dim_tol_driver.rs
// occt: BinMXCAFDoc_DimTolDriver

/// Driver for dimension and tolerance attributes in XCAF documents.
pub struct BinmxcafdocDimTolDriver {
    driver_id: usize,
}

impl BinmxcafdocDimTolDriver {
    /// Creates a new DimTol driver.
    pub fn new(driver_id: usize) -> Self {
        BinmxcafdocDimTolDriver { driver_id }
    }

    /// Returns the driver ID.
    pub fn driver_id(&self) -> usize {
        self.driver_id
    }

    /// Creates a new empty attribute.
    pub fn new_empty(&self) -> usize {
        0
    }

    /// Pastes attribute data (read operation).
    pub fn paste_read(&self, source_id: usize) -> usize {
        source_id
    }

    /// Pastes attribute data (write operation).
    pub fn paste_write(&self, attribute_id: usize) -> usize {
        attribute_id
    }

    /// Returns the driver type.
    pub fn driver_type(&self) -> &'static str {
        "DimTolDriver"
    }
}

impl Default for BinmxcafdocDimTolDriver {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = BinmxcafdocDimTolDriver::new(42);
        assert_eq!(driver.driver_id(), 42);
    }

    #[test]
    fn test_driver_type() {
        let driver = BinmxcafdocDimTolDriver::new(1);
        assert_eq!(driver.driver_type(), "DimTolDriver");
    }

    #[test]
    fn test_new_empty() {
        let driver = BinmxcafdocDimTolDriver::new(1);
        assert_eq!(driver.new_empty(), 0);
    }

    #[test]
    fn test_paste_operations() {
        let driver = BinmxcafdocDimTolDriver::new(1);
        assert_eq!(driver.paste_read(5), 5);
        assert_eq!(driver.paste_write(10), 10);
    }

    #[test]
    fn test_default() {
        let driver = BinmxcafdocDimTolDriver::default();
        assert_eq!(driver.driver_id(), 1);
    }
}

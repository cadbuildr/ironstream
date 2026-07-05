// FILE: bin_drivers_document_storage_driver.rs
// occt: BinDrivers_DocumentStorageDriver

#[derive(Default, Clone, Debug)]
pub struct BinDriversDocumentStorageDriver;

impl BinDriversDocumentStorageDriver {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _driver = BinDriversDocumentStorageDriver::new();
    }
}

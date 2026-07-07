// FILE: bin_drivers_document_retrieval_driver.rs
// occt: BinDrivers_DocumentRetrievalDriver

#[derive(Default, Clone, Debug)]
pub struct BinDriversDocumentRetrievalDriver;

impl BinDriversDocumentRetrievalDriver {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _driver = BinDriversDocumentRetrievalDriver::new();
    }
}

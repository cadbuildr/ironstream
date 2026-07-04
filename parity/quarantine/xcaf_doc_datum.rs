// FILE: xcaf_doc_datum.rs
// occt: XCAFDoc_Datum

/// attribute to store datum
#[derive(Debug, Clone)]
pub struct XCAFDoc_Datum {
    // TODO: Port fields from OCCT
}

impl XCAFDoc_Datum {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFDoc_Datum {
        }
    }
}

impl Default for XCAFDoc_Datum {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_doc_datum_creation() {
        let obj = XCAFDoc_Datum::new();
        let _default = XCAFDoc_Datum::default();
        // TODO: Add more tests from OCCT gtest
    }
}

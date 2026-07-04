// FILE: xcaf_doc_length_unit.rs
// occt: XCAFDoc_LengthUnit

/// Used to define a Length Unit attribute containing a length unit info
#[derive(Debug, Clone)]
pub struct XCAFDoc_LengthUnit {
    // TODO: Port fields from OCCT
}

impl XCAFDoc_LengthUnit {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFDoc_LengthUnit {
        }
    }
}

impl Default for XCAFDoc_LengthUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_doc_length_unit_creation() {
        let obj = XCAFDoc_LengthUnit::new();
        let _default = XCAFDoc_LengthUnit::default();
        // TODO: Add more tests from OCCT gtest
    }
}

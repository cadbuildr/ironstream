// FILE: xcaf_doc_dim_tol.rs
// occt: XCAFDoc_DimTol

/// attribute to store dimension and tolerance
#[derive(Debug, Clone)]
pub struct XCAFDoc_DimTol {
    // TODO: Port fields from OCCT
}

impl XCAFDoc_DimTol {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFDoc_DimTol {
        }
    }
}

impl Default for XCAFDoc_DimTol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_doc_dim_tol_creation() {
        let obj = XCAFDoc_DimTol::new();
        let _default = XCAFDoc_DimTol::default();
        // TODO: Add more tests from OCCT gtest
    }
}

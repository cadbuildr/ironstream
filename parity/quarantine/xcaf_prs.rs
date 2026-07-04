// FILE: xcaf_prs.rs
// occt: XCAFPrs

/// Presentation (visualiation, selection etc.) tools for
//! DECAF documents
#[derive(Debug, Clone)]
pub struct XCAFPrs {
    // TODO: Port fields from OCCT
}

impl XCAFPrs {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFPrs {
        }
    }
}

impl Default for XCAFPrs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_prs_creation() {
        let obj = XCAFPrs::new();
        let _default = XCAFPrs::default();
        // TODO: Add more tests from OCCT gtest
    }
}

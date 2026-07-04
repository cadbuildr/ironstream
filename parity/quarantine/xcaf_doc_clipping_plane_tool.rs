// FILE: xcaf_doc_clipping_plane_tool.rs
// occt: XCAFDoc_ClippingPlaneTool

/// Provide tool for management of ClippingPlane section of document.
//! Provide tool to store, retriev
#[derive(Debug, Clone)]
pub struct XCAFDoc_ClippingPlaneTool {
    // TODO: Port fields from OCCT
}

impl XCAFDoc_ClippingPlaneTool {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFDoc_ClippingPlaneTool {
        }
    }
}

impl Default for XCAFDoc_ClippingPlaneTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_doc_clipping_plane_tool_creation() {
        let obj = XCAFDoc_ClippingPlaneTool::new();
        let _default = XCAFDoc_ClippingPlaneTool::default();
        // TODO: Add more tests from OCCT gtest
    }
}

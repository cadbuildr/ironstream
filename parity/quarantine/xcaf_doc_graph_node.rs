// FILE: xcaf_doc_graph_node.rs
// occt: XCAFDoc_GraphNode

/// This attribute allow user multirelation tree of labels.
//! This GraphNode is experimental Graph tha
#[derive(Debug, Clone)]
pub struct XCAFDoc_GraphNode {
    // TODO: Port fields from OCCT
}

impl XCAFDoc_GraphNode {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFDoc_GraphNode {
        }
    }
}

impl Default for XCAFDoc_GraphNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_doc_graph_node_creation() {
        let obj = XCAFDoc_GraphNode::new();
        let _default = XCAFDoc_GraphNode::default();
        // TODO: Add more tests from OCCT gtest
    }
}

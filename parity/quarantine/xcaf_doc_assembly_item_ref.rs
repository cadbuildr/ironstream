// FILE: xcaf_doc_assembly_item_ref.rs
// occt: XCAFDoc_AssemblyItemRef

/// An attribute that describes a weak reference to an assembly item
//! or to a subshape or to an assem
#[derive(Debug, Clone)]
pub struct XCAFDoc_AssemblyItemRef {
    // TODO: Port fields from OCCT
}

impl XCAFDoc_AssemblyItemRef {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFDoc_AssemblyItemRef {
        }
    }
}

impl Default for XCAFDoc_AssemblyItemRef {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_doc_assembly_item_ref_creation() {
        let obj = XCAFDoc_AssemblyItemRef::new();
        let _default = XCAFDoc_AssemblyItemRef::default();
        // TODO: Add more tests from OCCT gtest
    }
}

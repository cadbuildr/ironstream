// FILE: xcaf_doc_editor.rs
// occt: XCAFDoc_Editor

/// Tool for edit structure of document.
#[derive(Debug, Clone)]
pub struct XCAFDoc_Editor {
    // TODO: Port fields from OCCT
}

impl XCAFDoc_Editor {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFDoc_Editor {
        }
    }
}

impl Default for XCAFDoc_Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_doc_editor_creation() {
        let obj = XCAFDoc_Editor::new();
        let _default = XCAFDoc_Editor::default();
        // TODO: Add more tests from OCCT gtest
    }
}

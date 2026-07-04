// FILE: xcaf_doc_note_balloon.rs
// occt: XCAFDoc_NoteBalloon

/// A comment note attribute.
//! Contains a textual comment.
#[derive(Debug, Clone)]
pub struct XCAFDoc_NoteBalloon {
    // TODO: Port fields from OCCT
}

impl XCAFDoc_NoteBalloon {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFDoc_NoteBalloon {
        }
    }
}

impl Default for XCAFDoc_NoteBalloon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_doc_note_balloon_creation() {
        let obj = XCAFDoc_NoteBalloon::new();
        let _default = XCAFDoc_NoteBalloon::default();
        // TODO: Add more tests from OCCT gtest
    }
}

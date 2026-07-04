// FILE: xcaf_doc_note_comment.rs
// occt: XCAFDoc_NoteComment

/// A comment note attribute.
//! Contains a textual comment.
#[derive(Debug, Clone)]
pub struct XCAFDoc_NoteComment {
    // TODO: Port fields from OCCT
}

impl XCAFDoc_NoteComment {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFDoc_NoteComment {
        }
    }
}

impl Default for XCAFDoc_NoteComment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_doc_note_comment_creation() {
        let obj = XCAFDoc_NoteComment::new();
        let _default = XCAFDoc_NoteComment::default();
        // TODO: Add more tests from OCCT gtest
    }
}

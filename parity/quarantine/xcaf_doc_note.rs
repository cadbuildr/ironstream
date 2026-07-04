// FILE: xcaf_doc_note.rs
// occt: XCAFDoc_Note

/// A base note attribute.
//! Any note contains the name of the user created the note
//! and the creat
#[derive(Debug, Clone)]
pub struct XCAFDoc_Note {
    // TODO: Port fields from OCCT
}

impl XCAFDoc_Note {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFDoc_Note {
        }
    }
}

impl Default for XCAFDoc_Note {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_doc_note_creation() {
        let obj = XCAFDoc_Note::new();
        let _default = XCAFDoc_Note::default();
        // TODO: Add more tests from OCCT gtest
    }
}

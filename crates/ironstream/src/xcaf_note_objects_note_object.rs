// FILE: xcaf_note_objects_note_object.rs
// occt: XCAFNoteObjects_NoteObject

/// object to store note auxiliary data
#[derive(Debug, Clone)]
pub struct XCAFNoteObjects_NoteObject {
    // TODO: Port fields from OCCT
}

impl XCAFNoteObjects_NoteObject {
    /// Creates a new instance
    pub fn new() -> Self {
        XCAFNoteObjects_NoteObject {
        }
    }
}

impl Default for XCAFNoteObjects_NoteObject {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_note_objects_note_object_creation() {
        let obj = XCAFNoteObjects_NoteObject::new();
        let _default = XCAFNoteObjects_NoteObject::default();
        // TODO: Add more tests from OCCT gtest
    }
}

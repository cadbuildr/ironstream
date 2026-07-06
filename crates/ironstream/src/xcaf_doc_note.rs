// FILE: xcaf_doc_note.rs
// occt: XCAFDoc_Note

// Base note attribute, mirroring OCCT's XCAFDoc_Note (TDF_Attribute).
// A note stores the name of the user who created it and the creation
// timestamp. A note is orphan when no annotated items reference it via
// the note-reference graph node.

/// Base note attribute.
/// Mirrors OCCT XCAFDoc_Note: stores user name and timestamp.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct XcafDocNote {
    user_name: String,
    time_stamp: String,
    /// Local model of the note-reference graph node children count:
    /// the number of annotated items referencing this note
    /// (OCCT: XCAFDoc_GraphNode found by XCAFDoc::NoteRefGUID()).
    ref_children: usize,
}

impl XcafDocNote {
    /// Creates an empty note (protected default constructor in OCCT).
    pub fn new() -> Self {
        XcafDocNote {
            user_name: String::new(),
            time_stamp: String::new(),
            ref_children: 0,
        }
    }

    /// Sets the user name and the timestamp of the note (XCAFDoc_Note::Set).
    pub fn set(&mut self, user_name: &str, time_stamp: &str) {
        // OCCT calls Backup() before modification; transaction plumbing is
        // out of scope for this self-contained port.
        self.user_name = user_name.to_string();
        self.time_stamp = time_stamp.to_string();
    }

    /// Returns the user name who created the note.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Returns the timestamp when the note was created.
    pub fn time_stamp(&self) -> &str {
        &self.time_stamp
    }

    /// Checks if the note isn't linked to any annotated items.
    /// OCCT: no father graph node attribute found, or it has zero children.
    pub fn is_orphan(&self) -> bool {
        self.ref_children == 0
    }

    /// Local helper mirroring the effect of XCAFDoc_NoteItemTool linking an
    /// annotated item to this note through the reference graph node.
    pub fn add_ref_child(&mut self) {
        self.ref_children += 1;
    }

    /// Local helper mirroring the removal of an annotated-item link.
    pub fn remove_ref_child(&mut self) {
        if self.ref_children > 0 {
            self.ref_children -= 1;
        }
    }

    /// Restores the note contents from another note (TDF_Attribute::Restore).
    pub fn restore(&mut self, from: &XcafDocNote) {
        self.user_name = from.user_name.clone();
        self.time_stamp = from.time_stamp.clone();
    }

    /// Pastes the note contents into another note (TDF_Attribute::Paste).
    pub fn paste(&self, into: &mut XcafDocNote) {
        into.user_name = self.user_name.clone();
        into.time_stamp = self.time_stamp.clone();
    }

    /// Dumps the note to a string (TDF_Attribute::Dump).
    pub fn dump(&self) -> String {
        format!("Note: {} on {}", self.user_name, self.time_stamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_note_is_empty_and_orphan() {
        let note = XcafDocNote::new();
        assert_eq!(note.user_name(), "");
        assert_eq!(note.time_stamp(), "");
        assert!(note.is_orphan());
    }

    #[test]
    fn test_set_user_name_and_timestamp() {
        let mut note = XcafDocNote::new();
        note.set("clement", "2026-07-05T12:00:00");
        assert_eq!(note.user_name(), "clement");
        assert_eq!(note.time_stamp(), "2026-07-05T12:00:00");
    }

    #[test]
    fn test_orphan_tracking() {
        let mut note = XcafDocNote::new();
        assert!(note.is_orphan());
        note.add_ref_child();
        assert!(!note.is_orphan());
        note.remove_ref_child();
        assert!(note.is_orphan());
        // Removing when already zero stays orphan and does not underflow.
        note.remove_ref_child();
        assert!(note.is_orphan());
    }

    #[test]
    fn test_restore() {
        let mut src = XcafDocNote::new();
        src.set("alice", "ts1");
        let mut dst = XcafDocNote::new();
        dst.restore(&src);
        assert_eq!(dst.user_name(), "alice");
        assert_eq!(dst.time_stamp(), "ts1");
    }

    #[test]
    fn test_paste() {
        let mut src = XcafDocNote::new();
        src.set("bob", "ts2");
        let mut dst = XcafDocNote::new();
        src.paste(&mut dst);
        assert_eq!(dst.user_name(), "bob");
        assert_eq!(dst.time_stamp(), "ts2");
    }

    #[test]
    fn test_dump() {
        let mut note = XcafDocNote::new();
        note.set("carol", "ts3");
        assert_eq!(note.dump(), "Note: carol on ts3");
    }
}

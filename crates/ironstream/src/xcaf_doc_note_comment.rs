// FILE: xcaf_doc_note_comment.rs
// occt: XCAFDoc_NoteComment
//
// A note attribute containing a textual comment.
// Inherits XCAFDoc_Note (user name + timestamp); adds a comment string.
// TDF plumbing (label / attribute storage) is modeled locally.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// GUID of the XCAFDoc_NoteComment attribute (from OCCT).
pub const NOTE_COMMENT_GUID: &str = "FDEA4C52-0F54-484c-B590-579E18F7B5D4";

/// Local model of a TDF_Label: a container of attributes keyed by GUID.
#[derive(Default, Clone)]
pub struct TdfLabel {
    attrs: Rc<RefCell<HashMap<String, Rc<RefCell<XCAFDocNoteComment>>>>>,
}

impl TdfLabel {
    pub fn new() -> Self {
        Self::default()
    }

    /// TDF_Label::FindAttribute analogue.
    pub fn find_attribute(&self, guid: &str) -> Option<Rc<RefCell<XCAFDocNoteComment>>> {
        self.attrs.borrow().get(guid).cloned()
    }

    /// TDF_Label::AddAttribute analogue.
    pub fn add_attribute(&self, attr: Rc<RefCell<XCAFDocNoteComment>>) {
        let guid = attr.borrow().id().to_string();
        self.attrs.borrow_mut().insert(guid, attr);
    }
}

/// Base part: XCAFDoc_Note (user name and timestamp).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XCAFDocNote {
    user_name: String,
    time_stamp: String,
}

impl XCAFDocNote {
    /// XCAFDoc_Note::Set.
    pub fn set(&mut self, user_name: &str, time_stamp: &str) {
        self.user_name = user_name.to_string();
        self.time_stamp = time_stamp.to_string();
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn time_stamp(&self) -> &str {
        &self.time_stamp
    }
}

/// XCAFDoc_NoteComment: a comment note attribute.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XCAFDocNoteComment {
    note: XCAFDocNote,
    comment: String,
}

impl XCAFDocNoteComment {
    /// OCCT GetID.
    pub fn get_id() -> &'static str {
        NOTE_COMMENT_GUID
    }

    /// OCCT ID (dynamic).
    pub fn id(&self) -> &'static str {
        Self::get_id()
    }

    /// OCCT default ctor.
    pub fn new() -> Self {
        Self::default()
    }

    /// OCCT static Get: finds the attribute on a label.
    pub fn get(label: &TdfLabel) -> Option<Rc<RefCell<XCAFDocNoteComment>>> {
        label.find_attribute(Self::get_id())
    }

    /// OCCT static Set: creates and attaches the attribute if the label
    /// does not have one yet; returns the existing one otherwise.
    pub fn set(
        label: &TdfLabel,
        user_name: &str,
        time_stamp: &str,
        comment: &str,
    ) -> Rc<RefCell<XCAFDocNoteComment>> {
        if let Some(existing) = label.find_attribute(Self::get_id()) {
            return existing;
        }
        let mut attr = XCAFDocNoteComment::new();
        attr.note.set(user_name, time_stamp);
        attr.set_comment(comment);
        let attr = Rc::new(RefCell::new(attr));
        label.add_attribute(attr.clone());
        attr
    }

    /// OCCT instance Set(comment).
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string();
    }

    /// OCCT Comment.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Access to the inherited note part.
    pub fn note(&self) -> &XCAFDocNote {
        &self.note
    }

    /// Mutable access to the inherited note part.
    pub fn note_mut(&mut self) -> &mut XCAFDocNote {
        &mut self.note
    }

    /// OCCT Restore: copy contents from another attribute.
    pub fn restore(&mut self, other: &XCAFDocNoteComment) {
        self.note = other.note.clone();
        self.comment = other.comment.clone();
    }

    /// OCCT Paste: copy contents into another attribute.
    pub fn paste(&self, into: &mut XCAFDocNoteComment) {
        into.note.set(self.note.user_name(), self.note.time_stamp());
        into.set_comment(&self.comment);
    }

    /// OCCT Dump.
    pub fn dump(&self) -> String {
        format!(
            "Note : {} on {}\nComment : {}",
            if self.note.user_name.is_empty() {
                "<anonymous>"
            } else {
                &self.note.user_name
            },
            if self.note.time_stamp.is_empty() {
                "<unknown>"
            } else {
                &self.note.time_stamp
            },
            if self.comment.is_empty() {
                "<empty>"
            } else {
                &self.comment
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid() {
        assert_eq!(
            XCAFDocNoteComment::get_id(),
            "FDEA4C52-0F54-484c-B590-579E18F7B5D4"
        );
        let c = XCAFDocNoteComment::new();
        assert_eq!(c.id(), XCAFDocNoteComment::get_id());
    }

    #[test]
    fn test_set_creates_attribute_on_label() {
        let label = TdfLabel::new();
        assert!(XCAFDocNoteComment::get(&label).is_none());

        let attr = XCAFDocNoteComment::set(&label, "user", "2026-07-02", "hello");
        assert_eq!(attr.borrow().comment(), "hello");
        assert_eq!(attr.borrow().note().user_name(), "user");
        assert_eq!(attr.borrow().note().time_stamp(), "2026-07-02");

        let found = XCAFDocNoteComment::get(&label).expect("attribute must be found");
        assert!(Rc::ptr_eq(&found, &attr));
    }

    #[test]
    fn test_set_twice_returns_existing() {
        let label = TdfLabel::new();
        let first = XCAFDocNoteComment::set(&label, "u1", "t1", "c1");
        let second = XCAFDocNoteComment::set(&label, "u2", "t2", "c2");
        // OCCT Set does not overwrite an existing attribute.
        assert!(Rc::ptr_eq(&first, &second));
        assert_eq!(second.borrow().comment(), "c1");
        assert_eq!(second.borrow().note().user_name(), "u1");
    }

    #[test]
    fn test_set_comment() {
        let mut c = XCAFDocNoteComment::new();
        assert_eq!(c.comment(), "");
        c.set_comment("a comment");
        assert_eq!(c.comment(), "a comment");
    }

    #[test]
    fn test_restore_and_paste() {
        let mut src = XCAFDocNoteComment::new();
        src.note_mut().set("alice", "2020-01-01");
        src.set_comment("original");

        let mut restored = XCAFDocNoteComment::new();
        restored.restore(&src);
        assert_eq!(restored, src);

        let mut pasted = XCAFDocNoteComment::new();
        src.paste(&mut pasted);
        assert_eq!(pasted.comment(), "original");
        assert_eq!(pasted.note().user_name(), "alice");
    }

    #[test]
    fn test_dump() {
        let mut c = XCAFDocNoteComment::new();
        assert_eq!(c.dump(), "Note : <anonymous> on <unknown>\nComment : <empty>");
        c.note_mut().set("bob", "ts");
        c.set_comment("hi");
        assert_eq!(c.dump(), "Note : bob on ts\nComment : hi");
    }
}

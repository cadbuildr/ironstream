// FILE: xcaf_doc_note_balloon.rs
// occt: XCAFDoc_NoteBalloon
//
// A "balloon" note attribute: derives from XCAFDoc_NoteComment
// (which itself derives from XCAFDoc_Note) and only differs by GUID.
// TDF plumbing (label / attribute storage) is modeled locally.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// GUID of the XCAFDoc_NoteBalloon attribute (from OCCT).
pub const NOTE_BALLOON_GUID: &str = "1127951D-87D5-4ecc-89D5-D1406576C43F";

/// Local model of a TDF_Label: a container of attributes keyed by GUID.
#[derive(Default, Clone)]
pub struct TdfLabel {
    attrs: Rc<RefCell<HashMap<String, Rc<RefCell<XCAFDocNoteBalloon>>>>>,
}

impl TdfLabel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find_attribute(&self, guid: &str) -> Option<Rc<RefCell<XCAFDocNoteBalloon>>> {
        self.attrs.borrow().get(guid).cloned()
    }

    pub fn add_attribute(&self, attr: Rc<RefCell<XCAFDocNoteBalloon>>) {
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

/// XCAFDoc_NoteBalloon: comment note shown as a balloon.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XCAFDocNoteBalloon {
    note: XCAFDocNote,
    comment: String,
}

impl XCAFDocNoteBalloon {
    /// OCCT GetID: balloon-specific GUID, distinct from NoteComment's.
    pub fn get_id() -> &'static str {
        NOTE_BALLOON_GUID
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
    pub fn get(label: &TdfLabel) -> Option<Rc<RefCell<XCAFDocNoteBalloon>>> {
        label.find_attribute(Self::get_id())
    }

    /// OCCT static Set: creates and attaches the attribute if the label
    /// does not carry one yet; returns the existing one otherwise.
    pub fn set(
        label: &TdfLabel,
        user_name: &str,
        time_stamp: &str,
        comment: &str,
    ) -> Rc<RefCell<XCAFDocNoteBalloon>> {
        if let Some(existing) = label.find_attribute(Self::get_id()) {
            return existing;
        }
        let mut attr = XCAFDocNoteBalloon::new();
        attr.note.set(user_name, time_stamp);
        attr.set_comment(comment);
        let attr = Rc::new(RefCell::new(attr));
        label.add_attribute(attr.clone());
        attr
    }

    /// Inherited XCAFDoc_NoteComment::Set(comment).
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string();
    }

    /// Inherited XCAFDoc_NoteComment::Comment.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid_differs_from_note_comment() {
        assert_eq!(
            XCAFDocNoteBalloon::get_id(),
            "1127951D-87D5-4ecc-89D5-D1406576C43F"
        );
        // XCAFDoc_NoteComment GUID from OCCT — must be different.
        assert_ne!(
            XCAFDocNoteBalloon::get_id(),
            "FDEA4C52-0F54-484c-B590-579E18F7B5D4"
        );
    }

    #[test]
    fn test_set_creates_attribute_on_label() {
        let label = TdfLabel::new();
        assert!(XCAFDocNoteBalloon::get(&label).is_none());

        let attr = XCAFDocNoteBalloon::set(&label, "user", "2026-07-02", "balloon text");
        assert_eq!(attr.borrow().comment(), "balloon text");
        assert_eq!(attr.borrow().note().user_name(), "user");
        assert_eq!(attr.borrow().note().time_stamp(), "2026-07-02");

        let found = XCAFDocNoteBalloon::get(&label).expect("attribute must be found");
        assert!(Rc::ptr_eq(&found, &attr));
    }

    #[test]
    fn test_set_twice_returns_existing() {
        let label = TdfLabel::new();
        let first = XCAFDocNoteBalloon::set(&label, "u1", "t1", "c1");
        let second = XCAFDocNoteBalloon::set(&label, "u2", "t2", "c2");
        assert!(Rc::ptr_eq(&first, &second));
        assert_eq!(second.borrow().comment(), "c1");
    }

    #[test]
    fn test_comment_accessors() {
        let mut b = XCAFDocNoteBalloon::new();
        assert_eq!(b.comment(), "");
        b.set_comment("note");
        assert_eq!(b.comment(), "note");
        b.note_mut().set("carol", "2021");
        assert_eq!(b.note().user_name(), "carol");
        assert_eq!(b.note().time_stamp(), "2021");
    }
}

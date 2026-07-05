// FILE: bin_mxcaf_doc_note_comment_driver.rs
// occt: BinMXCAFDoc_NoteCommentDriver
//
// Faithful port of OCCT BinMXCAFDoc_NoteCommentDriver
// (BinMXCAFDoc_NoteCommentDriver.cxx). The driver first delegates to
// BinMXCAFDoc_NoteDriver for the base note payload, then handles the comment:
//   <ExtendedString userName> <ExtendedString timeStamp>   (base)
//   <ExtendedString comment>                               (this driver)
//
// The module is self-contained: the base-driver payload logic is reproduced
// locally (mirroring the `BinMXCAFDoc_NoteDriver::Paste(...)` calls).

/// Local model of the XCAFDoc_NoteComment attribute (base note + comment).
#[derive(Debug, Clone, PartialEq)]
pub struct NoteCommentAttribute {
    user_name: String,
    time_stamp: String,
    comment: String,
}

impl NoteCommentAttribute {
    /// Mirrors `new XCAFDoc_NoteComment()`.
    pub fn new_empty() -> Self {
        NoteCommentAttribute {
            user_name: String::new(),
            time_stamp: String::new(),
            comment: String::new(),
        }
    }

    /// Mirrors XCAFDoc_Note::Set(userName, timeStamp).
    pub fn set_note(&mut self, user_name: &str, time_stamp: &str) {
        self.user_name = user_name.to_string();
        self.time_stamp = time_stamp.to_string();
    }

    /// Mirrors XCAFDoc_NoteComment::Set(comment).
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string();
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn time_stamp(&self) -> &str {
        &self.time_stamp
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }
}

/// Local stand-in for BinObjMgt_Persistent (ExtendedString subset).
pub struct NcPersistentStream {
    data: Vec<u8>,
    pos: usize,
    err: bool,
}

impl NcPersistentStream {
    pub fn new() -> Self {
        NcPersistentStream {
            data: Vec::new(),
            pos: 0,
            err: false,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        NcPersistentStream {
            data: bytes.to_vec(),
            pos: 0,
            err: false,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn is_error(&self) -> bool {
        self.err
    }

    fn align_put(&mut self, n: usize) {
        while self.data.len() % n != 0 {
            self.data.push(0);
        }
    }

    fn align_get(&mut self, n: usize) {
        while self.pos % n != 0 {
            self.pos += 1;
        }
    }

    /// BinObjMgt_Persistent::PutExtendedString.
    pub fn put_extended_string(&mut self, s: &str) {
        self.align_put(4);
        for u in s.encode_utf16() {
            self.data.extend_from_slice(&u.to_be_bytes());
        }
        self.data.extend_from_slice(&0u16.to_be_bytes());
    }

    /// BinObjMgt_Persistent::GetExtendedString.
    pub fn get_extended_string(&mut self) -> Option<String> {
        self.align_get(4);
        let start = self.pos;
        let mut units: Vec<u16> = Vec::new();
        loop {
            if self.pos + 2 > self.data.len() {
                self.err = true;
                self.pos = start;
                return None;
            }
            let u = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
            self.pos += 2;
            if u == 0 {
                break;
            }
            units.push(u);
        }
        Some(String::from_utf16_lossy(&units))
    }
}

impl Default for NcPersistentStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of BinMXCAFDoc_NoteCommentDriver.
pub struct BinMXCAFDocNoteCommentDriver {
    name: String,
}

impl BinMXCAFDocNoteCommentDriver {
    /// Mirrors the public constructor: named after XCAFDoc_NoteComment.
    pub fn new() -> Self {
        BinMXCAFDocNoteCommentDriver {
            name: "XCAFDoc_NoteComment".to_string(),
        }
    }

    /// Mirrors the protected constructor taking an explicit name.
    pub fn new_named(the_name: &str) -> Self {
        BinMXCAFDocNoteCommentDriver {
            name: the_name.to_string(),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.name
    }

    pub fn new_empty(&self) -> NoteCommentAttribute {
        NoteCommentAttribute::new_empty()
    }

    /// Local reproduction of BinMXCAFDoc_NoteDriver::Paste(read).
    fn base_paste_read(source: &mut NcPersistentStream, target: &mut NoteCommentAttribute) -> bool {
        let user_name = match source.get_extended_string() {
            Some(v) => v,
            None => return false,
        };
        let time_stamp = match source.get_extended_string() {
            Some(v) => v,
            None => return false,
        };
        target.set_note(&user_name, &time_stamp);
        true
    }

    /// Local reproduction of BinMXCAFDoc_NoteDriver::Paste(write).
    fn base_paste_write(source: &NoteCommentAttribute, target: &mut NcPersistentStream) {
        target.put_extended_string(source.user_name());
        target.put_extended_string(source.time_stamp());
    }

    /// Mirrors Paste(read): base note payload first, then the comment.
    pub fn paste_read(
        &self,
        source: &mut NcPersistentStream,
        target: &mut NoteCommentAttribute,
    ) -> bool {
        if !Self::base_paste_read(source, target) {
            return false;
        }
        let comment = match source.get_extended_string() {
            Some(v) => v,
            None => return false,
        };
        target.set_comment(&comment);
        true
    }

    /// Mirrors Paste(write): base note payload, then `theTarget << aNote->Comment()`.
    pub fn paste_write(&self, source: &NoteCommentAttribute, target: &mut NcPersistentStream) {
        Self::base_paste_write(source, target);
        target.put_extended_string(source.comment());
    }
}

impl Default for BinMXCAFDocNoteCommentDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_comment_note() {
        let driver = BinMXCAFDocNoteCommentDriver::new();
        let mut src = driver.new_empty();
        src.set_note("reviewer", "2026-07-02T14:30:00");
        src.set_comment("Wall thickness below spec near flange.");

        let mut stream = NcPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = NcPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst.user_name(), "reviewer");
        assert_eq!(dst.time_stamp(), "2026-07-02T14:30:00");
        assert_eq!(dst.comment(), "Wall thickness below spec near flange.");
        assert_eq!(src, dst);
    }

    #[test]
    fn roundtrip_empty_comment() {
        let driver = BinMXCAFDocNoteCommentDriver::new();
        let mut src = driver.new_empty();
        src.set_note("u", "t");

        let mut stream = NcPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = NcPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst.comment(), "");
        assert_eq!(src, dst);
    }

    #[test]
    fn missing_comment_field_fails() {
        // Write only the base payload: the comment read must fail, so the
        // driver returns false like OCCT's `!(theSource >> aComment)`.
        let driver = BinMXCAFDocNoteCommentDriver::new();
        let mut src = driver.new_empty();
        src.set_note("user", "stamp");

        let mut stream = NcPersistentStream::new();
        BinMXCAFDocNoteCommentDriver::base_paste_write(&src, &mut stream);

        let mut back = NcPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(!driver.paste_read(&mut back, &mut dst));
    }

    #[test]
    fn driver_metadata() {
        assert_eq!(
            BinMXCAFDocNoteCommentDriver::new().type_name(),
            "XCAFDoc_NoteComment"
        );
        assert_eq!(
            BinMXCAFDocNoteCommentDriver::new_named("Derived").type_name(),
            "Derived"
        );
    }
}

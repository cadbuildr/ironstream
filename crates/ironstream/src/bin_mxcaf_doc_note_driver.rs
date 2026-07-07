// FILE: bin_mxcaf_doc_note_driver.rs
// occt: BinMXCAFDoc_NoteDriver
//
// Faithful port of OCCT BinMXCAFDoc_NoteDriver (BinMXCAFDoc_NoteDriver.cxx).
// It is the abstract base driver for XCAFDoc_Note attributes; its payload is
//   <ExtendedString userName> <ExtendedString timeStamp>
// ExtendedString is stored as in BinObjMgt_Persistent::PutExtendedString:
// word-aligned start, UTF-16 characters in file byte order (big-endian),
// terminated by a NUL char16.

/// Local model of the XCAFDoc_Note base attribute payload.
#[derive(Debug, Clone, PartialEq)]
pub struct NoteAttributeBase {
    user_name: String,
    time_stamp: String,
}

impl NoteAttributeBase {
    pub fn new_empty() -> Self {
        NoteAttributeBase {
            user_name: String::new(),
            time_stamp: String::new(),
        }
    }

    /// Mirrors XCAFDoc_Note::Set(userName, timeStamp).
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

/// Local stand-in for BinObjMgt_Persistent (ExtendedString subset).
pub struct NotePersistentStream {
    data: Vec<u8>,
    pos: usize,
    err: bool,
}

impl NotePersistentStream {
    pub fn new() -> Self {
        NotePersistentStream {
            data: Vec::new(),
            pos: 0,
            err: false,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        NotePersistentStream {
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

    /// BinObjMgt_Persistent::PutExtendedString — word-aligned, UTF-16 BE
    /// characters, NUL char16 terminator.
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

impl Default for NotePersistentStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of BinMXCAFDoc_NoteDriver. In OCCT this is an abstract base
/// (constructed with a derived attribute name); the payload logic itself is
/// concrete and is what is ported and tested here.
pub struct BinMXCAFDocNoteDriver {
    name: String,
}

impl BinMXCAFDocNoteDriver {
    /// Mirrors the protected constructor taking the derived attribute name.
    pub fn new(the_name: &str) -> Self {
        BinMXCAFDocNoteDriver {
            name: the_name.to_string(),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.name
    }

    /// Mirrors Paste(read): `theSource >> aUserName >> aTimeStamp` then
    /// `aNote->Set(aUserName, aTimeStamp)`.
    pub fn paste_read(
        &self,
        source: &mut NotePersistentStream,
        target: &mut NoteAttributeBase,
    ) -> bool {
        let user_name = match source.get_extended_string() {
            Some(v) => v,
            None => return false,
        };
        let time_stamp = match source.get_extended_string() {
            Some(v) => v,
            None => return false,
        };
        target.set(&user_name, &time_stamp);
        true
    }

    /// Mirrors Paste(write): `theTarget << aNote->UserName() << aNote->TimeStamp()`.
    pub fn paste_write(&self, source: &NoteAttributeBase, target: &mut NotePersistentStream) {
        target.put_extended_string(source.user_name());
        target.put_extended_string(source.time_stamp());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_user_and_timestamp() {
        let driver = BinMXCAFDocNoteDriver::new("XCAFDoc_NoteComment");
        let mut src = NoteAttributeBase::new_empty();
        src.set("j.smith", "2026-07-02T10:00:00");

        let mut stream = NotePersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = NotePersistentStream::from_bytes(stream.bytes());
        let mut dst = NoteAttributeBase::new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst.user_name(), "j.smith");
        assert_eq!(dst.time_stamp(), "2026-07-02T10:00:00");
        assert_eq!(src, dst);
    }

    #[test]
    fn roundtrip_non_ascii_extended_string() {
        // ExtendedString is UTF-16: exercise non-Latin1 payload.
        let driver = BinMXCAFDocNoteDriver::new("XCAFDoc_Note");
        let mut src = NoteAttributeBase::new_empty();
        src.set("Пользователь", "2026年07月02日");

        let mut stream = NotePersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = NotePersistentStream::from_bytes(stream.bytes());
        let mut dst = NoteAttributeBase::new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst.user_name(), "Пользователь");
        assert_eq!(dst.time_stamp(), "2026年07月02日");
    }

    #[test]
    fn extended_string_alignment_between_fields() {
        // "abc" -> 3 chars (6 bytes) + terminator (2 bytes) = 8 bytes, then the
        // second string must start realigned on a 4-byte boundary (offset 8).
        let driver = BinMXCAFDocNoteDriver::new("XCAFDoc_Note");
        let mut src = NoteAttributeBase::new_empty();
        src.set("abc", "t");

        let mut stream = NotePersistentStream::new();
        driver.paste_write(&src, &mut stream);
        assert_eq!(stream.bytes().len(), 12); // 8 + (1 char + NUL) = 12

        let mut back = NotePersistentStream::from_bytes(stream.bytes());
        let mut dst = NoteAttributeBase::new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst, src);
    }

    #[test]
    fn truncated_stream_fails() {
        let driver = BinMXCAFDocNoteDriver::new("XCAFDoc_Note");
        let mut src = NoteAttributeBase::new_empty();
        src.set("user", "stamp");
        let mut stream = NotePersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let cut = &stream.bytes()[..6]; // inside the first string
        let mut back = NotePersistentStream::from_bytes(cut);
        let mut dst = NoteAttributeBase::new_empty();
        assert!(!driver.paste_read(&mut back, &mut dst));
        assert!(back.is_error());
    }
}

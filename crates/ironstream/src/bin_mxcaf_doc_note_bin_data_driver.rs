// FILE: bin_mxcaf_doc_note_bin_data_driver.rs
// occt: BinMXCAFDoc_NoteBinDataDriver
//
// Faithful port of OCCT BinMXCAFDoc_NoteBinDataDriver
// (BinMXCAFDoc_NoteBinDataDriver.cxx). The driver first delegates to
// BinMXCAFDoc_NoteDriver for the base note payload, then handles binary data:
//   <ExtendedString userName> <ExtendedString timeStamp>       (base)
//   <ExtendedString title> <AsciiString MIMEtype> <Integer size>
//   [<ByteArray data[size]>  only when size > 0]
//
// The BinObjMgt_Persistent plumbing is modelled locally by
// `NbdPersistentStream` (big-endian file byte order, 4-byte word alignment
// for integers/string starts, unaligned raw byte arrays).

/// Local model of the XCAFDoc_NoteBinData attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct NoteBinDataAttribute {
    user_name: String,
    time_stamp: String,
    title: String,
    mime_type: String,
    /// None mirrors a null NCollection_HArray1<uint8_t> handle (Size() == 0).
    data: Option<Vec<u8>>,
}

impl NoteBinDataAttribute {
    /// Mirrors `new XCAFDoc_NoteBinData()`.
    pub fn new_empty() -> Self {
        NoteBinDataAttribute {
            user_name: String::new(),
            time_stamp: String::new(),
            title: String::new(),
            mime_type: String::new(),
            data: None,
        }
    }

    /// Mirrors XCAFDoc_Note::Set(userName, timeStamp).
    pub fn set_note(&mut self, user_name: &str, time_stamp: &str) {
        self.user_name = user_name.to_string();
        self.time_stamp = time_stamp.to_string();
    }

    /// Mirrors XCAFDoc_NoteBinData::Set(title, MIMEtype, data).
    pub fn set_data(&mut self, title: &str, mime_type: &str, data: Option<Vec<u8>>) {
        self.title = title.to_string();
        self.mime_type = mime_type.to_string();
        self.data = data;
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn time_stamp(&self) -> &str {
        &self.time_stamp
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    /// Mirrors XCAFDoc_NoteBinData::Size().
    pub fn size(&self) -> i32 {
        self.data.as_ref().map_or(0, |d| d.len() as i32)
    }

    pub fn data(&self) -> Option<&[u8]> {
        self.data.as_deref()
    }
}

/// Local stand-in for BinObjMgt_Persistent (Integer, AsciiString,
/// ExtendedString and ByteArray subset).
pub struct NbdPersistentStream {
    data: Vec<u8>,
    pos: usize,
    err: bool,
}

impl NbdPersistentStream {
    pub fn new() -> Self {
        NbdPersistentStream {
            data: Vec::new(),
            pos: 0,
            err: false,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        NbdPersistentStream {
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

    /// BinObjMgt_Persistent::PutInteger — 4-byte aligned, file byte order.
    pub fn put_integer(&mut self, v: i32) {
        self.align_put(4);
        self.data.extend_from_slice(&v.to_be_bytes());
    }

    /// BinObjMgt_Persistent::GetInteger.
    pub fn get_integer(&mut self) -> Option<i32> {
        self.align_get(4);
        if self.pos + 4 > self.data.len() {
            self.err = true;
            return None;
        }
        let mut b = [0u8; 4];
        b.copy_from_slice(&self.data[self.pos..self.pos + 4]);
        self.pos += 4;
        Some(i32::from_be_bytes(b))
    }

    /// BinObjMgt_Persistent::PutAsciiString — word-aligned, NUL-terminated.
    pub fn put_ascii_string(&mut self, s: &str) {
        self.align_put(4);
        self.data.extend_from_slice(s.as_bytes());
        self.data.push(0);
    }

    /// BinObjMgt_Persistent::GetAsciiString.
    pub fn get_ascii_string(&mut self) -> Option<String> {
        self.align_get(4);
        let start = self.pos;
        while self.pos < self.data.len() && self.data[self.pos] != 0 {
            self.pos += 1;
        }
        if self.pos >= self.data.len() {
            self.err = true;
            self.pos = start;
            return None;
        }
        let s = String::from_utf8_lossy(&self.data[start..self.pos]).into_owned();
        self.pos += 1;
        Some(s)
    }

    /// BinObjMgt_Persistent::PutExtendedString — word-aligned UTF-16 BE, NUL char16.
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

    /// BinObjMgt_Persistent::PutByteArray — byte-aligned raw bytes.
    pub fn put_byte_array(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    /// BinObjMgt_Persistent::GetByteArray.
    pub fn get_byte_array(&mut self, len: usize) -> Option<Vec<u8>> {
        if self.pos + len > self.data.len() {
            self.err = true;
            return None;
        }
        let out = self.data[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Some(out)
    }
}

impl Default for NbdPersistentStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of BinMXCAFDoc_NoteBinDataDriver.
pub struct BinMXCAFDocNoteBinDataDriver {
    name: String,
}

impl BinMXCAFDocNoteBinDataDriver {
    /// Mirrors the constructor: driver named after XCAFDoc_NoteBinData.
    pub fn new() -> Self {
        BinMXCAFDocNoteBinDataDriver {
            name: "XCAFDoc_NoteBinData".to_string(),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.name
    }

    pub fn new_empty(&self) -> NoteBinDataAttribute {
        NoteBinDataAttribute::new_empty()
    }

    /// Local reproduction of BinMXCAFDoc_NoteDriver::Paste(read).
    fn base_paste_read(source: &mut NbdPersistentStream, target: &mut NoteBinDataAttribute) -> bool {
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
    fn base_paste_write(source: &NoteBinDataAttribute, target: &mut NbdPersistentStream) {
        target.put_extended_string(source.user_name());
        target.put_extended_string(source.time_stamp());
    }

    /// Mirrors Paste(read): base payload, then
    /// `theSource >> aTitle >> aMIMEtype >> nbSize`, then the byte array when
    /// nbSize > 0.
    pub fn paste_read(
        &self,
        source: &mut NbdPersistentStream,
        target: &mut NoteBinDataAttribute,
    ) -> bool {
        if !Self::base_paste_read(source, target) {
            return false;
        }
        let title = match source.get_extended_string() {
            Some(v) => v,
            None => return false,
        };
        let mime_type = match source.get_ascii_string() {
            Some(v) => v,
            None => return false,
        };
        let nb_size = match source.get_integer() {
            Some(v) => v,
            None => return false,
        };
        let data = if nb_size > 0 {
            match source.get_byte_array(nb_size as usize) {
                Some(d) => Some(d),
                None => return false,
            }
        } else {
            None
        };
        target.set_data(&title, &mime_type, data);
        true
    }

    /// Mirrors Paste(write): base payload, then
    /// `theTarget << Title << MIMEtype << Size` and PutByteArray when Size > 0.
    pub fn paste_write(&self, source: &NoteBinDataAttribute, target: &mut NbdPersistentStream) {
        Self::base_paste_write(source, target);
        target.put_extended_string(source.title());
        target.put_ascii_string(source.mime_type());
        target.put_integer(source.size());
        if source.size() > 0 {
            target.put_byte_array(source.data().unwrap());
        }
    }
}

impl Default for BinMXCAFDocNoteBinDataDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_with_binary_payload() {
        let driver = BinMXCAFDocNoteBinDataDriver::new();
        let mut src = driver.new_empty();
        src.set_note("author", "2026-07-02T09:15:00");
        let payload: Vec<u8> = (0u16..255).map(|v| v as u8).collect();
        src.set_data("scan.bin", "application/octet-stream", Some(payload.clone()));

        let mut stream = NbdPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = NbdPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst.user_name(), "author");
        assert_eq!(dst.time_stamp(), "2026-07-02T09:15:00");
        assert_eq!(dst.title(), "scan.bin");
        assert_eq!(dst.mime_type(), "application/octet-stream");
        assert_eq!(dst.size(), 255);
        assert_eq!(dst.data(), Some(payload.as_slice()));
        assert_eq!(src, dst);
    }

    #[test]
    fn roundtrip_without_data() {
        // Size == 0: no byte array is written and the handle stays null.
        let driver = BinMXCAFDocNoteBinDataDriver::new();
        let mut src = driver.new_empty();
        src.set_note("u", "t");
        src.set_data("empty", "text/plain", None);

        let mut stream = NbdPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = NbdPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst.size(), 0);
        assert!(dst.data().is_none());
        assert_eq!(src, dst);
    }

    #[test]
    fn odd_size_byte_array_then_realigned_reads() {
        // A 3-byte unaligned array checks that byte arrays are written
        // without padding, exactly as PutByteArray (alignOffset(1)).
        let driver = BinMXCAFDocNoteBinDataDriver::new();
        let mut src = driver.new_empty();
        src.set_note("a", "b");
        src.set_data("t", "m", Some(vec![1, 2, 3]));

        let mut stream = NbdPersistentStream::new();
        driver.paste_write(&src, &mut stream);
        // Stream ends exactly after the 3 raw bytes.
        let n = stream.bytes().len();
        assert_eq!(&stream.bytes()[n - 3..], &[1, 2, 3]);

        let mut back = NbdPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst, src);
    }

    #[test]
    fn truncated_byte_array_fails() {
        let driver = BinMXCAFDocNoteBinDataDriver::new();
        let mut src = driver.new_empty();
        src.set_note("a", "b");
        src.set_data("t", "m", Some(vec![9; 16]));

        let mut stream = NbdPersistentStream::new();
        driver.paste_write(&src, &mut stream);
        let cut = &stream.bytes()[..stream.bytes().len() - 8];
        let mut back = NbdPersistentStream::from_bytes(cut);
        let mut dst = driver.new_empty();
        assert!(!driver.paste_read(&mut back, &mut dst));
    }

    #[test]
    fn driver_metadata() {
        assert_eq!(
            BinMXCAFDocNoteBinDataDriver::new().type_name(),
            "XCAFDoc_NoteBinData"
        );
    }
}

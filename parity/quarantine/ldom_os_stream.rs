// FILE: ldom_os_stream.rs
// occt: LDOM_OSStream

/// Byte order mark types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BOMType {
    Undefined = 0,
    Utf8 = 1,
    Utf16BE = 2,
    Utf16LE = 3,
    Utf32BE = 4,
    Utf32LE = 5,
    Utf7 = 6,
    Utf1 = 7,
    UtfEbcdic = 8,
    Scsu = 9,
    Bocu1 = 10,
    Gb18030 = 11,
}

/// Output stream buffer element
struct StringElem {
    buf: Vec<u8>,
    len: usize,
}

/// LDOM output stream for collecting output into strings.
pub struct LDOMOSStream {
    max_buf: usize,
    length: usize,
    first_string: Option<Box<StringElem>>,
    cur_string: Option<Box<StringElem>>,
    bom: BOMType,
}

impl LDOMOSStream {
    /// Constructor with default buffer size
    pub fn new(max_buf: usize) -> Self {
        LDOMOSStream {
            max_buf,
            length: 0,
            first_string: None,
            cur_string: None,
            bom: BOMType::Undefined,
        }
    }

    /// Get the full length of contained data
    pub fn length(&self) -> usize {
        self.length
    }

    /// Get the concatenated string result
    pub fn str(&self) -> String {
        let mut result = String::new();
        if let Some(ref first) = self.first_string {
            let s = String::from_utf8_lossy(&first.buf[..first.len]);
            result.push_str(&s);
        }
        result
    }

    /// Clear the stream
    pub fn clear(&mut self) {
        self.first_string = None;
        self.cur_string = None;
        self.length = 0;
    }

    /// Write data to the stream
    pub fn write(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if self.first_string.is_none() {
            self.first_string = Some(Box::new(StringElem {
                buf: vec![0u8; self.max_buf],
                len: 0,
            }));
            self.cur_string = self.first_string.as_mut();
        }

        self.length += data.len();

        if let Some(ref mut cur) = self.cur_string {
            let remaining = cur.buf.len() - cur.len;
            if remaining >= data.len() {
                cur.buf[cur.len..cur.len + data.len()].copy_from_slice(data);
                cur.len += data.len();
            } else {
                // TODO: Handle overflow case
            }
        }
    }

    /// Get the byte order mark
    pub fn get_bom(&self) -> BOMType {
        self.bom
    }

    /// Set the byte order mark
    pub fn set_bom(&mut self, bom: BOMType) {
        self.bom = bom;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_creation() {
        let stream = LDOMOSStream::new(1024);
        assert_eq!(stream.length(), 0);
        assert_eq!(stream.bom, BOMType::Undefined);
    }

    #[test]
    fn test_write_data() {
        let mut stream = LDOMOSStream::new(1024);
        stream.write(b"hello");
        assert_eq!(stream.length(), 5);
    }

    #[test]
    fn test_clear() {
        let mut stream = LDOMOSStream::new(1024);
        stream.write(b"data");
        stream.clear();
        assert_eq!(stream.length(), 0);
    }

    #[test]
    fn test_bom_type() {
        let mut stream = LDOMOSStream::new(1024);
        stream.set_bom(BOMType::Utf8);
        assert_eq!(stream.get_bom(), BOMType::Utf8);
    }

    #[test]
    fn test_str_result() {
        let mut stream = LDOMOSStream::new(1024);
        stream.write(b"test");
        let s = stream.str();
        assert_eq!(s, "test");
    }
}

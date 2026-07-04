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

/// Output stream buffer element (LDOM_SBuffer::LDOM_StringElem).
/// OCCT chains these in a linked list; a Vec preserves the same order.
struct StringElem {
    buf: Vec<u8>,
    len: usize,
}

impl StringElem {
    fn new(capacity: usize) -> Self {
        StringElem {
            buf: vec![0u8; capacity],
            len: 0,
        }
    }
}

/// LDOM output stream for collecting output into strings.
pub struct LDOMOSStream {
    max_buf: usize,
    length: usize,
    strings: Vec<StringElem>,
    bom: BOMType,
}

impl LDOMOSStream {
    /// Constructor with default buffer size
    pub fn new(max_buf: usize) -> Self {
        // OCCT allocates the first buffer element at construction
        LDOMOSStream {
            max_buf,
            length: 0,
            strings: vec![StringElem::new(max_buf)],
            bom: BOMType::Undefined,
        }
    }

    /// Get the full length of contained data
    pub fn length(&self) -> usize {
        self.length
    }

    /// Get the concatenated string result (LDOM_SBuffer::str)
    pub fn str(&self) -> String {
        let mut result = String::with_capacity(self.length);
        for elem in &self.strings {
            result.push_str(&String::from_utf8_lossy(&elem.buf[..elem.len]));
        }
        result
    }

    /// Clear the stream (LDOM_SBuffer::Clear)
    pub fn clear(&mut self) {
        self.strings.clear();
        self.strings.push(StringElem::new(self.max_buf));
        self.length = 0;
    }

    /// Write data to the stream (LDOM_SBuffer::xsputn)
    pub fn write(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let mut remaining = data;
        loop {
            let cur = self.strings.last_mut().expect("at least one buffer");
            let free = cur.buf.len() - cur.len;
            if free >= remaining.len() {
                cur.buf[cur.len..cur.len + remaining.len()].copy_from_slice(remaining);
                cur.len += remaining.len();
                break;
            }
            // Fill what fits, then chain a new element sized max(rest, max_buf),
            // as OCCT does in xsputn's overflow branches.
            if free > 0 {
                cur.buf[cur.len..cur.len + free].copy_from_slice(&remaining[..free]);
                cur.len += free;
                remaining = &remaining[free..];
            }
            let next_cap = std::cmp::max(remaining.len(), self.max_buf);
            self.strings.push(StringElem::new(next_cap));
        }
        self.length += data.len();
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
        assert_eq!(stream.str(), "");
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

    #[test]
    fn test_overflow_across_buffers() {
        // Buffer smaller than the data forces chaining of elements
        let mut stream = LDOMOSStream::new(4);
        stream.write(b"abcdef");
        stream.write(b"ghij");
        assert_eq!(stream.length(), 10);
        assert_eq!(stream.str(), "abcdefghij");
    }
}

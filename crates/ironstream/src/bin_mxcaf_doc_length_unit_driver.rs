// FILE: bin_mxcaf_doc_length_unit_driver.rs
// occt: BinMXCAFDoc_LengthUnitDriver
//
// Faithful port of OCCT BinMXCAFDoc_LengthUnitDriver (BinMXCAFDoc_LengthUnitDriver.cxx).
// The driver serializes an XCAFDoc_LengthUnit attribute to the OCAF binary
// persistence stream. Payload layout (per the .cxx Paste methods):
//   <AsciiString unit-name> <Real scale-factor>
//
// The OCAF plumbing (BinObjMgt_Persistent) is modelled locally by
// `LuUnitPersistentStream`, which reproduces the file byte layout of
// BinObjMgt_Persistent: big-endian file byte order (OCCT inverses on
// little-endian machines), 4-byte word alignment before integers, reals and
// string starts, and NUL-terminated ASCII strings.

/// Local model of the XCAFDoc_LengthUnit attribute payload.
#[derive(Debug, Clone, PartialEq)]
pub struct LuLengthUnitAttribute {
    unit_name: String,
    unit_value: f64,
}

impl LuLengthUnitAttribute {
    /// Mirrors XCAFDoc_LengthUnit: default-constructed empty attribute.
    pub fn new_empty() -> Self {
        LuLengthUnitAttribute {
            unit_name: String::new(),
            unit_value: 1.0,
        }
    }

    /// Mirrors XCAFDoc_LengthUnit::Set(name, scaleFactor).
    pub fn set(&mut self, name: &str, scale_factor: f64) {
        self.unit_name = name.to_string();
        self.unit_value = scale_factor;
    }

    /// Mirrors XCAFDoc_LengthUnit::GetUnitName().
    pub fn get_unit_name(&self) -> &str {
        &self.unit_name
    }

    /// Mirrors XCAFDoc_LengthUnit::GetUnitValue().
    pub fn get_unit_value(&self) -> f64 {
        self.unit_value
    }
}

/// Local stand-in for BinObjMgt_Persistent restricted to the operations the
/// length-unit driver needs (AsciiString and Real).
pub struct LuUnitPersistentStream {
    data: Vec<u8>,
    pos: usize,
    err: bool,
}

impl LuUnitPersistentStream {
    pub fn new() -> Self {
        LuUnitPersistentStream {
            data: Vec::new(),
            pos: 0,
            err: false,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        LuUnitPersistentStream {
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

    /// BinObjMgt_Persistent::PutReal — 4-byte aligned, 8 bytes, file byte order.
    pub fn put_real(&mut self, v: f64) {
        self.align_put(4);
        self.data.extend_from_slice(&v.to_bits().to_be_bytes());
    }

    /// BinObjMgt_Persistent::GetReal.
    pub fn get_real(&mut self) -> Option<f64> {
        self.align_get(4);
        if self.pos + 8 > self.data.len() {
            self.err = true;
            return None;
        }
        let mut b = [0u8; 8];
        b.copy_from_slice(&self.data[self.pos..self.pos + 8]);
        self.pos += 8;
        Some(f64::from_bits(u64::from_be_bytes(b)))
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
        self.pos += 1; // count the end null char
        Some(s)
    }
}

impl Default for LuUnitPersistentStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of BinMXCAFDoc_LengthUnitDriver.
pub struct BinMXCAFDocLengthUnitDriver {
    name: String,
}

impl BinMXCAFDocLengthUnitDriver {
    /// Mirrors the constructor: driver named after XCAFDoc_LengthUnit.
    pub fn new() -> Self {
        BinMXCAFDocLengthUnitDriver {
            name: "XCAFDoc_LengthUnit".to_string(),
        }
    }

    /// Mirrors BinMDF_ADriver name access.
    pub fn type_name(&self) -> &str {
        &self.name
    }

    /// Mirrors NewEmpty(): a fresh XCAFDoc_LengthUnit.
    pub fn new_empty(&self) -> LuLengthUnitAttribute {
        LuLengthUnitAttribute::new_empty()
    }

    /// Mirrors Paste(read): `theSource >> aName >> aScaleFactor`, then
    /// `anAtt->Set(aName, aScaleFactor)`. Returns false on stream error.
    pub fn paste_read(
        &self,
        source: &mut LuUnitPersistentStream,
        target: &mut LuLengthUnitAttribute,
    ) -> bool {
        let name = match source.get_ascii_string() {
            Some(n) => n,
            None => return false,
        };
        let scale_factor = match source.get_real() {
            Some(v) => v,
            None => return false,
        };
        target.set(&name, scale_factor);
        true
    }

    /// Mirrors Paste(write): `theTarget << anAtt->GetUnitName() << anAtt->GetUnitValue()`.
    pub fn paste_write(&self, source: &LuLengthUnitAttribute, target: &mut LuUnitPersistentStream) {
        target.put_ascii_string(source.get_unit_name());
        target.put_real(source.get_unit_value());
    }
}

impl Default for BinMXCAFDocLengthUnitDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_millimeter_unit() {
        let driver = BinMXCAFDocLengthUnitDriver::new();
        let mut src = driver.new_empty();
        src.set("mm", 0.001);

        let mut stream = LuUnitPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = LuUnitPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst.get_unit_name(), "mm");
        assert_eq!(dst.get_unit_value(), 0.001);
        assert_eq!(src, dst);
    }

    #[test]
    fn roundtrip_longer_name_alignment() {
        // A 5-char name ("meter" + NUL = 6 bytes) forces re-alignment before the real.
        let driver = BinMXCAFDocLengthUnitDriver::new();
        let mut src = driver.new_empty();
        src.set("meter", 1.0);

        let mut stream = LuUnitPersistentStream::new();
        driver.paste_write(&src, &mut stream);
        // name(6 bytes) padded to 8, then 8-byte real
        assert_eq!(stream.bytes().len(), 16);

        let mut back = LuUnitPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst, src);
    }

    #[test]
    fn truncated_stream_fails() {
        let driver = BinMXCAFDocLengthUnitDriver::new();
        let mut src = driver.new_empty();
        src.set("in", 0.0254);
        let mut stream = LuUnitPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        // Drop the trailing real bytes: read must return false, like isOk in OCCT.
        let cut = &stream.bytes()[..stream.bytes().len() - 8];
        let mut back = LuUnitPersistentStream::from_bytes(cut);
        let mut dst = driver.new_empty();
        assert!(!driver.paste_read(&mut back, &mut dst));
        assert!(back.is_error());
    }

    #[test]
    fn driver_metadata() {
        let driver = BinMXCAFDocLengthUnitDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_LengthUnit");
        assert_eq!(driver.new_empty().get_unit_value(), 1.0);
    }
}

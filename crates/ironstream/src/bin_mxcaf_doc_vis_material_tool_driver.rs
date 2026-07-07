// FILE: bin_mxcaf_doc_vis_material_tool_driver.rs
// occt: BinMXCAFDoc_VisMaterialToolDriver
//
// Faithful port of OCCT BinMXCAFDoc_VisMaterialToolDriver
// (BinMXCAFDoc_VisMaterialToolDriver.cxx). XCAFDoc_VisMaterialTool is a pure
// marker/tool attribute: its binary payload is EMPTY. Paste(read) always
// returns true without touching the stream, and Paste(write) writes nothing.
// That "nothing" IS the contract, and the tests verify it byte-for-byte.

/// Local model of the XCAFDoc_VisMaterialTool attribute (stateless in
/// persistence terms — its material table lives in child labels).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct XcafVisMaterialToolAttribute;

impl XcafVisMaterialToolAttribute {
    /// Mirrors `new XCAFDoc_VisMaterialTool()`.
    pub fn new_empty() -> Self {
        XcafVisMaterialToolAttribute
    }
}

/// Local stand-in for BinObjMgt_Persistent — only what this driver touches,
/// which is nothing beyond position bookkeeping.
pub struct VmtPersistentStream {
    data: Vec<u8>,
    pos: usize,
}

impl VmtPersistentStream {
    pub fn new() -> Self {
        VmtPersistentStream {
            data: Vec::new(),
            pos: 0,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        VmtPersistentStream {
            data: bytes.to_vec(),
            pos: 0,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn position(&self) -> usize {
        self.pos
    }
}

impl Default for VmtPersistentStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of BinMXCAFDoc_VisMaterialToolDriver.
pub struct BinMXCAFDocVisMaterialToolDriver {
    name: String,
}

impl BinMXCAFDocVisMaterialToolDriver {
    /// Mirrors the constructor: driver named after XCAFDoc_VisMaterialTool.
    pub fn new() -> Self {
        BinMXCAFDocVisMaterialToolDriver {
            name: "XCAFDoc_VisMaterialTool".to_string(),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.name
    }

    /// Mirrors NewEmpty().
    pub fn new_empty(&self) -> XcafVisMaterialToolAttribute {
        XcafVisMaterialToolAttribute::new_empty()
    }

    /// Mirrors Paste(read): `{ return true; }` — consumes nothing.
    pub fn paste_read(
        &self,
        _source: &mut VmtPersistentStream,
        _target: &mut XcafVisMaterialToolAttribute,
    ) -> bool {
        true
    }

    /// Mirrors Paste(write): `{}` — produces nothing.
    pub fn paste_write(
        &self,
        _source: &XcafVisMaterialToolAttribute,
        _target: &mut VmtPersistentStream,
    ) {
    }
}

impl Default for BinMXCAFDocVisMaterialToolDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_produces_empty_payload() {
        let driver = BinMXCAFDocVisMaterialToolDriver::new();
        let att = driver.new_empty();
        let mut stream = VmtPersistentStream::new();
        driver.paste_write(&att, &mut stream);
        assert!(stream.bytes().is_empty());
    }

    #[test]
    fn read_succeeds_and_consumes_nothing() {
        let driver = BinMXCAFDocVisMaterialToolDriver::new();
        // Even with surrounding bytes in the stream (other attributes' data),
        // the tool driver must not consume any of them.
        let mut stream = VmtPersistentStream::from_bytes(&[1, 2, 3, 4]);
        let mut att = driver.new_empty();
        assert!(driver.paste_read(&mut stream, &mut att));
        assert_eq!(stream.position(), 0);
        assert_eq!(att, XcafVisMaterialToolAttribute::new_empty());
    }

    #[test]
    fn roundtrip_is_identity() {
        let driver = BinMXCAFDocVisMaterialToolDriver::new();
        let src = driver.new_empty();
        let mut stream = VmtPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = VmtPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(src, dst);
    }

    #[test]
    fn driver_metadata() {
        assert_eq!(
            BinMXCAFDocVisMaterialToolDriver::new().type_name(),
            "XCAFDoc_VisMaterialTool"
        );
    }
}

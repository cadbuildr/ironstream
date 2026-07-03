// FILE: bin_ocaf.rs
// occt: BinMDF_ADriver, BinMDF_ADriverTable, BinMDF_RRelocationTable,
//       BinOCAFDrivers_DocumentRetrievalDriver

/// Version stamp written at the start of a binary OCAF file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BinOcafVersion {
    pub major: u16,
    pub minor: u16,
}

impl Default for BinOcafVersion {
    fn default() -> Self { Self { major: 1, minor: 0 } }
}

impl BinOcafVersion {
    pub fn new(major: u16, minor: u16) -> Self { Self { major, minor } }
    pub fn as_u32(&self) -> u32 { (self.major as u32) << 16 | self.minor as u32 }
}

/// Type of attribute stored in a binary OCAF stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum BinMdfAttrType {
    #[default]
    Integer,
    Real,
    AsciiString,
    ExtString,
    IntArray,
    RealArray,
    ByteArray,
    Comment,
    Name,
    Shape,
    Function,
    Reference,
    Color,
    Material,
    Layer,
}

/// In-memory binary record for one attribute.
/// occt: BinMDF_ADriver (result of Paste operation)
#[derive(Clone, Debug)]
pub struct BinMdfAttributeRecord {
    pub attr_type: BinMdfAttrType,
    pub label_entry: String,   // "0:1:2:3" format
    pub payload: Vec<u8>,
}

impl BinMdfAttributeRecord {
    pub fn new(attr_type: BinMdfAttrType, label_entry: impl Into<String>, payload: Vec<u8>) -> Self {
        Self { attr_type, label_entry: label_entry.into(), payload }
    }

    pub fn byte_size(&self) -> usize { self.payload.len() }
}

/// Relocation table: maps old label id → new label id.
/// occt: BinMDF_RRelocationTable
#[derive(Clone, Debug, Default)]
pub struct BinMdfRelocationTable {
    pub label_map: Vec<(u32, u32)>,   // (old, new)
    pub attr_map: Vec<(u32, u32)>,    // (old_attr_id, new_attr_id)
}

impl BinMdfRelocationTable {
    pub fn new() -> Self { Self::default() }

    pub fn bind_label(&mut self, old_id: u32, new_id: u32) {
        if !self.label_map.iter().any(|(o, _)| *o == old_id) {
            self.label_map.push((old_id, new_id));
        }
    }

    pub fn find_label(&self, old_id: u32) -> Option<u32> {
        self.label_map.iter().find(|(o, _)| *o == old_id).map(|(_, n)| *n)
    }

    pub fn bind_attr(&mut self, old_id: u32, new_id: u32) {
        if !self.attr_map.iter().any(|(o, _)| *o == old_id) {
            self.attr_map.push((old_id, new_id));
        }
    }

    pub fn find_attr(&self, old_id: u32) -> Option<u32> {
        self.attr_map.iter().find(|(o, _)| *o == old_id).map(|(_, n)| *n)
    }

    pub fn clear(&mut self) { self.label_map.clear(); self.attr_map.clear(); }
}

/// Driver table: maps attr_type → driver id.
/// occt: BinMDF_ADriverTable
#[derive(Clone, Debug, Default)]
pub struct BinMdfADriverTable {
    pub drivers: Vec<(BinMdfAttrType, String)>,
}

impl BinMdfADriverTable {
    pub fn new() -> Self { Self::default() }

    pub fn add_driver(&mut self, attr_type: BinMdfAttrType, driver_name: impl Into<String>) {
        let name = driver_name.into();
        if !self.drivers.iter().any(|(t, _)| *t == attr_type) {
            self.drivers.push((attr_type, name));
        }
    }

    pub fn find_driver(&self, attr_type: BinMdfAttrType) -> Option<&str> {
        self.drivers.iter().find(|(t, _)| *t == attr_type).map(|(_, n)| n.as_str())
    }

    pub fn nb_drivers(&self) -> usize { self.drivers.len() }
}

/// Binary OCAF document writer/reader stub.
/// occt: BinOCAFDrivers (simplified)
#[derive(Clone, Debug)]
pub struct BinOcafDocument {
    pub version: BinOcafVersion,
    pub driver_table: BinMdfADriverTable,
    pub records: Vec<BinMdfAttributeRecord>,
    pub is_stored: bool,
}

impl Default for BinOcafDocument {
    fn default() -> Self {
        Self {
            version: BinOcafVersion::default(),
            driver_table: BinMdfADriverTable::new(),
            records: Vec::new(),
            is_stored: false,
        }
    }
}

impl BinOcafDocument {
    pub fn new() -> Self { Self::default() }

    pub fn add_record(&mut self, rec: BinMdfAttributeRecord) { self.records.push(rec); }
    pub fn nb_records(&self) -> usize { self.records.len() }

    /// Stub: "write" to a byte buffer.
    pub fn store(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.version.as_u32().to_le_bytes());
        buf.extend_from_slice(&(self.records.len() as u32).to_le_bytes());
        for r in &self.records {
            buf.extend_from_slice(&(r.payload.len() as u32).to_le_bytes());
            buf.extend_from_slice(&r.payload);
        }
        self.is_stored = true;
        buf
    }

    /// Stub: "read" from a byte buffer (just reads the header).
    pub fn retrieve(buf: &[u8]) -> Option<(BinOcafVersion, usize)> {
        if buf.len() < 8 { return None; }
        let v = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let n = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]) as usize;
        let major = (v >> 16) as u16;
        let minor = (v & 0xFFFF) as u16;
        Some((BinOcafVersion::new(major, minor), n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_as_u32() {
        let v = BinOcafVersion::new(2, 3);
        assert_eq!(v.as_u32(), (2u32 << 16) | 3u32);
    }

    #[test]
    fn relocation_table_labels() {
        let mut t = BinMdfRelocationTable::new();
        t.bind_label(1, 100);
        t.bind_label(2, 200);
        t.bind_label(1, 999); // dup – ignored
        assert_eq!(t.find_label(1), Some(100));
        assert_eq!(t.find_label(3), None);
    }

    #[test]
    fn driver_table_add_find() {
        let mut dt = BinMdfADriverTable::new();
        dt.add_driver(BinMdfAttrType::Integer, "IntDriver");
        dt.add_driver(BinMdfAttrType::Real, "RealDriver");
        dt.add_driver(BinMdfAttrType::Integer, "Dup"); // ignored
        assert_eq!(dt.nb_drivers(), 2);
        assert_eq!(dt.find_driver(BinMdfAttrType::Real), Some("RealDriver"));
        assert!(dt.find_driver(BinMdfAttrType::Color).is_none());
    }

    #[test]
    fn document_store_retrieve() {
        let mut doc = BinOcafDocument::new();
        doc.add_record(BinMdfAttributeRecord::new(
            BinMdfAttrType::Integer, "0:1", vec![1, 0, 0, 0],
        ));
        let buf = doc.store();
        assert!(doc.is_stored);
        let (ver, count) = BinOcafDocument::retrieve(&buf).unwrap();
        assert_eq!(ver.major, 1);
        assert_eq!(count, 1);
    }

    #[test]
    fn document_retrieve_empty_buf() {
        assert!(BinOcafDocument::retrieve(&[]).is_none());
        assert!(BinOcafDocument::retrieve(&[0, 1]).is_none());
    }

    #[test]
    fn relocation_clear() {
        let mut t = BinMdfRelocationTable::new();
        t.bind_label(5, 50);
        t.bind_attr(10, 100);
        t.clear();
        assert!(t.find_label(5).is_none());
        assert!(t.find_attr(10).is_none());
    }
}

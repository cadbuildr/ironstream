// FILE: bin_mxcaf_doc_material_driver.rs
// occt: BinMXCAFDoc_MaterialDriver
//
// Faithful port of OCCT BinMXCAFDoc_MaterialDriver (BinMXCAFDoc_MaterialDriver.cxx).
// Payload layout, exactly as in the .cxx Paste methods:
//   <AsciiString name> <AsciiString description> <Real density>
//   <AsciiString densName> <AsciiString densValType>
// Null handle strings are written as "" (see local pasteString in the .cxx).
//
// The BinObjMgt_Persistent plumbing is modelled locally by
// `MatPersistentStream` (big-endian file byte order, 4-byte word alignment
// before reals and string starts, NUL-terminated ASCII strings).

/// Local model of the XCAFDoc_Material attribute payload.
/// The TCollection_HAsciiString handles are modelled as Option<String>
/// (None == null handle).
#[derive(Debug, Clone, PartialEq)]
pub struct MatMaterialAttribute {
    name: Option<String>,
    description: Option<String>,
    density: f64,
    dens_name: Option<String>,
    dens_val_type: Option<String>,
}

impl MatMaterialAttribute {
    /// Mirrors `new XCAFDoc_Material()`.
    pub fn new_empty() -> Self {
        MatMaterialAttribute {
            name: None,
            description: None,
            density: 0.0,
            dens_name: None,
            dens_val_type: None,
        }
    }

    /// Mirrors XCAFDoc_Material::Set(name, descr, density, densName, densValType).
    pub fn set(
        &mut self,
        name: Option<&str>,
        description: Option<&str>,
        density: f64,
        dens_name: Option<&str>,
        dens_val_type: Option<&str>,
    ) {
        self.name = name.map(|s| s.to_string());
        self.description = description.map(|s| s.to_string());
        self.density = density;
        self.dens_name = dens_name.map(|s| s.to_string());
        self.dens_val_type = dens_val_type.map(|s| s.to_string());
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn get_density(&self) -> f64 {
        self.density
    }

    pub fn get_dens_name(&self) -> Option<&str> {
        self.dens_name.as_deref()
    }

    pub fn get_dens_val_type(&self) -> Option<&str> {
        self.dens_val_type.as_deref()
    }
}

/// Local stand-in for BinObjMgt_Persistent (AsciiString + Real subset).
pub struct MatPersistentStream {
    data: Vec<u8>,
    pos: usize,
    err: bool,
}

impl MatPersistentStream {
    pub fn new() -> Self {
        MatPersistentStream {
            data: Vec::new(),
            pos: 0,
            err: false,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        MatPersistentStream {
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

    /// BinObjMgt_Persistent::PutReal.
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
        self.pos += 1;
        Some(s)
    }
}

impl Default for MatPersistentStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of BinMXCAFDoc_MaterialDriver.
pub struct BinMXCAFDocMaterialDriver {
    name: String,
}

impl BinMXCAFDocMaterialDriver {
    pub fn new() -> Self {
        BinMXCAFDocMaterialDriver {
            name: "XCAFDoc_Material".to_string(),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.name
    }

    pub fn new_empty(&self) -> MatMaterialAttribute {
        MatMaterialAttribute::new_empty()
    }

    /// Mirrors the static pasteString() helper: null handle -> "".
    fn paste_string(target: &mut MatPersistentStream, s: Option<&str>) {
        target.put_ascii_string(s.unwrap_or(""));
    }

    /// Mirrors Paste(read):
    /// `theSource >> aName >> aDescr >> aDensity >> aDensName >> aDensValType`
    /// then Set with new HAsciiString handles for every field.
    pub fn paste_read(
        &self,
        source: &mut MatPersistentStream,
        target: &mut MatMaterialAttribute,
    ) -> bool {
        let name = match source.get_ascii_string() {
            Some(v) => v,
            None => return false,
        };
        let descr = match source.get_ascii_string() {
            Some(v) => v,
            None => return false,
        };
        let density = match source.get_real() {
            Some(v) => v,
            None => return false,
        };
        let dens_name = match source.get_ascii_string() {
            Some(v) => v,
            None => return false,
        };
        let dens_val_type = match source.get_ascii_string() {
            Some(v) => v,
            None => return false,
        };
        // OCCT always builds non-null handles on read.
        target.set(
            Some(&name),
            Some(&descr),
            density,
            Some(&dens_name),
            Some(&dens_val_type),
        );
        true
    }

    /// Mirrors Paste(write): name, description, density, densName, densValType.
    pub fn paste_write(&self, source: &MatMaterialAttribute, target: &mut MatPersistentStream) {
        Self::paste_string(target, source.get_name());
        Self::paste_string(target, source.get_description());
        target.put_real(source.get_density());
        Self::paste_string(target, source.get_dens_name());
        Self::paste_string(target, source.get_dens_val_type());
    }
}

impl Default for BinMXCAFDocMaterialDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_full_material() {
        let driver = BinMXCAFDocMaterialDriver::new();
        let mut src = driver.new_empty();
        src.set(
            Some("Steel"),
            Some("structural steel S235"),
            7850.0,
            Some("density"),
            Some("kg/m^3"),
        );

        let mut stream = MatPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = MatPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst.get_name(), Some("Steel"));
        assert_eq!(dst.get_description(), Some("structural steel S235"));
        assert_eq!(dst.get_density(), 7850.0);
        assert_eq!(dst.get_dens_name(), Some("density"));
        assert_eq!(dst.get_dens_val_type(), Some("kg/m^3"));
    }

    #[test]
    fn null_handles_serialize_as_empty_strings() {
        // pasteString() writes "" for null handles; on read they come back
        // as empty (non-null) strings — matching OCCT behaviour.
        let driver = BinMXCAFDocMaterialDriver::new();
        let mut src = driver.new_empty();
        src.set(None, None, 2.5, None, None);

        let mut stream = MatPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        let mut back = MatPersistentStream::from_bytes(stream.bytes());
        let mut dst = driver.new_empty();
        assert!(driver.paste_read(&mut back, &mut dst));
        assert_eq!(dst.get_name(), Some(""));
        assert_eq!(dst.get_description(), Some(""));
        assert_eq!(dst.get_density(), 2.5);
        assert_eq!(dst.get_dens_name(), Some(""));
        assert_eq!(dst.get_dens_val_type(), Some(""));
    }

    #[test]
    fn truncated_stream_fails() {
        let driver = BinMXCAFDocMaterialDriver::new();
        let mut src = driver.new_empty();
        src.set(Some("Al"), Some("aluminium"), 2700.0, Some("d"), Some("t"));
        let mut stream = MatPersistentStream::new();
        driver.paste_write(&src, &mut stream);

        // Truncate inside the density real.
        let cut = &stream.bytes()[..14];
        let mut back = MatPersistentStream::from_bytes(cut);
        let mut dst = driver.new_empty();
        assert!(!driver.paste_read(&mut back, &mut dst));
    }

    #[test]
    fn driver_metadata() {
        let driver = BinMXCAFDocMaterialDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_Material");
    }
}

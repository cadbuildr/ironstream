// FILE: bin_mxcaf_doc_location_driver.rs
// occt: BinMXCAFDoc_LocationDriver
//
// Faithful port of OCCT BinMXCAFDoc_LocationDriver
// (BinMXCAFDoc_LocationDriver.cxx). The driver translates a TopLoc_Location
// chain to/from the OCAF binary stream. This port implements the
// self-contained pre-TDocStd_FormatVersion_VERSION_6 layout, which is exactly
// what the .cxx reader parses when no named-shape driver is attached:
//
//   location := <Integer id>                      (0 terminates: identity)
//               <Integer power>
//               <Integer readDatum> <Integer datumId>
//               [ when readDatum == -1 (first occurrence of the datum):
//                 <Real scaleFactor> <Integer trsfForm>
//                 <Real mat[1,1]> ... <Real mat[3,3]>   (row major, 9 values)
//                 <Real x> <Real y> <Real z> ]
//               location                          (recursion: NextLocation)
//
// Reading composes `theLoc = aNextLoc * TopLoc_Location(aDatum).Powered(aPower)`,
// which in the item-list representation restores the datum item as list head.
// Datum sharing goes through the relocation tables (Bind/Find), also modelled
// locally. Reading a >= VERSION_6 stream without a named-shape driver fails,
// mirroring `if (aFileVer >= VERSION_6 && myNSDriver.IsNull()) return false`.

use std::collections::HashMap;
use std::rc::Rc;

/// Local model of TopLoc_Datum3D: an elementary gp_Trsf.
#[derive(Debug, Clone, PartialEq)]
pub struct LocDatum3D {
    pub scale_factor: f64,
    /// gp_TrsfForm as integer (gp_Identity = 0, gp_Rotation = 1, ...).
    pub trsf_form: i32,
    /// Vectorial part, rows 1..3 x cols 1..3.
    pub matrix: [[f64; 3]; 3],
    /// Translation part.
    pub translation: [f64; 3],
}

impl LocDatum3D {
    pub fn identity() -> Self {
        LocDatum3D {
            scale_factor: 1.0,
            trsf_form: 0,
            matrix: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            translation: [0.0, 0.0, 0.0],
        }
    }

    pub fn translation_xyz(x: f64, y: f64, z: f64) -> Self {
        LocDatum3D {
            scale_factor: 1.0,
            trsf_form: 2, // gp_Translation
            matrix: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            translation: [x, y, z],
        }
    }
}

/// One elementary item of a TopLoc_Location list: datum raised to a power.
/// Datums are shared through Rc, mirroring TopLoc_Datum3D handles.
#[derive(Debug, Clone)]
pub struct LocItem {
    pub datum: Rc<LocDatum3D>,
    pub power: i32,
}

/// Local model of TopLoc_Location: list of items, head = FirstDatum/FirstPower;
/// empty list = identity. NextLocation() is the tail.
#[derive(Debug, Clone, Default)]
pub struct LocLocation {
    pub items: Vec<LocItem>,
}

impl LocLocation {
    pub fn identity() -> Self {
        LocLocation { items: Vec::new() }
    }

    pub fn is_identity(&self) -> bool {
        self.items.is_empty()
    }

    /// Structural equality (datum contents + powers).
    pub fn same_as(&self, other: &LocLocation) -> bool {
        self.items.len() == other.items.len()
            && self
                .items
                .iter()
                .zip(other.items.iter())
                .all(|(a, b)| a.power == b.power && *a.datum == *b.datum)
    }

    /// Mirrors `aNextLoc * TopLoc_Location(aDatum).Powered(aPower)` as used by
    /// the reader: the datum item becomes the head of the resulting list and
    /// `self` (aNextLoc) becomes the tail — restoring head::tail structure.
    pub fn multiplied_by_powered(&self, datum: Rc<LocDatum3D>, power: i32) -> LocLocation {
        let mut items = Vec::with_capacity(self.items.len() + 1);
        items.push(LocItem { datum, power });
        items.extend(self.items.iter().cloned());
        LocLocation { items }
    }
}

/// Local model of the XCAFDoc_Location attribute.
#[derive(Debug, Clone, Default)]
pub struct XcafLocationAttribute {
    location: LocLocation,
}

impl XcafLocationAttribute {
    pub fn new_empty() -> Self {
        Self::default()
    }

    /// Mirrors XCAFDoc_Location::Set(loc).
    pub fn set(&mut self, loc: LocLocation) {
        self.location = loc;
    }

    /// Mirrors XCAFDoc_Location::Get().
    pub fn get(&self) -> &LocLocation {
        &self.location
    }
}

/// TDocStd_FormatVersion_VERSION_6 threshold from the .cxx version checks.
pub const LOC_FORMAT_VERSION_6: i32 = 6;

/// Read relocation table ~ BinObjMgt_RRelocationTable: bound datums plus the
/// header data carrying the storage version
/// (`theMap.GetHeaderData()->StorageVersion()`).
pub struct LocRRelocTable {
    storage_version: i32,
    bound: HashMap<i32, Rc<LocDatum3D>>,
}

impl LocRRelocTable {
    pub fn new(storage_version: i32) -> Self {
        LocRRelocTable {
            storage_version,
            bound: HashMap::new(),
        }
    }

    pub fn storage_version(&self) -> i32 {
        self.storage_version
    }

    pub fn is_bound(&self, id: i32) -> bool {
        self.bound.contains_key(&id)
    }

    pub fn find(&self, id: i32) -> Rc<LocDatum3D> {
        Rc::clone(&self.bound[&id])
    }

    pub fn bind(&mut self, id: i32, datum: Rc<LocDatum3D>) {
        self.bound.insert(id, datum);
    }
}

/// Write relocation table: assigns persistent ids to shared datums
/// (identity = handle identity, i.e. the Rc pointer).
#[derive(Default)]
pub struct LocWRelocTable {
    ids: HashMap<usize, i32>,
    next_id: i32,
}

impl LocWRelocTable {
    pub fn new() -> Self {
        LocWRelocTable {
            ids: HashMap::new(),
            next_id: 0,
        }
    }

    /// Returns (id, was_already_bound).
    fn add(&mut self, datum: &Rc<LocDatum3D>) -> (i32, bool) {
        let key = Rc::as_ptr(datum) as usize;
        if let Some(&id) = self.ids.get(&key) {
            return (id, true);
        }
        self.next_id += 1;
        self.ids.insert(key, self.next_id);
        (self.next_id, false)
    }
}

/// Local stand-in for BinObjMgt_Persistent (Integer + Real subset).
pub struct LocPersistentStream {
    data: Vec<u8>,
    pos: usize,
    err: bool,
}

impl LocPersistentStream {
    pub fn new() -> Self {
        LocPersistentStream {
            data: Vec::new(),
            pos: 0,
            err: false,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        LocPersistentStream {
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

    /// BinObjMgt_Persistent::PutInteger.
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
}

impl Default for LocPersistentStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of BinMXCAFDoc_LocationDriver.
pub struct BinMXCAFDocLocationDriver {
    name: String,
}

impl BinMXCAFDocLocationDriver {
    pub fn new() -> Self {
        BinMXCAFDocLocationDriver {
            name: "XCAFDoc_Location".to_string(),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.name
    }

    pub fn new_empty(&self) -> XcafLocationAttribute {
        XcafLocationAttribute::new_empty()
    }

    /// Mirrors Paste(read): Translate the location and Set it on the target
    /// attribute (the attribute is set even when translation fails, as in the
    /// .cxx which sets `aLoc` unconditionally and returns `aRes`).
    pub fn paste_read(
        &self,
        source: &mut LocPersistentStream,
        target: &mut XcafLocationAttribute,
        reloc: &mut LocRRelocTable,
    ) -> bool {
        let mut loc = LocLocation::identity();
        let res = self.translate_read(source, &mut loc, reloc);
        target.set(loc);
        res
    }

    /// Mirrors Paste(write).
    pub fn paste_write(
        &self,
        source: &XcafLocationAttribute,
        target: &mut LocPersistentStream,
        reloc: &mut LocWRelocTable,
    ) {
        self.translate_write(source.get(), target, reloc);
    }

    /// Mirrors `Translate(theSource, theLoc, theMap)` from the .cxx for the
    /// pre-VERSION_6 layout (inline datums, relocation via datum ids).
    pub fn translate_read(
        &self,
        source: &mut LocPersistentStream,
        the_loc: &mut LocLocation,
        the_map: &mut LocRRelocTable,
    ) -> bool {
        let an_id = match source.get_integer() {
            Some(v) => v,
            None => return false,
        };
        if an_id == 0 {
            // identity, end of the location chain
            *the_loc = LocLocation::identity();
            return true;
        }

        // `if (aFileVer >= VERSION_6 && myNSDriver.IsNull()) return false;`
        // — this local port has no named-shape driver.
        let file_ver = the_map.storage_version();
        if file_ver >= LOC_FORMAT_VERSION_6 {
            return false;
        }

        let a_power = match source.get_integer() {
            Some(v) => v,
            None => return false,
        };
        let a_read_datum = match source.get_integer() {
            Some(v) => v,
            None => return false,
        };
        let a_datum_id = match source.get_integer() {
            Some(v) => v,
            None => return false,
        };

        let a_datum: Rc<LocDatum3D>;
        if a_read_datum != -1 {
            if the_map.is_bound(a_datum_id) {
                a_datum = the_map.find(a_datum_id);
            } else {
                return false;
            }
        } else {
            // read the datum's transformation
            let scale_factor = match source.get_real() {
                Some(v) => v,
                None => return false,
            };
            let form = match source.get_integer() {
                Some(v) => v,
                None => return false,
            };
            let mut matrix = [[0.0f64; 3]; 3];
            for row in matrix.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = match source.get_real() {
                        Some(v) => v,
                        None => return false,
                    };
                }
            }
            let x = match source.get_real() {
                Some(v) => v,
                None => return false,
            };
            let y = match source.get_real() {
                Some(v) => v,
                None => return false,
            };
            let z = match source.get_real() {
                Some(v) => v,
                None => return false,
            };
            let datum = Rc::new(LocDatum3D {
                scale_factor,
                trsf_form: form,
                matrix,
                translation: [x, y, z],
            });
            the_map.bind(a_datum_id, Rc::clone(&datum));
            a_datum = datum;
        }

        // Get Next Location
        let mut a_next_loc = LocLocation::identity();
        if !self.translate_read(source, &mut a_next_loc, the_map) {
            return false;
        }

        // Calculate the result:
        // theLoc = aNextLoc * TopLoc_Location(aDatum).Powered(aPower)
        *the_loc = a_next_loc.multiplied_by_powered(a_datum, a_power);
        true
    }

    /// Mirrors `Translate(theLoc, theTarget, theMap)` in the pre-VERSION_6
    /// layout: first item is written with its datum (inline on first
    /// occurrence, by relocation id afterwards), then recursion on
    /// NextLocation; identity terminates with a single 0.
    pub fn translate_write(
        &self,
        the_loc: &LocLocation,
        target: &mut LocPersistentStream,
        the_map: &mut LocWRelocTable,
    ) {
        if the_loc.is_identity() {
            target.put_integer(0);
            return;
        }

        let first = &the_loc.items[0];
        let (datum_id, was_bound) = the_map.add(&first.datum);

        // non-zero chain marker (the reader only tests it against 0)
        target.put_integer(datum_id);
        target.put_integer(first.power);
        if was_bound {
            // datum already stored: reference it (readDatum != -1)
            target.put_integer(datum_id);
            target.put_integer(datum_id);
        } else {
            // first occurrence: inline the transformation (readDatum == -1)
            target.put_integer(-1);
            target.put_integer(datum_id);
            target.put_real(first.datum.scale_factor);
            target.put_integer(first.datum.trsf_form);
            for row in &first.datum.matrix {
                for &cell in row {
                    target.put_real(cell);
                }
            }
            target.put_real(first.datum.translation[0]);
            target.put_real(first.datum.translation[1]);
            target.put_real(first.datum.translation[2]);
        }

        // recursion on NextLocation (tail of the item list)
        let next = LocLocation {
            items: the_loc.items[1..].to_vec(),
        };
        self.translate_write(&next, target, the_map);
    }
}

impl Default for BinMXCAFDocLocationDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRE_V6: i32 = 5;

    fn rot_z_datum() -> LocDatum3D {
        // 90 degree rotation about Z (gp_Rotation = 1).
        LocDatum3D {
            scale_factor: 1.0,
            trsf_form: 1,
            matrix: [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
            translation: [0.0, 0.0, 0.0],
        }
    }

    #[test]
    fn identity_location_is_single_zero() {
        let driver = BinMXCAFDocLocationDriver::new();
        let mut att = driver.new_empty();
        att.set(LocLocation::identity());

        let mut wmap = LocWRelocTable::new();
        let mut stream = LocPersistentStream::new();
        driver.paste_write(&att, &mut stream, &mut wmap);
        assert_eq!(stream.bytes(), &[0, 0, 0, 0]);

        let mut back = LocPersistentStream::from_bytes(stream.bytes());
        let mut out = driver.new_empty();
        let mut rmap = LocRRelocTable::new(PRE_V6);
        assert!(driver.paste_read(&mut back, &mut out, &mut rmap));
        assert!(out.get().is_identity());
    }

    #[test]
    fn roundtrip_single_datum() {
        let driver = BinMXCAFDocLocationDriver::new();
        let datum = Rc::new(LocDatum3D::translation_xyz(10.0, -2.5, 3.25));
        let loc = LocLocation::identity().multiplied_by_powered(datum, 1);

        let mut att = driver.new_empty();
        att.set(loc.clone());

        let mut wmap = LocWRelocTable::new();
        let mut stream = LocPersistentStream::new();
        driver.paste_write(&att, &mut stream, &mut wmap);

        let mut back = LocPersistentStream::from_bytes(stream.bytes());
        let mut out = driver.new_empty();
        let mut rmap = LocRRelocTable::new(PRE_V6);
        assert!(driver.paste_read(&mut back, &mut out, &mut rmap));
        assert!(out.get().same_as(&loc));
        assert_eq!(out.get().items[0].datum.translation, [10.0, -2.5, 3.25]);
        assert_eq!(out.get().items[0].datum.trsf_form, 2);
    }

    #[test]
    fn roundtrip_chain_with_shared_datum() {
        // Chain: rot^2 * trans^1 * rot^-1 where both rot items share ONE
        // TopLoc_Datum3D handle. The relocation tables must keep the sharing.
        let driver = BinMXCAFDocLocationDriver::new();
        let rot = Rc::new(rot_z_datum());
        let trans = Rc::new(LocDatum3D::translation_xyz(1.0, 2.0, 3.0));
        let loc = LocLocation {
            items: vec![
                LocItem {
                    datum: Rc::clone(&rot),
                    power: 2,
                },
                LocItem {
                    datum: Rc::clone(&trans),
                    power: 1,
                },
                LocItem {
                    datum: Rc::clone(&rot),
                    power: -1,
                },
            ],
        };

        let mut att = driver.new_empty();
        att.set(loc.clone());

        let mut wmap = LocWRelocTable::new();
        let mut stream = LocPersistentStream::new();
        driver.paste_write(&att, &mut stream, &mut wmap);

        let mut back = LocPersistentStream::from_bytes(stream.bytes());
        let mut out = driver.new_empty();
        let mut rmap = LocRRelocTable::new(PRE_V6);
        assert!(driver.paste_read(&mut back, &mut out, &mut rmap));

        assert!(out.get().same_as(&loc));
        // Sharing is restored: items 0 and 2 point to the SAME datum handle.
        assert!(Rc::ptr_eq(
            &out.get().items[0].datum,
            &out.get().items[2].datum
        ));
        assert!(!Rc::ptr_eq(
            &out.get().items[0].datum,
            &out.get().items[1].datum
        ));
        assert_eq!(out.get().items[0].power, 2);
        assert_eq!(out.get().items[1].power, 1);
        assert_eq!(out.get().items[2].power, -1);
    }

    #[test]
    fn version6_stream_without_ns_driver_fails() {
        // `aFileVer >= VERSION_6 && myNSDriver.IsNull()` => Paste returns false.
        let driver = BinMXCAFDocLocationDriver::new();
        let datum = Rc::new(LocDatum3D::identity());
        let loc = LocLocation::identity().multiplied_by_powered(datum, 1);
        let mut att = driver.new_empty();
        att.set(loc);

        let mut wmap = LocWRelocTable::new();
        let mut stream = LocPersistentStream::new();
        driver.paste_write(&att, &mut stream, &mut wmap);

        let mut back = LocPersistentStream::from_bytes(stream.bytes());
        let mut out = driver.new_empty();
        let mut rmap = LocRRelocTable::new(LOC_FORMAT_VERSION_6);
        assert!(!driver.paste_read(&mut back, &mut out, &mut rmap));
    }

    #[test]
    fn unbound_datum_reference_fails() {
        // readDatum != -1 with an id never bound must fail like the .cxx.
        let driver = BinMXCAFDocLocationDriver::new();
        let mut stream = LocPersistentStream::new();
        stream.put_integer(7); // anId != 0
        stream.put_integer(1); // power
        stream.put_integer(42); // readDatum != -1
        stream.put_integer(42); // datumID that was never bound

        let mut back = LocPersistentStream::from_bytes(stream.bytes());
        let mut out = driver.new_empty();
        let mut rmap = LocRRelocTable::new(PRE_V6);
        assert!(!driver.paste_read(&mut back, &mut out, &mut rmap));
    }

    #[test]
    fn truncated_stream_fails() {
        let driver = BinMXCAFDocLocationDriver::new();
        let datum = Rc::new(LocDatum3D::translation_xyz(4.0, 5.0, 6.0));
        let loc = LocLocation::identity().multiplied_by_powered(datum, 1);
        let mut att = driver.new_empty();
        att.set(loc);

        let mut wmap = LocWRelocTable::new();
        let mut stream = LocPersistentStream::new();
        driver.paste_write(&att, &mut stream, &mut wmap);

        let cut = &stream.bytes()[..stream.bytes().len() - 12];
        let mut back = LocPersistentStream::from_bytes(cut);
        let mut out = driver.new_empty();
        let mut rmap = LocRRelocTable::new(PRE_V6);
        assert!(!driver.paste_read(&mut back, &mut out, &mut rmap));
    }

    #[test]
    fn driver_metadata() {
        assert_eq!(
            BinMXCAFDocLocationDriver::new().type_name(),
            "XCAFDoc_Location"
        );
    }
}

// FILE: bin_t_obj_drivers.rs
// occt: BinTObjDrivers
//
// Faithful port of OCCT BinTObjDrivers (BinTObjDrivers.cxx/.hxx), the
// package class registering storage/retrieval drivers for TObj binary
// persistence:
//   - Factory(guid): returns the document storage or retrieval driver for the
//     plugin GUIDs "f78ff4a2-a779-11d5-aab4-0050044b1af1" (storage) and
//     "f78ff4a3-a779-11d5-aab4-0050044b1af1" (retrieval), falling back to
//     BinLDrivers::Factory for other GUIDs (BinL plugin GUIDs
//     13a56835/13a56836-8269-11d5-aab2-0050044b1af1).
//   - DefineFormat(app): defines format "TObjBin" / "Binary TObj OCAF
//     Document" / extension "cbf" with its retrieval + storage drivers.
//   - AddDrivers(table, messenger): registers the five TObj attribute
//     drivers (Model, Object, Reference, XYZ, IntSparseArray).
//
// The OCAF application/driver-table plumbing is modelled by small local
// types; GUID identity and registration behaviour are real and tested.

/// Local model of Standard_GUID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TObjGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl TObjGuid {
    /// Parses the canonical "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx" form used
    /// by the Standard_GUID string constructor.
    pub fn from_string(s: &str) -> Option<TObjGuid> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 5
            || parts[0].len() != 8
            || parts[1].len() != 4
            || parts[2].len() != 4
            || parts[3].len() != 4
            || parts[4].len() != 12
        {
            return None;
        }
        let data1 = u32::from_str_radix(parts[0], 16).ok()?;
        let data2 = u16::from_str_radix(parts[1], 16).ok()?;
        let data3 = u16::from_str_radix(parts[2], 16).ok()?;
        let mut data4 = [0u8; 8];
        let tail: String = format!("{}{}", parts[3], parts[4]);
        for (i, chunk) in tail.as_bytes().chunks(2).enumerate() {
            let hex = std::str::from_utf8(chunk).ok()?;
            data4[i] = u8::from_str_radix(hex, 16).ok()?;
        }
        Some(TObjGuid {
            data1,
            data2,
            data3,
            data4,
        })
    }
}

/// `static Standard_GUID BinStorageDriver("f78ff4a2-a779-11d5-aab4-0050044b1af1")`.
pub fn tobj_bin_storage_driver_guid() -> TObjGuid {
    TObjGuid::from_string("f78ff4a2-a779-11d5-aab4-0050044b1af1").unwrap()
}

/// `static Standard_GUID BinRetrievalDriver("f78ff4a3-a779-11d5-aab4-0050044b1af1")`.
pub fn tobj_bin_retrieval_driver_guid() -> TObjGuid {
    TObjGuid::from_string("f78ff4a3-a779-11d5-aab4-0050044b1af1").unwrap()
}

/// BinLDrivers plugin GUIDs (from BinLDrivers.cxx), used by the fallback.
pub fn tobj_binl_storage_driver_guid() -> TObjGuid {
    TObjGuid::from_string("13a56835-8269-11d5-aab2-0050044b1af1").unwrap()
}

pub fn tobj_binl_retrieval_driver_guid() -> TObjGuid {
    TObjGuid::from_string("13a56836-8269-11d5-aab2-0050044b1af1").unwrap()
}

/// The transient driver instances the factories can hand out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TObjFactoryProduct {
    /// BinTObjDrivers_DocumentStorageDriver
    TObjDocumentStorageDriver,
    /// BinTObjDrivers_DocumentRetrievalDriver
    TObjDocumentRetrievalDriver,
    /// BinLDrivers_DocumentStorageDriver (via BinLDrivers::Factory fallback)
    BinLDocumentStorageDriver,
    /// BinLDrivers_DocumentRetrievalDriver (via BinLDrivers::Factory fallback)
    BinLDocumentRetrievalDriver,
}

/// Descriptor of one BinMDF_ADriver registered by AddDrivers. The TObj
/// attribute drivers are constructed with a null name, so the served
/// attribute type comes from the attribute created by NewEmpty().
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TObjAttributeDriverEntry {
    pub driver_class: &'static str,
    pub attribute_type: &'static str,
}

/// Local model of BinMDF_ADriverTable.
#[derive(Debug, Default)]
pub struct TObjDriverTable {
    drivers: Vec<TObjAttributeDriverEntry>,
}

impl TObjDriverTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors BinMDF_ADriverTable::AddDriver.
    pub fn add_driver(&mut self, entry: TObjAttributeDriverEntry) {
        self.drivers.push(entry);
    }

    pub fn drivers(&self) -> &[TObjAttributeDriverEntry] {
        &self.drivers
    }

    pub fn has_driver_for(&self, attribute_type: &str) -> bool {
        self.drivers
            .iter()
            .any(|d| d.attribute_type == attribute_type)
    }
}

/// One document format definition, as recorded by TDocStd_Application::DefineFormat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TObjFormatDefinition {
    pub format: String,
    pub description: String,
    pub extension: String,
    pub retrieval_driver: TObjFactoryProduct,
    pub storage_driver: TObjFactoryProduct,
}

/// Local model of TDocStd_Application (format registry part).
#[derive(Debug, Default)]
pub struct TObjDocApplication {
    formats: Vec<TObjFormatDefinition>,
}

impl TObjDocApplication {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors TDocStd_Application::DefineFormat.
    pub fn define_format(
        &mut self,
        format: &str,
        description: &str,
        extension: &str,
        retrieval_driver: TObjFactoryProduct,
        storage_driver: TObjFactoryProduct,
    ) {
        self.formats.push(TObjFormatDefinition {
            format: format.to_string(),
            description: description.to_string(),
            extension: extension.to_string(),
            retrieval_driver,
            storage_driver,
        });
    }

    pub fn formats(&self) -> &[TObjFormatDefinition] {
        &self.formats
    }
}

/// Local model of the messenger handle passed to AddDrivers.
#[derive(Debug, Default)]
pub struct TObjMessenger;

/// Port of the BinTObjDrivers package class.
pub struct BinTObjDrivers;

impl BinTObjDrivers {
    /// Mirrors BinTObjDrivers::Factory: TObjBin storage/retrieval plugin
    /// GUIDs are served directly; anything else is forwarded to
    /// BinLDrivers::Factory (modelled by `binl_factory`). None mirrors the
    /// fallback ultimately failing for an unknown GUID.
    pub fn factory(guid: &TObjGuid) -> Option<TObjFactoryProduct> {
        if *guid == tobj_bin_storage_driver_guid() {
            return Some(TObjFactoryProduct::TObjDocumentStorageDriver);
        }
        if *guid == tobj_bin_retrieval_driver_guid() {
            return Some(TObjFactoryProduct::TObjDocumentRetrievalDriver);
        }
        Self::binl_factory(guid)
    }

    /// Local model of the BinLDrivers::Factory fallback.
    fn binl_factory(guid: &TObjGuid) -> Option<TObjFactoryProduct> {
        if *guid == tobj_binl_storage_driver_guid() {
            return Some(TObjFactoryProduct::BinLDocumentStorageDriver);
        }
        if *guid == tobj_binl_retrieval_driver_guid() {
            return Some(TObjFactoryProduct::BinLDocumentRetrievalDriver);
        }
        None
    }

    /// Mirrors BinTObjDrivers::DefineFormat: defines format "TObjBin".
    pub fn define_format(app: &mut TObjDocApplication) {
        app.define_format(
            "TObjBin",
            "Binary TObj OCAF Document",
            "cbf",
            TObjFactoryProduct::TObjDocumentRetrievalDriver,
            TObjFactoryProduct::TObjDocumentStorageDriver,
        );
    }

    /// Mirrors BinTObjDrivers::AddDrivers: registers the five TObj attribute
    /// drivers, in the .cxx order.
    pub fn add_drivers(table: &mut TObjDriverTable, _msg_drv: &TObjMessenger) {
        table.add_driver(TObjAttributeDriverEntry {
            driver_class: "BinTObjDrivers_ModelDriver",
            attribute_type: "TObj_TModel",
        });
        table.add_driver(TObjAttributeDriverEntry {
            driver_class: "BinTObjDrivers_ObjectDriver",
            attribute_type: "TObj_TObject",
        });
        table.add_driver(TObjAttributeDriverEntry {
            driver_class: "BinTObjDrivers_ReferenceDriver",
            attribute_type: "TObj_TReference",
        });
        table.add_driver(TObjAttributeDriverEntry {
            driver_class: "BinTObjDrivers_XYZDriver",
            attribute_type: "TObj_TXYZ",
        });
        table.add_driver(TObjAttributeDriverEntry {
            driver_class: "BinTObjDrivers_IntSparseArrayDriver",
            attribute_type: "TObj_TIntSparseArray",
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_parsing_matches_fields() {
        let g = tobj_bin_storage_driver_guid();
        assert_eq!(g.data1, 0xf78ff4a2);
        assert_eq!(g.data2, 0xa779);
        assert_eq!(g.data3, 0x11d5);
        assert_eq!(g.data4, [0xaa, 0xb4, 0x00, 0x50, 0x04, 0x4b, 0x1a, 0xf1]);
    }

    #[test]
    fn factory_serves_tobj_plugin_guids() {
        assert_eq!(
            BinTObjDrivers::factory(&tobj_bin_storage_driver_guid()),
            Some(TObjFactoryProduct::TObjDocumentStorageDriver)
        );
        assert_eq!(
            BinTObjDrivers::factory(&tobj_bin_retrieval_driver_guid()),
            Some(TObjFactoryProduct::TObjDocumentRetrievalDriver)
        );
    }

    #[test]
    fn factory_falls_back_to_binl() {
        assert_eq!(
            BinTObjDrivers::factory(&tobj_binl_storage_driver_guid()),
            Some(TObjFactoryProduct::BinLDocumentStorageDriver)
        );
        assert_eq!(
            BinTObjDrivers::factory(&tobj_binl_retrieval_driver_guid()),
            Some(TObjFactoryProduct::BinLDocumentRetrievalDriver)
        );
        let unknown = TObjGuid::from_string("00000000-0000-0000-0000-000000000000").unwrap();
        assert_eq!(BinTObjDrivers::factory(&unknown), None);
    }

    #[test]
    fn define_format_registers_tobjbin() {
        let mut app = TObjDocApplication::new();
        BinTObjDrivers::define_format(&mut app);
        assert_eq!(app.formats().len(), 1);
        let f = &app.formats()[0];
        assert_eq!(f.format, "TObjBin");
        assert_eq!(f.description, "Binary TObj OCAF Document");
        assert_eq!(f.extension, "cbf");
        assert_eq!(
            f.retrieval_driver,
            TObjFactoryProduct::TObjDocumentRetrievalDriver
        );
        assert_eq!(
            f.storage_driver,
            TObjFactoryProduct::TObjDocumentStorageDriver
        );
    }

    #[test]
    fn add_drivers_registers_all_five_in_order() {
        let mut table = TObjDriverTable::new();
        BinTObjDrivers::add_drivers(&mut table, &TObjMessenger);
        let types: Vec<&str> = table.drivers().iter().map(|d| d.attribute_type).collect();
        assert_eq!(
            types,
            vec![
                "TObj_TModel",
                "TObj_TObject",
                "TObj_TReference",
                "TObj_TXYZ",
                "TObj_TIntSparseArray"
            ]
        );
        assert!(table.has_driver_for("TObj_TXYZ"));
        assert!(!table.has_driver_for("TObj_TNameContainer"));
    }

    #[test]
    fn malformed_guid_strings_rejected() {
        assert!(TObjGuid::from_string("not-a-guid").is_none());
        assert!(TObjGuid::from_string("f78ff4a2a77911d5aab40050044b1af1").is_none());
        assert!(TObjGuid::from_string("f78ff4a2-a779-11d5-aab4-0050044b1aZZ").is_none());
    }
}

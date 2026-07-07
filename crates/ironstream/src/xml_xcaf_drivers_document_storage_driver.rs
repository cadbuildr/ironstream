// FILE: xml_xcaf_drivers_document_storage_driver.rs
// occt: XmlXCAFDrivers_DocumentStorageDriver
//
// Port of OCCT XmlXCAFDrivers_DocumentStorageDriver
// (DataExchange/TKXmlXCAF/XmlXCAFDrivers). The XCAF storage driver
// extends the plain XML document storage driver by
//  1. registering the "xcaf" XML namespace in its constructor, and
//  2. extending the attribute-driver table with the native XCAF
//     drivers (XmlMXCAFDoc::AddDrivers) on top of the standard ones.
// The base driver, driver table and message plumbing are modeled
// locally.

/// XCAF namespace registered by the constructor.
pub const XCAF_NS_PREFIX: &str = "xcaf";
pub const XCAF_NS_URI: &str = "http://www.opencascade.org/OCAF/XML/XCAF";

/// Local model of XmlMDF_ADriverTable: attribute drivers keyed by the
/// attribute type name they serve (AddDriver replaces an existing
/// driver for the same type).
#[derive(Debug, Default)]
pub struct XmlMdfADriverTable {
    drivers: Vec<String>,
}

impl XmlMdfADriverTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// XmlMDF_ADriverTable::AddDriver.
    pub fn add_driver(&mut self, type_name: &str) {
        if !self.drivers.iter().any(|d| d == type_name) {
            self.drivers.push(type_name.to_string());
        }
    }

    pub fn has_driver(&self, type_name: &str) -> bool {
        self.drivers.iter().any(|d| d == type_name)
    }

    pub fn len(&self) -> usize {
        self.drivers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.drivers.is_empty()
    }
}

/// Local model of XmlDrivers::AttributeDrivers: the standard OCAF
/// attribute drivers (representative subset of the full table).
pub fn standard_attribute_drivers() -> XmlMdfADriverTable {
    let mut table = XmlMdfADriverTable::new();
    for name in [
        "TDF_Reference",
        "TDF_TagSource",
        "TDataStd_Integer",
        "TDataStd_Real",
        "TDataStd_Name",
        "TDataStd_Comment",
        "TDataStd_UAttribute",
        "TDataStd_IntegerArray",
        "TDataStd_RealArray",
        "TDataStd_RealList",
        "TDataXtd_Point",
        "TDataXtd_Plane",
        "TNaming_NamedShape",
        "TDocStd_Owner",
    ] {
        table.add_driver(name);
    }
    table
}

/// Local model of XmlMXCAFDoc::AddDrivers: adds the native XCAF
/// attribute drivers (list from XmlMXCAFDoc.cxx).
pub fn add_xcaf_drivers(table: &mut XmlMdfADriverTable) {
    for name in [
        "XCAFDoc_Centroid",
        "XCAFDoc_Color",
        "XCAFDoc_GraphNode",
        "XCAFDoc_Location",
        "XCAFDoc_LengthUnit",
        "XCAFDoc_AssemblyItemRef",
        "XCAFDoc_Datum",
        "XCAFDoc_DimTol",
        "XCAFDoc_Material",
        "XCAFDoc_VisMaterial",
        "XCAFDoc_NoteComment",
        "XCAFDoc_NoteBinData",
        "XCAFDoc_VisMaterialTool",
    ] {
        table.add_driver(name);
    }
}

/// Local model of the XmlDrivers_DocumentStorageDriver base class:
/// copyright string plus registered XML namespaces.
#[derive(Debug)]
pub struct XmlDriversDocumentStorageDriver {
    copyright: String,
    namespaces: Vec<(String, String)>,
}

impl XmlDriversDocumentStorageDriver {
    pub fn new(copyright: &str) -> Self {
        XmlDriversDocumentStorageDriver {
            copyright: copyright.to_string(),
            namespaces: Vec::new(),
        }
    }

    pub fn copyright(&self) -> &str {
        &self.copyright
    }

    /// XmlLDrivers_DocumentStorageDriver::AddNamespace.
    pub fn add_namespace(&mut self, prefix: &str, uri: &str) {
        self.namespaces
            .push((prefix.to_string(), uri.to_string()));
    }

    pub fn namespaces(&self) -> &[(String, String)] {
        &self.namespaces
    }

    /// Base AttributeDrivers: the standard driver table.
    pub fn attribute_drivers(&self) -> XmlMdfADriverTable {
        standard_attribute_drivers()
    }
}

/// XmlXCAFDrivers_DocumentStorageDriver: storage driver of an XCAF
/// (XS) document.
#[derive(Debug)]
pub struct XmlXCAFDriversDocumentStorageDriver {
    base: XmlDriversDocumentStorageDriver,
}

impl XmlXCAFDriversDocumentStorageDriver {
    /// OCCT ctor: forwards the copyright to the base driver and
    /// registers the "xcaf" namespace.
    pub fn new(copyright: &str) -> Self {
        let mut base = XmlDriversDocumentStorageDriver::new(copyright);
        base.add_namespace(XCAF_NS_PREFIX, XCAF_NS_URI);
        XmlXCAFDriversDocumentStorageDriver { base }
    }

    pub fn copyright(&self) -> &str {
        self.base.copyright()
    }

    pub fn namespaces(&self) -> &[(String, String)] {
        self.base.namespaces()
    }

    /// OCCT AttributeDrivers override: standard drivers plus the
    /// native XCAF drivers.
    pub fn attribute_drivers(&self) -> XmlMdfADriverTable {
        // Standard drivers
        let mut table = self.base.attribute_drivers();
        // Native drivers
        add_xcaf_drivers(&mut table);
        table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctor_stores_copyright() {
        let d = XmlXCAFDriversDocumentStorageDriver::new("Copyright: Open Cascade 2026");
        assert_eq!(d.copyright(), "Copyright: Open Cascade 2026");
    }

    #[test]
    fn test_ctor_registers_xcaf_namespace() {
        let d = XmlXCAFDriversDocumentStorageDriver::new("c");
        assert_eq!(
            d.namespaces(),
            &[(
                "xcaf".to_string(),
                "http://www.opencascade.org/OCAF/XML/XCAF".to_string()
            )]
        );
    }

    #[test]
    fn test_attribute_drivers_contains_standard_drivers() {
        let d = XmlXCAFDriversDocumentStorageDriver::new("c");
        let table = d.attribute_drivers();
        assert!(table.has_driver("TDataStd_Integer"));
        assert!(table.has_driver("TDF_Reference"));
        assert!(table.has_driver("TNaming_NamedShape"));
    }

    #[test]
    fn test_attribute_drivers_contains_xcaf_drivers() {
        let d = XmlXCAFDriversDocumentStorageDriver::new("c");
        let table = d.attribute_drivers();
        for name in [
            "XCAFDoc_Centroid",
            "XCAFDoc_Color",
            "XCAFDoc_GraphNode",
            "XCAFDoc_Location",
            "XCAFDoc_LengthUnit",
            "XCAFDoc_AssemblyItemRef",
            "XCAFDoc_Datum",
            "XCAFDoc_DimTol",
            "XCAFDoc_Material",
            "XCAFDoc_VisMaterial",
            "XCAFDoc_NoteComment",
            "XCAFDoc_NoteBinData",
            "XCAFDoc_VisMaterialTool",
        ] {
            assert!(table.has_driver(name), "missing driver {}", name);
        }
    }

    #[test]
    fn test_xcaf_table_is_superset_of_base_table() {
        let d = XmlXCAFDriversDocumentStorageDriver::new("c");
        let base_table = standard_attribute_drivers();
        let xcaf_table = d.attribute_drivers();
        assert!(xcaf_table.len() > base_table.len());
        assert_eq!(xcaf_table.len(), base_table.len() + 13);
    }

    #[test]
    fn test_base_driver_has_no_xcaf_drivers() {
        let base = XmlDriversDocumentStorageDriver::new("c");
        let table = base.attribute_drivers();
        assert!(!table.has_driver("XCAFDoc_Color"));
        assert!(table.has_driver("TDataStd_Name"));
        // Base driver registers no namespaces by itself.
        assert!(base.namespaces().is_empty());
    }

    #[test]
    fn test_add_driver_is_idempotent() {
        let mut table = XmlMdfADriverTable::new();
        table.add_driver("X");
        table.add_driver("X");
        assert_eq!(table.len(), 1);
        assert!(table.has_driver("X"));
        assert!(!table.is_empty());
    }
}

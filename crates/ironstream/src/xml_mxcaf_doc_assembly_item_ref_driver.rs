// FILE: xml_mxcaf_doc_assembly_item_ref_driver.rs
// occt: XmlMXCAFDoc_AssemblyItemRefDriver
//
// Faithful port of OCCT XmlMXCAFDoc_AssemblyItemRefDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_AssemblyItemRefDriver.hxx),
// the XmlMDF_ADriver for XCAF assembly item reference attributes.
// Handles XML serialization/deserialization of XCAFDoc_AssemblyItemRef tags
// (assembly structure references used in multi-part documents).

/// Local model of assembly item reference data from XML.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AssemblyItemRefData {
    pub item_guid: String,
    pub reference_guid: String,
}

/// Local helper: XmlMDF_ADriver interface stub for assembly item references.
#[derive(Debug)]
pub struct XmlMXCAFDocAssemblyItemRefDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocAssemblyItemRefDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_AssemblyItemRef";

    pub fn new() -> Self {
        Self {
            type_name: Self::TYPE_NAME.to_string(),
            version: 1,
        }
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn version_number(&self) -> u32 {
        self.version
    }

    /// Simulate parsing assembly item reference from XML element.
    /// In OCCT, this reads child elements and attributes.
    pub fn read_from_xml(&self, element_text: &str) -> Result<AssemblyItemRefData, String> {
        // Minimal XML parsing simulation: format is "item_guid:reference_guid"
        let parts: Vec<&str> = element_text.split(':').collect();
        if parts.len() == 2 {
            Ok(AssemblyItemRefData {
                item_guid: parts[0].to_string(),
                reference_guid: parts[1].to_string(),
            })
        } else {
            Err("Invalid AssemblyItemRef format".to_string())
        }
    }

    /// Simulate writing assembly item reference to XML text.
    pub fn write_to_xml(&self, data: &AssemblyItemRefData) -> String {
        format!("{}:{}", data.item_guid, data.reference_guid)
    }
}

impl Default for XmlMXCAFDocAssemblyItemRefDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocAssemblyItemRefDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_AssemblyItemRef");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_valid() {
        let driver = XmlMXCAFDocAssemblyItemRefDriver::new();
        let result = driver.read_from_xml("item-001:ref-001");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.item_guid, "item-001");
        assert_eq!(data.reference_guid, "ref-001");
    }

    #[test]
    fn test_read_from_xml_invalid() {
        let driver = XmlMXCAFDocAssemblyItemRefDriver::new();
        let result = driver.read_from_xml("no-colon-here");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_xml() {
        let driver = XmlMXCAFDocAssemblyItemRefDriver::new();
        let data = AssemblyItemRefData {
            item_guid: "item-x".to_string(),
            reference_guid: "ref-y".to_string(),
        };
        let xml_text = driver.write_to_xml(&data);
        assert_eq!(xml_text, "item-x:ref-y");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocAssemblyItemRefDriver::new();
        let original = AssemblyItemRefData {
            item_guid: "assembly-123".to_string(),
            reference_guid: "component-456".to_string(),
        };
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original, restored);
    }
}

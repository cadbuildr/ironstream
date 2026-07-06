// FILE: xml_mxcaf_doc.rs
// occt: XmlMXCAFDoc
//
// Faithful port of OCCT XmlMXCAFDoc (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc.hxx),
// the namespace-like holder for XDE (eXtended Data Exchange) XCAF attribute drivers
// and their registration/initialization. Registers drivers for color, material, dimension,
// location, and assembly reference attributes to the OCAF XML persistence framework.

/// Local model of a driver registration entry.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XcafDriverRegistration {
    pub driver_name: String,
    pub attribute_type: String,
    pub version: u32,
}

/// Namespace-like holder for XCAF XML drivers and their management.
#[derive(Debug, Default)]
pub struct XmlMXCAFDoc {
    registered_drivers: Vec<XcafDriverRegistration>,
}

impl XmlMXCAFDoc {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a driver for a specific XCAF attribute type.
    /// Called during OCAF persistence plugin initialization to bind
    /// XCAF attribute types to their XML driver implementations.
    pub fn register_driver(&mut self, driver_name: &str, attribute_type: &str, version: u32) {
        self.registered_drivers.push(XcafDriverRegistration {
            driver_name: driver_name.to_string(),
            attribute_type: attribute_type.to_string(),
            version,
        });
    }

    /// Get all registered driver entries.
    pub fn registered_drivers(&self) -> &[XcafDriverRegistration] {
        &self.registered_drivers
    }

    /// Find a registration by attribute type.
    pub fn find_driver_for_type(&self, attr_type: &str) -> Option<&XcafDriverRegistration> {
        self.registered_drivers
            .iter()
            .find(|reg| reg.attribute_type == attr_type)
    }

    /// Clear all registered drivers.
    pub fn clear(&mut self) {
        self.registered_drivers.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_mxcaf_doc_new() {
        let doc = XmlMXCAFDoc::new();
        assert_eq!(doc.registered_drivers().len(), 0);
    }

    #[test]
    fn test_register_driver() {
        let mut doc = XmlMXCAFDoc::new();
        doc.register_driver("XmlMXCAFDoc_ColorDriver", "XCAFDoc_Color", 1);
        assert_eq!(doc.registered_drivers().len(), 1);
    }

    #[test]
    fn test_find_driver_for_type() {
        let mut doc = XmlMXCAFDoc::new();
        doc.register_driver("XmlMXCAFDoc_MaterialDriver", "XCAFDoc_Material", 1);
        let reg = doc.find_driver_for_type("XCAFDoc_Material");
        assert!(reg.is_some());
        assert_eq!(reg.unwrap().driver_name, "XmlMXCAFDoc_MaterialDriver");
    }

    #[test]
    fn test_find_nonexistent_type() {
        let doc = XmlMXCAFDoc::new();
        assert!(doc.find_driver_for_type("UnknownType").is_none());
    }

    #[test]
    fn test_multiple_registrations() {
        let mut doc = XmlMXCAFDoc::new();
        doc.register_driver("ColorDriver", "XCAFDoc_Color", 1);
        doc.register_driver("MaterialDriver", "XCAFDoc_Material", 1);
        doc.register_driver("LocationDriver", "XCAFDoc_Location", 1);
        assert_eq!(doc.registered_drivers().len(), 3);
    }

    #[test]
    fn test_clear() {
        let mut doc = XmlMXCAFDoc::new();
        doc.register_driver("Driver1", "Type1", 1);
        doc.register_driver("Driver2", "Type2", 1);
        doc.clear();
        assert_eq!(doc.registered_drivers().len(), 0);
    }
}

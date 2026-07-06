// FILE: xml_mxcaf_doc_vis_material_tool_driver.rs
// occt: XmlMXCAFDoc_VisMaterialToolDriver
//
// Port of OCCT XmlMXCAFDoc_VisMaterialToolDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_VisMaterialToolDriver.cxx).
// XCAFDoc_VisMaterialTool is a marker/tool attribute without own data,
// so the driver's read Paste always succeeds without touching the target
// and the write Paste leaves the XML element untouched.

use std::collections::HashMap;

/// Local model of an XmlObjMgt_Element: XML attributes by name.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XmlElement {
    attributes: HashMap<String, String>,
}

impl XmlElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    pub fn set_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), value.to_string());
    }

    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }
}

/// Local model of the transient XCAFDoc_VisMaterialTool attribute.
/// The tool itself carries no persistent data (materials live in child
/// labels handled by other drivers).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XCAFDocVisMaterialTool;

impl XCAFDocVisMaterialTool {
    pub fn new() -> Self {
        Self
    }
}

/// XmlMDF_ADriver for XCAFDoc_VisMaterialTool.
#[derive(Debug)]
pub struct XmlMXCAFDocVisMaterialToolDriver {
    namespace: String,
    type_name: String,
}

impl XmlMXCAFDocVisMaterialToolDriver {
    /// OCCT ctor: XmlMDF_ADriver(msgDriver, "xcaf", "VisMaterialTool").
    pub fn new() -> Self {
        XmlMXCAFDocVisMaterialToolDriver {
            namespace: "xcaf".to_string(),
            type_name: "VisMaterialTool".to_string(),
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// OCCT NewEmpty: creates a new XCAFDoc_VisMaterialTool.
    pub fn new_empty(&self) -> XCAFDocVisMaterialTool {
        XCAFDocVisMaterialTool::new()
    }

    /// OCCT Paste (persistent -> transient): the tool has no data,
    /// always succeeds and leaves the target unchanged.
    pub fn paste_from_xml(
        &self,
        _source: &XmlElement,
        _target: &mut XCAFDocVisMaterialTool,
    ) -> bool {
        true
    }

    /// OCCT Paste (transient -> persistent): writes nothing.
    pub fn paste_to_xml(&self, _source: &XCAFDocVisMaterialTool, _target: &mut XmlElement) {}
}

impl Default for XmlMXCAFDocVisMaterialToolDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_names() {
        let driver = XmlMXCAFDocVisMaterialToolDriver::new();
        assert_eq!(driver.namespace(), "xcaf");
        assert_eq!(driver.type_name(), "VisMaterialTool");
    }

    #[test]
    fn test_new_empty() {
        let driver = XmlMXCAFDocVisMaterialToolDriver::new();
        assert_eq!(driver.new_empty(), XCAFDocVisMaterialTool::new());
    }

    #[test]
    fn test_paste_from_xml_always_succeeds() {
        let driver = XmlMXCAFDocVisMaterialToolDriver::new();
        let mut tool = XCAFDocVisMaterialTool::new();

        // Empty element.
        let el = XmlElement::new();
        assert!(driver.paste_from_xml(&el, &mut tool));

        // Element with unrelated attributes: still succeeds, tool unchanged.
        let mut el2 = XmlElement::new();
        el2.set_attribute("junk", "value");
        assert!(driver.paste_from_xml(&el2, &mut tool));
        assert_eq!(tool, XCAFDocVisMaterialTool::new());
    }

    #[test]
    fn test_paste_to_xml_writes_nothing() {
        let driver = XmlMXCAFDocVisMaterialToolDriver::new();
        let tool = XCAFDocVisMaterialTool::new();
        let mut el = XmlElement::new();
        driver.paste_to_xml(&tool, &mut el);
        assert_eq!(el.attribute_count(), 0);

        // Pre-existing attributes are preserved untouched.
        let mut el2 = XmlElement::new();
        el2.set_attribute("existing", "1");
        driver.paste_to_xml(&tool, &mut el2);
        assert_eq!(el2.attribute_count(), 1);
        assert_eq!(el2.get_attribute("existing"), Some("1"));
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocVisMaterialToolDriver::new();
        let tool = driver.new_empty();

        let mut el = XmlElement::new();
        driver.paste_to_xml(&tool, &mut el);

        let mut restored = driver.new_empty();
        assert!(driver.paste_from_xml(&el, &mut restored));
        assert_eq!(restored, tool);
    }
}

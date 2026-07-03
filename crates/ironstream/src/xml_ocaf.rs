// FILE: xml_ocaf.rs
// occt: XmlMDF_ADriver, XmlMDF_ADriverTable, XmlOCAFDrivers_DocumentRetrievalDriver

/// Attribute type tag for XML OCAF serialization.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum XmlMdfAttrType {
    #[default]
    Integer,
    Real,
    AsciiString,
    ExtString,
    IntArray,
    RealArray,
    ByteArray,
    Name,
    Comment,
    Reference,
    Shape,
    Color,
    Material,
    Layer,
    Function,
    NoteComment,
    NoteBalloon,
}

impl XmlMdfAttrType {
    pub fn element_name(&self) -> &'static str {
        match self {
            Self::Integer     => "TDF_Integer",
            Self::Real        => "TDF_Real",
            Self::AsciiString => "TDF_AsciiString",
            Self::ExtString   => "TDF_ExtString",
            Self::IntArray    => "TDF_IntegerArray",
            Self::RealArray   => "TDF_RealArray",
            Self::ByteArray   => "TDF_ByteArray",
            Self::Name        => "TDataStd_Name",
            Self::Comment     => "TDataStd_Comment",
            Self::Reference   => "TDF_Reference",
            Self::Shape       => "TNaming_NamedShape",
            Self::Color       => "XCAFDoc_Color",
            Self::Material    => "XCAFDoc_Material",
            Self::Layer       => "XCAFDoc_Layer",
            Self::Function    => "TFunction_Function",
            Self::NoteComment => "XCAFNote_Comment",
            Self::NoteBalloon => "XCAFNote_Balloon",
        }
    }
}

/// One XML attribute record (element tag + key-value attributes).
#[derive(Clone, Debug, Default)]
pub struct XmlMdfRecord {
    pub attr_type: XmlMdfAttrType,
    pub label_entry: String,
    pub attributes: Vec<(String, String)>,
    pub text_content: Option<String>,
}

impl XmlMdfRecord {
    pub fn new(attr_type: XmlMdfAttrType, label_entry: impl Into<String>) -> Self {
        Self { attr_type, label_entry: label_entry.into(), attributes: Vec::new(), text_content: None }
    }

    pub fn set_attr(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let k = key.into();
        if let Some(e) = self.attributes.iter_mut().find(|(kk, _)| *kk == k) {
            e.1 = value.into();
        } else {
            self.attributes.push((k, value.into()));
        }
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attributes.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }

    pub fn set_text(&mut self, t: impl Into<String>) { self.text_content = Some(t.into()); }

    /// Render as a minimal XML element (no namespace handling).
    pub fn to_xml(&self) -> String {
        let tag = self.attr_type.element_name();
        let mut s = format!("<{}", tag);
        s.push_str(&format!(" label=\"{}\"", self.label_entry));
        for (k, v) in &self.attributes {
            s.push_str(&format!(" {}=\"{}\"", k, v));
        }
        if let Some(text) = &self.text_content {
            s.push('>');
            s.push_str(text);
            s.push_str(&format!("</{}>", tag));
        } else {
            s.push_str("/>");
        }
        s
    }
}

/// Driver table mapping attr_type → driver name.
/// occt: XmlMDF_ADriverTable
#[derive(Clone, Debug, Default)]
pub struct XmlMdfADriverTable {
    pub drivers: Vec<(XmlMdfAttrType, String)>,
}

impl XmlMdfADriverTable {
    pub fn new() -> Self { Self::default() }

    pub fn add_driver(&mut self, t: XmlMdfAttrType, name: impl Into<String>) {
        if !self.drivers.iter().any(|(tt, _)| *tt == t) {
            self.drivers.push((t, name.into()));
        }
    }

    pub fn find_by_element_name(&self, elem: &str) -> Option<XmlMdfAttrType> {
        self.drivers.iter()
            .find(|(t, _)| t.element_name() == elem)
            .map(|(t, _)| *t)
    }

    pub fn find_driver_name(&self, t: XmlMdfAttrType) -> Option<&str> {
        self.drivers.iter().find(|(tt, _)| *tt == t).map(|(_, n)| n.as_str())
    }

    pub fn nb_drivers(&self) -> usize { self.drivers.len() }
}

/// XML OCAF document stub.
/// occt: XmlOCAFDrivers (simplified)
#[derive(Clone, Debug, Default)]
pub struct XmlOcafDocument {
    pub version: u32,
    pub driver_table: XmlMdfADriverTable,
    pub records: Vec<XmlMdfRecord>,
    pub is_stored: bool,
}

impl XmlOcafDocument {
    pub fn new() -> Self { Self { version: 1, ..Self::default() } }

    pub fn add_record(&mut self, rec: XmlMdfRecord) { self.records.push(rec); }
    pub fn nb_records(&self) -> usize { self.records.len() }

    /// Produce a minimal XML document string (stub).
    pub fn to_xml_string(&self) -> String {
        let mut out = format!("<?xml version=\"1.0\"?>\n<Document format_version=\"{}\">\n", self.version);
        for rec in &self.records {
            out.push_str("  ");
            out.push_str(&rec.to_xml());
            out.push('\n');
        }
        out.push_str("</Document>");
        self.clone(); // mark intent
        out
    }

    pub fn store(&mut self) -> String {
        let xml = self.to_xml_string();
        self.is_stored = true;
        xml
    }

    /// Parse the header of an XML document stub (reads format_version attribute).
    pub fn retrieve_version(xml: &str) -> Option<u32> {
        let key = "format_version=\"";
        let start = xml.find(key)? + key.len();
        let end = xml[start..].find('"')?;
        xml[start..start + end].parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attr_type_element_name() {
        assert_eq!(XmlMdfAttrType::Integer.element_name(), "TDF_Integer");
        assert_eq!(XmlMdfAttrType::Color.element_name(), "XCAFDoc_Color");
    }

    #[test]
    fn record_to_xml() {
        let mut r = XmlMdfRecord::new(XmlMdfAttrType::Integer, "0:1:1");
        r.set_attr("value", "42");
        let xml = r.to_xml();
        assert!(xml.contains("TDF_Integer"));
        assert!(xml.contains("label=\"0:1:1\""));
        assert!(xml.contains("value=\"42\""));
    }

    #[test]
    fn record_text_content() {
        let mut r = XmlMdfRecord::new(XmlMdfAttrType::AsciiString, "0:1:2");
        r.set_text("hello");
        let xml = r.to_xml();
        assert!(xml.contains(">hello<"));
    }

    #[test]
    fn driver_table_find() {
        let mut dt = XmlMdfADriverTable::new();
        dt.add_driver(XmlMdfAttrType::Shape, "ShapeDriver");
        dt.add_driver(XmlMdfAttrType::Shape, "Dup"); // ignored
        assert_eq!(dt.nb_drivers(), 1);
        assert_eq!(dt.find_driver_name(XmlMdfAttrType::Shape), Some("ShapeDriver"));
        let t = dt.find_by_element_name("TNaming_NamedShape");
        assert_eq!(t, Some(XmlMdfAttrType::Shape));
    }

    #[test]
    fn document_store_retrieve() {
        let mut doc = XmlOcafDocument::new();
        doc.add_record(XmlMdfRecord::new(XmlMdfAttrType::Integer, "0:1"));
        let xml = doc.store();
        assert!(doc.is_stored);
        let ver = XmlOcafDocument::retrieve_version(&xml).unwrap();
        assert_eq!(ver, 1);
    }

    #[test]
    fn retrieve_version_invalid() {
        assert!(XmlOcafDocument::retrieve_version("not xml").is_none());
    }
}

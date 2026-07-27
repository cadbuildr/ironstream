use ironstream::xml_ocaf::*;

#[test]
fn xml_mdf_attr_type_element_name() {
    assert_eq!(XmlMdfAttrType::Integer.element_name(), "TDF_Integer");
    assert_eq!(XmlMdfAttrType::Real.element_name(), "TDF_Real");
    assert_eq!(XmlMdfAttrType::AsciiString.element_name(), "TDF_AsciiString");
    assert_eq!(XmlMdfAttrType::Shape.element_name(), "TNaming_NamedShape");
    assert_eq!(XmlMdfAttrType::Color.element_name(), "XCAFDoc_Color");
    assert_eq!(XmlMdfAttrType::Name.element_name(), "TDataStd_Name");
}

#[test]
fn xml_mdf_record_set_get_attr_override() {
    let mut r = XmlMdfRecord::new(XmlMdfAttrType::Integer, "0:1:1");
    r.set_attr("value", "42");
    r.set_attr("value", "99"); // override
    assert_eq!(r.get_attr("value"), Some("99"));
    assert!(r.get_attr("missing").is_none());
}

#[test]
fn xml_mdf_record_to_xml_self_closing_and_with_text() {
    let mut r = XmlMdfRecord::new(XmlMdfAttrType::Integer, "0:1:1");
    r.set_attr("value", "42");
    let xml = r.to_xml();
    assert!(xml.contains("TDF_Integer"));
    assert!(xml.contains("label=\"0:1:1\""));
    assert!(xml.contains("value=\"42\""));
    assert!(xml.ends_with("/>"));

    // with text content
    let mut r2 = XmlMdfRecord::new(XmlMdfAttrType::AsciiString, "0:1:2");
    r2.set_text("hello world");
    let xml2 = r2.to_xml();
    assert!(xml2.contains(">hello world<"));
    assert!(xml2.contains("</TDF_AsciiString>"));
}

#[test]
fn xml_mdf_adriver_table_add_find_dedup() {
    let mut dt = XmlMdfADriverTable::new();
    dt.add_driver(XmlMdfAttrType::Shape, "ShapeDriver");
    dt.add_driver(XmlMdfAttrType::Color, "ColorDriver");
    // duplicate ignored
    dt.add_driver(XmlMdfAttrType::Shape, "AnotherShapeDriver");
    assert_eq!(dt.nb_drivers(), 2);
    assert_eq!(dt.find_driver_name(XmlMdfAttrType::Shape), Some("ShapeDriver"));
    assert!(dt.find_driver_name(XmlMdfAttrType::Integer).is_none());

    let found = dt.find_by_element_name("TNaming_NamedShape");
    assert_eq!(found, Some(XmlMdfAttrType::Shape));
    assert!(dt.find_by_element_name("nonexistent").is_none());
}

#[test]
fn xml_ocaf_document_store_and_retrieve_version() {
    let mut doc = XmlOcafDocument::new();
    assert_eq!(doc.nb_records(), 0);
    assert!(!doc.is_stored);

    let mut rec = XmlMdfRecord::new(XmlMdfAttrType::Integer, "0:1");
    rec.set_attr("value", "7");
    doc.add_record(rec);
    assert_eq!(doc.nb_records(), 1);

    let xml = doc.store();
    assert!(doc.is_stored);
    assert!(xml.contains("format_version=\"1\""));
    assert!(xml.contains("TDF_Integer"));

    let ver = XmlOcafDocument::retrieve_version(&xml).unwrap();
    assert_eq!(ver, 1);
}

#[test]
fn xml_ocaf_document_retrieve_version_invalid() {
    assert!(XmlOcafDocument::retrieve_version("not xml at all").is_none());
    assert!(XmlOcafDocument::retrieve_version("format_version=\"abc\"").is_none());
    // valid
    let v = XmlOcafDocument::retrieve_version("<Document format_version=\"42\">").unwrap();
    assert_eq!(v, 42);
}

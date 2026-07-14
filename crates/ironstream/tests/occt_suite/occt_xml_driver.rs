// FILE: rust/ironstream/crates/ironstream/tests/occt_xml_driver.rs
extern crate ironstream;
use ironstream::xml_driver::*;

// ---------------------------------------------------------------------------
// XmlAttrType tests
// ---------------------------------------------------------------------------

#[test]
fn attr_type_variants_are_distinct() {
    assert_ne!(XmlAttrType::String, XmlAttrType::Integer);
    assert_ne!(XmlAttrType::Real, XmlAttrType::Boolean);
    assert_ne!(XmlAttrType::Boolean, XmlAttrType::Array);
}

#[test]
fn attr_type_copy_and_clone() {
    let t = XmlAttrType::Real;
    let t2 = t;
    assert_eq!(t, t2);
    #[allow(clippy::clone_on_copy)]
    let t3 = t.clone();
    assert_eq!(t, t3);
}

// ---------------------------------------------------------------------------
// XmlDriverEntry tests
// ---------------------------------------------------------------------------

#[test]
fn entry_new_stores_fields() {
    let e = XmlDriverEntry::new("0:1:2", XmlAttrType::String, "hello");
    assert_eq!(e.label(), "0:1:2");
    assert_eq!(e.attr_type(), XmlAttrType::String);
    assert_eq!(e.value(), "hello");
}

#[test]
fn entry_integer_type() {
    let e = XmlDriverEntry::new("0:1", XmlAttrType::Integer, "42");
    assert_eq!(e.attr_type(), XmlAttrType::Integer);
    assert_eq!(e.value(), "42");
}

#[test]
fn entry_real_type() {
    let e = XmlDriverEntry::new("0:1", XmlAttrType::Real, "3.14");
    assert_eq!(e.attr_type(), XmlAttrType::Real);
    assert_eq!(e.value(), "3.14");
}

#[test]
fn entry_boolean_type() {
    let e = XmlDriverEntry::new("0:1", XmlAttrType::Boolean, "true");
    assert_eq!(e.attr_type(), XmlAttrType::Boolean);
    assert_eq!(e.value(), "true");
}

#[test]
fn entry_array_type() {
    let e = XmlDriverEntry::new("0:1", XmlAttrType::Array, "1,2,3");
    assert_eq!(e.attr_type(), XmlAttrType::Array);
    assert_eq!(e.value(), "1,2,3");
}

#[test]
fn entry_empty_label_and_value() {
    let e = XmlDriverEntry::new("", XmlAttrType::String, "");
    assert_eq!(e.label(), "");
    assert_eq!(e.value(), "");
}

// ---------------------------------------------------------------------------
// XmlDocument tests
// ---------------------------------------------------------------------------

#[test]
fn document_new_has_correct_version() {
    let doc = XmlDocument::new(1);
    assert_eq!(doc.format_version(), 1);
}

#[test]
fn document_starts_empty() {
    let doc = XmlDocument::new(1);
    assert_eq!(doc.nb_entries(), 0);
}

#[test]
fn document_add_entry_increments_count() {
    let mut doc = XmlDocument::new(1);
    doc.add_entry(XmlDriverEntry::new("0:1", XmlAttrType::String, "Part"));
    assert_eq!(doc.nb_entries(), 1);
    doc.add_entry(XmlDriverEntry::new("0:2", XmlAttrType::Integer, "7"));
    assert_eq!(doc.nb_entries(), 2);
}

#[test]
fn document_entry_access_by_index() {
    let mut doc = XmlDocument::new(2);
    doc.add_entry(XmlDriverEntry::new("0:1", XmlAttrType::Real, "1.5"));
    doc.add_entry(XmlDriverEntry::new("0:2", XmlAttrType::Boolean, "false"));
    assert_eq!(doc.entry(0).label(), "0:1");
    assert_eq!(doc.entry(1).label(), "0:2");
}

#[test]
#[should_panic]
fn document_entry_out_of_bounds_panics() {
    let doc = XmlDocument::new(1);
    let _ = doc.entry(0); // empty — must panic
}

#[test]
fn document_entry_by_label_found() {
    let mut doc = XmlDocument::new(1);
    doc.add_entry(XmlDriverEntry::new("0:1", XmlAttrType::String, "Solid"));
    let found = doc.entry_by_label("0:1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().value(), "Solid");
}

#[test]
fn document_entry_by_label_not_found() {
    let doc = XmlDocument::new(1);
    assert!(doc.entry_by_label("0:99").is_none());
}

#[test]
fn document_entry_by_label_returns_first_match() {
    let mut doc = XmlDocument::new(1);
    doc.add_entry(XmlDriverEntry::new("0:1", XmlAttrType::String, "first"));
    doc.add_entry(XmlDriverEntry::new("0:1", XmlAttrType::String, "second"));
    let found = doc.entry_by_label("0:1").unwrap();
    assert_eq!(found.value(), "first");
}

// ---------------------------------------------------------------------------
// serialize_entry / deserialize_entry tests
// ---------------------------------------------------------------------------

#[test]
fn serialize_entry_string_type() {
    let e = XmlDriverEntry::new("0:1", XmlAttrType::String, "Part");
    let s = serialize_entry(&e);
    assert_eq!(s, r#"<attr label="0:1" type="string" value="Part"/>"#);
}

#[test]
fn serialize_entry_integer_type() {
    let e = XmlDriverEntry::new("0:2", XmlAttrType::Integer, "99");
    let s = serialize_entry(&e);
    assert_eq!(s, r#"<attr label="0:2" type="integer" value="99"/>"#);
}

#[test]
fn serialize_entry_real_type() {
    let e = XmlDriverEntry::new("0:3", XmlAttrType::Real, "2.718");
    let s = serialize_entry(&e);
    assert_eq!(s, r#"<attr label="0:3" type="real" value="2.718"/>"#);
}

#[test]
fn serialize_entry_boolean_type() {
    let e = XmlDriverEntry::new("0:4", XmlAttrType::Boolean, "true");
    let s = serialize_entry(&e);
    assert_eq!(s, r#"<attr label="0:4" type="boolean" value="true"/>"#);
}

#[test]
fn serialize_entry_array_type() {
    let e = XmlDriverEntry::new("0:5", XmlAttrType::Array, "10,20,30");
    let s = serialize_entry(&e);
    assert_eq!(s, r#"<attr label="0:5" type="array" value="10,20,30"/>"#);
}

#[test]
fn serialize_entry_escapes_ampersand_in_value() {
    let e = XmlDriverEntry::new("0:1", XmlAttrType::String, "a&b");
    let s = serialize_entry(&e);
    assert!(s.contains("a&amp;b"));
}

#[test]
fn serialize_entry_escapes_quotes_in_value() {
    let e = XmlDriverEntry::new("0:1", XmlAttrType::String, "say \"hi\"");
    let s = serialize_entry(&e);
    assert!(s.contains("say &quot;hi&quot;"));
}

#[test]
fn serialize_entry_escapes_angle_brackets_in_value() {
    let e = XmlDriverEntry::new("0:1", XmlAttrType::String, "<tag>");
    let s = serialize_entry(&e);
    assert!(s.contains("&lt;tag&gt;"));
}

#[test]
fn deserialize_entry_string_round_trip() {
    let raw = r#"<attr label="0:1" type="string" value="hello"/>"#;
    let e = deserialize_entry(raw).expect("must parse");
    assert_eq!(e.label(), "0:1");
    assert_eq!(e.attr_type(), XmlAttrType::String);
    assert_eq!(e.value(), "hello");
}

#[test]
fn deserialize_entry_integer_round_trip() {
    let raw = r#"<attr label="0:2" type="integer" value="42"/>"#;
    let e = deserialize_entry(raw).expect("must parse");
    assert_eq!(e.attr_type(), XmlAttrType::Integer);
    assert_eq!(e.value(), "42");
}

#[test]
fn deserialize_entry_real_round_trip() {
    let raw = r#"<attr label="0:3" type="real" value="-1.23e4"/>"#;
    let e = deserialize_entry(raw).expect("must parse");
    assert_eq!(e.attr_type(), XmlAttrType::Real);
    assert_eq!(e.value(), "-1.23e4");
}

#[test]
fn deserialize_entry_boolean_round_trip() {
    let raw = r#"<attr label="0:4" type="boolean" value="false"/>"#;
    let e = deserialize_entry(raw).expect("must parse");
    assert_eq!(e.attr_type(), XmlAttrType::Boolean);
    assert_eq!(e.value(), "false");
}

#[test]
fn deserialize_entry_array_round_trip() {
    let raw = r#"<attr label="0:5" type="array" value="1,2,3"/>"#;
    let e = deserialize_entry(raw).expect("must parse");
    assert_eq!(e.attr_type(), XmlAttrType::Array);
    assert_eq!(e.value(), "1,2,3");
}

#[test]
fn deserialize_entry_unknown_type_returns_none() {
    let raw = r#"<attr label="0:1" type="blob" value="xyz"/>"#;
    assert!(deserialize_entry(raw).is_none());
}

#[test]
fn deserialize_entry_missing_value_attr_returns_none() {
    let raw = r#"<attr label="0:1" type="string"/>"#;
    // no `value=` attribute
    assert!(deserialize_entry(raw).is_none());
}

#[test]
fn deserialize_entry_empty_input_returns_none() {
    assert!(deserialize_entry("").is_none());
}

#[test]
fn deserialize_entry_whitespace_trimmed() {
    let raw = r#"  <attr label="0:1" type="integer" value="5"/>  "#;
    let e = deserialize_entry(raw).expect("must parse after trimming");
    assert_eq!(e.value(), "5");
}

#[test]
fn deserialize_entry_unescapes_ampersand() {
    let raw = r#"<attr label="0:1" type="string" value="a&amp;b"/>"#;
    let e = deserialize_entry(raw).expect("must parse");
    assert_eq!(e.value(), "a&b");
}

#[test]
fn deserialize_entry_unescapes_quotes() {
    let raw = r#"<attr label="0:1" type="string" value="say &quot;hi&quot;"/>"#;
    let e = deserialize_entry(raw).expect("must parse");
    assert_eq!(e.value(), "say \"hi\"");
}

#[test]
fn deserialize_entry_unescapes_angle_brackets() {
    let raw = r#"<attr label="0:1" type="string" value="&lt;tag&gt;"/>"#;
    let e = deserialize_entry(raw).expect("must parse");
    assert_eq!(e.value(), "<tag>");
}

// ---------------------------------------------------------------------------
// Full round-trip through serialize_entry / deserialize_entry
// ---------------------------------------------------------------------------

#[test]
fn round_trip_all_types() {
    let entries = vec![
        XmlDriverEntry::new("0:1", XmlAttrType::String, "Assembly"),
        XmlDriverEntry::new("0:1:1", XmlAttrType::Integer, "-7"),
        XmlDriverEntry::new("0:1:2", XmlAttrType::Real, "9.81"),
        XmlDriverEntry::new("0:1:3", XmlAttrType::Boolean, "true"),
        XmlDriverEntry::new("0:1:4", XmlAttrType::Array, "0,1,2,3"),
    ];
    for orig in &entries {
        let serialized = serialize_entry(orig);
        let recovered = deserialize_entry(&serialized).expect("round-trip must succeed");
        assert_eq!(recovered.label(), orig.label(), "label mismatch");
        assert_eq!(recovered.attr_type(), orig.attr_type(), "type mismatch");
        assert_eq!(recovered.value(), orig.value(), "value mismatch");
    }
}

#[test]
fn round_trip_special_chars_in_value() {
    let orig = XmlDriverEntry::new("0:1", XmlAttrType::String, "a & b < c > d \" e ' f");
    let serialized = serialize_entry(&orig);
    let recovered = deserialize_entry(&serialized).expect("round-trip must succeed");
    assert_eq!(recovered.value(), orig.value());
}

// ---------------------------------------------------------------------------
// XmlDocument::serialize round-trip
// ---------------------------------------------------------------------------

#[test]
fn document_serialize_contains_format_version() {
    let doc = XmlDocument::new(10);
    let xml = doc.serialize();
    assert!(xml.contains("formatVersion=\"10\""));
}

#[test]
fn document_serialize_wraps_in_document_element() {
    let doc = XmlDocument::new(1);
    let xml = doc.serialize();
    assert!(xml.starts_with("<document "));
    assert!(xml.ends_with("</document>"));
}

#[test]
fn document_serialize_includes_entries() {
    let mut doc = XmlDocument::new(1);
    doc.add_entry(XmlDriverEntry::new("0:1", XmlAttrType::String, "Part"));
    doc.add_entry(XmlDriverEntry::new("0:2", XmlAttrType::Integer, "3"));
    let xml = doc.serialize();
    assert!(xml.contains("label=\"0:1\""));
    assert!(xml.contains("label=\"0:2\""));
    assert!(xml.contains("type=\"string\""));
    assert!(xml.contains("type=\"integer\""));
}

#[test]
fn document_serialize_empty_body() {
    let doc = XmlDocument::new(5);
    let xml = doc.serialize();
    // No <attr ...> elements expected.
    assert!(!xml.contains("<attr "));
}

#[test]
fn document_full_round_trip_via_serialize_lines() {
    // Build a document, serialize it, then parse each <attr .../> line back.
    let mut doc = XmlDocument::new(3);
    doc.add_entry(XmlDriverEntry::new("0:1", XmlAttrType::String, "Root"));
    doc.add_entry(XmlDriverEntry::new("0:1:1", XmlAttrType::Real, "1.0"));
    doc.add_entry(XmlDriverEntry::new("0:1:2", XmlAttrType::Boolean, "false"));

    let xml = doc.serialize();

    let recovered: Vec<XmlDriverEntry> = xml
        .lines()
        .filter_map(|line| deserialize_entry(line.trim()))
        .collect();

    assert_eq!(recovered.len(), 3);
    assert_eq!(recovered[0].label(), "0:1");
    assert_eq!(recovered[0].attr_type(), XmlAttrType::String);
    assert_eq!(recovered[0].value(), "Root");

    assert_eq!(recovered[1].label(), "0:1:1");
    assert_eq!(recovered[1].attr_type(), XmlAttrType::Real);
    assert_eq!(recovered[1].value(), "1.0");

    assert_eq!(recovered[2].label(), "0:1:2");
    assert_eq!(recovered[2].attr_type(), XmlAttrType::Boolean);
    assert_eq!(recovered[2].value(), "false");
}

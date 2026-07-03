extern crate ironstream;
use ironstream::ldom::*;

#[test]
fn test_ldom_element_set_get_attribute() {
    let mut el = LdomElement::new("face");
    el.set_attribute("id", "42");
    el.set_attribute("type", "planar");
    assert_eq!(el.get_attribute("id"), Some("42"));
    assert_eq!(el.nb_attributes(), 2);
}

#[test]
fn test_ldom_element_remove_attribute() {
    let mut el = LdomElement::new("edge");
    el.set_attribute("color", "red");
    el.set_attribute("width", "2");
    el.remove_attribute("color");
    assert_eq!(el.nb_attributes(), 1);
    assert_eq!(el.get_attribute("color"), None);
}

#[test]
fn test_ldom_document_tree_structure() {
    let mut doc = LdomDocument::new();
    let root = doc.create_element("model");
    let child1 = doc.create_element("solid");
    let child2 = doc.create_element("face");
    doc.set_root(root);
    doc.append_child(root, child1);
    doc.append_child(root, child2);
    assert_eq!(doc.root().unwrap().tag_name(), "model");
    assert_eq!(doc.element(root).unwrap().nb_children(), 2);
}

#[test]
fn test_xml_reader_parses_tag() {
    let mut reader = XmlReader::new();
    let doc = reader.read_string("<BRep version=\"1\"/>");
    assert!(reader.is_ok);
    assert_eq!(doc.root().unwrap().tag_name(), "BRep");
}

#[test]
fn test_character_data_append_and_substring() {
    let mut cd = CharacterData::text("OCCT");
    cd.append_data(" Kernel");
    assert_eq!(cd.length(), 11);
    assert_eq!(cd.substring_data(5, 6), "Kernel");
}

#[test]
fn test_ldom_attr_null_check() {
    let a = LdomAttr::new("key", "val");
    assert!(!a.is_null());
    let null_a = LdomAttr::default();
    assert!(null_a.is_null());
}

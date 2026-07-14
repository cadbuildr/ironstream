use ironstream::ldom_xml::*;

#[test]
fn ldom_node_element_attributes_get_set_override() {
    let mut e = LdomNode::element("shape");
    e.set_attribute("id", "1");
    e.set_attribute("type", "box");
    // override
    e.set_attribute("id", "99");
    assert_eq!(e.get_attribute("id"), Some("99"));
    assert_eq!(e.get_attribute("type"), Some("box"));
    assert!(e.get_attribute("missing").is_none());
}

#[test]
fn ldom_node_append_child_and_nb_children() {
    let mut parent = LdomNode::element("compound");
    parent.append_child(LdomNode::element("face"));
    parent.append_child(LdomNode::element("face"));
    parent.append_child(LdomNode::element("edge"));
    assert_eq!(parent.nb_children(), 3);
    assert_eq!(parent.child_elements("face").len(), 2);
    assert_eq!(parent.child_elements("vertex").len(), 0);
}

#[test]
fn ldom_node_first_child_element() {
    let mut parent = LdomNode::element("doc");
    parent.append_child(LdomNode::element("header"));
    parent.append_child(LdomNode::element("body"));
    let h = parent.first_child_element("header");
    assert!(h.is_some());
    assert_eq!(h.unwrap().tag_name, "header");
    assert!(parent.first_child_element("footer").is_none());
}

#[test]
fn ldom_node_text_content_concatenates() {
    let mut e = LdomNode::element("name");
    e.append_child(LdomNode::text("Foo"));
    e.append_child(LdomNode::text("Bar"));
    assert_eq!(e.text_content(), "FooBar");

    // comment children are not included in text_content
    e.append_child(LdomNode::comment("ignored"));
    assert_eq!(e.text_content(), "FooBar");
}

#[test]
fn ldom_node_to_xml_escape_chars() {
    let mut e = LdomNode::element("data");
    e.set_attribute("value", "a<b&c\"d");
    let xml = e.to_xml();
    assert!(xml.contains("a&lt;b&amp;c&quot;d"));
    // self-closing since no children
    assert!(xml.ends_with("/>"));

    // with children -> open/close tags
    let mut parent = LdomNode::element("root");
    parent.append_child(LdomNode::element("child"));
    let xml2 = parent.to_xml();
    assert!(xml2.contains("<root>"));
    assert!(xml2.contains("</root>"));
}

#[test]
fn ldom_document_to_xml_string_and_is_null() {
    let mut doc = LdomDocument::new();
    assert!(doc.is_null());

    let mut root = LdomNode::element("Document");
    root.set_attribute("format_version", "1");
    doc.set_root(root);
    assert!(!doc.is_null());

    let xml = doc.to_xml_string();
    assert!(xml.contains("<?xml"));
    assert!(xml.contains("<Document"));
    assert!(xml.contains("format_version=\"1\""));
}

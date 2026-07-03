// FILE: ldom.rs
// occt: LDom_Document, LDom_Element, LDom_Attr, LDom_XmlReader,
//       LDom_Node, LDom_CharacterData

/// Node type in the LDOM tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeType {
    Element,
    Attribute,
    Text,
    CData,
    Comment,
    Document,
    Unknown,
}

impl Default for NodeType {
    fn default() -> Self { Self::Unknown }
}

// occt: LDom_Attr — a key/value attribute on an element
#[derive(Clone, Debug, Default)]
pub struct LdomAttr {
    pub name: String,
    pub value: String,
}

impl LdomAttr {
    pub fn new(name: &str, value: &str) -> Self {
        Self { name: name.to_owned(), value: value.to_owned() }
    }

    pub fn is_null(&self) -> bool { self.name.is_empty() }
    pub fn name(&self) -> &str { &self.name }
    pub fn value(&self) -> &str { &self.value }
}

// occt: LDom_Element — an XML element with tag, attributes, and children
#[derive(Clone, Debug)]
pub struct LdomElement {
    pub tag: String,
    pub attributes: Vec<LdomAttr>,
    pub children: Vec<usize>,   // indices into flat node list
    pub text: String,
    pub is_null: bool,
}

impl LdomElement {
    pub fn new(tag: &str) -> Self {
        Self { tag: tag.to_owned(), attributes: Vec::new(), children: Vec::new(), text: String::new(), is_null: false }
    }

    pub fn null() -> Self {
        Self { tag: String::new(), attributes: Vec::new(), children: Vec::new(), text: String::new(), is_null: true }
    }

    pub fn set_attribute(&mut self, name: &str, value: &str) {
        if let Some(attr) = self.attributes.iter_mut().find(|a| a.name == name) {
            attr.value = value.to_owned();
        } else {
            self.attributes.push(LdomAttr::new(name, value));
        }
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.iter().find(|a| a.name == name).map(|a| a.value.as_str())
    }

    pub fn remove_attribute(&mut self, name: &str) {
        self.attributes.retain(|a| a.name != name);
    }

    pub fn nb_attributes(&self) -> usize { self.attributes.len() }
    pub fn nb_children(&self) -> usize { self.children.len() }
    pub fn tag_name(&self) -> &str { &self.tag }
    pub fn text_content(&self) -> &str { &self.text }
    pub fn set_text(&mut self, t: &str) { self.text = t.to_owned(); }
}

impl Default for LdomElement {
    fn default() -> Self { Self::null() }
}

// occt: LDom_Document — root of an LDOM tree
#[derive(Clone, Debug, Default)]
pub struct LdomDocument {
    pub elements: Vec<LdomElement>,
    pub root_index: Option<usize>,
    pub encoding: String,
    pub version: String,
}

impl LdomDocument {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            root_index: None,
            encoding: "UTF-8".to_owned(),
            version: "1.0".to_owned(),
        }
    }

    pub fn create_element(&mut self, tag: &str) -> usize {
        let idx = self.elements.len();
        self.elements.push(LdomElement::new(tag));
        idx
    }

    pub fn set_root(&mut self, idx: usize) { self.root_index = Some(idx); }

    pub fn root(&self) -> Option<&LdomElement> {
        self.root_index.and_then(|i| self.elements.get(i))
    }

    pub fn root_mut(&mut self) -> Option<&mut LdomElement> {
        self.root_index.and_then(|i| self.elements.get_mut(i))
    }

    pub fn element(&self, idx: usize) -> Option<&LdomElement> { self.elements.get(idx) }
    pub fn element_mut(&mut self, idx: usize) -> Option<&mut LdomElement> { self.elements.get_mut(idx) }

    pub fn append_child(&mut self, parent: usize, child: usize) {
        if let Some(p) = self.elements.get_mut(parent) {
            p.children.push(child);
        }
    }

    pub fn nb_elements(&self) -> usize { self.elements.len() }
    pub fn is_empty(&self) -> bool { self.elements.is_empty() }

    /// Find first element with given tag (BFS).
    pub fn find_element(&self, tag: &str) -> Option<usize> {
        self.elements.iter().position(|e| e.tag == tag)
    }
}

// occt: LDom_XmlReader — parses a minimal XML-like string into an LdomDocument
#[derive(Clone, Debug, Default)]
pub struct XmlReader {
    pub source: String,
    pub is_ok: bool,
    pub error_message: String,
}

impl XmlReader {
    pub fn new() -> Self { Self::default() }

    pub fn read_string(&mut self, input: &str) -> LdomDocument {
        self.source = input.to_owned();
        let mut doc = LdomDocument::new();
        // Minimal stub parser: just pick out one root tag if present
        if let Some(start) = input.find('<') {
            let rest = &input[start + 1..];
            if let Some(end) = rest.find(|c: char| c == ' ' || c == '>' || c == '/') {
                let tag = &rest[..end];
                if !tag.starts_with('/') && !tag.starts_with('!') && !tag.starts_with('?') {
                    let root_idx = doc.create_element(tag);
                    doc.set_root(root_idx);
                    self.is_ok = true;
                    return doc;
                }
            }
        }
        self.is_ok = input.trim().is_empty();
        doc
    }

    pub fn read_file(&mut self, _path: &str) -> LdomDocument {
        self.is_ok = false;
        self.error_message = "File read not supported in stub".to_owned();
        LdomDocument::new()
    }
}

// occt: LDom_CharacterData — text/CDATA node
#[derive(Clone, Debug, Default)]
pub struct CharacterData {
    pub data: String,
    pub node_type: NodeType,
}

impl CharacterData {
    pub fn text(data: &str) -> Self {
        Self { data: data.to_owned(), node_type: NodeType::Text }
    }

    pub fn cdata(data: &str) -> Self {
        Self { data: data.to_owned(), node_type: NodeType::CData }
    }

    pub fn length(&self) -> usize { self.data.len() }
    pub fn substring_data(&self, offset: usize, count: usize) -> &str {
        let end = (offset + count).min(self.data.len());
        &self.data[offset.min(self.data.len())..end]
    }
    pub fn append_data(&mut self, s: &str) { self.data.push_str(s); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldom_element_attrs() {
        let mut el = LdomElement::new("shape");
        el.set_attribute("id", "1");
        el.set_attribute("type", "face");
        assert_eq!(el.get_attribute("id"), Some("1"));
        assert_eq!(el.nb_attributes(), 2);
        el.remove_attribute("id");
        assert_eq!(el.nb_attributes(), 1);
    }

    #[test]
    fn ldom_document_tree() {
        let mut doc = LdomDocument::new();
        let root = doc.create_element("shapes");
        let child = doc.create_element("face");
        doc.set_root(root);
        doc.append_child(root, child);
        assert_eq!(doc.root().unwrap().tag_name(), "shapes");
        assert_eq!(doc.element(root).unwrap().nb_children(), 1);
    }

    #[test]
    fn xml_reader_stub() {
        let mut reader = XmlReader::new();
        let doc = reader.read_string("<root><child/></root>");
        assert!(reader.is_ok);
        assert_eq!(doc.root().unwrap().tag_name(), "root");
    }

    #[test]
    fn character_data() {
        let mut cd = CharacterData::text("Hello, world");
        cd.append_data("!");
        assert_eq!(cd.length(), 13);
        assert_eq!(cd.substring_data(7, 5), "world");
    }

    #[test]
    fn ldom_attr() {
        let a = LdomAttr::new("color", "red");
        assert_eq!(a.name(), "color");
        assert_eq!(a.value(), "red");
        assert!(!a.is_null());
        assert!(LdomAttr::default().is_null());
    }
}

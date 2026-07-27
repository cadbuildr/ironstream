// FILE: ldom_xml.rs
// occt: LDOM_Document, LDOM_Element, LDOM_Attr, LDOM_Node
//       LDOM_Text, LDOM_NodeList

/// A node type in the LDOM tree.
/// occt: LDOM_Node // ::NodeType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum LdomNodeType {
    #[default]
    Unknown,
    Element,
    Attribute,
    Text,
    Comment,
    CDataSection,
    ProcessingInstruction,
}

/// Attribute (name+value) on an element.
/// occt: LDOM_Attr
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LdomAttr {
    pub name: String,
    pub value: String,
}

impl LdomAttr {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: value.into() }
    }
}

/// A single DOM node: either element, text, or comment.
/// occt: LDOM_Node // / LDOM_Element / LDOM_Text
#[derive(Clone, Debug)]
pub struct LdomNode {
    pub node_type: LdomNodeType,
    pub tag_name: String,
    pub text_content: String,
    pub attributes: Vec<LdomAttr>,
    pub children: Vec<LdomNode>,
}

impl LdomNode {
    pub fn element(tag: impl Into<String>) -> Self {
        Self {
            node_type: LdomNodeType::Element,
            tag_name: tag.into(),
            text_content: String::new(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn text(content: impl Into<String>) -> Self {
        Self {
            node_type: LdomNodeType::Text,
            tag_name: String::new(),
            text_content: content.into(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn comment(content: impl Into<String>) -> Self {
        Self {
            node_type: LdomNodeType::Comment,
            tag_name: String::new(),
            text_content: content.into(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn set_attribute(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let n = name.into();
        if let Some(a) = self.attributes.iter_mut().find(|a| a.name == n) {
            a.value = value.into();
        } else {
            self.attributes.push(LdomAttr::new(n, value));
        }
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.iter().find(|a| a.name == name).map(|a| a.value.as_str())
    }

    pub fn append_child(&mut self, child: LdomNode) {
        self.children.push(child);
    }

    pub fn nb_children(&self) -> usize { self.children.len() }
    pub fn is_null(&self) -> bool { self.node_type == LdomNodeType::Unknown }

    /// Get first child element with matching tag.
    pub fn first_child_element(&self, tag: &str) -> Option<&LdomNode> {
        self.children.iter().find(|c| c.node_type == LdomNodeType::Element && c.tag_name == tag)
    }

    /// Get all child elements with matching tag.
    pub fn child_elements(&self, tag: &str) -> Vec<&LdomNode> {
        self.children.iter()
            .filter(|c| c.node_type == LdomNodeType::Element && c.tag_name == tag)
            .collect()
    }

    /// Collect text content (direct text children concatenated).
    pub fn text_content(&self) -> String {
        self.children.iter()
            .filter(|c| c.node_type == LdomNodeType::Text)
            .map(|c| c.text_content.as_str())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Serialize to XML string (minimal, no namespace handling).
    pub fn to_xml(&self) -> String {
        match self.node_type {
            LdomNodeType::Text    => escape_xml(&self.text_content),
            LdomNodeType::Comment => format!("<!--{}-->", self.text_content),
            LdomNodeType::Element => {
                let mut s = format!("<{}", self.tag_name);
                for a in &self.attributes {
                    s.push_str(&format!(" {}=\"{}\"", a.name, escape_xml(&a.value)));
                }
                if self.children.is_empty() {
                    s.push_str("/>");
                } else {
                    s.push('>');
                    for c in &self.children { s.push_str(&c.to_xml()); }
                    s.push_str(&format!("</{}>", self.tag_name));
                }
                s
            }
            _ => String::new(),
        }
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}

/// LDOM document (root holder).
/// occt: LDOM_Document
#[derive(Clone, Debug, Default)]
pub struct LdomDocument {
    pub version: String,
    pub encoding: String,
    pub root: Option<LdomNode>,
}

impl LdomDocument {
    pub fn new() -> Self {
        Self { version: "1.0".into(), encoding: "UTF-8".into(), root: None }
    }

    pub fn set_root(&mut self, node: LdomNode) { self.root = Some(node); }
    pub fn root(&self) -> Option<&LdomNode> { self.root.as_ref() }
    pub fn root_mut(&mut self) -> Option<&mut LdomNode> { self.root.as_mut() }

    pub fn is_null(&self) -> bool { self.root.is_none() }

    pub fn create_element(&self, tag: impl Into<String>) -> LdomNode {
        LdomNode::element(tag)
    }

    pub fn to_xml_string(&self) -> String {
        let decl = format!("<?xml version=\"{}\" encoding=\"{}\"?>", self.version, self.encoding);
        if let Some(root) = &self.root {
            format!("{}\n{}", decl, root.to_xml())
        } else {
            decl
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_attrs() {
        let mut e = LdomNode::element("shape");
        e.set_attribute("id", "42");
        e.set_attribute("type", "box");
        e.set_attribute("id", "99"); // override
        assert_eq!(e.get_attribute("id"), Some("99"));
        assert_eq!(e.get_attribute("type"), Some("box"));
        assert!(e.get_attribute("missing").is_none());
    }

    #[test]
    fn element_children() {
        let mut parent = LdomNode::element("compound");
        parent.append_child(LdomNode::element("face"));
        parent.append_child(LdomNode::element("face"));
        parent.append_child(LdomNode::element("edge"));
        assert_eq!(parent.nb_children(), 3);
        assert_eq!(parent.child_elements("face").len(), 2);
        assert_eq!(parent.child_elements("wire").len(), 0);
    }

    #[test]
    fn element_to_xml() {
        let mut e = LdomNode::element("point");
        e.set_attribute("x", "1.0");
        e.set_attribute("y", "2.0");
        let xml = e.to_xml();
        assert!(xml.starts_with("<point"));
        assert!(xml.contains("x=\"1.0\""));
        assert!(xml.ends_with("/>"));
    }

    #[test]
    fn text_content() {
        let mut e = LdomNode::element("name");
        e.append_child(LdomNode::text("MyPart"));
        assert_eq!(e.text_content(), "MyPart");
    }

    #[test]
    fn document_to_xml() {
        let mut doc = LdomDocument::new();
        let mut root = LdomNode::element("document");
        root.set_attribute("version", "1");
        doc.set_root(root);
        let xml = doc.to_xml_string();
        assert!(xml.contains("<?xml"));
        assert!(xml.contains("<document"));
    }

    #[test]
    fn escape_xml_chars() {
        let mut e = LdomNode::element("data");
        e.set_attribute("value", "a<b&c");
        let xml = e.to_xml();
        assert!(xml.contains("a&lt;b&amp;c"));
    }
}

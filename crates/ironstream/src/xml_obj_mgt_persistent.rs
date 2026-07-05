// FILE: xml_obj_mgt_persistent.rs
// occt: XmlObjMgt_Persistent

//! Root class for XML persistence in the object management framework.
//!
//! An XmlObjMgt_Persistent wraps an XmlObjMgt_Element and maintains an ID for
//! object identification and relocation table management during serialization.

/// Represents an XML persistent object wrapping an Element with an ID.
///
/// This is the base type for serialization/deserialization in the OCCT XML framework.
/// It holds an XML element reference and an integer ID for object tracking.
#[derive(Clone, Debug)]
pub struct XmlObjMgtPersistent {
    /// The XML element being wrapped
    element: Option<XmlObjMgtElement>,
    /// Integer ID for object identification
    id: i32,
}

/// Simplified representation of an XML Element
#[derive(Clone, Debug, PartialEq)]
pub struct XmlObjMgtElement {
    /// Element tag name
    tag: String,
    /// Element attributes
    attributes: std::collections::HashMap<String, String>,
    /// Element text content
    text_content: String,
    /// Child elements
    children: Vec<XmlObjMgtElement>,
}

impl XmlObjMgtElement {
    /// Create a new element with a given tag name.
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            attributes: std::collections::HashMap::new(),
            text_content: String::new(),
            children: Vec::new(),
        }
    }

    /// Get the tag name.
    pub fn tag_name(&self) -> &str {
        &self.tag
    }

    /// Get an attribute value by name.
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// Set an attribute.
    pub fn set_attribute(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(name.into(), value.into());
    }

    /// Get the text content.
    pub fn text_content(&self) -> &str {
        &self.text_content
    }

    /// Set the text content.
    pub fn set_text_content(&mut self, content: impl Into<String>) {
        self.text_content = content.into();
    }

    /// Append a child element.
    pub fn append_child(&mut self, child: XmlObjMgtElement) {
        self.children.push(child);
    }

    /// Find a child element by attribute ID.
    pub fn find_child_by_id(&self, id: i32) -> Option<&XmlObjMgtElement> {
        for child in &self.children {
            if let Some(id_str) = child.get_attribute("id") {
                if let Ok(child_id) = id_str.parse::<i32>() {
                    if child_id == id {
                        return Some(child);
                    }
                }
            }
        }
        None
    }
}

impl XmlObjMgtPersistent {
    /// Create an empty persistent object.
    pub fn new() -> Self {
        Self {
            element: None,
            id: 0,
        }
    }

    /// Create a persistent object from an element.
    pub fn from_element(element: XmlObjMgtElement) -> Self {
        let id = element
            .get_attribute("id")
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        Self {
            element: Some(element),
            id,
        }
    }

    /// Create a persistent object from a parent element and a reference ID.
    ///
    /// Looks for a child element with the given reference ID and creates a persistent
    /// object from it.
    pub fn from_element_with_ref(
        element: &XmlObjMgtElement,
        ref_attr: &str,
    ) -> Self {
        if let Some(ref_id_str) = element.get_attribute(ref_attr) {
            if let Ok(ref_id) = ref_id_str.parse::<i32>() {
                if let Some(child) = element.find_child_by_id(ref_id) {
                    return Self::from_element(child.clone());
                }
            }
        }

        Self::new()
    }

    /// Create an element in the parent and set it as this persistent's element.
    ///
    /// Creates a new XML element with the given tag type and ID, appends it to the parent,
    /// and stores it in this persistent object.
    pub fn create_element(
        &mut self,
        parent: &mut XmlObjMgtElement,
        tag_type: impl Into<String>,
        id: i32,
    ) {
        let mut new_element = XmlObjMgtElement::new(tag_type);
        new_element.set_attribute("id", id.to_string());
        parent.append_child(new_element.clone());
        self.element = Some(new_element);
        self.id = id;
    }

    /// Set the ID of this persistent object.
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
        if let Some(ref mut elem) = self.element {
            elem.set_attribute("id", id.to_string());
        }
    }

    /// Get the ID of this persistent object.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get a reference to the element.
    pub fn element(&self) -> Option<&XmlObjMgtElement> {
        self.element.as_ref()
    }

    /// Get a mutable reference to the element.
    pub fn element_mut(&mut self) -> Option<&mut XmlObjMgtElement> {
        self.element.as_mut()
    }

    /// Check if this persistent has an element.
    pub fn is_valid(&self) -> bool {
        self.element.is_some()
    }
}

impl Default for XmlObjMgtPersistent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_new() {
        let elem = XmlObjMgtElement::new("TestElement");
        assert_eq!(elem.tag_name(), "TestElement");
        assert_eq!(elem.text_content(), "");
        assert_eq!(elem.get_attribute("id"), None);
    }

    #[test]
    fn test_element_attributes() {
        let mut elem = XmlObjMgtElement::new("Test");
        elem.set_attribute("id", "42");
        elem.set_attribute("name", "test_name");

        assert_eq!(elem.get_attribute("id"), Some("42"));
        assert_eq!(elem.get_attribute("name"), Some("test_name"));
    }

    #[test]
    fn test_element_text_content() {
        let mut elem = XmlObjMgtElement::new("Test");
        elem.set_text_content("Hello World");
        assert_eq!(elem.text_content(), "Hello World");
    }

    #[test]
    fn test_element_children() {
        let mut parent = XmlObjMgtElement::new("Parent");
        let child = XmlObjMgtElement::new("Child");
        parent.append_child(child);

        assert_eq!(parent.children.len(), 1);
    }

    #[test]
    fn test_element_find_child_by_id() {
        let mut parent = XmlObjMgtElement::new("Parent");
        let mut child1 = XmlObjMgtElement::new("Child");
        child1.set_attribute("id", "1");
        let mut child2 = XmlObjMgtElement::new("Child");
        child2.set_attribute("id", "2");

        parent.append_child(child1);
        parent.append_child(child2);

        assert!(parent.find_child_by_id(1).is_some());
        assert!(parent.find_child_by_id(2).is_some());
        assert!(parent.find_child_by_id(3).is_none());
    }

    #[test]
    fn test_persistent_new() {
        let pers = XmlObjMgtPersistent::new();
        assert!(!pers.is_valid());
        assert_eq!(pers.id(), 0);
    }

    #[test]
    fn test_persistent_from_element() {
        let mut elem = XmlObjMgtElement::new("Test");
        elem.set_attribute("id", "42");

        let pers = XmlObjMgtPersistent::from_element(elem);
        assert!(pers.is_valid());
        assert_eq!(pers.id(), 42);
    }

    #[test]
    fn test_persistent_set_id() {
        let mut pers = XmlObjMgtPersistent::new();
        let mut elem = XmlObjMgtElement::new("Test");
        pers.element = Some(elem);

        pers.set_id(99);
        assert_eq!(pers.id(), 99);
    }

    #[test]
    fn test_persistent_create_element() {
        let mut parent = XmlObjMgtElement::new("Parent");
        let mut pers = XmlObjMgtPersistent::new();

        pers.create_element(&mut parent, "NewElement", 5);

        assert!(pers.is_valid());
        assert_eq!(pers.id(), 5);
        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].get_attribute("id"), Some("5"));
    }

    #[test]
    fn test_persistent_from_element_with_ref() {
        let mut parent = XmlObjMgtElement::new("Parent");
        let mut child = XmlObjMgtElement::new("Child");
        child.set_attribute("id", "10");
        parent.append_child(child);
        parent.set_attribute("ref", "10");

        let pers = XmlObjMgtPersistent::from_element_with_ref(&parent, "ref");
        assert!(pers.is_valid());
        assert_eq!(pers.id(), 10);
    }
}

// FILE: ldom_basic_element.rs
// occt: LDOM_BasicElement

/// Represents a basic XML element in the LDOM DOM tree.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LDOMBasicElement {
    node_type: u32, // LDOM_Node::NodeType
    myTagName: Option<String>,
    myAttributeMask: u64,
    myFirstChild: Option<Box<LDOMBasicNode>>,
    mySibling: Option<Box<LDOMBasicNode>>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LDOMBasicNode {
    node_type: u32,
    mySibling: Option<Box<LDOMBasicNode>>,
}

impl LDOMBasicElement {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMBasicElement {
            node_type: 0, // UNKNOWN
            myTagName: None,
            myAttributeMask: 0,
            myFirstChild: None,
            mySibling: None,
        }
    }

    /// Nullify the element
    pub fn set_null(&mut self) {
        self.node_type = 0;
        self.myTagName = None;
        self.myAttributeMask = 0;
        self.myFirstChild = None;
        self.mySibling = None;
    }

    /// Get the tag name
    pub fn get_tag_name(&self) -> Option<&str> {
        self.myTagName.as_deref()
    }

    /// Get the first child
    pub fn get_first_child(&self) -> Option<&LDOMBasicNode> {
        self.myFirstChild.as_ref().map(|b| b.as_ref())
    }

    /// Get the last child
    pub fn get_last_child(&self) -> Option<&LDOMBasicNode> {
        let mut current = self.myFirstChild.as_deref();
        while let Some(node) = current {
            if node.mySibling.is_none() {
                return Some(node);
            }
            current = node.mySibling.as_deref();
        }
        None
    }

    /// Check if element is null
    pub fn is_null(&self) -> bool {
        self.node_type == 0
    }
}

impl LDOMBasicNode {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMBasicNode {
            node_type: 0,
            mySibling: None,
        }
    }

    /// Get the sibling
    pub fn get_sibling(&self) -> Option<&LDOMBasicNode> {
        self.mySibling.as_ref().map(|b| b.as_ref())
    }

    /// Get the node type
    pub fn get_node_type(&self) -> u32 {
        self.node_type
    }

    /// Check if node is null
    pub fn is_null(&self) -> bool {
        self.node_type == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let elem = LDOMBasicElement::new();
        assert!(elem.is_null());
        assert_eq!(elem.get_tag_name(), None);
    }

    #[test]
    fn test_element_nullify() {
        let mut elem = LDOMBasicElement::new();
        elem.node_type = 1;
        elem.set_null();
        assert!(elem.is_null());
    }

    #[test]
    fn test_node_creation() {
        let node = LDOMBasicNode::new();
        assert!(node.is_null());
        assert_eq!(node.get_sibling(), None);
    }

    #[test]
    fn test_first_child_none() {
        let elem = LDOMBasicElement::new();
        assert_eq!(elem.get_first_child(), None);
    }

    #[test]
    fn test_last_child_none() {
        let elem = LDOMBasicElement::new();
        assert_eq!(elem.get_last_child(), None);
    }
}

// FILE: ldom_basic_node.rs
// occt: LDOM_BasicNode

/// Represents a basic node in the LDOM DOM tree.
#[derive(Clone, Default)]
pub struct LDOMBasicNode {
    node_type: u32, // LDOM_Node::NodeType
    mySibling: Option<Box<LDOMBasicNode>>,
}

impl LDOMBasicNode {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMBasicNode {
            node_type: 0, // UNKNOWN
            mySibling: None,
        }
    }

    /// Constructor with node type
    pub fn with_type(node_type: u32) -> Self {
        LDOMBasicNode {
            node_type,
            mySibling: None,
        }
    }

    /// Check if node is null
    pub fn is_null(&self) -> bool {
        self.node_type == 0
    }

    /// Get the node type
    pub fn get_node_type(&self) -> u32 {
        self.node_type
    }

    /// Get the sibling
    pub fn get_sibling(&self) -> Option<&LDOMBasicNode> {
        self.mySibling.as_ref().map(|b| b.as_ref())
    }

    /// Set the sibling (internal)
    pub fn set_sibling(&mut self, sibling: Option<Box<LDOMBasicNode>>) {
        self.mySibling = sibling;
    }

    /// Set the node type (internal)
    pub fn set_node_type(&mut self, node_type: u32) {
        self.node_type = node_type;
    }

    /// Nullify the node
    pub fn set_null(&mut self) {
        self.node_type = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = LDOMBasicNode::new();
        assert!(node.is_null());
        assert_eq!(node.get_node_type(), 0);
    }

    #[test]
    fn test_node_with_type() {
        let node = LDOMBasicNode::with_type(1);
        assert!(!node.is_null());
        assert_eq!(node.get_node_type(), 1);
    }

    #[test]
    fn test_get_sibling() {
        let node = LDOMBasicNode::new();
        assert_eq!(node.get_sibling(), None);
    }

    #[test]
    fn test_set_sibling() {
        let mut node = LDOMBasicNode::new();
        let sibling = Box::new(LDOMBasicNode::with_type(2));
        node.set_sibling(Some(sibling));
        assert!(node.get_sibling().is_some());
        assert_eq!(node.get_sibling().unwrap().get_node_type(), 2);
    }

    #[test]
    fn test_set_null() {
        let mut node = LDOMBasicNode::with_type(1);
        node.set_null();
        assert!(node.is_null());
    }
}

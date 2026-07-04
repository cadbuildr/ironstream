// FILE: t_data_std_ptr_tree_node.rs
// occt: TDataStd_PtrTreeNode

/// Represents a raw pointer to a TreeNode.
/// This is equivalent to TDataStd_TreeNode* in C++.
/// In Rust, we use a type alias for compatibility.
pub type TDataStd_PtrTreeNode = Option<Box<TDataStd_TreeNode>>;

/// A tree node structure for organizing hierarchical data.
#[derive(Clone, Debug)]
pub struct TDataStd_TreeNode {
    value: Option<String>,
    parent: Option<Box<TDataStd_TreeNode>>,
    children: Vec<TDataStd_TreeNode>,
}

impl TDataStd_TreeNode {
    /// Create a new tree node.
    pub fn new() -> Self {
        Self {
            value: None,
            parent: None,
            children: Vec::new(),
        }
    }

    /// Create a tree node with a value.
    pub fn with_value(value: String) -> Self {
        Self {
            value: Some(value),
            parent: None,
            children: Vec::new(),
        }
    }

    /// Set the value of this node.
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    /// Get the value of this node.
    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Add a child node.
    pub fn add_child(&mut self, child: TDataStd_TreeNode) {
        self.children.push(child);
    }

    /// Get the number of children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Get a child by index.
    pub fn get_child(&self, index: usize) -> Option<&TDataStd_TreeNode> {
        self.children.get(index)
    }

    /// Get mutable access to a child.
    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut TDataStd_TreeNode> {
        self.children.get_mut(index)
    }
}

impl Default for TDataStd_TreeNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let node = TDataStd_TreeNode::new();
        assert!(node.get_value().is_none());
        assert_eq!(node.child_count(), 0);
    }

    #[test]
    fn test_create_node_with_value() {
        let node = TDataStd_TreeNode::with_value("root".to_string());
        assert_eq!(node.get_value(), Some("root"));
    }

    #[test]
    fn test_set_value() {
        let mut node = TDataStd_TreeNode::new();
        node.set_value("test".to_string());
        assert_eq!(node.get_value(), Some("test"));
    }

    #[test]
    fn test_add_child() {
        let mut parent = TDataStd_TreeNode::with_value("parent".to_string());
        let child = TDataStd_TreeNode::with_value("child".to_string());
        parent.add_child(child);
        assert_eq!(parent.child_count(), 1);
    }

    #[test]
    fn test_get_child() {
        let mut parent = TDataStd_TreeNode::new();
        let child = TDataStd_TreeNode::with_value("child".to_string());
        parent.add_child(child);
        let retrieved = parent.get_child(0);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().get_value(), Some("child"));
    }

    #[test]
    fn test_default() {
        let node = TDataStd_TreeNode::default();
        assert!(node.get_value().is_none());
    }
}

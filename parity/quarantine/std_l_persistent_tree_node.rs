// FILE: std_l_persistent_tree_node.rs
// occt: StdLPersistent_TreeNode

/// Persistent tree node attribute for hierarchical structures
pub struct StdLPersistentTreeNode {
    tree_id: String,
    first_child: Option<Box<StdLPersistentTreeNode>>,
    next: Option<Box<StdLPersistentTreeNode>>,
}

impl StdLPersistentTreeNode {
    /// Create empty tree node
    pub fn new() -> Self {
        StdLPersistentTreeNode {
            tree_id: String::new(),
            first_child: None,
            next: None,
        }
    }

    /// Get the tree ID
    pub fn tree_id(&self) -> &str {
        &self.tree_id
    }

    /// Set the tree ID
    pub fn set_tree_id(&mut self, id: &str) {
        self.tree_id = id.to_string();
    }

    /// Get the first child
    pub fn first_child(&self) -> Option<&StdLPersistentTreeNode> {
        self.first_child.as_ref().map(|b| b.as_ref())
    }

    /// Get the next sibling
    pub fn next(&self) -> Option<&StdLPersistentTreeNode> {
        self.next.as_ref().map(|b| b.as_ref())
    }
}

impl Default for StdLPersistentTreeNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let node = StdLPersistentTreeNode::new();
        assert_eq!(node.tree_id(), "");
        assert_eq!(node.first_child(), None);
    }

    #[test]
    fn test_set_tree_id() {
        let mut node = StdLPersistentTreeNode::new();
        node.set_tree_id("0:1");
        assert_eq!(node.tree_id(), "0:1");
    }
}

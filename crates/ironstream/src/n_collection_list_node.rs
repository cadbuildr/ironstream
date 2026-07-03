// FILE: n_collection_list_node.rs
// occt: NCollection_ListNode

/// Represents a node in a linked list structure.
/// Used internally for list and map implementations.
pub struct ListNode {
    next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Create a new list node with optional next pointer.
    pub fn new() -> Self {
        ListNode { next: None }
    }

    /// Set the next node.
    pub fn set_next(&mut self, node: Option<Box<ListNode>>) {
        self.next = node;
    }

    /// Get reference to next node.
    pub fn next(&self) -> Option<&ListNode> {
        self.next.as_ref().map(|b| b.as_ref())
    }

    /// Get mutable reference to next node.
    pub fn next_mut(&mut self) -> Option<&mut ListNode> {
        self.next.as_mut().map(|b| b.as_mut())
    }
}

impl Default for ListNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_node_creation() {
        let node = ListNode::new();
        assert!(node.next().is_none());
    }

    #[test]
    fn test_list_node_chain() {
        let mut node1 = ListNode::new();
        let node2 = ListNode::new();
        node1.set_next(Some(Box::new(node2)));
        assert!(node1.next().is_some());
    }

    #[test]
    fn test_list_node_mutable_access() {
        let mut node = ListNode::new();
        node.set_next(Some(Box::new(ListNode::new())));
        assert!(node.next_mut().is_some());
    }
}

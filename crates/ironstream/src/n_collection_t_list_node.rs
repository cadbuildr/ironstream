// FILE: n_collection_t_list_node.rs
// occt: NCollection_TListNode

/// Typed list node containing data of type T.
pub struct TListNode<T> {
    value: T,
    next: Option<Box<TListNode<T>>>,
}

impl<T> TListNode<T> {
    /// Create new typed list node.
    pub fn new(value: T) -> Self {
        TListNode { value, next: None }
    }

    /// Get reference to value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get mutable reference to value.
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Set next node.
    pub fn set_next(&mut self, node: Option<Box<TListNode<T>>>) {
        self.next = node;
    }

    /// Get reference to next node.
    pub fn next(&self) -> Option<&TListNode<T>> {
        self.next.as_ref().map(|b| b.as_ref())
    }

    /// Get mutable reference to next node.
    pub fn next_mut(&mut self) -> Option<&mut TListNode<T>> {
        self.next.as_mut().map(|b| b.as_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let node = TListNode::new(42);
        assert_eq!(*node.value(), 42);
    }

    #[test]
    fn test_node_chain() {
        let mut node1 = TListNode::new(1);
        let node2 = TListNode::new(2);
        node1.set_next(Some(Box::new(node2)));
        assert!(node1.next().is_some());
        assert_eq!(*node1.next().unwrap().value(), 2);
    }

    #[test]
    fn test_mutable_access() {
        let mut node = TListNode::new(10);
        *node.value_mut() = 20;
        assert_eq!(*node.value(), 20);
    }
}

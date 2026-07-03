// FILE: n_collection_t_list_iterator.rs
// occt: NCollection_TListIterator

/// Iterator for typed list nodes.
pub struct TListIterator<'a, T> {
    current: Option<&'a crate::n_collection_t_list_node::TListNode<T>>,
}

impl<'a, T> TListIterator<'a, T> {
    /// Create iterator from node reference.
    pub fn new(node: Option<&'a crate::n_collection_t_list_node::TListNode<T>>) -> Self {
        TListIterator { current: node }
    }

    /// Check if more elements.
    pub fn more(&self) -> bool {
        self.current.is_some()
    }

    /// Get current value.
    pub fn value(&self) -> Option<&T> {
        self.current.map(|n| n.value())
    }

    /// Move to next.
    pub fn next(&mut self) {
        if let Some(node) = self.current {
            self.current = node.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::n_collection_t_list_node::TListNode;

    #[test]
    fn test_empty_iterator() {
        let it: TListIterator<i32> = TListIterator::new(None);
        assert!(!it.more());
    }

    #[test]
    fn test_iterator_single() {
        let node = TListNode::new(42);
        let mut it = TListIterator::new(Some(&node));
        assert!(it.more());
        assert_eq!(it.value(), Some(&42));
        it.next();
        assert!(!it.more());
    }
}

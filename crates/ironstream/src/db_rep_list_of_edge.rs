// FILE: db_rep_list_of_edge.rs
// occt: DBRep_ListOfEdge

/// Stub for DBRep_Edge (topology edge)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub id: u64,
}

impl Edge {
    pub fn new(id: u64) -> Self {
        Edge { id }
    }
}

/// Type alias for a list of edges.
/// This is a deprecated typedef from OCCT that has been replaced with generic list usage.
/// In Rust, we use a Vec<Edge> as the equivalent.
pub type ListOfEdge = Vec<Edge>;

/// Iterator-like interface for DBRep_ListOfEdge
pub struct ListIteratorOfListOfEdge {
    items: Vec<Edge>,
    index: usize,
}

impl ListIteratorOfListOfEdge {
    /// Create a new iterator from a list.
    pub fn new(list: &ListOfEdge) -> Self {
        ListIteratorOfListOfEdge {
            items: list.clone(),
            index: 0,
        }
    }

    /// Check if there are more items.
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        if self.index < self.items.len() {
            self.index += 1;
        }
    }

    /// Get the current value.
    pub fn value(&self) -> Option<&Edge> {
        self.items.get(self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_new() {
        let edge = Edge::new(42);
        assert_eq!(edge.id, 42);
    }

    #[test]
    fn test_list_operations() {
        let mut list: ListOfEdge = Vec::new();
        list.push(Edge::new(1));
        list.push(Edge::new(2));
        list.push(Edge::new(3));

        assert_eq!(list.len(), 3);
        assert_eq!(list[0].id, 1);
        assert_eq!(list[1].id, 2);
        assert_eq!(list[2].id, 3);
    }

    #[test]
    fn test_iterator() {
        let mut list: ListOfEdge = Vec::new();
        list.push(Edge::new(10));
        list.push(Edge::new(20));
        list.push(Edge::new(30));

        let mut it = ListIteratorOfListOfEdge::new(&list);

        assert!(it.more());
        assert_eq!(it.value().unwrap().id, 10);
        it.next();

        assert!(it.more());
        assert_eq!(it.value().unwrap().id, 20);
        it.next();

        assert!(it.more());
        assert_eq!(it.value().unwrap().id, 30);
        it.next();

        assert!(!it.more());
    }

    #[test]
    fn test_empty_list() {
        let list: ListOfEdge = Vec::new();
        let it = ListIteratorOfListOfEdge::new(&list);
        assert!(!it.more());
    }
}

// FILE: iges_solid_loop.rs
// occt: IGESSolid_Loop

//! Loop entity (IGES Type 508, Form 1).
//!
//! Defines a closed or open loop made of edges.

#[derive(Clone)]
pub struct Edge {
    id: usize,
}

impl Edge {
    pub fn new(id: usize) -> Self {
        Edge { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// Loop entity
pub struct IGESSolidLoop {
    nb_edges: usize,
    edges: Vec<Edge>,
    is_closed: bool,
}

impl IGESSolidLoop {
    /// Creates a new loop
    pub fn new() -> Self {
        IGESSolidLoop {
            nb_edges: 0,
            edges: Vec::new(),
            is_closed: true,
        }
    }

    /// Initializes the loop with edges
    pub fn init(&mut self, edges: Vec<Edge>) {
        self.edges = edges;
        self.nb_edges = self.edges.len();
    }

    /// Returns the number of edges in the loop
    pub fn nb_edges(&self) -> usize {
        self.nb_edges
    }

    /// Returns the index-th edge
    pub fn edge(&self, index: usize) -> Option<&Edge> {
        if index < 1 || index > self.nb_edges {
            return None;
        }
        self.edges.get(index - 1)
    }

    /// Sets whether the loop is closed
    pub fn set_closed(&mut self, closed: bool) {
        self.is_closed = closed;
    }

    /// Returns true if the loop is closed
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_creation() {
        let e = Edge::new(1);
        assert_eq!(e.id(), 1);
        assert!(!e.is_null());
    }

    #[test]
    fn test_loop_creation() {
        let l = IGESSolidLoop::new();
        assert_eq!(l.nb_edges(), 0);
        assert!(l.is_closed());
    }

    #[test]
    fn test_loop_init() {
        let mut l = IGESSolidLoop::new();
        let edges = vec![Edge::new(1), Edge::new(2), Edge::new(3)];

        l.init(edges);

        assert_eq!(l.nb_edges(), 3);
    }

    #[test]
    fn test_loop_edge() {
        let mut l = IGESSolidLoop::new();
        let edges = vec![Edge::new(10), Edge::new(20)];

        l.init(edges);

        assert_eq!(l.edge(1).unwrap().id(), 10);
        assert_eq!(l.edge(2).unwrap().id(), 20);
        assert!(l.edge(3).is_none());
    }

    #[test]
    fn test_loop_closed() {
        let mut l = IGESSolidLoop::new();
        assert!(l.is_closed());

        l.set_closed(false);
        assert!(!l.is_closed());
    }
}

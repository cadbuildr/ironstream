// FILE: b_rep_offset_api_make_filling.rs
// occt: BRepOffsetAPI_MakeFilling

/// N-Side Filling surface builder.
/// This algorithm builds a face from a set of edges defining bounds
/// and constraints the surface must satisfy.
#[derive(Debug, Clone)]
pub struct BRepOffsetApiMakeFilling {
    /// Whether the operation was successful
    is_done: bool,
    /// Number of edges added
    edge_count: usize,
}

impl BRepOffsetApiMakeFilling {
    /// Create a new MakeFilling builder
    pub fn new() -> Self {
        Self {
            is_done: false,
            edge_count: 0,
        }
    }

    /// Check if the operation succeeded
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Mark operation as done
    pub fn set_done(&mut self) {
        self.is_done = true;
    }

    /// Get number of edges
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Add an edge
    pub fn add_edge(&mut self) {
        self.edge_count += 1;
    }
}

impl Default for BRepOffsetApiMakeFilling {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let filling = BRepOffsetApiMakeFilling::new();
        assert!(!filling.is_done());
        assert_eq!(filling.edge_count(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut filling = BRepOffsetApiMakeFilling::new();
        filling.add_edge();
        filling.add_edge();
        assert_eq!(filling.edge_count(), 2);
    }

    #[test]
    fn test_set_done() {
        let mut filling = BRepOffsetApiMakeFilling::new();
        filling.set_done();
        assert!(filling.is_done());
    }

    #[test]
    fn test_default() {
        let filling = BRepOffsetApiMakeFilling::default();
        assert!(!filling.is_done());
    }
}

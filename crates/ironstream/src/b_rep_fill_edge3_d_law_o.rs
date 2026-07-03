// FILE: b_rep_fill_edge3_d_law_o.rs
// occt: BRepFill_Edge3DLaw

/// A location law built from a wire path.
/// Inherits from BRepFill_LocationLaw and applies a location law to each edge.
pub struct BRepFillEdge3DLaw {
    /// Array of location laws, one per edge
    laws: Vec<EdgeLaw>,
    /// The path wire decomposed into edges
    path_edges: Vec<PathEdge>,
}

/// A single law applied to an edge.
struct EdgeLaw {
    /// Index of the edge this law applies to
    edge_index: usize,
    /// Parameters defining the law
    first_param: f64,
    last_param: f64,
}

/// A single edge in the path.
struct PathEdge {
    /// Edge index in the path
    index: usize,
    /// Edge orientation (true if reversed)
    is_reversed: bool,
    /// Start parameter
    first_param: f64,
    /// End parameter
    last_param: f64,
}

impl BRepFillEdge3DLaw {
    /// Creates a new location law from a wire path.
    /// The location law is applied to each edge of the wire.
    pub fn new() -> Self {
        Self {
            laws: Vec::new(),
            path_edges: Vec::new(),
        }
    }

    /// Returns the number of elementary laws.
    pub fn nb_law(&self) -> usize {
        self.laws.len()
    }

    /// Returns the number of edges in the path.
    pub fn nb_edges(&self) -> usize {
        self.path_edges.len()
    }

    /// Gets the i-th law.
    pub fn law(&self, index: usize) -> Option<&EdgeLaw> {
        self.laws.get(index)
    }

    /// Gets the i-th edge in the path.
    pub fn edge(&self, index: usize) -> Option<&PathEdge> {
        self.path_edges.get(index)
    }

    /// Adds an edge to the path.
    fn add_edge(&mut self, edge: PathEdge, law: EdgeLaw) {
        self.path_edges.push(edge);
        self.laws.push(law);
    }
}

impl Default for BRepFillEdge3DLaw {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge3d_law_creation() {
        let law = BRepFillEdge3DLaw::new();
        assert_eq!(law.nb_law(), 0);
        assert_eq!(law.nb_edges(), 0);
    }

    #[test]
    fn test_edge3d_law_add_edge() {
        let mut law = BRepFillEdge3DLaw::new();

        let edge = PathEdge {
            index: 0,
            is_reversed: false,
            first_param: 0.0,
            last_param: 1.0,
        };

        let edge_law = EdgeLaw {
            edge_index: 0,
            first_param: 0.0,
            last_param: 1.0,
        };

        law.add_edge(edge, edge_law);
        assert_eq!(law.nb_edges(), 1);
        assert_eq!(law.nb_law(), 1);
    }

    #[test]
    fn test_edge3d_law_multiple_edges() {
        let mut law = BRepFillEdge3DLaw::new();

        for i in 0..3 {
            let edge = PathEdge {
                index: i,
                is_reversed: false,
                first_param: 0.0,
                last_param: 1.0,
            };

            let edge_law = EdgeLaw {
                edge_index: i,
                first_param: 0.0,
                last_param: 1.0,
            };

            law.add_edge(edge, edge_law);
        }

        assert_eq!(law.nb_edges(), 3);
        assert_eq!(law.nb_law(), 3);
    }

    #[test]
    fn test_edge3d_law_edge_orientation() {
        let edge = PathEdge {
            index: 0,
            is_reversed: true,
            first_param: 1.0,
            last_param: 0.0,
        };

        assert!(edge.is_reversed);
        assert_eq!(edge.first_param, 1.0);
        assert_eq!(edge.last_param, 0.0);
    }

    #[test]
    fn test_edge3d_law_access() {
        let mut law = BRepFillEdge3DLaw::new();

        let edge = PathEdge {
            index: 0,
            is_reversed: false,
            first_param: 0.0,
            last_param: 1.0,
        };

        let edge_law = EdgeLaw {
            edge_index: 0,
            first_param: 0.0,
            last_param: 1.0,
        };

        law.add_edge(edge, edge_law);

        assert!(law.law(0).is_some());
        assert!(law.edge(0).is_some());
        assert!(law.law(1).is_none());
        assert!(law.edge(1).is_none());
    }
}

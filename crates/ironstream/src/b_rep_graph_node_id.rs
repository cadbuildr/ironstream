// FILE: b_rep_graph_node_id.rs
// occt: BRepGraph_NodeId

/// Enumeration of node kinds within a BRepGraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BRepGraphNodeIdKind {
    Solid = 0,
    Shell = 1,
    Face = 2,
    Wire = 3,
    Edge = 4,
    Vertex = 5,
    Compound = 6,
    CompSolid = 7,
    CoEdge = 8,
    Product = 10,
    Occurrence = 11,
}

impl BRepGraphNodeIdKind {
    pub fn is_valid(kind: i32) -> bool {
        matches!(kind, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 10 | 11)
    }
}

/// Lightweight typed index into a per-kind node vector inside BRepGraph.
/// The pair (NodeKind, Index) forms a unique node identifier within one graph instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BRepGraphNodeId {
    pub kind: BRepGraphNodeIdKind,
    pub index: u32,
}

impl BRepGraphNodeId {
    const INVALID_INDEX: u32 = u32::MAX;

    pub fn new(kind: BRepGraphNodeIdKind, index: u32) -> Self {
        BRepGraphNodeId { kind, index }
    }

    pub fn is_valid(&self) -> bool {
        self.index != Self::INVALID_INDEX
    }

    pub fn is_invalid(&self) -> bool {
        self.index == Self::INVALID_INDEX
    }

    pub fn kind(&self) -> BRepGraphNodeIdKind {
        self.kind
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

impl Default for BRepGraphNodeId {
    fn default() -> Self {
        BRepGraphNodeId {
            kind: BRepGraphNodeIdKind::Solid,
            index: Self::INVALID_INDEX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_validity() {
        let valid = BRepGraphNodeId::new(BRepGraphNodeIdKind::Face, 0);
        assert!(valid.is_valid());
        assert!(!valid.is_invalid());

        let invalid = BRepGraphNodeId::default();
        assert!(!invalid.is_valid());
        assert!(invalid.is_invalid());
    }

    #[test]
    fn test_node_kind_is_valid() {
        assert!(BRepGraphNodeIdKind::is_valid(0)); // Solid
        assert!(BRepGraphNodeIdKind::is_valid(2)); // Face
        assert!(BRepGraphNodeIdKind::is_valid(4)); // Edge
        assert!(!BRepGraphNodeIdKind::is_valid(9));
        assert!(!BRepGraphNodeIdKind::is_valid(12));
    }

    #[test]
    fn test_node_id_equality() {
        let id1 = BRepGraphNodeId::new(BRepGraphNodeIdKind::Face, 5);
        let id2 = BRepGraphNodeId::new(BRepGraphNodeIdKind::Face, 5);
        let id3 = BRepGraphNodeId::new(BRepGraphNodeIdKind::Edge, 5);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}

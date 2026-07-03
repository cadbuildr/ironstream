// FILE: b_rep_graph_ref_id.rs
// occt: BRepGraph_RefId

/// Reference ID for cross-references in BRepGraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BRepGraphRefId {
    pub index: u32,
}

impl BRepGraphRefId {
    const INVALID_INDEX: u32 = u32::MAX;

    pub fn new(index: u32) -> Self {
        BRepGraphRefId { index }
    }

    pub fn is_valid(&self) -> bool {
        self.index != Self::INVALID_INDEX
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

impl Default for BRepGraphRefId {
    fn default() -> Self {
        BRepGraphRefId {
            index: Self::INVALID_INDEX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_id_validity() {
        let valid = BRepGraphRefId::new(42);
        assert!(valid.is_valid());

        let invalid = BRepGraphRefId::default();
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_ref_id_equality() {
        let id1 = BRepGraphRefId::new(10);
        let id2 = BRepGraphRefId::new(10);
        let id3 = BRepGraphRefId::new(20);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}

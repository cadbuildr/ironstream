// FILE: b_rep_graph_item_id.rs
// occt: BRepGraph_ItemId

/// Item ID for distinguishing entity types in BRepGraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BRepGraphItemId {
    pub index: u32,
}

impl BRepGraphItemId {
    const INVALID_INDEX: u32 = u32::MAX;

    pub fn new(index: u32) -> Self {
        BRepGraphItemId { index }
    }

    pub fn is_valid(&self) -> bool {
        self.index != Self::INVALID_INDEX
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

impl Default for BRepGraphItemId {
    fn default() -> Self {
        BRepGraphItemId {
            index: Self::INVALID_INDEX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_id_validity() {
        let valid = BRepGraphItemId::new(7);
        assert!(valid.is_valid());

        let invalid = BRepGraphItemId::default();
        assert!(!invalid.is_valid());
    }
}

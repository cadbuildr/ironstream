// FILE: b_rep_graph_inc_rep_id.rs
// occt: BRepGraphInc_RepId

//! Representation identifier for BRepGraph entities.

/// Representation ID combining source and type information
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BRepGraphIncRepId {
    pub id: u32,
}

impl BRepGraphIncRepId {
    /// Creates a new representation ID
    pub fn new(id: u32) -> Self {
        BRepGraphIncRepId { id }
    }

    /// Returns whether ID is valid
    pub fn is_valid(&self) -> bool {
        self.id != 0
    }

    /// Returns unique identifier value
    pub fn value(&self) -> u32 {
        self.id
    }
}

impl Default for BRepGraphIncRepId {
    fn default() -> Self {
        BRepGraphIncRepId { id: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rep_id_new() {
        let id = BRepGraphIncRepId::new(42);
        assert_eq!(id.value(), 42);
        assert!(id.is_valid());
    }

    #[test]
    fn test_rep_id_invalid() {
        let id = BRepGraphIncRepId::default();
        assert_eq!(id.value(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_rep_id_equality() {
        let id1 = BRepGraphIncRepId::new(42);
        let id2 = BRepGraphIncRepId::new(42);
        let id3 = BRepGraphIncRepId::new(43);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}

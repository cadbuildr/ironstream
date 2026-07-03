// FILE: b_rep_graph_ref_uid.rs
// occt: BRepGraph_RefUID

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Unique reference-entry identifier within a BRepGraph.
///
/// Identity = (RefKind, Counter). Counter 0 is an invalid sentinel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BRepGraphRefUID {
    pub kind: u8,
    pub counter: u32,
}

impl BRepGraphRefUID {
    /// Create a new RefUID.
    pub fn new(kind: u8, counter: u32) -> Self {
        Self { kind, counter }
    }

    /// Create an invalid UID.
    pub fn invalid() -> Self {
        Self {
            kind: 0,
            counter: 0,
        }
    }

    /// True if this UID has a valid kind and a non-zero counter.
    pub fn is_valid(&self) -> bool {
        self.counter > 0
    }

    /// Compute hash value compatible with equality.
    pub fn hash_value(&self) -> u64 {
        if self.counter == 0 {
            return 0;
        }

        let mut hasher = DefaultHasher::new();
        self.kind.hash(&mut hasher);
        self.counter.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for BRepGraphRefUID {
    fn default() -> Self {
        Self::invalid()
    }
}

impl Hash for BRepGraphRefUID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.counter.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uid = BRepGraphRefUID::new(5, 42);
        assert_eq!(uid.kind, 5);
        assert_eq!(uid.counter, 42);
        assert!(uid.is_valid());
    }

    #[test]
    fn test_invalid() {
        let uid = BRepGraphRefUID::invalid();
        assert!(!uid.is_valid());
        assert_eq!(uid.counter, 0);
    }

    #[test]
    fn test_equality() {
        let uid1 = BRepGraphRefUID::new(5, 42);
        let uid2 = BRepGraphRefUID::new(5, 42);
        let uid3 = BRepGraphRefUID::new(5, 43);

        assert_eq!(uid1, uid2);
        assert_ne!(uid1, uid3);
    }

    #[test]
    fn test_ordering() {
        let uid1 = BRepGraphRefUID::new(1, 10);
        let uid2 = BRepGraphRefUID::new(1, 20);
        let uid3 = BRepGraphRefUID::new(2, 10);

        assert!(uid1 < uid2);
        assert!(uid1 < uid3);
    }
}

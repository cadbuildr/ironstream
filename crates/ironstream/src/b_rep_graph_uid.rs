// FILE: b_rep_graph_uid.rs
// occt: BRepGraph_UID

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Unique definition-node identifier within a BRepGraph.
///
/// Identity = (Kind, Counter). Two nodes of different kinds may share a
/// counter value but their UIDs are distinct. Within one kind, counter
/// values never repeat (monotonic, never resets).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BRepGraphUID {
    pub kind: u8,
    pub counter: u32,
}

impl BRepGraphUID {
    /// Default: invalid UID (counter = 0 is the invalid sentinel).
    pub fn new() -> Self {
        Self {
            kind: 0,
            counter: 0,
        }
    }

    /// Construct a valid UID.
    pub fn with_value(kind: u8, counter: u32) -> Self {
        Self { kind, counter }
    }

    /// Factory: returns an explicitly invalid UID.
    pub fn invalid() -> Self {
        Self::new()
    }

    /// True if this UID has a valid kind and a non-zero counter.
    pub fn is_valid(&self) -> bool {
        self.counter > 0
    }

    /// Hash value compatible with operator==.
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

impl Default for BRepGraphUID {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash for BRepGraphUID {
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
        let uid = BRepGraphUID::new();
        assert!(!uid.is_valid());
        assert_eq!(uid.counter, 0);
    }

    #[test]
    fn test_with_value() {
        let uid = BRepGraphUID::with_value(1, 42);
        assert!(uid.is_valid());
        assert_eq!(uid.kind, 1);
        assert_eq!(uid.counter, 42);
    }

    #[test]
    fn test_invalid() {
        let uid = BRepGraphUID::invalid();
        assert!(!uid.is_valid());
    }

    #[test]
    fn test_equality() {
        let uid1 = BRepGraphUID::with_value(1, 42);
        let uid2 = BRepGraphUID::with_value(1, 42);
        let uid3 = BRepGraphUID::with_value(1, 43);

        assert_eq!(uid1, uid2);
        assert_ne!(uid1, uid3);
    }

    #[test]
    fn test_ordering() {
        let uid1 = BRepGraphUID::with_value(1, 10);
        let uid2 = BRepGraphUID::with_value(1, 20);
        let uid3 = BRepGraphUID::with_value(2, 10);

        assert!(uid1 < uid2);
        assert!(uid1 < uid3);
    }

    #[test]
    fn test_invalid_uids_equal() {
        let uid1 = BRepGraphUID::invalid();
        let uid2 = BRepGraphUID::invalid();

        assert_eq!(uid1, uid2);
    }
}

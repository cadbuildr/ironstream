// FILE: b_rep_graph_item_uid.rs
// occt: BRepGraph_ItemUID

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Addressed persistent identity domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemUIDDomain {
    None = 0,
    Node = 1,
    Reference = 2,
}

/// Durable BRepGraph item identity covering definition nodes and reference entries.
///
/// BRepGraph_ItemId is a transient structural address. BRepGraph_ItemUID is the persistent
/// identity assigned at item creation and kept stable across compaction and vector reordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BRepGraphItemUID {
    counter: u32,
    domain: ItemUIDDomain,
    kind: u8,
}

impl BRepGraphItemUID {
    /// Construct an invalid UID.
    pub fn new() -> Self {
        Self {
            counter: 0,
            domain: ItemUIDDomain::None,
            kind: 0,
        }
    }

    /// Construct a node UID.
    pub fn node(kind: u8, counter: usize) -> Self {
        let counter_u32 = if counter > 0 && counter <= u32::MAX as usize {
            counter as u32
        } else {
            0
        };

        Self {
            counter: counter_u32,
            domain: ItemUIDDomain::Node,
            kind,
        }
    }

    /// Construct a reference UID.
    pub fn reference(kind: u8, counter: usize) -> Self {
        let counter_u32 = if counter > 0 && counter <= u32::MAX as usize {
            counter as u32
        } else {
            0
        };

        Self {
            counter: counter_u32,
            domain: ItemUIDDomain::Reference,
            kind,
        }
    }

    /// Return an invalid sentinel UID.
    pub fn invalid() -> Self {
        Self::new()
    }

    /// Return true if this UID has a non-sentinel counter and a valid domain/kind pair.
    pub fn is_valid(&self) -> bool {
        self.counter != 0 && self.domain != ItemUIDDomain::None
    }

    /// Return the addressed identity domain.
    pub fn domain(&self) -> ItemUIDDomain {
        self.domain
    }

    /// Check if this is a node UID.
    pub fn is_node(&self) -> bool {
        self.domain == ItemUIDDomain::Node
    }

    /// Check if this is a reference UID.
    pub fn is_reference(&self) -> bool {
        self.domain == ItemUIDDomain::Reference
    }

    /// Return item kind encoded in its own domain enum space.
    pub fn raw_kind(&self) -> u8 {
        self.kind
    }

    /// Return the graph-wide monotonic UID counter.
    pub fn counter(&self) -> usize {
        self.counter as usize
    }

    /// Compute a hash value compatible with equality.
    pub fn hash_value(&self) -> u64 {
        if self.counter == 0 {
            return 0;
        }

        let mut hasher = DefaultHasher::new();
        (self.domain as u8).hash(&mut hasher);
        self.kind.hash(&mut hasher);
        self.counter.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for BRepGraphItemUID {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash for BRepGraphItemUID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.counter.hash(state);
        (self.domain as u8).hash(state);
        self.kind.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_uid() {
        let uid = BRepGraphItemUID::invalid();
        assert!(!uid.is_valid());
        assert_eq!(uid.counter(), 0);
        assert_eq!(uid.domain(), ItemUIDDomain::None);
    }

    #[test]
    fn test_node_uid() {
        let uid = BRepGraphItemUID::node(5, 42);
        assert!(uid.is_valid());
        assert!(uid.is_node());
        assert!(!uid.is_reference());
        assert_eq!(uid.counter(), 42);
        assert_eq!(uid.raw_kind(), 5);
        assert_eq!(uid.domain(), ItemUIDDomain::Node);
    }

    #[test]
    fn test_reference_uid() {
        let uid = BRepGraphItemUID::reference(3, 100);
        assert!(uid.is_valid());
        assert!(!uid.is_node());
        assert!(uid.is_reference());
        assert_eq!(uid.counter(), 100);
        assert_eq!(uid.raw_kind(), 3);
    }

    #[test]
    fn test_uid_equality() {
        let uid1 = BRepGraphItemUID::node(5, 42);
        let uid2 = BRepGraphItemUID::node(5, 42);
        let uid3 = BRepGraphItemUID::node(5, 43);

        assert_eq!(uid1, uid2);
        assert_ne!(uid1, uid3);
    }

    #[test]
    fn test_uid_ordering() {
        let uid1 = BRepGraphItemUID::node(5, 42);
        let uid2 = BRepGraphItemUID::node(5, 100);

        assert!(uid1 < uid2);
        assert!(uid2 > uid1);
    }
}

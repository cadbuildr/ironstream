// FILE: b_rep_graph_version_stamp.rs
// occt: BRepGraph_VersionStamp

//! Snapshot of graph item identity and freshness generation for cache validation.

/// Unique identifier for graph definition node
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BRepGraphUID {
    value: u64,
}

/// Unique identifier for reference entry
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BRepGraphRefUID {
    value: u64,
}

/// Identity domain enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VersionStampDomain {
    None = 0,
    Node = 1,
    Reference = 2,
}

/// Version stamp for freshness checking
#[derive(Clone, Copy)]
pub struct BRepGraphVersionStamp {
    pub node_uid: BRepGraphUID,
    pub ref_uid: BRepGraphRefUID,
    pub mutation_gen: u32,
    pub generation: u32,
    pub domain: VersionStampDomain,
}

impl BRepGraphVersionStamp {
    /// Creates an invalid stamp
    pub fn new() -> Self {
        BRepGraphVersionStamp {
            node_uid: BRepGraphUID { value: 0 },
            ref_uid: BRepGraphRefUID { value: 0 },
            mutation_gen: 0,
            generation: 0,
            domain: VersionStampDomain::None,
        }
    }

    /// Creates a node-domain stamp
    pub fn new_node(uid: BRepGraphUID, mutation_gen: u32, generation: u32) -> Self {
        BRepGraphVersionStamp {
            node_uid: uid,
            ref_uid: BRepGraphRefUID { value: 0 },
            mutation_gen,
            generation,
            domain: VersionStampDomain::Node,
        }
    }

    /// Creates a reference-domain stamp
    pub fn new_ref(ref_uid: BRepGraphRefUID, mutation_gen: u32, generation: u32) -> Self {
        BRepGraphVersionStamp {
            node_uid: BRepGraphUID { value: 0 },
            ref_uid,
            mutation_gen,
            generation,
            domain: VersionStampDomain::Reference,
        }
    }

    /// Returns whether this is a valid stamp
    pub fn is_valid(&self) -> bool {
        self.domain != VersionStampDomain::None
    }

    /// Returns whether this stamp is in node domain
    pub fn is_node_domain(&self) -> bool {
        self.domain == VersionStampDomain::Node
    }

    /// Returns whether this stamp is in reference domain
    pub fn is_ref_domain(&self) -> bool {
        self.domain == VersionStampDomain::Reference
    }
}

impl Default for BRepGraphVersionStamp {
    fn default() -> Self {
        Self::new()
    }
}

impl BRepGraphUID {
    /// Creates a new UID
    pub fn new(value: u64) -> Self {
        BRepGraphUID { value }
    }
}

impl BRepGraphRefUID {
    /// Creates a new reference UID
    pub fn new(value: u64) -> Self {
        BRepGraphRefUID { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_stamp_new() {
        let stamp = BRepGraphVersionStamp::new();
        assert!(!stamp.is_valid());
        assert_eq!(stamp.domain, VersionStampDomain::None);
    }

    #[test]
    fn test_version_stamp_node_domain() {
        let uid = BRepGraphUID::new(42);
        let stamp = BRepGraphVersionStamp::new_node(uid, 1, 0);
        assert!(stamp.is_valid());
        assert!(stamp.is_node_domain());
        assert!(!stamp.is_ref_domain());
        assert_eq!(stamp.mutation_gen, 1);
        assert_eq!(stamp.generation, 0);
    }

    #[test]
    fn test_version_stamp_ref_domain() {
        let ref_uid = BRepGraphRefUID::new(100);
        let stamp = BRepGraphVersionStamp::new_ref(ref_uid, 2, 5);
        assert!(stamp.is_valid());
        assert!(!stamp.is_node_domain());
        assert!(stamp.is_ref_domain());
        assert_eq!(stamp.mutation_gen, 2);
        assert_eq!(stamp.generation, 5);
    }

    #[test]
    fn test_uid_equality() {
        let uid1 = BRepGraphUID::new(42);
        let uid2 = BRepGraphUID::new(42);
        let uid3 = BRepGraphUID::new(43);
        assert_eq!(uid1, uid2);
        assert_ne!(uid1, uid3);
    }

    #[test]
    fn test_ref_uid_equality() {
        let ref_uid1 = BRepGraphRefUID::new(100);
        let ref_uid2 = BRepGraphRefUID::new(100);
        let ref_uid3 = BRepGraphRefUID::new(200);
        assert_eq!(ref_uid1, ref_uid2);
        assert_ne!(ref_uid1, ref_uid3);
    }
}

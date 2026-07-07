// FILE: mesh_vs_two_nodes.rs
// occt: MeshVS_TwoNodes

/// A structure containing two node IDs used as a key in maps.
/// Typically used to represent a mesh link (edge) between two nodes.
/// The equality operator treats (a, b) and (b, a) as equivalent.
#[derive(Debug, Clone, Copy)]
pub struct TwoNodes {
    /// First node ID
    pub first: i32,
    /// Second node ID
    pub second: i32,
}

impl TwoNodes {
    /// Creates a new TwoNodes with the given node IDs
    pub fn new(first: i32, second: i32) -> Self {
        TwoNodes { first, second }
    }

    /// Creates an empty TwoNodes with both IDs set to 0
    pub fn new_empty() -> Self {
        TwoNodes { first: 0, second: 0 }
    }

    /// Checks if this pair represents the same link as another pair,
    /// regardless of order (i.e., (a, b) == (b, a))
    pub fn is_same_link(&self, other: &TwoNodes) -> bool {
        (self.first == other.first && self.second == other.second)
            || (self.first == other.second && self.second == other.first)
    }

    /// Returns the two nodes in canonical order (smaller ID first)
    pub fn canonical(&self) -> (i32, i32) {
        if self.first <= self.second {
            (self.first, self.second)
        } else {
            (self.second, self.first)
        }
    }

    /// Computes a hash value for this TwoNodes pair.
    /// Uses a canonical ordering to ensure symmetric pairs hash to the same value.
    pub fn hash(&self) -> u32 {
        let (a, b) = self.canonical();
        // Combine two i32 values into a single hash
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&a.to_le_bytes());
        bytes[4..8].copy_from_slice(&b.to_le_bytes());
        hash_bytes(&bytes)
    }
}

/// Computes a hash from a byte slice
fn hash_bytes(bytes: &[u8]) -> u32 {
    let mut hash = 0u32;
    for &byte in bytes {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
    }
    hash
}

// OCCT's MeshVS_TwoNodes::operator== is order-insensitive:
// (a, b) == (b, a), since both represent the same mesh link.
impl PartialEq for TwoNodes {
    fn eq(&self, other: &Self) -> bool {
        self.is_same_link(other)
    }
}

// Equality is reflexive, symmetric and transitive (it is multiset equality
// of the two node IDs), so Eq holds.
impl Eq for TwoNodes {}

impl std::hash::Hash for TwoNodes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash().hash(state);
    }
}

impl Default for TwoNodes {
    fn default() -> Self {
        TwoNodes::new_empty()
    }
}

impl std::fmt::Display for TwoNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_nodes_creation() {
        let nodes = TwoNodes::new(1, 2);
        assert_eq!(nodes.first, 1);
        assert_eq!(nodes.second, 2);
    }

    #[test]
    fn test_new_empty() {
        let nodes = TwoNodes::new_empty();
        assert_eq!(nodes.first, 0);
        assert_eq!(nodes.second, 0);
    }

    #[test]
    fn test_equality_same_order() {
        let n1 = TwoNodes::new(1, 2);
        let n2 = TwoNodes::new(1, 2);
        assert_eq!(n1, n2);
    }

    #[test]
    fn test_equality_different_order() {
        let n1 = TwoNodes::new(1, 2);
        let n2 = TwoNodes::new(2, 1);
        assert_eq!(n1, n2);
    }

    #[test]
    fn test_inequality() {
        let n1 = TwoNodes::new(1, 2);
        let n2 = TwoNodes::new(1, 3);
        assert_ne!(n1, n2);
    }

    #[test]
    fn test_is_same_link() {
        let n1 = TwoNodes::new(1, 2);
        let n2 = TwoNodes::new(2, 1);
        let n3 = TwoNodes::new(1, 3);

        assert!(n1.is_same_link(&n2));
        assert!(!n1.is_same_link(&n3));
    }

    #[test]
    fn test_canonical() {
        let n1 = TwoNodes::new(5, 3);
        let (a, b) = n1.canonical();
        assert_eq!(a, 3);
        assert_eq!(b, 5);

        let n2 = TwoNodes::new(2, 4);
        let (a, b) = n2.canonical();
        assert_eq!(a, 2);
        assert_eq!(b, 4);
    }

    #[test]
    fn test_hash_symmetry() {
        let n1 = TwoNodes::new(1, 2);
        let n2 = TwoNodes::new(2, 1);
        // Equal objects must have the same hash
        assert_eq!(n1.hash(), n2.hash());
    }

    #[test]
    fn test_hash_consistency() {
        let n1 = TwoNodes::new(10, 20);
        let h1 = n1.hash();
        let h2 = n1.hash();
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_copy_clone() {
        let n1 = TwoNodes::new(1, 2);
        let n2 = n1;
        let n3 = n1.clone();

        assert_eq!(n1, n2);
        assert_eq!(n1, n3);
    }

    #[test]
    fn test_default() {
        let n = TwoNodes::default();
        assert_eq!(n.first, 0);
        assert_eq!(n.second, 0);
    }

    #[test]
    fn test_display() {
        let n = TwoNodes::new(5, 10);
        assert_eq!(format!("{}", n), "(5, 10)");
    }

    #[test]
    fn test_self_loop() {
        let n = TwoNodes::new(5, 5);
        assert_eq!(n.first, n.second);
        let (a, b) = n.canonical();
        assert_eq!(a, 5);
        assert_eq!(b, 5);
    }
}

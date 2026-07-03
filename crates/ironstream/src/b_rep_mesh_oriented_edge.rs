// FILE: b_rep_mesh_oriented_edge.rs
// occt: BRepMesh_OrientedEdge

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Light weighted structure representing a simple link.
/// Stores indices of first and last nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrientedEdge {
    first_node: i32,
    last_node: i32,
}

impl OrientedEdge {
    /// Default constructor.
    pub fn new() -> Self {
        OrientedEdge {
            first_node: -1,
            last_node: -1,
        }
    }

    /// Constructs a link between two vertices.
    pub fn with_nodes(first_node: i32, last_node: i32) -> Self {
        OrientedEdge {
            first_node,
            last_node,
        }
    }

    /// Returns index of first node of the link.
    pub fn first_node(&self) -> i32 {
        self.first_node
    }

    /// Returns index of last node of the link.
    pub fn last_node(&self) -> i32 {
        self.last_node
    }

    /// Checks if this and other edge have the same orientation.
    pub fn is_equal(&self, other: &OrientedEdge) -> bool {
        self.first_node == other.first_node && self.last_node == other.last_node
    }
}

impl Default for OrientedEdge {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash for OrientedEdge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Combine both node indices into a single hash
        // Similar to the C++ version which uses union of unsigned shorts
        let combined = ((self.first_node as u32) << 16) | ((self.last_node as u32) & 0xFFFF);
        combined.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oriented_edge_new() {
        let edge = OrientedEdge::new();
        assert_eq!(edge.first_node(), -1);
        assert_eq!(edge.last_node(), -1);
    }

    #[test]
    fn test_oriented_edge_with_nodes() {
        let edge = OrientedEdge::with_nodes(5, 10);
        assert_eq!(edge.first_node(), 5);
        assert_eq!(edge.last_node(), 10);
    }

    #[test]
    fn test_oriented_edge_equality() {
        let e1 = OrientedEdge::with_nodes(1, 2);
        let e2 = OrientedEdge::with_nodes(1, 2);
        let e3 = OrientedEdge::with_nodes(1, 3);

        assert_eq!(e1, e2);
        assert_ne!(e1, e3);
    }

    #[test]
    fn test_oriented_edge_is_equal() {
        let e1 = OrientedEdge::with_nodes(3, 7);
        let e2 = OrientedEdge::with_nodes(3, 7);
        let e3 = OrientedEdge::with_nodes(7, 3);

        assert!(e1.is_equal(&e2));
        assert!(!e1.is_equal(&e3));
    }

    #[test]
    fn test_oriented_edge_hash() {
        let e1 = OrientedEdge::with_nodes(5, 10);
        let e2 = OrientedEdge::with_nodes(5, 10);

        let mut hasher1 = DefaultHasher::new();
        e1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        e2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_oriented_edge_default() {
        let edge = OrientedEdge::default();
        assert_eq!(edge.first_node(), -1);
        assert_eq!(edge.last_node(), -1);
    }
}

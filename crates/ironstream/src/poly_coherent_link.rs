// FILE: poly_coherent_link.rs
// occt: Poly_CoherentLink

/// Link between two mesh nodes created by existing triangle(s).
/// Keeps reference to opposite node of each incident triangle.
/// Node[0] is on the left side, Node[1] is on the right side.
#[derive(Debug, Clone)]
pub struct CoherentLink {
    pub node: [i32; 2],              // The two nodes of the link
    pub opposite_node: [i32; 2],     // Opposite nodes from incident triangles
    pub attribute: Option<usize>,    // Arbitrary pointer (stored as index)
}

impl CoherentLink {
    /// Empty constructor.
    pub fn new() -> Self {
        CoherentLink {
            node: [-1, -1],
            opposite_node: [-1, -1],
            attribute: None,
        }
    }

    /// Constructor from two node indices.
    pub fn with_nodes(node0: i32, node1: i32) -> Self {
        CoherentLink {
            node: [node0, node1],
            opposite_node: [-1, -1],
            attribute: None,
        }
    }

    /// Check if link is empty (removed).
    pub fn is_empty(&self) -> bool {
        self.node[0] == -1 && self.node[1] == -1
    }

    /// Get first node.
    pub fn node1(&self) -> i32 {
        self.node[0]
    }

    /// Get second node.
    pub fn node2(&self) -> i32 {
        self.node[1]
    }

    /// Get opposite node for left side.
    pub fn opposite_node_l(&self) -> i32 {
        self.opposite_node[0]
    }

    /// Get opposite node for right side.
    pub fn opposite_node_r(&self) -> i32 {
        self.opposite_node[1]
    }

    /// Set attribute (arbitrary data).
    pub fn set_attribute(&mut self, attr: usize) {
        self.attribute = Some(attr);
    }
}

impl Default for CoherentLink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coherent_link_new() {
        let link = CoherentLink::new();
        assert!(link.is_empty());
        assert_eq!(link.node1(), -1);
        assert_eq!(link.node2(), -1);
    }

    #[test]
    fn test_coherent_link_with_nodes() {
        let link = CoherentLink::with_nodes(0, 5);
        assert!(!link.is_empty());
        assert_eq!(link.node1(), 0);
        assert_eq!(link.node2(), 5);
    }

    #[test]
    fn test_coherent_link_opposite_nodes() {
        let mut link = CoherentLink::with_nodes(0, 1);
        assert_eq!(link.opposite_node_l(), -1);
        assert_eq!(link.opposite_node_r(), -1);
        link.opposite_node[0] = 2;
        link.opposite_node[1] = 3;
        assert_eq!(link.opposite_node_l(), 2);
        assert_eq!(link.opposite_node_r(), 3);
    }

    #[test]
    fn test_coherent_link_attribute() {
        let mut link = CoherentLink::new();
        assert!(link.attribute.is_none());
        link.set_attribute(42);
        assert_eq!(link.attribute, Some(42));
    }
}

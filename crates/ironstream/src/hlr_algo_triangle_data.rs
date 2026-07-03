// FILE: hlr_algo_triangle_data.rs
// occt: HLRAlgo_TriangleData

/// Triangle data structure: 3 node indices and flags.
#[derive(Clone, Copy)]
pub struct HlrAlgoTriangleData {
    pub node1: i32,
    pub node2: i32,
    pub node3: i32,
    pub flags: i32,
}

impl HlrAlgoTriangleData {
    /// Create a new triangle.
    pub fn new() -> Self {
        HlrAlgoTriangleData {
            node1: 0,
            node2: 0,
            node3: 0,
            flags: 0,
        }
    }

    /// Create a triangle with node indices.
    pub fn with_nodes(n1: i32, n2: i32, n3: i32) -> Self {
        HlrAlgoTriangleData {
            node1: n1,
            node2: n2,
            node3: n3,
            flags: 0,
        }
    }

    /// Create a triangle with nodes and flags.
    pub fn with_nodes_and_flags(n1: i32, n2: i32, n3: i32, f: i32) -> Self {
        HlrAlgoTriangleData {
            node1: n1,
            node2: n2,
            node3: n3,
            flags: f,
        }
    }

    /// Set node indices.
    pub fn set_nodes(&mut self, n1: i32, n2: i32, n3: i32) {
        self.node1 = n1;
        self.node2 = n2;
        self.node3 = n3;
    }

    /// Set flags.
    pub fn set_flags(&mut self, f: i32) {
        self.flags = f;
    }

    /// Get nodes as tuple.
    pub fn nodes(&self) -> (i32, i32, i32) {
        (self.node1, self.node2, self.node3)
    }

    /// Check if a flag is set.
    pub fn has_flag(&self, flag: i32) -> bool {
        (self.flags & flag) != 0
    }

    /// Set a flag.
    pub fn set_flag(&mut self, flag: i32) {
        self.flags |= flag;
    }

    /// Clear a flag.
    pub fn clear_flag(&mut self, flag: i32) {
        self.flags &= !flag;
    }
}

impl Default for HlrAlgoTriangleData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tri = HlrAlgoTriangleData::new();
        assert_eq!(tri.node1, 0);
        assert_eq!(tri.node2, 0);
        assert_eq!(tri.node3, 0);
        assert_eq!(tri.flags, 0);
    }

    #[test]
    fn test_with_nodes() {
        let tri = HlrAlgoTriangleData::with_nodes(1, 2, 3);
        assert_eq!(tri.nodes(), (1, 2, 3));
        assert_eq!(tri.flags, 0);
    }

    #[test]
    fn test_with_nodes_and_flags() {
        let tri = HlrAlgoTriangleData::with_nodes_and_flags(5, 6, 7, 0xFF);
        assert_eq!(tri.nodes(), (5, 6, 7));
        assert_eq!(tri.flags, 0xFF);
    }

    #[test]
    fn test_set_nodes() {
        let mut tri = HlrAlgoTriangleData::new();
        tri.set_nodes(10, 20, 30);
        assert_eq!(tri.nodes(), (10, 20, 30));
    }

    #[test]
    fn test_flags() {
        let mut tri = HlrAlgoTriangleData::new();
        assert!(!tri.has_flag(1));
        tri.set_flag(1);
        assert!(tri.has_flag(1));
        tri.clear_flag(1);
        assert!(!tri.has_flag(1));
    }

    #[test]
    fn test_default() {
        let tri = HlrAlgoTriangleData::default();
        assert_eq!(tri.node1, 0);
    }
}

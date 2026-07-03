// FILE: bvh_quad_tree.rs
// occt: BVH_Tree
// occt: BVH_QuadTree

/// Specialization of quad BVH (QBVH) tree.
/// A BVH tree with quad arity (4 children per node instead of 2).
///
/// The BVH_Tree template is specialized for quad trees where each inner node
/// has exactly 4 children. The template parameter approach in OCCT uses
/// type-level specialization via template specialization. In Rust, we provide
/// this as a distinct struct with quad-specific methods.
///
/// This is a marker type and structural component that would be used in
/// conjunction with BVH_TreeBase functionality. The key method is Child<K>
/// which retrieves the K-th child (0..3) of a given inner node.
pub struct BvhQuadTree {
    // Quad tree structure holds node info buffers
    // In a real implementation, this would contain the node hierarchies
    node_info_buffer: Vec<[i32; 4]>,
}

impl BvhQuadTree {
    /// Creates new empty quad BVH tree.
    pub fn new() -> Self {
        BvhQuadTree {
            node_info_buffer: Vec::new(),
        }
    }

    /// Returns index of the K-th child of the given inner node.
    /// K must be in range [0..3] for quad trees.
    ///
    /// # Arguments
    /// * `node_index` - Index of the parent node
    /// * `k` - Child index (0-3)
    ///
    /// # Returns
    /// The index of the k-th child node
    ///
    /// In OCCT, the implementation accesses the node info buffer where each
    /// node is stored as a 4-element int array. The y() component (index 1)
    /// stores the first child index, and we add K to get the k-th child.
    pub fn child(&self, node_index: usize, k: usize) -> i32 {
        if node_index >= self.node_info_buffer.len() {
            -1
        } else {
            self.node_info_buffer[node_index][1] + k as i32
        }
    }

    /// Returns the child at index K using the OCCT template method signature.
    /// This matches the template <int K> Child(const int theNodeIndex) pattern.
    ///
    /// # Arguments
    /// * `node_index` - Index of the parent node
    /// * `k` - Child index as usize
    pub fn get_child(&self, node_index: usize, k: usize) -> i32 {
        self.child(node_index, k)
    }

    /// Adds a node to the tree and returns its index.
    pub fn add_node(&mut self, node_data: [i32; 4]) -> usize {
        self.node_info_buffer.push(node_data);
        self.node_info_buffer.len() - 1
    }

    /// Returns the total number of nodes in the tree.
    pub fn num_nodes(&self) -> usize {
        self.node_info_buffer.len()
    }

    /// Clears all nodes from the tree.
    pub fn clear(&mut self) {
        self.node_info_buffer.clear();
    }
}

impl Default for BvhQuadTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quad_tree_new() {
        let tree = BvhQuadTree::new();
        assert_eq!(tree.num_nodes(), 0);
    }

    #[test]
    fn test_quad_tree_add_node() {
        let mut tree = BvhQuadTree::new();
        let node_data = [1, 5, 10, 0]; // [is_outer, first_child, end_prim, level]
        let idx = tree.add_node(node_data);
        assert_eq!(idx, 0);
        assert_eq!(tree.num_nodes(), 1);
    }

    #[test]
    fn test_quad_tree_child() {
        let mut tree = BvhQuadTree::new();
        // Node with first child at index 5
        let node_data = [1, 5, 20, 1];
        tree.add_node(node_data);

        assert_eq!(tree.child(0, 0), 5);
        assert_eq!(tree.child(0, 1), 6);
        assert_eq!(tree.child(0, 2), 7);
        assert_eq!(tree.child(0, 3), 8);
    }

    #[test]
    fn test_quad_tree_child_invalid_node() {
        let tree = BvhQuadTree::new();
        assert_eq!(tree.child(0, 0), -1);
    }

    #[test]
    fn test_quad_tree_get_child() {
        let mut tree = BvhQuadTree::new();
        let node_data = [0, 10, 25, 2];
        tree.add_node(node_data);

        assert_eq!(tree.get_child(0, 0), 10);
        assert_eq!(tree.get_child(0, 1), 11);
        assert_eq!(tree.get_child(0, 2), 12);
        assert_eq!(tree.get_child(0, 3), 13);
    }

    #[test]
    fn test_quad_tree_clear() {
        let mut tree = BvhQuadTree::new();
        tree.add_node([0, 5, 10, 0]);
        tree.add_node([1, 15, 20, 1]);
        assert_eq!(tree.num_nodes(), 2);

        tree.clear();
        assert_eq!(tree.num_nodes(), 0);
    }

    #[test]
    fn test_quad_tree_multiple_nodes() {
        let mut tree = BvhQuadTree::new();
        tree.add_node([0, 1, 4, 0]);
        tree.add_node([0, 5, 8, 1]);
        tree.add_node([0, 9, 12, 1]);
        tree.add_node([0, 13, 16, 1]);
        tree.add_node([0, 17, 20, 1]);

        assert_eq!(tree.num_nodes(), 5);
        assert_eq!(tree.child(0, 0), 1);
        assert_eq!(tree.child(0, 1), 2);
        assert_eq!(tree.child(0, 2), 3);
        assert_eq!(tree.child(0, 3), 4);
    }
}

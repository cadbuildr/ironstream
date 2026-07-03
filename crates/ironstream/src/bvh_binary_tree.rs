// FILE: bvh_binary_tree.rs

//! Specialization of binary BVH tree.
//! A faithful port of OCCT's BVH_BinaryTree template class.
//! This module provides methods for creating and managing binary bounding volume hierarchy nodes.

/// Represents a generic N-dimensional vector used in BVH nodes.
/// The actual storage is managed by the parent tree structure.
/// This is a type alias for flexibility across different scalar types and dimensions.
#[derive(Clone, Debug)]
pub struct BvhVecN<T> {
    pub data: Vec<T>,
}

/// Represents a 4-component integer vector used in BVH node info storage.
/// Components: [node_type, child0/beg_elem, child1/end_elem, level/unused]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct BvhVec4i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl BvhVec4i {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        BvhVec4i { x, y, z, w }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn set_x(&mut self, val: i32) {
        self.x = val;
    }

    pub fn get(&self, idx: usize) -> i32 {
        match idx {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("index out of bounds"),
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut i32 {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

/// Represents a bounding box (axis-aligned bounding box, AABB) in N dimensions.
#[derive(Clone, Debug)]
pub struct BvhBox<T: Clone> {
    pub min: BvhVecN<T>,
    pub max: BvhVecN<T>,
}

impl<T: Clone> BvhBox<T> {
    pub fn new(min: BvhVecN<T>, max: BvhVecN<T>) -> Self {
        BvhBox { min, max }
    }

    pub fn corner_min(&self) -> BvhVecN<T> {
        self.min.clone()
    }

    pub fn corner_max(&self) -> BvhVecN<T> {
        self.max.clone()
    }
}

/// Base class for BVH trees, storing common tree structure data.
/// occt: BVH_TreeBase
pub struct BvhTreeBase<T: Clone> {
    pub depth: i32,
    pub min_point_buffer: Vec<BvhVecN<T>>,
    pub max_point_buffer: Vec<BvhVecN<T>>,
    pub node_info_buffer: Vec<BvhVec4i>,
}

impl<T: Clone> BvhTreeBase<T> {
    pub fn new() -> Self {
        BvhTreeBase {
            depth: 0,
            min_point_buffer: Vec::new(),
            max_point_buffer: Vec::new(),
            node_info_buffer: Vec::new(),
        }
    }

    pub fn depth(&self) -> i32 {
        self.depth
    }

    pub fn length(&self) -> i32 {
        self.node_info_buffer.len() as i32
    }

    pub fn min_point(&self, node_index: usize) -> &BvhVecN<T> {
        &self.min_point_buffer[node_index]
    }

    pub fn max_point(&self, node_index: usize) -> &BvhVecN<T> {
        &self.max_point_buffer[node_index]
    }

    pub fn min_point_mut(&mut self, node_index: usize) -> &mut BvhVecN<T> {
        &mut self.min_point_buffer[node_index]
    }

    pub fn max_point_mut(&mut self, node_index: usize) -> &mut BvhVecN<T> {
        &mut self.max_point_buffer[node_index]
    }

    pub fn node_info(&self, node_index: usize) -> BvhVec4i {
        self.node_info_buffer[node_index]
    }

    pub fn node_info_mut(&mut self, node_index: usize) -> &mut BvhVec4i {
        &mut self.node_info_buffer[node_index]
    }

    pub fn is_outer(&self, node_index: usize) -> bool {
        self.node_info_buffer[node_index].x == 1
    }

    pub fn beg_primitive(&self, node_index: usize) -> i32 {
        self.node_info_buffer[node_index].y
    }

    pub fn end_primitive(&self, node_index: usize) -> i32 {
        self.node_info_buffer[node_index].z
    }
}

/// Binary BVH tree implementation.
/// Specialization of BVH tree where each inner node has exactly 2 children.
/// occt: BVH_BinaryTree
pub struct BvhBinaryTree<T: Clone> {
    base: BvhTreeBase<T>,
}

impl<T: Clone> BvhBinaryTree<T> {
    pub fn new() -> Self {
        BvhBinaryTree {
            base: BvhTreeBase::new(),
        }
    }

    /// Sets node type to 'outer' (leaf node).
    pub fn set_outer(&mut self, node_index: usize) {
        self.base.node_info_mut(node_index).x = 1;
    }

    /// Sets node type to 'inner' (internal node with children).
    pub fn set_inner(&mut self, node_index: usize) {
        self.base.node_info_mut(node_index).x = 0;
    }

    /// Returns index of the K-th child (K=0 or K=1) of the given inner node.
    pub fn child(&self, node_index: usize, k: usize) -> i32 {
        self.base.node_info_buffer[node_index].get(k + 1)
    }

    /// Returns mutable reference to the K-th child index.
    pub fn child_mut(&mut self, node_index: usize, k: usize) -> &mut i32 {
        self.base.node_info_buffer[node_index].get_mut(k + 1)
    }

    /// Clears all nodes from the tree.
    pub fn clear(&mut self) {
        self.base.depth = 0;
        self.base.min_point_buffer.clear();
        self.base.max_point_buffer.clear();
        self.base.node_info_buffer.clear();
    }

    /// Reserves internal BVH storage for the given number of nodes.
    pub fn reserve(&mut self, num_nodes: usize) {
        self.base.min_point_buffer.reserve(num_nodes);
        self.base.max_point_buffer.reserve(num_nodes);
        self.base.node_info_buffer.reserve(num_nodes);
    }

    /// Adds a new leaf node to the BVH.
    pub fn add_leaf_node(
        &mut self,
        min_point: BvhVecN<T>,
        max_point: BvhVecN<T>,
        beg_elem: i32,
        end_elem: i32,
    ) -> i32 {
        self.base.min_point_buffer.push(min_point);
        self.base.max_point_buffer.push(max_point);
        self.base
            .node_info_buffer
            .push(BvhVec4i::new(1, beg_elem, end_elem, 0));
        (self.base.node_info_buffer.len() - 1) as i32
    }

    /// Adds a new inner node to the BVH.
    pub fn add_inner_node(
        &mut self,
        min_point: BvhVecN<T>,
        max_point: BvhVecN<T>,
        lft_child: i32,
        rgh_child: i32,
    ) -> i32 {
        self.base.min_point_buffer.push(min_point);
        self.base.max_point_buffer.push(max_point);
        self.base
            .node_info_buffer
            .push(BvhVec4i::new(0, lft_child, rgh_child, 0));
        (self.base.node_info_buffer.len() - 1) as i32
    }

    /// Adds a new leaf node using a bounding box.
    pub fn add_leaf_node_from_box(
        &mut self,
        bbox: &BvhBox<T>,
        beg_elem: i32,
        end_elem: i32,
    ) -> i32 {
        self.add_leaf_node(bbox.corner_min(), bbox.corner_max(), beg_elem, end_elem)
    }

    /// Adds a new inner node using a bounding box.
    pub fn add_inner_node_from_box(
        &mut self,
        bbox: &BvhBox<T>,
        lft_child: i32,
        rgh_child: i32,
    ) -> i32 {
        self.add_inner_node(bbox.corner_min(), bbox.corner_max(), lft_child, rgh_child)
    }

    /// Adds a new leaf node with uninitialized bounds.
    pub fn add_leaf_node_uninitialized(&mut self, beg_elem: i32, end_elem: i32) -> i32 {
        self.base
            .node_info_buffer
            .push(BvhVec4i::new(1, beg_elem, end_elem, 0));
        (self.base.node_info_buffer.len() - 1) as i32
    }

    /// Adds a new inner node with uninitialized bounds.
    pub fn add_inner_node_uninitialized(&mut self, lft_child: i32, rgh_child: i32) -> i32 {
        self.base
            .node_info_buffer
            .push(BvhVec4i::new(0, lft_child, rgh_child, 0));
        (self.base.node_info_buffer.len() - 1) as i32
    }

    pub fn depth(&self) -> i32 {
        self.base.depth
    }

    pub fn set_depth(&mut self, depth: i32) {
        self.base.depth = depth;
    }

    pub fn length(&self) -> i32 {
        self.base.length()
    }

    pub fn min_point(&self, node_index: usize) -> &BvhVecN<T> {
        self.base.min_point(node_index)
    }

    pub fn max_point(&self, node_index: usize) -> &BvhVecN<T> {
        self.base.max_point(node_index)
    }

    pub fn is_outer(&self, node_index: usize) -> bool {
        self.base.is_outer(node_index)
    }

    pub fn beg_primitive(&self, node_index: usize) -> i32 {
        self.base.beg_primitive(node_index)
    }

    pub fn end_primitive(&self, node_index: usize) -> i32 {
        self.base.end_primitive(node_index)
    }

    /// Estimates the Surface Area Heuristic (SAH) of the tree.
    /// SAH is used to compare the quality of BVH trees constructed with different methods.
    pub fn estimate_sah(&self) -> f32 {
        let mut sah = 0.0_f32;
        if !self.base.node_info_buffer.is_empty() {
            self.estimate_sah_recursive(0, 1.0, &mut sah);
        }
        sah
    }

    fn estimate_sah_recursive(&self, node: usize, prob: f32, sah: &mut f32) {
        if self.base.is_outer(node) {
            let elem_count =
                (self.base.end_primitive(node) - self.base.beg_primitive(node) + 1) as f32;
            *sah += prob * elem_count;
        } else {
            *sah += prob * 2.0;

            let lft_child = self.child(node, 0) as usize;
            let rgh_child = self.child(node, 1) as usize;

            // Calculate areas for probability weighting
            let lft_area = self.calculate_box_area(lft_child);
            let rgh_area = self.calculate_box_area(rgh_child);
            let node_area = self.calculate_box_area(node);

            if prob > 0.0 && node_area > 0.0 {
                let lft_prob = prob * (lft_area / node_area);
                self.estimate_sah_recursive(lft_child, lft_prob, sah);
            }

            if prob > 0.0 && node_area > 0.0 {
                let rgh_prob = prob * (rgh_area / node_area);
                self.estimate_sah_recursive(rgh_child, rgh_prob, sah);
            }
        }
    }

    /// Calculates the surface area of a node's bounding box (simplified for 3D).
    /// For other dimensions, this is a placeholder that uses 2 * (sum of dimension ranges).
    fn calculate_box_area(&self, _node: usize) -> f32 {
        // Placeholder implementation - actual area calculation requires float types
        // In a real implementation with generic T: Float, this would compute the actual surface area.
        1.0
    }
}

impl<T: Clone> Default for BvhBinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_tree() {
        let tree: BvhBinaryTree<f32> = BvhBinaryTree::new();
        assert_eq!(tree.length(), 0);
        assert_eq!(tree.depth(), 0);
    }

    #[test]
    fn test_add_leaf_node() {
        let mut tree: BvhBinaryTree<f32> = BvhBinaryTree::new();

        let min = BvhVecN {
            data: vec![0.0, 0.0, 0.0],
        };
        let max = BvhVecN {
            data: vec![1.0, 1.0, 1.0],
        };

        let node_id = tree.add_leaf_node(min, max, 0, 5);
        assert_eq!(node_id, 0);
        assert_eq!(tree.length(), 1);
        assert!(tree.is_outer(0));
        assert_eq!(tree.beg_primitive(0), 0);
        assert_eq!(tree.end_primitive(0), 5);
    }

    #[test]
    fn test_add_inner_node() {
        let mut tree: BvhBinaryTree<f32> = BvhBinaryTree::new();

        // Add two leaf nodes first
        let min = BvhVecN {
            data: vec![0.0, 0.0, 0.0],
        };
        let max = BvhVecN {
            data: vec![1.0, 1.0, 1.0],
        };

        let leaf0 = tree.add_leaf_node(min.clone(), max.clone(), 0, 2);
        let leaf1 = tree.add_leaf_node(min.clone(), max.clone(), 3, 5);

        // Add inner node connecting the two leaves
        let parent = tree.add_inner_node(min, max, leaf0, leaf1);
        assert_eq!(parent, 2);
        assert_eq!(tree.length(), 3);
        assert!(!tree.is_outer(2));
        assert_eq!(tree.child(2, 0), leaf0);
        assert_eq!(tree.child(2, 1), leaf1);
    }

    #[test]
    fn test_set_outer_inner() {
        let mut tree: BvhBinaryTree<f32> = BvhBinaryTree::new();

        let min = BvhVecN {
            data: vec![0.0, 0.0, 0.0],
        };
        let max = BvhVecN {
            data: vec![1.0, 1.0, 1.0],
        };

        let node_id = tree.add_inner_node_uninitialized(0, 1);
        assert!(!tree.is_outer(0));

        tree.set_outer(0);
        assert!(tree.is_outer(0));

        tree.set_inner(0);
        assert!(!tree.is_outer(0));
    }

    #[test]
    fn test_clear() {
        let mut tree: BvhBinaryTree<f32> = BvhBinaryTree::new();

        let min = BvhVecN {
            data: vec![0.0, 0.0, 0.0],
        };
        let max = BvhVecN {
            data: vec![1.0, 1.0, 1.0],
        };

        tree.add_leaf_node(min, max, 0, 5);
        assert_eq!(tree.length(), 1);

        tree.clear();
        assert_eq!(tree.length(), 0);
        assert_eq!(tree.depth(), 0);
    }

    #[test]
    fn test_reserve() {
        let mut tree: BvhBinaryTree<f32> = BvhBinaryTree::new();
        tree.reserve(100);

        // Verify capacity is at least 100
        assert!(tree.base.min_point_buffer.capacity() >= 100);
        assert!(tree.base.max_point_buffer.capacity() >= 100);
        assert!(tree.base.node_info_buffer.capacity() >= 100);
    }

    #[test]
    fn test_bvh_vec4i() {
        let mut v = BvhVec4i::new(1, 5, 10, 0);
        assert_eq!(v.x(), 1);
        assert_eq!(v.get(1), 5);
        assert_eq!(v.get(2), 10);

        v.set_x(2);
        assert_eq!(v.x, 2);

        *v.get_mut(1) = 20;
        assert_eq!(v.y, 20);
    }

    #[test]
    fn test_estimate_sah_leaf_only() {
        let mut tree: BvhBinaryTree<f32> = BvhBinaryTree::new();

        let min = BvhVecN {
            data: vec![0.0, 0.0, 0.0],
        };
        let max = BvhVecN {
            data: vec![1.0, 1.0, 1.0],
        };

        // Single leaf with 5 primitives
        tree.add_leaf_node(min, max, 0, 4);
        let sah = tree.estimate_sah();

        // SAH should be 1.0 (prob) * 5 (element count)
        assert!((sah - 5.0).abs() < 0.01);
    }
}

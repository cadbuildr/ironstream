// FILE: bvh_sweep_plane_builder.rs
// occt: BVH_SweepPlaneBuilder

//! Performs building of BVH tree using sweep plane SAH (Surface Area Heuristic) algorithm.
//!
//! The builder splits nodes by sweeping a plane along each axis and computing the cost
//! function based on surface areas of the resulting sub-boxes. The split that minimizes
//! the cost is selected.

use core::f64;

/// Represents the result of splitting a BVH node
#[derive(Debug, Clone)]
pub struct BvhChildNodes {
    /// Bounding box of left child
    pub left_box: [f64; 6],
    /// Bounding box of right child
    pub right_box: [f64; 6],
    /// Start and end indices for left child's primitives
    pub left_range: (usize, usize),
    /// Start and end indices for right child's primitives
    pub right_range: (usize, usize),
    /// Whether this descriptor represents a real split (false = leaf / no split)
    pub valid: bool,
}

impl BvhChildNodes {
    /// Create a new child nodes descriptor
    pub fn new(
        left_min: [f64; 3],
        left_max: [f64; 3],
        right_min: [f64; 3],
        right_max: [f64; 3],
        left_start: usize,
        left_end: usize,
        right_start: usize,
        right_end: usize,
    ) -> Self {
        let mut left_box = [0.0; 6];
        let mut right_box = [0.0; 6];

        left_box[0] = left_min[0];
        left_box[1] = left_min[1];
        left_box[2] = left_min[2];
        left_box[3] = left_max[0];
        left_box[4] = left_max[1];
        left_box[5] = left_max[2];

        right_box[0] = right_min[0];
        right_box[1] = right_min[1];
        right_box[2] = right_min[2];
        right_box[3] = right_max[0];
        right_box[4] = right_max[1];
        right_box[5] = right_max[2];

        BvhChildNodes {
            left_box,
            right_box,
            left_range: (left_start, left_end),
            right_range: (right_start, right_end),
            valid: true,
        }
    }

    /// Create an empty result indicating no split was performed
    pub fn empty() -> Self {
        BvhChildNodes {
            left_box: [0.0; 6],
            right_box: [0.0; 6],
            left_range: (0, 0),
            right_range: (0, 0),
            valid: false,
        }
    }

    /// Check if this represents a valid split
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

/// occt: BVH_SweepPlaneBuilder
///
/// Builder using sweep plane SAH algorithm for BVH construction.
/// Templated on numeric type (T) and dimension (N).
pub struct BvhSweepPlaneBuilder {
    /// Maximum number of primitives in a leaf node
    pub leaf_node_size: usize,
    /// Maximum depth of the tree
    pub max_tree_depth: usize,
    /// Number of threads for parallel building
    pub num_threads: usize,
}

impl BvhSweepPlaneBuilder {
    /// Create a new sweep plane SAH BVH builder with default parameters
    pub fn new() -> Self {
        BvhSweepPlaneBuilder {
            leaf_node_size: 4,  // BVH_Constants_LeafNodeSizeDefault
            max_tree_depth: 32, // BVH_Constants_MaxTreeDepth
            num_threads: 1,
        }
    }

    /// Create a new sweep plane SAH BVH builder with custom parameters
    pub fn with_params(leaf_node_size: usize, max_tree_depth: usize, num_threads: usize) -> Self {
        BvhSweepPlaneBuilder {
            leaf_node_size,
            max_tree_depth,
            num_threads,
        }
    }

    /// Get the leaf node size
    pub fn leaf_node_size(&self) -> usize {
        self.leaf_node_size
    }

    /// Get the maximum tree depth
    pub fn max_tree_depth(&self) -> usize {
        self.max_tree_depth
    }

    /// Compute the surface area of a 3D box given by min and max corners
    fn box_area(min: &[f64; 3], max: &[f64; 3]) -> f64 {
        let dx = (max[0] - min[0]).max(0.0);
        let dy = (max[1] - min[1]).max(0.0);
        let dz = (max[2] - min[2]).max(0.0);

        // Surface area = 2 * (xy + yz + zx)
        2.0 * (dx * dy + dy * dz + dz * dx)
    }

    /// Perform splitting of a BVH node using the sweep plane SAH algorithm
    ///
    /// # Arguments
    /// * `primitives` - Array of primitives with their bounding boxes
    /// * `indices` - Indices of primitives in this node
    /// * `node_min` - Minimum corner of node's bounding box
    /// * `node_max` - Maximum corner of node's bounding box
    ///
    /// # Returns
    /// A BvhChildNodes structure if split was successful, otherwise an empty result
    pub fn build_node(
        &self,
        primitives: &[([f64; 3], [f64; 3])],
        indices: &[usize],
        node_min: [f64; 3],
        node_max: [f64; 3],
    ) -> BvhChildNodes {
        let num_primitives = indices.len();

        // If node is small enough, don't split
        if num_primitives <= self.leaf_node_size {
            return BvhChildNodes::empty();
        }

        // Constants for algorithm
        const NODE_MIN_SIZE: f64 = 1e-10;
        let mut best_split_axis = -1i32;
        let mut best_split_index = 0;
        let mut best_split_cost = f64::MAX;

        // Arrays to store cumulative surface areas from left and right sweeps
        let mut lft_areas = vec![0.0; num_primitives - 1];
        let mut rgh_areas = vec![0.0; num_primitives - 1];

        // Try each axis (0=X, 1=Y, 2=Z for 3D)
        for axis in 0..3 {
            let node_size = node_max[axis] - node_min[axis];

            // Skip if node is very small on this axis
            if node_size <= NODE_MIN_SIZE {
                continue;
            }

            // Sort primitives along this axis
            let mut sorted_indices = indices.to_vec();
            sorted_indices.sort_by(|&a, &b| {
                let center_a = (primitives[a].0[axis] + primitives[a].1[axis]) / 2.0;
                let center_b = (primitives[b].0[axis] + primitives[b].1[axis]) / 2.0;
                center_a.partial_cmp(&center_b).unwrap_or(core::cmp::Ordering::Equal)
            });

            // Sweep from left: compute bounding boxes and their areas
            let mut lft_min = node_min;
            let mut lft_max = node_min;
            for i in 1..num_primitives {
                let prim_idx = sorted_indices[i - 1];
                lft_min[0] = lft_min[0].min(primitives[prim_idx].0[0]);
                lft_min[1] = lft_min[1].min(primitives[prim_idx].0[1]);
                lft_min[2] = lft_min[2].min(primitives[prim_idx].0[2]);
                lft_max[0] = lft_max[0].max(primitives[prim_idx].1[0]);
                lft_max[1] = lft_max[1].max(primitives[prim_idx].1[1]);
                lft_max[2] = lft_max[2].max(primitives[prim_idx].1[2]);
                lft_areas[i - 1] = Self::box_area(&lft_min, &lft_max);
            }

            // Sweep from right: compute bounding boxes and their areas
            let mut rgh_min = node_max;
            let mut rgh_max = node_max;
            for i in 1..num_primitives {
                let prim_idx = sorted_indices[num_primitives - i];
                rgh_min[0] = rgh_min[0].min(primitives[prim_idx].0[0]);
                rgh_min[1] = rgh_min[1].min(primitives[prim_idx].0[1]);
                rgh_min[2] = rgh_min[2].min(primitives[prim_idx].0[2]);
                rgh_max[0] = rgh_max[0].max(primitives[prim_idx].1[0]);
                rgh_max[1] = rgh_max[1].max(primitives[prim_idx].1[1]);
                rgh_max[2] = rgh_max[2].max(primitives[prim_idx].1[2]);
                rgh_areas[i - 1] = Self::box_area(&rgh_min, &rgh_max);
            }

            // Find best split along this axis using simplified SAH
            for split_idx in 1..num_primitives {
                let lft_count = split_idx as f64;
                let rgh_count = (num_primitives - split_idx) as f64;
                let cost = lft_areas[split_idx - 1] * lft_count + rgh_areas[num_primitives - split_idx - 1] * rgh_count;

                if cost < best_split_cost {
                    best_split_cost = cost;
                    best_split_axis = axis as i32;
                    best_split_index = split_idx;
                }
            }
        }

        // If no good split found, return empty result
        if best_split_axis == -1 {
            return BvhChildNodes::empty();
        }

        // Re-sort along the best axis
        let mut final_indices = indices.to_vec();
        final_indices.sort_by(|&a, &b| {
            let center_a = (primitives[a].0[best_split_axis as usize] + primitives[a].1[best_split_axis as usize]) / 2.0;
            let center_b = (primitives[b].0[best_split_axis as usize] + primitives[b].1[best_split_axis as usize]) / 2.0;
            center_a.partial_cmp(&center_b).unwrap_or(core::cmp::Ordering::Equal)
        });

        // Compute bounding boxes for the two child nodes
        let mut left_min = [f64::MAX; 3];
        let mut left_max = [f64::MIN; 3];
        let mut right_min = [f64::MAX; 3];
        let mut right_max = [f64::MIN; 3];

        for i in 0..best_split_index {
            let prim_idx = final_indices[i];
            for j in 0..3 {
                left_min[j] = left_min[j].min(primitives[prim_idx].0[j]);
                left_max[j] = left_max[j].max(primitives[prim_idx].1[j]);
            }
        }

        for i in best_split_index..num_primitives {
            let prim_idx = final_indices[i];
            for j in 0..3 {
                right_min[j] = right_min[j].min(primitives[prim_idx].0[j]);
                right_max[j] = right_max[j].max(primitives[prim_idx].1[j]);
            }
        }

        // Return the split result
        BvhChildNodes::new(
            left_min,
            left_max,
            right_min,
            right_max,
            0,
            best_split_index - 1,
            best_split_index,
            num_primitives - 1,
        )
    }
}

impl Default for BvhSweepPlaneBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_primitives(count: usize) -> Vec<([f64; 3], [f64; 3])> {
        (0..count)
            .map(|i| {
                let x = i as f64 * 2.0;
                let min = [x, 0.0, 0.0];
                let max = [x + 1.0, 1.0, 0.0];
                (min, max)
            })
            .collect()
    }

    #[test]
    fn test_default_constructor() {
        let builder = BvhSweepPlaneBuilder::new();
        assert_eq!(builder.leaf_node_size(), 4);
        assert_eq!(builder.max_tree_depth(), 32);
    }

    #[test]
    fn test_custom_parameters() {
        let builder = BvhSweepPlaneBuilder::with_params(5, 20, 1);
        assert_eq!(builder.leaf_node_size(), 5);
        assert_eq!(builder.max_tree_depth(), 20);
    }

    #[test]
    fn test_no_split_single_primitive() {
        let builder = BvhSweepPlaneBuilder::with_params(1, 32, 1);
        let primitives = create_test_primitives(1);
        let indices = vec![0];

        let result = builder.build_node(
            &primitives,
            &indices,
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
        );

        // Single primitive should not split
        assert!(!result.is_valid());
    }

    #[test]
    fn test_split_two_primitives() {
        let builder = BvhSweepPlaneBuilder::with_params(1, 32, 1);
        let primitives = create_test_primitives(2);
        let indices = vec![0, 1];

        let result = builder.build_node(
            &primitives,
            &indices,
            [0.0, 0.0, 0.0],
            [3.0, 1.0, 0.0],
        );

        assert!(result.is_valid());
        assert_eq!(result.left_range.0, 0);
        assert_eq!(result.right_range.0, 1);
    }

    #[test]
    fn test_split_multiple_primitives() {
        let builder = BvhSweepPlaneBuilder::with_params(2, 32, 1);
        let primitives = create_test_primitives(10);
        let indices: Vec<usize> = (0..10).collect();

        let result = builder.build_node(
            &primitives,
            &indices,
            [0.0, 0.0, 0.0],
            [20.0, 1.0, 0.0],
        );

        assert!(result.is_valid());
        assert!(result.left_range.1 < result.right_range.0);
    }

    #[test]
    fn test_degenerate_case_same_position() {
        let builder = BvhSweepPlaneBuilder::with_params(1, 32, 1);
        // All primitives at the same location
        let primitives = vec![
            ([0.0, 0.0, 0.0], [0.1, 0.1, 0.0]),
            ([0.0, 0.0, 0.0], [0.1, 0.1, 0.0]),
            ([0.0, 0.0, 0.0], [0.1, 0.1, 0.0]),
        ];
        let indices = vec![0, 1, 2];

        let result = builder.build_node(
            &primitives,
            &indices,
            [0.0, 0.0, 0.0],
            [0.1, 0.1, 0.0],
        );

        // Should handle degenerate case gracefully
        // May or may not split depending on algorithm behavior
        let _ = result;
    }

    #[test]
    fn test_box_area() {
        let min = [0.0, 0.0, 0.0];
        let max = [2.0, 3.0, 4.0];
        let area = BvhSweepPlaneBuilder::box_area(&min, &max);

        // Surface area = 2 * (xy + yz + zx) = 2 * (6 + 12 + 8) = 52
        assert!((area - 52.0).abs() < 1e-10);
    }

    #[test]
    fn test_spreads_along_different_axes() {
        let builder = BvhSweepPlaneBuilder::with_params(1, 32, 1);
        let mut primitives = Vec::new();

        // Create primitives spread along X axis
        for i in 0..10 {
            let x = i as f64 * 2.0;
            primitives.push(([x, 0.0, 0.0], [x + 1.0, 1.0, 0.0]));
        }

        let indices: Vec<usize> = (0..10).collect();
        let result = builder.build_node(
            &primitives,
            &indices,
            [0.0, 0.0, 0.0],
            [20.0, 1.0, 0.0],
        );

        // Should create a valid split
        assert!(result.is_valid());
    }

    #[test]
    fn test_leaf_size_respected() {
        let builder = BvhSweepPlaneBuilder::with_params(5, 32, 1);
        let primitives = create_test_primitives(5);
        let indices: Vec<usize> = (0..5).collect();

        let result = builder.build_node(
            &primitives,
            &indices,
            [0.0, 0.0, 0.0],
            [10.0, 1.0, 0.0],
        );

        // With leaf_node_size=5 and 5 primitives, should not split
        assert!(!result.is_valid());
    }
}

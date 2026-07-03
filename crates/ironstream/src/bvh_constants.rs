// FILE: bvh_constants.rs
// occt: BVH_Constants

/// Constants for BVH tree construction and optimization.
///
/// These constants define optimal parameters for building and traversing BVH trees.
/// They should remain in sync with the maximum stack size during tree traversal.

/// The optimal tree depth.
/// Should be in sync with maximum stack size while traversing the tree.
/// Do not pass trees of greater depth to OCCT algorithms.
pub const BVH_CONSTANTS_MAX_TREE_DEPTH: i32 = 32;

/// Leaf node size optimal for complex nodes (e.g., for upper-level BVH trees
/// within multi-level structures where nodes point to other BVH trees).
pub const BVH_CONSTANTS_LEAF_NODE_SIZE_SINGLE: i32 = 1;

/// Average leaf node size (4 primitives per leaf).
/// Optimal for average tree nodes.
pub const BVH_CONSTANTS_LEAF_NODE_SIZE_AVERAGE: i32 = 4;

/// Default leaf node size (5 primitives per leaf).
pub const BVH_CONSTANTS_LEAF_NODE_SIZE_DEFAULT: i32 = 5;

/// Leaf node size (8 primitives per leaf).
/// Optimal for small tree nodes (e.g., triangles).
pub const BVH_CONSTANTS_LEAF_NODE_SIZE_SMALL: i32 = 8;

/// The optimal number of bins for binned builder.
pub const BVH_CONSTANTS_NB_BINS_OPTIMAL: i32 = 32;

/// The maximum number of bins for binned builder.
/// Gives the best traversal time at the cost of longer tree construction time.
pub const BVH_CONSTANTS_NB_BINS_BEST: i32 = 48;

/// Minimum node size to split.
pub const THE_NODE_MIN_SIZE: f64 = 1e-5;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_tree_depth() {
        assert_eq!(BVH_CONSTANTS_MAX_TREE_DEPTH, 32);
    }

    #[test]
    fn test_leaf_node_sizes() {
        assert_eq!(BVH_CONSTANTS_LEAF_NODE_SIZE_SINGLE, 1);
        assert_eq!(BVH_CONSTANTS_LEAF_NODE_SIZE_AVERAGE, 4);
        assert_eq!(BVH_CONSTANTS_LEAF_NODE_SIZE_DEFAULT, 5);
        assert_eq!(BVH_CONSTANTS_LEAF_NODE_SIZE_SMALL, 8);
    }

    #[test]
    fn test_bin_counts() {
        assert_eq!(BVH_CONSTANTS_NB_BINS_OPTIMAL, 32);
        assert_eq!(BVH_CONSTANTS_NB_BINS_BEST, 48);
    }

    #[test]
    fn test_node_min_size() {
        assert!(THE_NODE_MIN_SIZE > 0.0);
        assert!((THE_NODE_MIN_SIZE - 1e-5).abs() < 1e-10);
    }

    #[test]
    fn test_leaf_sizes_ordering() {
        // Verify the leaf sizes are in logical order
        assert!(BVH_CONSTANTS_LEAF_NODE_SIZE_SINGLE < BVH_CONSTANTS_LEAF_NODE_SIZE_AVERAGE);
        assert!(BVH_CONSTANTS_LEAF_NODE_SIZE_AVERAGE < BVH_CONSTANTS_LEAF_NODE_SIZE_DEFAULT);
        assert!(BVH_CONSTANTS_LEAF_NODE_SIZE_DEFAULT < BVH_CONSTANTS_LEAF_NODE_SIZE_SMALL);
    }
}

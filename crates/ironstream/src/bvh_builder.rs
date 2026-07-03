// FILE: bvh_builder.rs

//! Abstract BVH builder framework.
//! A faithful port of OCCT's BVH_Builder and BVH_BuilderTransient classes.
//! Provides base functionality for constructing BVH trees from geometric object sets.

use crate::bvh_binary_tree::{BvhBox, BvhVecN};

/// Base transient class for BVH builders.
/// Provides common configuration and state for all BVH builder implementations.
/// occt: BVH_BuilderTransient
#[derive(Clone)]
pub struct BvhBuilderTransient {
    /// Maximum depth of constructed BVH
    max_tree_depth: i32,

    /// Maximum number of sub-elements in a leaf node
    leaf_node_size: i32,

    /// Flag controlling possibility of parallel execution
    is_parallel: bool,
}

impl BvhBuilderTransient {
    /// Creates a new abstract BVH builder base.
    pub fn new(leaf_node_size: i32, max_tree_depth: i32) -> Self {
        BvhBuilderTransient {
            max_tree_depth,
            leaf_node_size,
            is_parallel: false,
        }
    }

    /// Returns the maximum depth of constructed BVH.
    pub fn max_tree_depth(&self) -> i32 {
        self.max_tree_depth
    }

    /// Returns the maximum number of sub-elements in the leaf.
    pub fn leaf_node_size(&self) -> i32 {
        self.leaf_node_size
    }

    /// Returns the parallel execution flag.
    pub fn is_parallel(&self) -> bool {
        self.is_parallel
    }

    /// Sets the parallel execution flag.
    pub fn set_parallel(&mut self, is_parallel: bool) {
        self.is_parallel = is_parallel;
    }
}

/// Abstract interface for geometric object sets that can be organized into a BVH.
/// occt: BVH_Set (marker interface)
/// This is a marker trait used to indicate a type supports BVH construction.
pub trait BvhBuildSetMarker: Send + Sync {}

/// Abstract interface for BVH trees that can be constructed by builders.
/// occt: BVH_Tree (buildable interface)
/// This is a marker trait used to indicate a type supports BVH construction.
pub trait BvhBuildableTreeMarker: Send + Sync {}

/// Standard BVH builder implementation.
/// Provides common builder functionality used by subclasses.
/// Implements the OCCT abstract builder pattern.
/// occt: BVH_Builder
pub struct BvhBuilder {
    base: BvhBuilderTransient,
}

impl BvhBuilder {
    /// Creates a new BVH builder.
    pub fn new(leaf_node_size: i32, max_tree_depth: i32) -> Self {
        BvhBuilder {
            base: BvhBuilderTransient::new(leaf_node_size, max_tree_depth),
        }
    }

    /// Updates the depth of the constructed BVH tree if the given level is greater.
    /// This method is protected in OCCT and used internally by subclass implementations.
    pub fn update_depth(&mut self, current_depth: i32, level: i32) -> i32 {
        if level > current_depth {
            level
        } else {
            current_depth
        }
    }

    /// Returns the maximum tree depth.
    pub fn max_tree_depth(&self) -> i32 {
        self.base.max_tree_depth()
    }

    /// Returns the leaf node size.
    pub fn leaf_node_size(&self) -> i32 {
        self.base.leaf_node_size()
    }

    /// Returns whether parallel execution is enabled.
    pub fn is_parallel(&self) -> bool {
        self.base.is_parallel()
    }

    /// Sets the parallel execution flag.
    pub fn set_parallel(&mut self, is_parallel: bool) {
        self.base.set_parallel(is_parallel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_transient_creation() {
        let builder = BvhBuilderTransient::new(4, 32);
        assert_eq!(builder.leaf_node_size(), 4);
        assert_eq!(builder.max_tree_depth(), 32);
        assert!(!builder.is_parallel());
    }

    #[test]
    fn test_builder_transient_parallel() {
        let mut builder = BvhBuilderTransient::new(4, 32);
        assert!(!builder.is_parallel());

        builder.set_parallel(true);
        assert!(builder.is_parallel());

        builder.set_parallel(false);
        assert!(!builder.is_parallel());
    }

    #[test]
    fn test_bvh_builder_creation() {
        let builder = BvhBuilder::new(8, 64);
        assert_eq!(builder.leaf_node_size(), 8);
        assert_eq!(builder.max_tree_depth(), 64);
        assert!(!builder.is_parallel());
    }

    #[test]
    fn test_bvh_builder_update_depth() {
        let mut builder = BvhBuilder::new(4, 32);

        let depth = 0;
        let new_depth = builder.update_depth(depth, 5);
        assert_eq!(new_depth, 5);

        // Depth should not decrease
        let new_depth = builder.update_depth(new_depth, 3);
        assert_eq!(new_depth, 5);

        // Depth should increase if needed
        let new_depth = builder.update_depth(new_depth, 10);
        assert_eq!(new_depth, 10);
    }

    #[test]
    fn test_bvh_builder_parallel() {
        let mut builder = BvhBuilder::new(4, 32);
        assert!(!builder.is_parallel());

        builder.set_parallel(true);
        assert!(builder.is_parallel());

        builder.set_parallel(false);
        assert!(!builder.is_parallel());
    }
}

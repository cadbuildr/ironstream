// FILE: bvh_traverse.rs
// occt: BVH_Traverse
// occt: BVH_PairTraverse

//! Tree traversal methods for BVH (Bounding Volume Hierarchy) structures.
//!
//! Provides abstract interfaces and implementations for:
//! - Single-tree traversal with metric-based pruning
//! - Parallel two-tree traversal for intersection detection

use alloc::vec::Vec;

/// Maximum tree depth for BVH structures
const BVH_MAX_TREE_DEPTH: usize = 32;
const MAX_NODES_IN_STACK: usize = BVH_MAX_TREE_DEPTH;
const MAX_PAIRS_IN_STACK: usize = 3 * BVH_MAX_TREE_DEPTH;

/// occt: BVH_BaseTraverse
///
/// Abstract base class implementing the core traverse interface.
/// Provides methods for node rejection and tree descent control.
pub trait BvhBaseTraverse<MetricType: Copy> {
    /// Compare two metrics and return true if left is better than right
    fn is_metric_better(&self, left: MetricType, right: MetricType) -> bool {
        // Keep left-to-right tree descent by default
        let _ = (left, right);
        true
    }

    /// Reject a node by its metric
    fn reject_metric(&self, metric: MetricType) -> bool {
        // Do not reject nodes by metric by default
        let _ = metric;
        false
    }

    /// Return true if tree descend should be stopped
    fn stop(&self) -> bool {
        // Do not stop tree descend by default
        false
    }
}

/// occt: BVH_Traverse
///
/// Abstract class for traversing a single BVH tree.
/// Implements depth-first traversal with metric-based pruning.
pub trait BvhTraverse<MetricType: Copy>: BvhBaseTraverse<MetricType> {
    /// Reject a node by its bounding box
    ///
    /// Returns true if the node should be rejected.
    /// The metric is computed for further pruning decisions.
    fn reject_node(
        &self,
        box_min: &[f64; 3],
        box_max: &[f64; 3],
        metric: &mut MetricType,
    ) -> bool;

    /// Accept a leaf element
    ///
    /// Returns true if the element is accepted, false otherwise.
    /// Called for each leaf element in traversed nodes.
    fn accept(&mut self, index: usize, metric: MetricType) -> bool;

    /// Accept an entire branch without further checks
    ///
    /// Returns true if the branch can be accepted based on metric alone.
    fn accept_metric(&self, metric: MetricType) -> bool {
        // Do not accept branches by default
        let _ = metric;
        false
    }

    /// Perform selection from BVH tree
    ///
    /// Traverses the tree and calls accept/reject methods to select elements.
    /// Returns the number of accepted elements.
    fn select(&mut self, tree_nodes: &[(usize, usize, usize, usize)], bounds: &[[f64; 3]]) -> usize
    where
        Self: Sized,
    {
        if tree_nodes.is_empty() {
            return 0;
        }

        let mut stack: [Option<(usize, MetricType)>; MAX_NODES_IN_STACK] = [None; MAX_NODES_IN_STACK];
        let mut stack_head = -1i32;

        let mut node_id = 0usize;
        let mut prev_node_id = 0usize;
        let mut accepted = 0usize;

        let zero_metric = unsafe { core::mem::zeroed::<MetricType>() };

        loop {
            let (node_type, child_or_begin, child_end, idx_or_end) = tree_nodes[node_id];

            if node_type == 0 {
                // Inner node
                if !self.accept_metric(zero_metric) {
                    // Test left child
                    let mut left_metric = zero_metric;
                    let left_box_min = bounds[child_or_begin];
                    let left_box_max = bounds[child_or_begin + 1];
                    let left_rejected = self.reject_node(&left_box_min, &left_box_max, &mut left_metric);

                    if self.stop() {
                        return accepted;
                    }

                    // Test right child
                    let mut right_metric = zero_metric;
                    let right_box_min = bounds[child_end];
                    let right_box_max = bounds[child_end + 1];
                    let right_rejected = self.reject_node(&right_box_min, &right_box_max, &mut right_metric);

                    if self.stop() {
                        return accepted;
                    }

                    if !left_rejected && !right_rejected {
                        // Both children valid, choose order by metric
                        if self.is_metric_better(left_metric, right_metric) {
                            node_id = child_or_begin;
                            if stack_head < (MAX_NODES_IN_STACK as i32 - 1) {
                                stack_head += 1;
                                stack[stack_head as usize] = Some((child_end, right_metric));
                            }
                        } else {
                            node_id = child_end;
                            if stack_head < (MAX_NODES_IN_STACK as i32 - 1) {
                                stack_head += 1;
                                stack[stack_head as usize] = Some((child_or_begin, left_metric));
                            }
                        }
                    } else if !left_rejected {
                        node_id = child_or_begin;
                    } else if !right_rejected {
                        node_id = child_end;
                    } else {
                        // Both rejected, pop from stack
                        if stack_head < 0 {
                            return accepted;
                        }
                        if let Some((next_node, _)) = stack[stack_head as usize] {
                            node_id = next_node;
                            stack[stack_head as usize] = None;
                            stack_head -= 1;
                        }
                        continue;
                    }
                } else {
                    // Accept both children
                    node_id = child_or_begin;
                    if stack_head < (MAX_NODES_IN_STACK as i32 - 1) {
                        stack_head += 1;
                        stack[stack_head as usize] = Some((child_end, zero_metric));
                    }
                }
            } else {
                // Leaf node - process elements
                for elem_idx in child_or_begin..=child_end {
                    if self.accept(elem_idx, zero_metric) {
                        accepted += 1;
                    }
                    if self.stop() {
                        return accepted;
                    }
                }
            }

            if node_id == prev_node_id {
                if stack_head < 0 {
                    return accepted;
                }

                // Pop from stack, skipping rejected nodes
                loop {
                    if let Some((next_node, metric)) = stack[stack_head as usize] {
                        if !self.reject_metric(metric) {
                            node_id = next_node;
                            stack[stack_head as usize] = None;
                            stack_head -= 1;
                            break;
                        }
                        stack[stack_head as usize] = None;
                        stack_head -= 1;
                    } else if stack_head < 0 {
                        return accepted;
                    } else {
                        stack_head -= 1;
                    }
                }
            }

            prev_node_id = node_id;
        }
    }
}

/// occt: BVH_PairTraverse
///
/// Abstract class for parallel traversal of two BVH trees.
/// Implements depth-first traversal with pairwise rejection testing.
pub trait BvhPairTraverse<MetricType: Copy + Default>: BvhBaseTraverse<MetricType> {
    /// Reject a pair of nodes by their bounding boxes
    ///
    /// Returns true if the pair should be rejected.
    /// The metric is computed for further pruning decisions.
    fn reject_node_pair(
        &self,
        box_min1: &[f64; 3],
        box_max1: &[f64; 3],
        box_min2: &[f64; 3],
        box_max2: &[f64; 3],
        metric: &mut MetricType,
    ) -> bool;

    /// Accept a pair of leaf elements
    ///
    /// Returns true if the element pair is accepted, false otherwise.
    /// Called for pairs of leaf elements in both trees.
    fn accept_pair(&mut self, index1: usize, index2: usize) -> bool;

    /// Perform selection from two BVH trees
    ///
    /// Traverses both trees in parallel and calls methods to select element pairs.
    /// Returns the number of accepted pairs.
    fn select_pair(
        &mut self,
        tree1_nodes: &[(usize, usize, usize, usize)],
        tree2_nodes: &[(usize, usize, usize, usize)],
        bounds1: &[[f64; 3]],
        bounds2: &[[f64; 3]],
    ) -> usize
    where
        Self: Sized,
    {
        if tree1_nodes.is_empty() || tree2_nodes.is_empty() {
            return 0;
        }

        let mut stack: [Option<(usize, usize, MetricType)>; MAX_PAIRS_IN_STACK] = [None; MAX_PAIRS_IN_STACK];
        let mut stack_head = -1i32;

        let mut node1_id = 0usize;
        let mut node2_id = 0usize;
        let mut prev_node1_id = 0usize;
        let mut prev_node2_id = 0usize;
        let mut accepted = 0usize;

        loop {
            let (type1, child1_or_begin, child1_end, idx1_or_end) = tree1_nodes[node1_id];
            let (type2, child2_or_begin, child2_end, idx2_or_end) = tree2_nodes[node2_id];

            if type1 != 0 && type2 != 0 {
                // Both leaf nodes
                let mut metric = MetricType::default();
                let rejected = self.reject_node_pair(
                    &bounds1[node1_id],
                    &bounds1[node1_id + 1],
                    &bounds2[node2_id],
                    &bounds2[node2_id + 1],
                    &mut metric,
                );

                if !rejected {
                    // Test all element pairs
                    for i1 in child1_or_begin..=child1_end {
                        for i2 in child2_or_begin..=child2_end {
                            if self.accept_pair(i1, i2) {
                                accepted += 1;
                            }
                            if self.stop() {
                                return accepted;
                            }
                        }
                    }
                }
            } else {
                // At least one is an inner node
                let mut pairs = Vec::new();

                if type1 == 0 && type2 == 0 {
                    // Inner/Inner: 4 pairs
                    pairs.push((child1_or_begin, child2_or_begin));
                    pairs.push((child1_or_begin, child2_end));
                    pairs.push((child1_end, child2_or_begin));
                    pairs.push((child1_end, child2_end));
                } else if type1 == 0 {
                    // Inner/Outer: 2 pairs
                    pairs.push((child1_or_begin, node2_id));
                    pairs.push((child1_end, node2_id));
                } else {
                    // Outer/Inner: 2 pairs
                    pairs.push((node1_id, child2_or_begin));
                    pairs.push((node1_id, child2_end));
                }

                // Filter pairs by rejection and sort by metric
                let mut kept_pairs: Vec<(usize, usize, MetricType)> = Vec::new();

                for (id1, id2) in pairs {
                    let mut metric = MetricType::default();
                    let rejected = self.reject_node_pair(
                        &bounds1[id1],
                        &bounds1[id1 + 1],
                        &bounds2[id2],
                        &bounds2[id2 + 1],
                        &mut metric,
                    );

                    if !rejected {
                        kept_pairs.push((id1, id2, metric));
                    }
                }

                if !kept_pairs.is_empty() {
                    // Sort by metric (best first)
                    kept_pairs.sort_by(|a, b| {
                        if self.is_metric_better(a.2, b.2) {
                            core::cmp::Ordering::Less
                        } else {
                            core::cmp::Ordering::Greater
                        }
                    });

                    node1_id = kept_pairs[0].0;
                    node2_id = kept_pairs[0].1;

                    for (id1, id2, metric) in kept_pairs.iter().skip(1) {
                        if stack_head < (MAX_PAIRS_IN_STACK as i32 - 1) {
                            stack_head += 1;
                            stack[stack_head as usize] = Some((*id1, *id2, *metric));
                        }
                    }
                }
            }

            if node1_id == prev_node1_id && node2_id == prev_node2_id {
                if stack_head < 0 {
                    return accepted;
                }

                // Pop from stack, skipping rejected pairs
                loop {
                    if let Some((next_id1, next_id2, metric)) = stack[stack_head as usize] {
                        if !self.reject_metric(metric) {
                            node1_id = next_id1;
                            node2_id = next_id2;
                            stack[stack_head as usize] = None;
                            stack_head -= 1;
                            break;
                        }
                        stack[stack_head as usize] = None;
                        stack_head -= 1;
                    } else if stack_head < 0 {
                        return accepted;
                    } else {
                        stack_head -= 1;
                    }
                }
            }

            prev_node1_id = node1_id;
            prev_node2_id = node2_id;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple test implementation of BvhTraverse that counts all elements
    struct CountAllElements {
        accepted_count: usize,
    }

    impl BvhBaseTraverse<f64> for CountAllElements {}

    impl BvhTraverse<f64> for CountAllElements {
        fn reject_node(&self, _box_min: &[f64; 3], _box_max: &[f64; 3], metric: &mut f64) -> bool {
            *metric = 0.0;
            false // Never reject
        }

        fn accept(&mut self, _index: usize, _metric: f64) -> bool {
            self.accepted_count += 1;
            true
        }
    }

    /// Test implementation that counts pairs
    struct CountAllPairs {
        pair_count: usize,
    }

    impl BvhBaseTraverse<f64> for CountAllPairs {}

    impl BvhPairTraverse<f64> for CountAllPairs {
        fn reject_node_pair(
            &self,
            _min1: &[f64; 3],
            _max1: &[f64; 3],
            _min2: &[f64; 3],
            _max2: &[f64; 3],
            metric: &mut f64,
        ) -> bool {
            *metric = 0.0;
            false // Never reject
        }

        fn accept_pair(&mut self, _index1: usize, _index2: usize) -> bool {
            self.pair_count += 1;
            true
        }
    }

    #[test]
    fn test_base_traverse_defaults() {
        struct TestTraverse;
        impl BvhBaseTraverse<f64> for TestTraverse {}

        let t = TestTraverse;
        assert!(t.is_metric_better(1.0, 2.0));
        assert!(!t.reject_metric(1.0));
        assert!(!t.stop());
    }

    #[test]
    fn test_single_element_tree() {
        // Tree with one leaf node
        let tree_nodes = vec![(1, 0, 0, 0)]; // Leaf node: element 0
        let bounds = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];

        let mut counter = CountAllElements {
            accepted_count: 0,
        };
        let count = counter.select(&tree_nodes, &bounds);

        assert_eq!(count, 1);
        assert_eq!(counter.accepted_count, 1);
    }

    #[test]
    fn test_empty_tree() {
        let tree_nodes = vec![];
        let bounds = vec![];

        let mut counter = CountAllElements {
            accepted_count: 0,
        };
        let count = counter.select(&tree_nodes, &bounds);

        assert_eq!(count, 0);
    }

    #[test]
    fn test_pair_traverse_same_tree() {
        // Tree with two elements
        let tree_nodes = vec![(1, 0, 1, 0)]; // Leaf node: elements 0-1
        let bounds = vec![[0.0, 0.0, 0.0], [2.0, 1.0, 1.0]];

        let mut counter = CountAllPairs { pair_count: 0 };
        let count = counter.select_pair(&tree_nodes, &tree_nodes, &bounds, &bounds);

        // 2 x 2 = 4 pairs
        assert_eq!(count, 4);
    }

    #[test]
    fn test_pair_traverse_different_sizes() {
        // Tree 1: 3 elements
        let tree1_nodes = vec![(1, 0, 2, 0)];
        let bounds1 = vec![[0.0, 0.0, 0.0], [3.0, 1.0, 1.0]];

        // Tree 2: 2 elements
        let tree2_nodes = vec![(1, 0, 1, 0)];
        let bounds2 = vec![[3.0, 0.0, 0.0], [5.0, 1.0, 1.0]];

        let mut counter = CountAllPairs { pair_count: 0 };
        let count = counter.select_pair(&tree1_nodes, &tree2_nodes, &bounds1, &bounds2);

        // 3 x 2 = 6 pairs
        assert_eq!(count, 6);
    }

    #[test]
    fn test_traverse_metric_better() {
        struct MetricTraverse {
            count: usize,
        }

        impl BvhBaseTraverse<f64> for MetricTraverse {
            fn is_metric_better(&self, left: f64, right: f64) -> bool {
                left < right // Smaller is better
            }
        }

        impl BvhTraverse<f64> for MetricTraverse {
            fn reject_node(&self, _min: &[f64; 3], _max: &[f64; 3], metric: &mut f64) -> bool {
                *metric = 0.0;
                false
            }

            fn accept(&mut self, _index: usize, _metric: f64) -> bool {
                self.count += 1;
                true
            }
        }

        let mut t = MetricTraverse { count: 0 };
        assert!(t.is_metric_better(0.5, 1.0));
        assert!(!t.is_metric_better(1.5, 1.0));
    }
}

// Re-export alloc if not using std
#[cfg(not(feature = "std"))]
extern crate alloc;

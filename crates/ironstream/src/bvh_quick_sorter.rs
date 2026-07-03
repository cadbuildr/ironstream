// FILE: bvh_quick_sorter.rs

// occt: BVH_QuickSorter
/// Performs centroid-based sorting of abstract set along the given axis (X - 0, Y - 1, Z - 2)
/// using a sorting algorithm that guarantees O(n log n) complexity.
pub struct BvhQuickSorter {
    /// Axis used to arrange the primitives (X - 0, Y - 1, Z - 2).
    axis: usize,
}

impl BvhQuickSorter {
    /// Creates new BVH quick sorter for the given axis.
    pub fn new(axis: usize) -> Self {
        BvhQuickSorter { axis }
    }

    /// Sorts the set by rearranging elements based on their center coordinates along the axis.
    /// This performs an in-place permutation using a cycle-based algorithm.
    pub fn perform<T: Copy + PartialOrd, F, S>(&self, set: &mut S, center_fn: F)
    where
        F: Fn(&S, usize, usize) -> T,
        S: SwappableSet,
    {
        let size = set.size();
        if size <= 1 {
            return;
        }

        // Step 1: Create index array for sorting
        let mut indices: Vec<usize> = (0..size).collect();

        // Step 2: Sort indices by center value using standard sort (O(n log n) guaranteed)
        let axis = self.axis;
        indices.sort_by(|&a, &b| {
            let center_a = center_fn(set, a, axis);
            let center_b = center_fn(set, b, axis);
            if center_a < center_b {
                std::cmp::Ordering::Less
            } else if center_a > center_b {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        // Step 3: Compute inverse permutation: inv_perm[i] = where element i should go
        let mut inv_perm = vec![0; size];
        for (i, &idx) in indices.iter().enumerate() {
            inv_perm[idx] = i;
        }

        // Step 4: Apply permutation using cycle-based algorithm - O(n) swaps total
        for i in 0..size {
            // Follow the cycle starting at position i
            while inv_perm[i] != i {
                let j = inv_perm[i];
                set.swap(i, j);
                inv_perm.swap(i, j);
            }
        }
    }

    /// Sorts the given (inclusive) range in the set.
    pub fn perform_range<T: Copy + PartialOrd, F, S>(
        &self,
        set: &mut S,
        start: usize,
        final_idx: usize,
        center_fn: F,
    ) where
        F: Fn(&S, usize, usize) -> T,
        S: SwappableSet,
    {
        let size = final_idx - start + 1;
        if size <= 1 {
            return;
        }

        // Step 1: Create index array for sorting
        let mut indices: Vec<usize> = (0..size).collect();

        // Step 2: Sort indices by center value
        let axis = self.axis;
        indices.sort_by(|&a, &b| {
            let center_a = center_fn(set, start + a, axis);
            let center_b = center_fn(set, start + b, axis);
            if center_a < center_b {
                std::cmp::Ordering::Less
            } else if center_a > center_b {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        // Step 3: Compute inverse permutation
        let mut inv_perm = vec![0; size];
        for (i, &idx) in indices.iter().enumerate() {
            inv_perm[idx] = i;
        }

        // Step 4: Apply permutation using cycle-based algorithm
        for i in 0..size {
            while inv_perm[i] != i {
                let j = inv_perm[i];
                set.swap(start + i, start + j);
                inv_perm.swap(i, j);
            }
        }
    }
}

/// Trait for sets that support swapping elements.
pub trait SwappableSet {
    fn size(&self) -> usize;
    fn swap(&mut self, i: usize, j: usize);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple test implementation of a swappable set
    struct TestSet {
        centroids: Vec<f64>,
    }

    impl TestSet {
        fn new(centroids: Vec<f64>) -> Self {
            TestSet { centroids }
        }
    }

    impl SwappableSet for TestSet {
        fn size(&self) -> usize {
            self.centroids.len()
        }

        fn swap(&mut self, i: usize, j: usize) {
            self.centroids.swap(i, j);
        }
    }

    #[test]
    fn test_constructor() {
        let _sorter_x = BvhQuickSorter::new(0);
        let _sorter_y = BvhQuickSorter::new(1);
        let _sorter_z = BvhQuickSorter::new(2);
        // Constructor should not crash
    }

    #[test]
    fn test_sort_empty_set() {
        let mut set = TestSet::new(vec![]);
        let sorter = BvhQuickSorter::new(0);

        // Sorting empty set should not crash
        sorter.perform(&mut set, |s, idx, _axis| s.centroids[idx]);
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_sort_single_element() {
        let mut set = TestSet::new(vec![1.5]);
        let sorter = BvhQuickSorter::new(0);
        sorter.perform(&mut set, |s, idx, _axis| s.centroids[idx]);

        // Single element should remain unchanged
        assert_eq!(set.size(), 1);
        assert!((set.centroids[0] - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_sort_along_x_axis() {
        // Create triangles with X centroids: 5, 1, 3
        let mut set = TestSet::new(vec![5.0, 1.0, 3.0]);
        let sorter = BvhQuickSorter::new(0);
        sorter.perform(&mut set, |s, idx, _axis| s.centroids[idx]);

        // After sorting: should be ordered 1, 3, 5
        assert!(set.centroids[0] < set.centroids[1]);
        assert!(set.centroids[1] < set.centroids[2]);

        // Verify actual values
        assert!((set.centroids[0] - 1.0).abs() < 1e-10);
        assert!((set.centroids[1] - 3.0).abs() < 1e-10);
        assert!((set.centroids[2] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_sort_along_y_axis() {
        let mut set = TestSet::new(vec![3.0, 1.0, 2.0]);
        let sorter = BvhQuickSorter::new(0); // In this test, axis doesn't affect single dimension
        sorter.perform(&mut set, |s, idx, _axis| s.centroids[idx]);

        // Should be ordered by centroid
        assert!(set.centroids[0] < set.centroids[1]);
        assert!(set.centroids[1] < set.centroids[2]);
    }

    #[test]
    fn test_sort_range_in_set() {
        // Create 5 elements with values: 0, 1, 2, 3, 4
        let mut set = TestSet::new(vec![0.5, 3.5, 2.5, 1.5, 4.5]);
        // Simulate: 0 unchanged, middle 3 reversed, 4 unchanged
        // Current: 0.5, 3.5, 2.5, 1.5, 4.5

        let sorter = BvhQuickSorter::new(0);
        sorter.perform_range(&mut set, 1, 3, |s, idx, _axis| s.centroids[idx]);

        // Elements 0 and 4 should remain unchanged
        assert!((set.centroids[0] - 0.5).abs() < 1e-10);
        assert!((set.centroids[4] - 4.5).abs() < 1e-10);

        // Elements 1-3 should be sorted: 1.5, 2.5, 3.5
        assert!((set.centroids[1] - 1.5).abs() < 1e-10);
        assert!((set.centroids[2] - 2.5).abs() < 1e-10);
        assert!((set.centroids[3] - 3.5).abs() < 1e-10);
    }

    #[test]
    fn test_already_sorted() {
        // Create already sorted elements
        let centroids: Vec<f64> = (0..10).map(|i| i as f64 + 0.5).collect();
        let mut set = TestSet::new(centroids);
        let sorter = BvhQuickSorter::new(0);
        sorter.perform(&mut set, |s, idx, _axis| s.centroids[idx]);

        // Should remain sorted
        for i in 0..9 {
            assert!(set.centroids[i] < set.centroids[i + 1]);
        }
    }

    #[test]
    fn test_reverse_sorted() {
        // Create reverse sorted elements
        let centroids: Vec<f64> = (0..10).map(|i| (9 - i) as f64 + 0.5).collect();
        let mut set = TestSet::new(centroids);
        let sorter = BvhQuickSorter::new(0);
        sorter.perform(&mut set, |s, idx, _axis| s.centroids[idx]);

        // Should become sorted
        for i in 0..9 {
            assert!(set.centroids[i] < set.centroids[i + 1]);
        }
    }

    #[test]
    fn test_duplicate_values() {
        // Create elements with duplicate centroids: 1, 2, 2, 2, 3
        let mut set = TestSet::new(vec![1.0, 2.0, 2.0, 2.0, 3.0]);
        let sorter = BvhQuickSorter::new(0);
        sorter.perform(&mut set, |s, idx, _axis| s.centroids[idx]);

        // Should be non-decreasing
        for i in 0..4 {
            assert!(set.centroids[i] <= set.centroids[i + 1]);
        }
    }

    #[test]
    fn test_all_same_value() {
        // All elements at same position
        let mut set = TestSet::new(vec![1.5, 1.5, 1.5, 1.5, 1.5]);
        let sorter = BvhQuickSorter::new(0);
        sorter.perform(&mut set, |s, idx, _axis| s.centroids[idx]);

        // All should have same centroid
        for i in 0..5 {
            assert!((set.centroids[i] - 1.5).abs() < 1e-10);
        }
    }

    #[test]
    fn test_large_dataset() {
        // Create 1000 elements with pseudo-random order
        let centroids: Vec<f64> = (0..1000).map(|i| ((i * 37) % 1000) as f64).collect();
        let mut set = TestSet::new(centroids);
        let sorter = BvhQuickSorter::new(0);
        sorter.perform(&mut set, |s, idx, _axis| s.centroids[idx]);

        // Verify sorted
        for i in 0..999 {
            assert!(set.centroids[i] <= set.centroids[i + 1]);
        }
    }

    #[test]
    fn test_two_elements() {
        // Two elements out of order
        let mut set = TestSet::new(vec![2.5, 0.5]);
        let sorter = BvhQuickSorter::new(0);
        sorter.perform(&mut set, |s, idx, _axis| s.centroids[idx]);

        assert!(set.centroids[0] < set.centroids[1]);
    }
}

// FILE: bvh_sorter.rs
// occt: BVH_Sorter // - Tool object to sort abstract primitive set

use crate::bvh_set::{BBox, SimpleBVHSet};
use crate::bvh_types::Vec3;

/// occt: BVH_Sorter // - Basic sorter for BVH sets with parallel flag support.
/// Performs sorting of primitive sets using Quicksort algorithm.
/// This is a template-like class in OCCT; we provide a concrete implementation for f64.
pub struct BVHSorter {
    is_parallel: bool,
}

impl BVHSorter {
    /// Creates a new BVH sorter with parallel execution disabled.
    pub fn new() -> Self {
        BVHSorter { is_parallel: false }
    }

    /// Creates a new BVH sorter with specified parallel setting.
    pub fn with_parallel(is_parallel: bool) -> Self {
        BVHSorter { is_parallel }
    }

    /// Returns whether parallel execution is enabled.
    pub fn is_parallel(&self) -> bool {
        self.is_parallel
    }

    /// Sets the parallel execution flag.
    pub fn set_parallel(&mut self, is_parallel: bool) {
        self.is_parallel = is_parallel;
    }

    /// Sorts the entire set using quicksort along axis 0.
    pub fn perform(&mut self, set: &mut SimpleBVHSet) {
        let size = set.size();
        if size > 1 {
            self.perform_range(set, 0, size - 1);
        }
    }

    /// Sorts the given inclusive range [start, final_index] in the set.
    pub fn perform_range(&mut self, set: &mut SimpleBVHSet, start: usize, final_index: usize) {
        if start < final_index {
            let pivot_index = self.partition(set, start, final_index, 0);

            if pivot_index > start {
                self.perform_range(set, start, pivot_index - 1);
            }
            if pivot_index < final_index {
                self.perform_range(set, pivot_index + 1, final_index);
            }
        }
    }

    /// Partitions the range using centroid along the specified axis as pivot.
    fn partition(
        &self,
        set: &mut SimpleBVHSet,
        start: usize,
        end: usize,
        axis: usize,
    ) -> usize {
        let mid = start + (end - start) / 2;
        let pivot_value = set.center(mid, axis);

        let mut i = start;
        let mut j = end;

        loop {
            while set.center(i, axis) < pivot_value {
                if i >= end {
                    break;
                }
                i += 1;
            }
            while set.center(j, axis) > pivot_value {
                if j <= start {
                    break;
                }
                j -= 1;
            }

            if i >= j {
                break;
            }

            set.swap(i, j);
            i += 1;
            if j > 0 {
                j -= 1;
            }
        }

        j
    }
}

impl Default for BVHSorter {
    fn default() -> Self {
        Self::new()
    }
}

// occt-ref: BVH_QuickSorter // - Optimized quicksort sorter with middle-element pivot selection.
/// Provides better average-case performance than basic quicksort.
pub struct BVHQuickSorter {
    is_parallel: bool,
}

impl BVHQuickSorter {
    /// Creates a new quicksort sorter with parallel execution disabled.
    pub fn new() -> Self {
        BVHQuickSorter { is_parallel: false }
    }

    /// Creates a new quicksort sorter with specified parallel setting.
    pub fn with_parallel(is_parallel: bool) -> Self {
        BVHQuickSorter { is_parallel }
    }

    /// Returns whether parallel execution is enabled.
    pub fn is_parallel(&self) -> bool {
        self.is_parallel
    }

    /// Sets the parallel execution flag.
    pub fn set_parallel(&mut self, is_parallel: bool) {
        self.is_parallel = is_parallel;
    }

    /// Sorts the entire set.
    pub fn perform(&mut self, set: &mut SimpleBVHSet) {
        let size = set.size();
        if size > 1 {
            self.perform_range(set, 0, size - 1);
        }
    }

    /// Sorts the given inclusive range [start, final_index] in the set.
    pub fn perform_range(&mut self, set: &mut SimpleBVHSet, start: usize, final_index: usize) {
        if start < final_index {
            let pivot_index = self.partition_quicksort(set, start, final_index, 0);

            if pivot_index > start {
                self.perform_range(set, start, pivot_index - 1);
            }
            if pivot_index < final_index {
                self.perform_range(set, pivot_index + 1, final_index);
            }
        }
    }

    /// Partitions the range using Hoare's partition scheme with middle-element pivot.
    fn partition_quicksort(
        &self,
        set: &mut SimpleBVHSet,
        start: usize,
        end: usize,
        axis: usize,
    ) -> usize {
        let mid = start + (end - start) / 2;
        let pivot_value = set.center(mid, axis);

        let mut i = start;
        let mut j = end;

        loop {
            while set.center(i, axis) < pivot_value {
                if i >= end {
                    break;
                }
                i += 1;
            }
            while set.center(j, axis) > pivot_value {
                if j <= start {
                    break;
                }
                j -= 1;
            }

            if i >= j {
                return j;
            }

            set.swap(i, j);
            i += 1;
            if j > 0 {
                j -= 1;
            }
        }
    }
}

impl Default for BVHQuickSorter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bvh_set::{BBox, SimpleBVHSet};
    use crate::bvh_types::Vec3;

    #[test]
    fn test_sorter_creation() {
        let sorter = BVHSorter::new();
        assert!(!sorter.is_parallel());
    }

    #[test]
    fn test_sorter_parallel_flag() {
        let mut sorter = BVHSorter::new();
        sorter.set_parallel(true);
        assert!(sorter.is_parallel());

        sorter.set_parallel(false);
        assert!(!sorter.is_parallel());
    }

    #[test]
    fn test_sorter_perform_full() {
        let mut set = SimpleBVHSet::new();

        let bbox1 = BBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let centroid1 = Vec3::new(3.0, 0.0, 0.0);

        let bbox2 = BBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0));
        let centroid2 = Vec3::new(1.0, 0.0, 0.0);

        let bbox3 = BBox::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(3.0, 3.0, 3.0));
        let centroid3 = Vec3::new(2.0, 0.0, 0.0);

        set.add_object(bbox1, centroid1);
        set.add_object(bbox2, centroid2);
        set.add_object(bbox3, centroid3);

        let mut sorter = BVHSorter::new();
        sorter.perform(&mut set);

        let center_0 = set.center(0, 0);
        let center_1 = set.center(1, 0);
        let center_2 = set.center(2, 0);

        assert!(center_0 <= center_1);
        assert!(center_1 <= center_2);
    }

    #[test]
    fn test_sorter_perform_range() {
        let mut set = SimpleBVHSet::new();

        let bbox1 = BBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let centroid1 = Vec3::new(5.0, 0.0, 0.0);

        let bbox2 = BBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0));
        let centroid2 = Vec3::new(2.0, 0.0, 0.0);

        let bbox3 = BBox::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(3.0, 3.0, 3.0));
        let centroid3 = Vec3::new(3.0, 0.0, 0.0);

        let bbox4 = BBox::new(Vec3::new(3.0, 3.0, 3.0), Vec3::new(4.0, 4.0, 4.0));
        let centroid4 = Vec3::new(4.0, 0.0, 0.0);

        set.add_object(bbox1, centroid1);
        set.add_object(bbox2, centroid2);
        set.add_object(bbox3, centroid3);
        set.add_object(bbox4, centroid4);

        let mut sorter = BVHSorter::new();
        sorter.perform_range(&mut set, 1, 2);

        let center_1 = set.center(1, 0);
        let center_2 = set.center(2, 0);
        assert!(center_1 <= center_2);
    }

    #[test]
    fn test_quicksorter_creation() {
        let sorter = BVHQuickSorter::new();
        assert!(!sorter.is_parallel());
    }

    #[test]
    fn test_quicksorter_perform_full() {
        let mut set = SimpleBVHSet::new();

        let bbox1 = BBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let centroid1 = Vec3::new(3.0, 0.0, 0.0);

        let bbox2 = BBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0));
        let centroid2 = Vec3::new(1.0, 0.0, 0.0);

        let bbox3 = BBox::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(3.0, 3.0, 3.0));
        let centroid3 = Vec3::new(2.0, 0.0, 0.0);

        set.add_object(bbox1, centroid1);
        set.add_object(bbox2, centroid2);
        set.add_object(bbox3, centroid3);

        let mut sorter = BVHQuickSorter::new();
        sorter.perform(&mut set);

        let center_0 = set.center(0, 0);
        let center_1 = set.center(1, 0);
        let center_2 = set.center(2, 0);

        assert!(center_0 <= center_1);
        assert!(center_1 <= center_2);
    }

    #[test]
    fn test_quicksorter_empty_set() {
        let mut set = SimpleBVHSet::new();
        let mut sorter = BVHQuickSorter::new();

        sorter.perform(&mut set);
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_quicksorter_single_element() {
        let mut set = SimpleBVHSet::new();
        let bbox = BBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let centroid = Vec3::new(0.5, 0.5, 0.5);
        set.add_object(bbox, centroid);

        let mut sorter = BVHQuickSorter::new();
        sorter.perform(&mut set);

        assert_eq!(set.size(), 1);
    }
}

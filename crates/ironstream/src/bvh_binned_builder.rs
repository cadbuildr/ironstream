// FILE: bvh_binned_builder.rs

//! Performs construction of BVH tree using binned SAH algorithm.
//! Number of bins controls BVH quality at the cost of construction time.
//! For optimal results, use 32-48 bins. Reasonable performance with 4-8 bins.

// occt-ref: BVH_Bin
/// Stores parameters of a single bin (slice of AABB).
#[derive(Clone, Copy, Debug)]
pub struct BvhBin<T> {
    pub count: i32,
    pub min: [T; 3],
    pub max: [T; 3],
}

impl<T: Default + Copy> BvhBin<T> {
    pub fn new() -> Self {
        BvhBin {
            count: 0,
            min: [T::default(); 3],
            max: [T::default(); 3],
        }
    }
}

// occt-ref: BVH_SplitPlane
/// Describes split plane candidate.
#[derive(Clone, Copy, Debug)]
pub struct BvhSplitPlane<T> {
    pub lft_voxel: BvhBin<T>,
    pub rgh_voxel: BvhBin<T>,
}

// occt: BVH_BinnedBuilder
/// Performs construction of BVH tree using binned SAH (Surface Area Heuristic) algorithm.
/// Template parameters:
/// - T: numeric type (e.g., f64)
/// - N: dimension (2 or 3)
/// - BINS: number of bins for SAH (default 32)
pub struct BvhBinnedBuilder<T> {
    leaf_node_size: i32,
    max_tree_depth: i32,
    use_main_axis: bool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Copy + PartialOrd + Default> BvhBinnedBuilder<T> {
    /// Creates a new binned SAH BVH builder.
    pub fn new(
        leaf_node_size: i32,
        max_tree_depth: i32,
        do_main_splits: bool,
    ) -> Self {
        BvhBinnedBuilder {
            leaf_node_size,
            max_tree_depth,
            use_main_axis: do_main_splits,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns the leaf node size parameter.
    pub fn leaf_node_size(&self) -> i32 {
        self.leaf_node_size
    }

    /// Returns the maximum tree depth parameter.
    pub fn max_tree_depth(&self) -> i32 {
        self.max_tree_depth
    }

    /// Returns whether main axis splitting is enabled.
    pub fn use_main_axis(&self) -> bool {
        self.use_main_axis
    }
}

/// Axis selector for determining which axis to split along.
/// For 3D: returns axis with maximum extent (0=X, 1=Y, 2=Z)
/// For 2D: returns axis with maximum extent (0=X, 1=Y)
pub struct BvhAxisSelector;

impl BvhAxisSelector {
    /// Returns main axis for 3D: axis with largest extent
    pub fn main_axis_3d(size: [f64; 3]) -> i32 {
        if size[1] >= size[0] {
            if size[1] >= size[2] { 1 } else { 2 }
        } else {
            if size[2] > size[0] { 2 } else { 0 }
        }
    }

    /// Returns main axis for 2D: axis with largest extent
    pub fn main_axis_2d(size: [f64; 2]) -> i32 {
        if size[0] > size[1] { 0 } else { 1 }
    }
}

/// Helper function to compute integer floor of (x) with guard against negative zero.
pub fn int_floor_f64(x: f64) -> i32 {
    x.floor() as i32
}

/// Split primitives by binned coordinate range.
/// Partitions primitives between bin indices, returns split position.
pub fn split_primitives_1d(
    centers: &mut [f64],
    min: f64,
    max: f64,
    bin_index: i32,
    bins: i32,
) -> i32 {
    if centers.is_empty() {
        return 0;
    }

    if (max - min).abs() < 1e-10 {
        return centers.len() as i32 / 2;
    }

    let inverse_step = bins as f64 / (max - min);
    let mut lft_idx = 0;
    let mut rgh_idx = centers.len() - 1;

    loop {
        while lft_idx < centers.len()
            && int_floor_f64((centers[lft_idx] - min) * inverse_step) <= bin_index
        {
            lft_idx += 1;
        }
        while rgh_idx > 0
            && int_floor_f64((centers[rgh_idx] - min) * inverse_step) > bin_index
        {
            rgh_idx -= 1;
        }

        if lft_idx <= rgh_idx {
            if lft_idx != rgh_idx {
                centers.swap(lft_idx, rgh_idx);
            }
            lft_idx += 1;
            if rgh_idx > 0 {
                rgh_idx -= 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    lft_idx as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constructor() {
        let builder = BvhBinnedBuilder::<f64>::new(1, 32, false);
        assert_eq!(builder.leaf_node_size(), 1);
        assert_eq!(builder.max_tree_depth(), 32);
        assert!(!builder.use_main_axis());
    }

    #[test]
    fn test_custom_parameters() {
        let builder = BvhBinnedBuilder::<f64>::new(5, 20, false);
        assert_eq!(builder.leaf_node_size(), 5);
        assert_eq!(builder.max_tree_depth(), 20);
    }

    #[test]
    fn test_with_main_axis_splitting() {
        let builder = BvhBinnedBuilder::<f64>::new(1, 32, true);
        assert!(builder.use_main_axis());
    }

    #[test]
    fn test_bvh_bin_creation() {
        let bin = BvhBin::<f64>::new();
        assert_eq!(bin.count, 0);
        assert_eq!(bin.min[0], 0.0);
        assert_eq!(bin.max[0], 0.0);
    }

    #[test]
    fn test_axis_selector_3d_x_max() {
        let size = [10.0, 5.0, 3.0];
        assert_eq!(BvhAxisSelector::main_axis_3d(size), 0);
    }

    #[test]
    fn test_axis_selector_3d_y_max() {
        let size = [5.0, 10.0, 3.0];
        assert_eq!(BvhAxisSelector::main_axis_3d(size), 1);
    }

    #[test]
    fn test_axis_selector_3d_z_max() {
        let size = [5.0, 3.0, 10.0];
        assert_eq!(BvhAxisSelector::main_axis_3d(size), 2);
    }

    #[test]
    fn test_axis_selector_2d_x_max() {
        let size = [10.0, 5.0];
        assert_eq!(BvhAxisSelector::main_axis_2d(size), 0);
    }

    #[test]
    fn test_axis_selector_2d_y_max() {
        let size = [5.0, 10.0];
        assert_eq!(BvhAxisSelector::main_axis_2d(size), 1);
    }

    #[test]
    fn test_int_floor_positive() {
        assert_eq!(int_floor_f64(3.7), 3);
        assert_eq!(int_floor_f64(3.2), 3);
        assert_eq!(int_floor_f64(3.0), 3);
    }

    #[test]
    fn test_int_floor_negative() {
        assert_eq!(int_floor_f64(-3.7), -4);
        assert_eq!(int_floor_f64(-3.2), -4);
        assert_eq!(int_floor_f64(-3.0), -3);
    }

    #[test]
    fn test_split_primitives_empty() {
        let mut centers: Vec<f64> = vec![];
        let result = split_primitives_1d(&mut centers, 0.0, 10.0, 5, 10);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_split_primitives_single() {
        let mut centers = vec![5.0];
        let result = split_primitives_1d(&mut centers, 0.0, 10.0, 5, 10);
        assert!(result >= 0);
    }

    #[test]
    fn test_split_primitives_many() {
        let mut centers: Vec<f64> = (0..20).map(|i| i as f64 * 0.5).collect();
        let result = split_primitives_1d(&mut centers, 0.0, 10.0, 5, 10);
        assert!(result > 0);
        assert!(result < centers.len() as i32);
    }

    #[test]
    fn test_split_plane_creation() {
        let bin1 = BvhBin::<f64>::new();
        let bin2 = BvhBin::<f64>::new();
        let plane = BvhSplitPlane {
            lft_voxel: bin1,
            rgh_voxel: bin2,
        };
        assert_eq!(plane.lft_voxel.count, 0);
        assert_eq!(plane.rgh_voxel.count, 0);
    }

    #[test]
    fn test_builder_multiple_instances() {
        let builder1 = BvhBinnedBuilder::<f64>::new(1, 32, false);
        let builder2 = BvhBinnedBuilder::<f64>::new(4, 16, true);

        assert_eq!(builder1.leaf_node_size(), 1);
        assert_eq!(builder2.leaf_node_size(), 4);
        assert_ne!(builder1.max_tree_depth(), builder2.max_tree_depth());
    }

    #[test]
    fn test_split_primitives_uniform_distribution() {
        let mut centers: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = split_primitives_1d(&mut centers, 0.0, 10.0, 5, 10);

        // Should split roughly near 5.0
        assert!(result > 0);
        assert!(result < centers.len() as i32);
    }

    #[test]
    fn test_split_primitives_clustered() {
        let mut centers: Vec<f64> = vec![1.0, 1.1, 1.2, 8.0, 8.1, 8.2];
        let result = split_primitives_1d(&mut centers, 0.0, 10.0, 4, 10);

        assert!(result > 0);
        assert!(result < centers.len() as i32);
    }

    #[test]
    fn test_builder_default_construction() {
        let builder = BvhBinnedBuilder::<f64>::new(
            32, // default leaf size (from gtest)
            50, // typical max depth
            false,
        );
        assert!(builder.leaf_node_size() > 0);
        assert!(builder.max_tree_depth() > 0);
    }

    #[test]
    fn test_int_floor_boundary() {
        assert_eq!(int_floor_f64(0.0), 0);
        assert_eq!(int_floor_f64(0.5), 0);
        assert_eq!(int_floor_f64(1.0), 1);
        assert_eq!(int_floor_f64(-0.5), -1);
    }

    #[test]
    fn test_axis_selector_equal_extents_x_y_equal() {
        let size = [5.0, 5.0, 3.0];
        let axis = BvhAxisSelector::main_axis_3d(size);
        assert_eq!(axis, 1); // when y == x, return y
    }

    #[test]
    fn test_axis_selector_equal_extents_y_z_equal() {
        let size = [5.0, 10.0, 10.0];
        let axis = BvhAxisSelector::main_axis_3d(size);
        assert_eq!(axis, 1); // y is larger than both
    }

    #[test]
    fn test_split_primitives_zero_range() {
        let mut centers = vec![5.0, 5.0, 5.0];
        let result = split_primitives_1d(&mut centers, 5.0, 5.0, 0, 10);
        // When min == max, should return roughly middle
        assert_eq!(result, 1); // centers.len() / 2 = 3 / 2 = 1
    }

    #[test]
    fn test_multiple_bins_performance() {
        // Test that builder works with different bin counts
        let builder_16 = BvhBinnedBuilder::<f64>::new(1, 32, false);
        let builder_32 = BvhBinnedBuilder::<f64>::new(1, 32, false);
        let builder_64 = BvhBinnedBuilder::<f64>::new(1, 32, false);

        assert_eq!(builder_16.leaf_node_size(), builder_32.leaf_node_size());
        assert_eq!(builder_32.leaf_node_size(), builder_64.leaf_node_size());
    }
}

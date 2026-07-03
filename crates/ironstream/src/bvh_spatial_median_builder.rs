// FILE: bvh_spatial_median_builder.rs

// occt: BVH_SpatialMedianBuilder

/// Performs building of BVH tree using spatial median split algorithm.
///
/// This builder is a specialization of BVH_BinnedBuilder with 2 bins, implementing
/// the spatial median split strategy. It divides the bounding box at the median
/// coordinate along the chosen axis rather than considering object distribution.
pub struct BvhSpatialMedianBuilder {
    /// Size of leaf nodes (max primitives per leaf).
    leaf_node_size: usize,
    /// Maximum tree depth allowed.
    max_tree_depth: usize,
    /// Whether to use main axis for split selection.
    use_main_axis: bool,
}

impl BvhSpatialMedianBuilder {
    /// Creates spatial median split builder with default parameters.
    pub fn new() -> Self {
        Self {
            leaf_node_size: 5, // BVH_Constants_LeafNodeSizeDefault
            max_tree_depth: 32, // BVH_Constants_MaxTreeDepth
            use_main_axis: false,
        }
    }

    /// Creates spatial median split builder with custom parameters.
    ///
    /// # Arguments
    /// * `leaf_node_size` - Maximum primitives in a leaf node
    /// * `max_tree_depth` - Maximum depth of the tree
    /// * `use_main_axis` - Whether to prefer splitting along the main (widest) axis
    pub fn with_params(leaf_node_size: usize, max_tree_depth: usize, use_main_axis: bool) -> Self {
        BvhSpatialMedianBuilder {
            leaf_node_size,
            max_tree_depth,
            use_main_axis,
        }
    }

    /// Returns the leaf node size.
    pub fn leaf_node_size(&self) -> usize {
        self.leaf_node_size
    }

    /// Returns the maximum tree depth.
    pub fn max_tree_depth(&self) -> usize {
        self.max_tree_depth
    }

    /// Returns whether main axis splitting is enabled.
    pub fn use_main_axis(&self) -> bool {
        self.use_main_axis
    }

    /// Selects the split axis based on configuration.
    /// If use_main_axis is true, returns the widest axis of the bounding box.
    /// Otherwise, cycles through axes in order (0, 1, 2, 0, 1, 2, ...).
    pub fn select_split_axis(&self, bbox_sizes: &[f64; 3], depth: usize) -> usize {
        if self.use_main_axis {
            // Find the axis with the maximum extent
            if bbox_sizes[0] >= bbox_sizes[1] && bbox_sizes[0] >= bbox_sizes[2] {
                0
            } else if bbox_sizes[1] >= bbox_sizes[2] {
                1
            } else {
                2
            }
        } else {
            // Cycle through axes based on depth
            depth % 3
        }
    }

    /// Computes the spatial median split position.
    /// Given a bounding box, returns the coordinate along the axis at the middle.
    pub fn compute_split_position(
        &self,
        bbox_min: &[f64; 3],
        bbox_max: &[f64; 3],
        axis: usize,
    ) -> f64 {
        (bbox_min[axis] + bbox_max[axis]) / 2.0
    }

    /// Classifies a centroid relative to the split plane.
    /// Returns true if the centroid is on the "left" (< split position) side.
    pub fn is_left_of_split(&self, centroid: &[f64; 3], split_pos: f64, axis: usize) -> bool {
        centroid[axis] < split_pos
    }
}

impl Default for BvhSpatialMedianBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constructor() {
        let builder = BvhSpatialMedianBuilder::new();
        assert_eq!(builder.leaf_node_size(), 5);
        assert_eq!(builder.max_tree_depth(), 32);
        assert!(!builder.use_main_axis());
    }

    #[test]
    fn test_custom_parameters() {
        let builder = BvhSpatialMedianBuilder::with_params(10, 20, true);
        assert_eq!(builder.leaf_node_size(), 10);
        assert_eq!(builder.max_tree_depth(), 20);
        assert!(builder.use_main_axis());
    }

    #[test]
    fn test_select_split_axis_main_axis_x() {
        let builder = BvhSpatialMedianBuilder::with_params(5, 32, true);
        let bbox_sizes = [10.0, 5.0, 3.0];
        assert_eq!(builder.select_split_axis(&bbox_sizes, 0), 0); // X is widest
    }

    #[test]
    fn test_select_split_axis_main_axis_y() {
        let builder = BvhSpatialMedianBuilder::with_params(5, 32, true);
        let bbox_sizes = [3.0, 10.0, 5.0];
        assert_eq!(builder.select_split_axis(&bbox_sizes, 0), 1); // Y is widest
    }

    #[test]
    fn test_select_split_axis_main_axis_z() {
        let builder = BvhSpatialMedianBuilder::with_params(5, 32, true);
        let bbox_sizes = [3.0, 5.0, 10.0];
        assert_eq!(builder.select_split_axis(&bbox_sizes, 0), 2); // Z is widest
    }

    #[test]
    fn test_select_split_axis_cycle_depth_0() {
        let builder = BvhSpatialMedianBuilder::with_params(5, 32, false);
        let bbox_sizes = [1.0, 1.0, 1.0];
        assert_eq!(builder.select_split_axis(&bbox_sizes, 0), 0);
    }

    #[test]
    fn test_select_split_axis_cycle_depth_1() {
        let builder = BvhSpatialMedianBuilder::with_params(5, 32, false);
        let bbox_sizes = [1.0, 1.0, 1.0];
        assert_eq!(builder.select_split_axis(&bbox_sizes, 1), 1);
    }

    #[test]
    fn test_select_split_axis_cycle_depth_2() {
        let builder = BvhSpatialMedianBuilder::with_params(5, 32, false);
        let bbox_sizes = [1.0, 1.0, 1.0];
        assert_eq!(builder.select_split_axis(&bbox_sizes, 2), 2);
    }

    #[test]
    fn test_select_split_axis_cycle_depth_3() {
        let builder = BvhSpatialMedianBuilder::with_params(5, 32, false);
        let bbox_sizes = [1.0, 1.0, 1.0];
        assert_eq!(builder.select_split_axis(&bbox_sizes, 3), 0); // Wraps around
    }

    #[test]
    fn test_compute_split_position() {
        let builder = BvhSpatialMedianBuilder::new();
        let bbox_min = [0.0, 0.0, 0.0];
        let bbox_max = [10.0, 20.0, 30.0];

        let split_x = builder.compute_split_position(&bbox_min, &bbox_max, 0);
        let split_y = builder.compute_split_position(&bbox_min, &bbox_max, 1);
        let split_z = builder.compute_split_position(&bbox_min, &bbox_max, 2);

        assert!((split_x - 5.0).abs() < 1e-10);
        assert!((split_y - 10.0).abs() < 1e-10);
        assert!((split_z - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_compute_split_position_negative() {
        let builder = BvhSpatialMedianBuilder::new();
        let bbox_min = [-10.0, -20.0, -30.0];
        let bbox_max = [10.0, 20.0, 30.0];

        let split_x = builder.compute_split_position(&bbox_min, &bbox_max, 0);
        let split_y = builder.compute_split_position(&bbox_min, &bbox_max, 1);
        let split_z = builder.compute_split_position(&bbox_min, &bbox_max, 2);

        assert!(split_x.abs() < 1e-10);
        assert!(split_y.abs() < 1e-10);
        assert!(split_z.abs() < 1e-10);
    }

    #[test]
    fn test_compute_split_position_asymmetric() {
        let builder = BvhSpatialMedianBuilder::new();
        let bbox_min = [0.0, 5.0, 10.0];
        let bbox_max = [2.0, 15.0, 30.0];

        let split_x = builder.compute_split_position(&bbox_min, &bbox_max, 0);
        let split_y = builder.compute_split_position(&bbox_min, &bbox_max, 1);
        let split_z = builder.compute_split_position(&bbox_min, &bbox_max, 2);

        assert!((split_x - 1.0).abs() < 1e-10);
        assert!((split_y - 10.0).abs() < 1e-10);
        assert!((split_z - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_left_of_split_x_axis() {
        let builder = BvhSpatialMedianBuilder::new();
        let centroid_left = [3.0, 10.0, 20.0];
        let centroid_right = [7.0, 10.0, 20.0];
        let split_pos = 5.0;

        assert!(builder.is_left_of_split(&centroid_left, split_pos, 0));
        assert!(!builder.is_left_of_split(&centroid_right, split_pos, 0));
    }

    #[test]
    fn test_is_left_of_split_y_axis() {
        let builder = BvhSpatialMedianBuilder::new();
        let centroid_left = [5.0, 8.0, 20.0];
        let centroid_right = [5.0, 12.0, 20.0];
        let split_pos = 10.0;

        assert!(builder.is_left_of_split(&centroid_left, split_pos, 1));
        assert!(!builder.is_left_of_split(&centroid_right, split_pos, 1));
    }

    #[test]
    fn test_is_left_of_split_z_axis() {
        let builder = BvhSpatialMedianBuilder::new();
        let centroid_left = [5.0, 10.0, 15.0];
        let centroid_right = [5.0, 10.0, 25.0];
        let split_pos = 20.0;

        assert!(builder.is_left_of_split(&centroid_left, split_pos, 2));
        assert!(!builder.is_left_of_split(&centroid_right, split_pos, 2));
    }

    #[test]
    fn test_is_left_of_split_on_boundary() {
        let builder = BvhSpatialMedianBuilder::new();
        let centroid_on_split = [5.0, 10.0, 20.0];
        let split_pos = 5.0;

        // Point exactly on split is not left of split
        assert!(!builder.is_left_of_split(&centroid_on_split, split_pos, 0));
    }

    #[test]
    fn test_multiple_axis_cycling() {
        let builder = BvhSpatialMedianBuilder::with_params(5, 32, false);
        let bbox_sizes = [1.0, 1.0, 1.0];

        // Test cycling through axes for multiple depths
        for depth in 0..9 {
            let expected_axis = depth % 3;
            assert_eq!(
                builder.select_split_axis(&bbox_sizes, depth),
                expected_axis
            );
        }
    }

    #[test]
    fn test_main_axis_with_tied_dimensions() {
        let builder = BvhSpatialMedianBuilder::with_params(5, 32, true);
        let bbox_sizes = [5.0, 5.0, 3.0]; // X and Y equal, Z smaller
        // Should pick X (first largest)
        assert_eq!(builder.select_split_axis(&bbox_sizes, 0), 0);
    }

    #[test]
    fn test_main_axis_with_all_equal() {
        let builder = BvhSpatialMedianBuilder::with_params(5, 32, true);
        let bbox_sizes = [5.0, 5.0, 5.0]; // All equal
        // Should pick X (first largest)
        assert_eq!(builder.select_split_axis(&bbox_sizes, 0), 0);
    }

    #[test]
    fn test_split_position_with_zero_size() {
        let builder = BvhSpatialMedianBuilder::new();
        let bbox_min = [5.0, 10.0, 15.0];
        let bbox_max = [5.0, 10.0, 15.0]; // Degenerate box

        let split_x = builder.compute_split_position(&bbox_min, &bbox_max, 0);
        let split_y = builder.compute_split_position(&bbox_min, &bbox_max, 1);
        let split_z = builder.compute_split_position(&bbox_min, &bbox_max, 2);

        assert!((split_x - 5.0).abs() < 1e-10);
        assert!((split_y - 10.0).abs() < 1e-10);
        assert!((split_z - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_float_precision() {
        let builder = BvhSpatialMedianBuilder::new();
        let bbox_min = [0.1, 0.2, 0.3];
        let bbox_max = [0.9, 1.8, 2.7];

        let split_x = builder.compute_split_position(&bbox_min, &bbox_max, 0);
        let split_y = builder.compute_split_position(&bbox_min, &bbox_max, 1);
        let split_z = builder.compute_split_position(&bbox_min, &bbox_max, 2);

        assert!((split_x - 0.5).abs() < 1e-10);
        assert!((split_y - 1.0).abs() < 1e-10);
        assert!((split_z - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_very_large_bbox() {
        let builder = BvhSpatialMedianBuilder::new();
        let bbox_min = [-1e6, -1e6, -1e6];
        let bbox_max = [1e6, 1e6, 1e6];

        let split_x = builder.compute_split_position(&bbox_min, &bbox_max, 0);
        let split_y = builder.compute_split_position(&bbox_min, &bbox_max, 1);
        let split_z = builder.compute_split_position(&bbox_min, &bbox_max, 2);

        assert!(split_x.abs() < 1e-5);
        assert!(split_y.abs() < 1e-5);
        assert!(split_z.abs() < 1e-5);
    }

    #[test]
    fn test_very_small_bbox() {
        let builder = BvhSpatialMedianBuilder::new();
        let bbox_min = [1e-6, 1e-6, 1e-6];
        let bbox_max = [2e-6, 2e-6, 2e-6];

        let split_x = builder.compute_split_position(&bbox_min, &bbox_max, 0);
        let split_y = builder.compute_split_position(&bbox_min, &bbox_max, 1);
        let split_z = builder.compute_split_position(&bbox_min, &bbox_max, 2);

        assert!((split_x - 1.5e-6).abs() < 1e-15);
        assert!((split_y - 1.5e-6).abs() < 1e-15);
        assert!((split_z - 1.5e-6).abs() < 1e-15);
    }

    #[test]
    fn test_centroid_classification_multiple_axes() {
        let builder = BvhSpatialMedianBuilder::new();
        let centroid = [3.0, 7.0, 15.0];

        // Test against different split positions on each axis
        assert!(builder.is_left_of_split(&centroid, 5.0, 0));
        assert!(builder.is_left_of_split(&centroid, 10.0, 1));
        assert!(builder.is_left_of_split(&centroid, 20.0, 2));

        assert!(!builder.is_left_of_split(&centroid, 2.0, 0));
        assert!(!builder.is_left_of_split(&centroid, 6.0, 1));
        assert!(!builder.is_left_of_split(&centroid, 10.0, 2));
    }

    #[test]
    fn test_negative_coordinates() {
        let builder = BvhSpatialMedianBuilder::new();
        let bbox_min = [-10.0, -20.0, -30.0];
        let bbox_max = [-2.0, -4.0, -6.0];

        let split_x = builder.compute_split_position(&bbox_min, &bbox_max, 0);
        let split_y = builder.compute_split_position(&bbox_min, &bbox_max, 1);
        let split_z = builder.compute_split_position(&bbox_min, &bbox_max, 2);

        assert!((split_x - (-6.0)).abs() < 1e-10);
        assert!((split_y - (-12.0)).abs() < 1e-10);
        assert!((split_z - (-18.0)).abs() < 1e-10);
    }
}

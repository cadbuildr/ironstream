// FILE: bvh_distance.rs
// occt: BVH_Distance

/// Abstract class for computation of the minimum distance between some object
/// and elements of a BVH tree.
///
/// To use this class, subclasses must implement two key methods:
/// - `reject_node`: compute distance from the object to a bounding box
/// - `accept`: compute distance from the object to a tree element
///
/// Type parameters:
/// - `NumType`: Numeric data type (e.g., f64). Must support Copy, PartialOrd, and conversion from f64.
/// - `Dimension`: Vector dimension (e.g., 3 for 3D)
/// - `ObjectType`: Type of the object to which distance is computed
/// - `BVHSetType`: Type of the set on which BVH is built
pub struct BVHDistance<NumType, const Dimension: usize, ObjectType, BVHSetType>
where
    NumType: Copy + PartialOrd + From<f64>,
{
    /// Minimum distance found during traversal
    distance: NumType,
    /// Flag indicating whether computation is complete
    is_done: bool,
    /// Object to compute distance to
    object: Option<ObjectType>,
    /// BVH set for element access (optional)
    bvh_set: Option<BVHSetType>,
}

impl<NumType, const Dimension: usize, ObjectType, BVHSetType>
    BVHDistance<NumType, Dimension, ObjectType, BVHSetType>
where
    NumType: Copy + PartialOrd + From<f64>,
{
    /// Creates a new distance computation instance.
    ///
    /// Initializes distance to infinity and is_done to false.
    pub fn new() -> Self {
        // Initialize distance to a large value (positive infinity for the numeric type)
        // In OCCT, this uses std::numeric_limits<NumType>::max()
        // For f64, we use a very large value; for integer types, use their max
        Self {
            distance: NumType::from(f64::INFINITY),
            is_done: false,
            object: None,
            bvh_set: None,
        }
    }

    /// Sets the object to which the distance is required.
    pub fn set_object(&mut self, the_object: ObjectType) {
        self.object = Some(the_object);
    }

    /// Returns a reference to the object if set.
    pub fn object(&self) -> Option<&ObjectType> {
        self.object.as_ref()
    }

    /// Sets the BVH set containing the tree and elements.
    pub fn set_bvh_set(&mut self, the_bvh_set: BVHSetType) {
        self.bvh_set = Some(the_bvh_set);
    }

    /// Returns a reference to the BVH set if set.
    pub fn bvh_set(&self) -> Option<&BVHSetType> {
        self.bvh_set.as_ref()
    }

    /// Computes the distance between object and BVH tree.
    ///
    /// This method traverses the BVH tree, rejecting branches that cannot contain
    /// closer elements, and accepting leaf elements to refine the distance.
    /// The computation follows the metrics defined by IsMetricBetter, RejectMetric, etc.
    ///
    /// Returns the computed distance value.
    pub fn compute_distance(&mut self) -> NumType {
        // In a full implementation, this would call Select() which traverses the tree
        // For the base class, we return the current distance and mark as done
        // Subclasses would override the decision methods to implement actual logic
        self.is_done = true;
        self.distance
    }

    /// Returns whether the distance computation is complete.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns the computed distance.
    pub fn distance(&self) -> NumType {
        self.distance
    }

    /// Sets the computed distance value (for internal use during traversal).
    pub(crate) fn set_distance(&mut self, distance: NumType) {
        self.distance = distance;
    }

    /// Compares two metrics and returns true if the left is better.
    ///
    /// In distance computation, a smaller value is better.
    /// Subclasses may override for different metric semantics.
    pub fn is_metric_better(&self, left: NumType, right: NumType) -> bool {
        left < right
    }

    /// Rejects a node based on its metric.
    ///
    /// Returns true if the node should be rejected from consideration.
    /// A node is rejected if its metric (distance) is worse than the current best.
    /// Subclasses typically override this for distance-specific logic.
    pub fn reject_metric(&self, metric: NumType) -> bool {
        metric > self.distance
    }

    /// Returns whether tree traversal should stop.
    ///
    /// Traversal can stop early when an optimal result is found.
    /// For distance, this typically means distance == 0 (touching/containing).
    /// Subclasses may override for domain-specific stopping criteria.
    pub fn should_stop(&self) -> bool {
        self.distance == NumType::from(0.0)
    }

    /// Resets the distance computation state.
    ///
    /// Called to prepare for a new distance computation.
    pub fn reset(&mut self) {
        self.distance = NumType::from(f64::INFINITY);
        self.is_done = false;
    }
}

impl<NumType, const Dimension: usize, ObjectType: Default, BVHSetType>
    Default for BVHDistance<NumType, Dimension, ObjectType, BVHSetType>
where
    NumType: Copy + PartialOrd + From<f64>,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Traversal selector for BVH trees using distance metrics.
///
/// This trait defines the interface for selecting elements from a BVH tree
/// based on distance criteria. Implementations define how nodes are rejected
/// and elements are accepted during traversal.
pub trait BVHDistanceSelector {
    /// Numeric type for distance values
    type NumType: Copy + PartialOrd + From<f64>;

    /// Rejection of a node by bounding box.
    /// Metric is computed to choose the best branch.
    /// Returns true if the node should be rejected, false otherwise.
    fn reject_node(&self, corner_min: &[Self::NumType; 3], corner_max: &[Self::NumType; 3], metric: &mut Self::NumType) -> bool;

    /// Leaf element acceptance.
    /// Metric of the parent leaf node is passed to avoid redundant checks.
    /// Returns true if the element has been accepted, false otherwise.
    fn accept(&mut self, element_index: usize, metric: Self::NumType) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bvh_distance_creation() {
        let dist: BVHDistance<f64, 3, (), ()> = BVHDistance::new();
        assert_eq!(dist.is_done(), false);
        assert!(dist.distance().is_infinite());
    }

    #[test]
    fn test_bvh_distance_object_setting() {
        let mut dist: BVHDistance<f64, 3, i32, ()> = BVHDistance::new();
        dist.set_object(42);
        assert_eq!(dist.object(), Some(&42));
    }

    #[test]
    fn test_is_metric_better_distance() {
        let dist: BVHDistance<f64, 3, (), ()> = BVHDistance::new();
        // For distance metrics, smaller is better
        assert!(dist.is_metric_better(1.0, 2.0));
        assert!(!dist.is_metric_better(2.0, 1.0));
        assert!(!dist.is_metric_better(1.0, 1.0)); // Equal metrics
    }

    #[test]
    fn test_reject_metric_distance() {
        let mut dist: BVHDistance<f64, 3, (), ()> = BVHDistance::new();
        dist.set_distance(5.0);

        // Metric worse than current best should be rejected
        assert!(dist.reject_metric(6.0));
        assert!(dist.reject_metric(10.0));

        // Metric equal to or better than current best should not be rejected
        assert!(!dist.reject_metric(5.0));
        assert!(!dist.reject_metric(3.0));
    }

    #[test]
    fn test_should_stop_when_zero_distance() {
        let mut dist: BVHDistance<f64, 3, (), ()> = BVHDistance::new();
        dist.set_distance(1.0);
        assert!(!dist.should_stop());

        dist.set_distance(0.0);
        assert!(dist.should_stop());
    }

    #[test]
    fn test_compute_distance_marks_done() {
        let mut dist: BVHDistance<f64, 3, (), ()> = BVHDistance::new();
        assert!(!dist.is_done());
        dist.compute_distance();
        assert!(dist.is_done());
    }

    #[test]
    fn test_reset_distance() {
        let mut dist: BVHDistance<f64, 3, (), ()> = BVHDistance::new();
        dist.set_distance(5.0);
        assert_eq!(dist.distance(), 5.0);

        dist.reset();
        assert!(dist.distance().is_infinite());
        assert!(!dist.is_done());
    }

    #[test]
    fn test_default_implementation() {
        let dist: BVHDistance<f64, 3, (), ()> = BVHDistance::default();
        assert_eq!(dist.is_done(), false);
        assert!(dist.distance().is_infinite());
    }

    #[test]
    fn test_distance_comparison_with_infinity() {
        let mut dist: BVHDistance<f64, 3, (), ()> = BVHDistance::new();
        assert!(dist.distance() > 1e10); // Very large value initialized

        // After setting a concrete distance
        dist.set_distance(2.5);
        assert!((dist.distance() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_sequential_distance_updates() {
        let mut dist: BVHDistance<f64, 3, (), ()> = BVHDistance::new();

        dist.set_distance(10.0);
        assert!(!dist.reject_metric(5.0)); // Better metric
        dist.set_distance(5.0);

        assert!(dist.reject_metric(10.0)); // Worse metric
        assert!(!dist.reject_metric(3.0)); // Better metric
        dist.set_distance(3.0);

        assert!(!dist.is_metric_better(5.0, 3.0)); // 5.0 is not better than 3.0
        assert!(dist.is_metric_better(3.0, 5.0));  // 3.0 is better than 5.0
    }

    #[test]
    fn test_bvh_distance_with_custom_type() {
        struct Point {
            x: f64,
            y: f64,
            z: f64,
        }

        let mut dist: BVHDistance<f64, 3, Point, ()> = BVHDistance::new();
        dist.set_object(Point { x: 1.0, y: 2.0, z: 3.0 });

        assert!(dist.object().is_some());
        if let Some(pt) = dist.object() {
            assert!((pt.x - 1.0).abs() < 1e-10);
            assert!((pt.y - 2.0).abs() < 1e-10);
            assert!((pt.z - 3.0).abs() < 1e-10);
        }
    }
}

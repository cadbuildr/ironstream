// FILE: select_mgr_axis_intersector.rs
// occt: SelectMgr_AxisIntersector

/// Simple 3D point representation for internal use
#[derive(Clone, Copy, Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Simple 3D direction representation for internal use
#[derive(Clone, Copy, Debug)]
struct Direction3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Direction3D {
    /// Models gp_Dir: normalizes and, like the gp_Dir constructor, refuses a
    /// null vector (Standard_ConstructionError in OCCT).
    fn new(x: f64, y: f64, z: f64) -> Self {
        let len = (x * x + y * y + z * z).sqrt();
        assert!(len > 0.0, "gp_Dir: null vector");
        Direction3D {
            x: x / len,
            y: y / len,
            z: z / len,
        }
    }

    fn dot(&self, other: &Direction3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// gp_Dir::Angle: angle between two unit vectors, in [0, PI].
    fn angle(&self, other: &Direction3D) -> f64 {
        self.dot(other).clamp(-1.0, 1.0).acos()
    }

    /// gp_Dir::IsEqual with an angular tolerance.
    fn is_equal_to(&self, other: &Direction3D, angular_tolerance: f64) -> bool {
        self.angle(other) <= angular_tolerance
    }
}

/// Precision::Angular() in OCCT.
const PRECISION_ANGULAR: f64 = 1.0e-12;

/// This class contains representation of selecting axis, created in case of point selection
/// and algorithms for overlap detection between this axis and sensitive entities.
pub struct SelectMgrAxisIntersector {
    axis_location: Point3D,
    axis_direction: Direction3D,
}

impl SelectMgrAxisIntersector {
    /// Empty constructor
    pub fn new() -> Self {
        SelectMgrAxisIntersector {
            axis_location: Point3D::new(0.0, 0.0, 0.0),
            axis_direction: Direction3D::new(0.0, 0.0, 1.0),
        }
    }

    /// Initializes selecting axis according to the input
    pub fn init(&mut self, loc_x: f64, loc_y: f64, loc_z: f64, dir_x: f64, dir_y: f64, dir_z: f64) {
        self.axis_location = Point3D::new(loc_x, loc_y, loc_z);
        self.axis_direction = Direction3D::new(dir_x, dir_y, dir_z);
    }

    /// Builds axis according to internal parameters.
    /// NOTE: this is a no-op for axis intersector as axis is already defined
    pub fn build(&mut self) {
        // Axis intersector doesn't need to build anything
    }

    /// Sets camera (does nothing for axis intersector, not applicable to this volume)
    pub fn set_camera(&mut self) {
        // Base implementation does nothing
    }

    /// Returns FALSE (not applicable to this volume)
    pub fn is_scalable(&self) -> bool {
        false
    }

    /// Returns near point along axis
    pub fn get_near_pnt(&self) -> (f64, f64, f64) {
        (self.axis_location.x, self.axis_location.y, self.axis_location.z)
    }

    /// Returns far point along axis (infinite)
    /// For an axis, we represent infinity by extending far along the direction
    pub fn get_far_pnt(&self) -> (f64, f64, f64) {
        let large = 1e10;
        (
            self.axis_location.x + self.axis_direction.x * large,
            self.axis_location.y + self.axis_direction.y * large,
            self.axis_location.z + self.axis_direction.z * large,
        )
    }

    /// Returns axis direction
    pub fn get_view_ray_direction(&self) -> (f64, f64, f64) {
        (
            self.axis_direction.x,
            self.axis_direction.y,
            self.axis_direction.z,
        )
    }

    /// Measures distance between start axis point and given point
    pub fn dist_to_geometry_center(&self, point_x: f64, point_y: f64, point_z: f64) -> f64 {
        let point = Point3D::new(point_x, point_y, point_z);
        self.axis_location.distance_to(&point)
    }

    /// Calculates the point on the axis ray that was detected during selection by given depth
    pub fn detected_point(&self, depth: f64) -> (f64, f64, f64) {
        (
            self.axis_location.x + self.axis_direction.x * depth,
            self.axis_location.y + self.axis_direction.y * depth,
            self.axis_location.z + self.axis_direction.z * depth,
        )
    }

    /// Check if a point has intersection with the axis: the vector from the
    /// axis origin to the point must be co-directed with the axis direction
    /// (compared with Precision::Angular(), as in hasIntersection).
    pub fn has_intersection_point(&self, point_x: f64, point_y: f64, point_z: f64) -> bool {
        let point = Point3D::new(point_x, point_y, point_z);
        let vec_to_point = Direction3D::new(
            point.x - self.axis_location.x,
            point.y - self.axis_location.y,
            point.z - self.axis_location.z,
        );
        self.axis_direction
            .is_equal_to(&vec_to_point, PRECISION_ANGULAR)
    }

    /// Intersection test between defined axis and axis-aligned bounding box
    pub fn overlaps_box(
        &self,
        box_min_x: f64,
        box_min_y: f64,
        box_min_z: f64,
        box_max_x: f64,
        box_max_y: f64,
        box_max_z: f64,
    ) -> bool {
        // Ray-box intersection test (BVH_Tools::RayBoxIntersection).
        let mut t_enter: f64 = f64::MIN;
        let mut t_leave: f64 = f64::MAX;

        // Check each axis
        for axis in 0..3 {
            let (dir_comp, loc_comp) = match axis {
                0 => (self.axis_direction.x, self.axis_location.x),
                1 => (self.axis_direction.y, self.axis_location.y),
                _ => (self.axis_direction.z, self.axis_location.z),
            };

            let (min_comp, max_comp) = match axis {
                0 => (box_min_x, box_max_x),
                1 => (box_min_y, box_max_y),
                _ => (box_min_z, box_max_z),
            };

            // Parallel ray: origin must lie within the slab.
            if (1.0 / dir_comp).is_infinite() {
                if loc_comp < min_comp || loc_comp > max_comp {
                    return false;
                }
            } else {
                let t1 = (min_comp - loc_comp) / dir_comp;
                let t2 = (max_comp - loc_comp) / dir_comp;
                t_enter = t_enter.max(t1.min(t2));
                t_leave = t_leave.min(t1.max(t2));
                if t_enter > t_leave {
                    return false;
                }
            }
        }

        // Reject an intersection fully behind the ray origin.
        t_leave >= 0.0
    }

    /// Intersection test between defined axis and given point (simple inclusion test)
    pub fn overlaps_point(&self, point_x: f64, point_y: f64, point_z: f64) -> bool {
        self.has_intersection_point(point_x, point_y, point_z)
    }
}

impl Default for SelectMgrAxisIntersector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis_intersector_creation() {
        let intersector = SelectMgrAxisIntersector::new();
        let (x, y, z) = intersector.get_near_pnt();
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }

    #[test]
    fn test_init_axis() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(1.0, 2.0, 3.0, 1.0, 0.0, 0.0);

        let (x, y, z) = intersector.get_near_pnt();
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);

        let (dx, dy, dz) = intersector.get_view_ray_direction();
        assert!((dx - 1.0).abs() < 1e-6);
        assert!(dy.abs() < 1e-6);
        assert!(dz.abs() < 1e-6);
    }

    #[test]
    fn test_is_scalable() {
        let intersector = SelectMgrAxisIntersector::new();
        assert!(!intersector.is_scalable());
    }

    #[test]
    fn test_get_far_pnt() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        let (fx, fy, fz) = intersector.get_far_pnt();
        // Far point should be along the direction
        assert!(fx > 1e9);
        assert!(fy.abs() < 1.0);
        assert!(fz.abs() < 1.0);
    }

    #[test]
    fn test_detected_point() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        let (x, y, z) = intersector.detected_point(5.0);
        assert!((x - 5.0).abs() < 1e-6);
        assert!(y.abs() < 1e-6);
        assert!(z.abs() < 1e-6);
    }

    #[test]
    fn test_dist_to_geometry_center() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        let dist = intersector.dist_to_geometry_center(3.0, 4.0, 0.0);
        assert!((dist - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_has_intersection_point_on_axis() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Point along the axis
        assert!(intersector.has_intersection_point(5.0, 0.0, 0.0));
    }

    #[test]
    fn test_has_intersection_point_off_axis() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Point not on the axis
        assert!(!intersector.has_intersection_point(0.0, 1.0, 1.0));
    }

    #[test]
    fn test_overlaps_box_containing_axis() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Box containing the axis
        assert!(intersector.overlaps_box(-1.0, -1.0, -1.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn test_overlaps_box_intersecting_axis() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Box partially overlapping the axis
        assert!(intersector.overlaps_box(-0.5, -0.5, -0.5, 2.0, 0.5, 0.5));
    }

    #[test]
    fn test_overlaps_box_not_intersecting() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Box completely away from the axis
        assert!(!intersector.overlaps_box(0.0, 2.0, -1.0, 1.0, 3.0, 1.0));
    }

    #[test]
    fn test_overlaps_box_behind_ray_origin() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Box entirely behind the ray origin: rejected (ray, not line).
        assert!(!intersector.overlaps_box(-3.0, -0.5, -0.5, -1.0, 0.5, 0.5));
    }

    #[test]
    fn test_overlaps_box_origin_inside() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Origin inside the box: t_enter < 0 < t_leave, accepted.
        assert!(intersector.overlaps_box(-1.0, -1.0, -1.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn test_overlaps_point_on_axis() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        assert!(intersector.overlaps_point(10.0, 0.0, 0.0));
    }

    #[test]
    fn test_overlaps_point_off_axis() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        assert!(!intersector.overlaps_point(0.0, 1.0, 1.0));
    }

    #[test]
    fn test_build_is_noop() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.init(1.0, 2.0, 3.0, 1.0, 0.0, 0.0);
        intersector.build(); // Should not panic or change state
        let (x, y, z) = intersector.get_near_pnt();
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
    }

    #[test]
    fn test_set_camera_is_noop() {
        let mut intersector = SelectMgrAxisIntersector::new();
        intersector.set_camera(); // Should not panic
    }
}

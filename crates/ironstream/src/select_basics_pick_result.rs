// FILE: select_basics_pick_result.rs
// occt: SelectBasics_PickResult

/// Vector3 type for surface normals.
#[derive(Debug, Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn set_values(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

/// Result of a pick/selection operation.
/// Contains depth, distance to center, picked point, and surface normal.
#[derive(Debug, Clone)]
pub struct SelectBasicsPickResult {
    /// The 3D point on the entity that was picked (in local coordinates)
    picked_point: (f64, f64, f64),
    /// Depth along the picking ray
    depth: f64,
    /// Distance from 3D projection of picked screen point to entity's geometry center
    dist_to_center: f64,
    /// Surface normal at picked point (unnormalized)
    normal: Vec3f,
}

impl SelectBasicsPickResult {
    /// Creates an invalid pick result with infinite depth.
    pub fn new_invalid() -> Self {
        SelectBasicsPickResult {
            picked_point: (f64::MAX, 0.0, 0.0),
            depth: f64::MAX,
            dist_to_center: f64::MAX,
            normal: Vec3f::zero(),
        }
    }

    /// Creates a pick result with specified values.
    pub fn new(
        depth: f64,
        dist_to_center: f64,
        picked_point: (f64, f64, f64),
    ) -> Self {
        SelectBasicsPickResult {
            picked_point,
            depth,
            dist_to_center,
            normal: Vec3f::zero(),
        }
    }

    /// Returns true if this result is valid (depth != MAX).
    pub fn is_valid(&self) -> bool {
        self.depth != f64::MAX
    }

    /// Invalidates this result by setting depth to MAX.
    pub fn invalidate(&mut self) {
        self.depth = f64::MAX;
        self.picked_point = (f64::MAX, 0.0, 0.0);
        self.normal = Vec3f::zero();
    }

    /// Returns the depth along the picking ray.
    pub fn depth(&self) -> f64 {
        self.depth
    }

    /// Sets the depth along the picking ray.
    pub fn set_depth(&mut self, depth: f64) {
        self.depth = depth;
    }

    /// Returns true if a picked point was set.
    pub fn has_picked_point(&self) -> bool {
        self.picked_point.0 != f64::MAX
    }

    /// Returns the picked point in local coordinates.
    pub fn picked_point(&self) -> (f64, f64, f64) {
        self.picked_point
    }

    /// Sets the picked point.
    pub fn set_picked_point(&mut self, point: (f64, f64, f64)) {
        self.picked_point = point;
    }

    /// Returns the distance to geometry center.
    pub fn dist_to_geom_center(&self) -> f64 {
        self.dist_to_center
    }

    /// Sets the distance to geometry center.
    pub fn set_dist_to_geom_center(&mut self, dist: f64) {
        self.dist_to_center = dist;
    }

    /// Returns the surface normal at the picked point (in local coordinates).
    pub fn surface_normal(&self) -> Vec3f {
        self.normal
    }

    /// Sets the surface normal.
    pub fn set_surface_normal(&mut self, normal: Vec3f) {
        self.normal = normal;
    }

    /// Sets the surface normal from a tuple.
    pub fn set_surface_normal_from_tuple(&mut self, normal: (f32, f32, f32)) {
        self.normal.set_values(normal.0, normal.1, normal.2);
    }

    /// Returns the closer of two pick results (by depth).
    pub fn min<'a>(result1: &'a SelectBasicsPickResult, result2: &'a SelectBasicsPickResult) -> &'a SelectBasicsPickResult {
        if result1.depth() <= result2.depth() {
            result1
        } else {
            result2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_invalid() {
        let result = SelectBasicsPickResult::new_invalid();
        assert!(!result.is_valid());
        assert_eq!(result.depth(), f64::MAX);
    }

    #[test]
    fn test_new_valid() {
        let result = SelectBasicsPickResult::new(10.5, 2.0, (1.0, 2.0, 3.0));
        assert!(result.is_valid());
        assert_eq!(result.depth(), 10.5);
        assert_eq!(result.dist_to_geom_center(), 2.0);
        assert_eq!(result.picked_point(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_invalidate() {
        let mut result = SelectBasicsPickResult::new(5.0, 1.0, (1.0, 1.0, 1.0));
        assert!(result.is_valid());

        result.invalidate();
        assert!(!result.is_valid());
    }

    #[test]
    fn test_set_depth() {
        let mut result = SelectBasicsPickResult::new_invalid();
        result.set_depth(15.0);
        assert_eq!(result.depth(), 15.0);
    }

    #[test]
    fn test_has_picked_point() {
        let mut result = SelectBasicsPickResult::new_invalid();
        assert!(!result.has_picked_point());

        result.set_picked_point((5.0, 5.0, 5.0));
        assert!(result.has_picked_point());
    }

    #[test]
    fn test_set_picked_point() {
        let mut result = SelectBasicsPickResult::new_invalid();
        result.set_picked_point((7.0, 8.0, 9.0));
        assert_eq!(result.picked_point(), (7.0, 8.0, 9.0));
    }

    #[test]
    fn test_dist_to_geom_center() {
        let mut result = SelectBasicsPickResult::new(5.0, 1.0, (1.0, 1.0, 1.0));
        assert_eq!(result.dist_to_geom_center(), 1.0);

        result.set_dist_to_geom_center(3.5);
        assert_eq!(result.dist_to_geom_center(), 3.5);
    }

    #[test]
    fn test_surface_normal() {
        let mut result = SelectBasicsPickResult::new_invalid();
        let normal = Vec3f::new(0.0, 1.0, 0.0);
        result.set_surface_normal(normal);

        let retrieved = result.surface_normal();
        assert_eq!(retrieved.x, 0.0);
        assert_eq!(retrieved.y, 1.0);
        assert_eq!(retrieved.z, 0.0);
    }

    #[test]
    fn test_set_surface_normal_from_tuple() {
        let mut result = SelectBasicsPickResult::new_invalid();
        result.set_surface_normal_from_tuple((1.0, 0.0, 0.0));

        let normal = result.surface_normal();
        assert_eq!(normal.x, 1.0);
        assert_eq!(normal.y, 0.0);
        assert_eq!(normal.z, 0.0);
    }

    #[test]
    fn test_min() {
        let result1 = SelectBasicsPickResult::new(5.0, 1.0, (1.0, 1.0, 1.0));
        let result2 = SelectBasicsPickResult::new(10.0, 2.0, (2.0, 2.0, 2.0));

        let closer = SelectBasicsPickResult::min(&result1, &result2);
        assert_eq!(closer.depth(), 5.0);
    }

    #[test]
    fn test_min_equal_depth() {
        let result1 = SelectBasicsPickResult::new(5.0, 1.0, (1.0, 1.0, 1.0));
        let result2 = SelectBasicsPickResult::new(5.0, 2.0, (2.0, 2.0, 2.0));

        let closer = SelectBasicsPickResult::min(&result1, &result2);
        assert_eq!(closer.depth(), 5.0);
        assert_eq!(closer.dist_to_geom_center(), 1.0); // result1 is returned
    }
}

// FILE: bvh_tools.rs
// occt: BVH_Tools

//! Static utility methods for BVH operations including distance calculations
//! and ray-box intersection tests.

use core::f64;

/// Projection state for point-triangle projection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BvhProjectionState {
    /// Point projects to a vertex
    Vertex,
    /// Point projects to an edge
    Edge,
    /// Point projects to the interior of the triangle
    Interior,
}

/// occt: BVH_Tools
///
/// Static methods for geometric computations on bounding boxes, points, triangles, and rays.
pub struct BvhTools;

impl BvhTools {
    /// Compute squared distance between two axis-aligned bounding boxes
    pub fn box_box_square_distance(
        min1: &[f64; 3],
        max1: &[f64; 3],
        min2: &[f64; 3],
        max2: &[f64; 3],
    ) -> f64 {
        let mut dist_sq = 0.0;

        for i in 0..3 {
            if min1[i] > max2[i] {
                let d = min1[i] - max2[i];
                dist_sq += d * d;
            } else if max1[i] < min2[i] {
                let d = min2[i] - max1[i];
                dist_sq += d * d;
            }
        }

        dist_sq
    }

    /// Compute squared distance from a point to an axis-aligned bounding box
    pub fn point_box_square_distance(
        point: &[f64; 3],
        box_min: &[f64; 3],
        box_max: &[f64; 3],
    ) -> f64 {
        let mut dist_sq = 0.0;

        for i in 0..3 {
            if point[i] < box_min[i] {
                let d = box_min[i] - point[i];
                dist_sq += d * d;
            } else if point[i] > box_max[i] {
                let d = point[i] - box_max[i];
                dist_sq += d * d;
            }
        }

        dist_sq
    }

    /// Project a point onto an axis-aligned bounding box (clamp point to box)
    pub fn point_box_projection(
        point: &[f64; 3],
        box_min: &[f64; 3],
        box_max: &[f64; 3],
    ) -> [f64; 3] {
        [
            point[0].max(box_min[0]).min(box_max[0]),
            point[1].max(box_min[1]).min(box_max[1]),
            point[2].max(box_min[2]).min(box_max[2]),
        ]
    }

    /// Compute intersection of a ray with an axis-aligned bounding box
    ///
    /// Uses the slab method with early exit optimization.
    /// Returns (enter_time, leave_time) if intersection occurs, None otherwise.
    pub fn ray_box_intersection(
        ray_origin: &[f64; 3],
        ray_direction: &[f64; 3],
        box_min: &[f64; 3],
        box_max: &[f64; 3],
    ) -> Option<(f64, f64)> {
        let mut enter_time = f64::NEG_INFINITY;
        let mut leave_time = f64::INFINITY;

        for i in 0..3 {
            if ray_direction[i] == 0.0 {
                // Ray is parallel to this axis slab
                if ray_origin[i] < box_min[i] || ray_origin[i] > box_max[i] {
                    return None; // Ray misses the slab
                }
                // Ray is within the slab, doesn't constrain intersection
                continue;
            }

            // Compute intersection distances for this axis
            let t1 = (box_min[i] - ray_origin[i]) / ray_direction[i];
            let t2 = (box_max[i] - ray_origin[i]) / ray_direction[i];

            // Ensure t_min <= t_max
            let (t_min, t_max) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

            // Update intersection interval
            enter_time = enter_time.max(t_min);
            leave_time = leave_time.min(t_max);

            // Early exit if no intersection
            if enter_time > leave_time {
                return None;
            }
        }

        // Check if intersection is behind the ray origin
        if leave_time < 0.0 {
            return None;
        }

        Some((enter_time, leave_time))
    }

    /// Project a point onto a triangle using Voronoi region testing
    ///
    /// Returns the closest point on the triangle to the given point.
    /// Optionally computes the projection state and edge/vertex indices.
    pub fn point_triangle_projection(
        point: &[f64; 3],
        v0: &[f64; 3],
        v1: &[f64; 3],
        v2: &[f64; 3],
    ) -> [f64; 3] {
        // Compute edge vectors
        let ab = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let ac = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
        let bc = [v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]];

        // Compute point-to-vertex vectors
        let ap = [point[0] - v0[0], point[1] - v0[1], point[2] - v0[2]];
        let bp = [point[0] - v1[0], point[1] - v1[1], point[2] - v1[2]];
        let cp = [point[0] - v2[0], point[1] - v2[1], point[2] - v2[2]];

        // Compute dot products for Voronoi region tests
        let ab_dot_ap = Self::dot(&ab, &ap);
        let ac_dot_ap = Self::dot(&ac, &ap);

        // Check if P is in vertex region outside V0
        if ab_dot_ap <= 0.0 && ac_dot_ap <= 0.0 {
            return [v0[0], v0[1], v0[2]];
        }

        let ba_dot_bp = -Self::dot(&ab, &bp);
        let bc_dot_bp = Self::dot(&bc, &bp);

        // Check if P is in vertex region outside V1
        if ba_dot_bp <= 0.0 && bc_dot_bp <= 0.0 {
            return [v1[0], v1[1], v1[2]];
        }

        let cb_dot_cp = -Self::dot(&bc, &cp);
        let ca_dot_cp = -Self::dot(&ac, &cp);

        // Check if P is in vertex region outside V2
        if ca_dot_cp <= 0.0 && cb_dot_cp <= 0.0 {
            return [v2[0], v2[1], v2[2]];
        }

        // Compute barycentric coordinates for edge/interior tests
        let ac_dot_bp = Self::dot(&ac, &bp);
        let vc = ab_dot_ap * ac_dot_bp + ba_dot_bp * ac_dot_ap;

        // Check if P is in edge region of AB
        if vc <= 0.0 && ab_dot_ap > 0.0 && ba_dot_bp > 0.0 {
            return Self::project_to_edge(v0, &ab, ab_dot_ap, ba_dot_bp);
        }

        let ab_dot_cp = Self::dot(&ab, &cp);
        let va = ba_dot_bp * ca_dot_cp - ab_dot_cp * ac_dot_bp;

        // Check if P is in edge region of BC
        if va <= 0.0 && bc_dot_bp > 0.0 && cb_dot_cp > 0.0 {
            return Self::project_to_edge(v1, &bc, bc_dot_bp, cb_dot_cp);
        }

        let vb = ab_dot_cp * ac_dot_ap + ab_dot_ap * ca_dot_cp;

        // Check if P is in edge region of CA
        if vb <= 0.0 && ac_dot_ap > 0.0 && ca_dot_cp > 0.0 {
            return Self::project_to_edge(v0, &ac, ac_dot_ap, ca_dot_cp);
        }

        // P is inside triangle - compute barycentric projection
        let norm = va + vb + vc;

        // Handle degenerate triangle (near-zero area)
        if norm.abs() <= f64::EPSILON * (va.abs() + vb.abs() + vc.abs()).max(1e-16) {
            // Return centroid
            return [
                (v0[0] + v1[0] + v2[0]) / 3.0,
                (v0[1] + v1[1] + v2[1]) / 3.0,
                (v0[2] + v1[2] + v2[2]) / 3.0,
            ];
        }

        // Barycentric interpolation
        [
            (v0[0] * va + v1[0] * vb + v2[0] * vc) / norm,
            (v0[1] * va + v1[1] * vb + v2[1] * vc) / norm,
            (v0[2] * va + v1[2] * vb + v2[2] * vc) / norm,
        ]
    }

    /// Compute squared distance from a point to a triangle
    pub fn point_triangle_square_distance(
        point: &[f64; 3],
        v0: &[f64; 3],
        v1: &[f64; 3],
        v2: &[f64; 3],
    ) -> f64 {
        let proj = Self::point_triangle_projection(point, v0, v1, v2);
        let dx = proj[0] - point[0];
        let dy = proj[1] - point[1];
        let dz = proj[2] - point[2];
        dx * dx + dy * dy + dz * dz
    }

    // Helper: compute dot product of two 3D vectors
    #[inline]
    fn dot(a: &[f64; 3], b: &[f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    // Helper: project point onto edge
    fn project_to_edge(
        edge_start: &[f64; 3],
        edge: &[f64; 3],
        dot1: f64,
        dot2: f64,
    ) -> [f64; 3] {
        let t = dot1 / (dot1 + dot2);
        [
            edge_start[0] + edge[0] * t,
            edge_start[1] + edge[1] * t,
            edge_start[2] + edge[2] * t,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_point_box_square_distance_inside() {
        let point = [0.5, 0.5, 0.5];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let dist = BvhTools::point_box_square_distance(&point, &box_min, &box_max);
        assert!((dist - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_point_box_square_distance_outside() {
        let point = [2.0, 0.5, 0.5];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let dist = BvhTools::point_box_square_distance(&point, &box_min, &box_max);
        assert!((dist - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_point_box_square_distance_corner() {
        let point = [2.0, 2.0, 2.0];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let dist = BvhTools::point_box_square_distance(&point, &box_min, &box_max);
        assert!((dist - 3.0).abs() < EPSILON); // 1^2 + 1^2 + 1^2 = 3
    }

    #[test]
    fn test_box_box_square_distance_separated() {
        let min1 = [0.0, 0.0, 0.0];
        let max1 = [1.0, 1.0, 1.0];
        let min2 = [2.0, 0.0, 0.0];
        let max2 = [3.0, 1.0, 1.0];

        let dist = BvhTools::box_box_square_distance(&min1, &max1, &min2, &max2);
        assert!((dist - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_box_box_square_distance_overlapping() {
        let min1 = [0.0, 0.0, 0.0];
        let max1 = [1.0, 1.0, 1.0];
        let min2 = [0.5, 0.5, 0.5];
        let max2 = [1.5, 1.5, 1.5];

        let dist = BvhTools::box_box_square_distance(&min1, &max1, &min2, &max2);
        assert!((dist - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_point_box_projection_inside() {
        let point = [0.5, 0.5, 0.5];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let proj = BvhTools::point_box_projection(&point, &box_min, &box_max);
        assert!((proj[0] - 0.5).abs() < EPSILON);
        assert!((proj[1] - 0.5).abs() < EPSILON);
        assert!((proj[2] - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_point_box_projection_outside() {
        let point = [2.0, 0.5, 0.5];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let proj = BvhTools::point_box_projection(&point, &box_min, &box_max);
        assert!((proj[0] - 1.0).abs() < EPSILON);
        assert!((proj[1] - 0.5).abs() < EPSILON);
        assert!((proj[2] - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_ray_box_intersection_hit() {
        let origin = [-1.0, 0.5, 0.5];
        let direction = [1.0, 0.0, 0.0];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let result = BvhTools::ray_box_intersection(&origin, &direction, &box_min, &box_max);
        assert!(result.is_some());
        let (enter, leave) = result.unwrap();
        assert!((enter - 1.0).abs() < EPSILON);
        assert!((leave - 2.0).abs() < EPSILON);
    }

    #[test]
    fn test_ray_box_intersection_miss() {
        let origin = [-1.0, 5.0, 0.5];
        let direction = [1.0, 0.0, 0.0];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let result = BvhTools::ray_box_intersection(&origin, &direction, &box_min, &box_max);
        assert!(result.is_none());
    }

    #[test]
    fn test_ray_box_intersection_inside() {
        let origin = [0.5, 0.5, 0.5];
        let direction = [1.0, 0.0, 0.0];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let result = BvhTools::ray_box_intersection(&origin, &direction, &box_min, &box_max);
        assert!(result.is_some());
        let (_enter, leave) = result.unwrap();
        assert!((leave - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_ray_box_intersection_parallel() {
        let origin = [-1.0, 0.5, 0.5];
        let direction = [1.0, 0.0, 0.0];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let result = BvhTools::ray_box_intersection(&origin, &direction, &box_min, &box_max);
        assert!(result.is_some());
    }

    #[test]
    fn test_ray_box_intersection_parallel_miss() {
        let origin = [-1.0, 2.0, 0.5];
        let direction = [1.0, 0.0, 0.0];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let result = BvhTools::ray_box_intersection(&origin, &direction, &box_min, &box_max);
        assert!(result.is_none());
    }

    #[test]
    fn test_point_triangle_projection_to_vertex() {
        let point = [-1.0, -1.0, 0.0];
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let proj = BvhTools::point_triangle_projection(&point, &v0, &v1, &v2);
        assert!((proj[0] - 0.0).abs() < EPSILON);
        assert!((proj[1] - 0.0).abs() < EPSILON);
        assert!((proj[2] - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_point_triangle_projection_to_edge() {
        let point = [0.5, -1.0, 0.0];
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let proj = BvhTools::point_triangle_projection(&point, &v0, &v1, &v2);
        assert!((proj[0] - 0.5).abs() < EPSILON);
        assert!((proj[1] - 0.0).abs() < EPSILON);
        assert!((proj[2] - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_point_triangle_projection_to_interior() {
        let point = [0.25, 0.25, 1.0];
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let proj = BvhTools::point_triangle_projection(&point, &v0, &v1, &v2);
        assert!((proj[0] - 0.25).abs() < EPSILON);
        assert!((proj[1] - 0.25).abs() < EPSILON);
        assert!((proj[2] - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_point_triangle_square_distance() {
        let point = [0.5, 0.5, 1.0];
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let dist_sq = BvhTools::point_triangle_square_distance(&point, &v0, &v1, &v2);
        // Projection is [0.5, 0.5, 0], distance is 1.0, dist_sq = 1.0
        assert!((dist_sq - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_point_triangle_centroid() {
        let point = [1.0, 1.0, 5.0];
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [3.0, 0.0, 0.0];
        let v2 = [0.0, 3.0, 0.0];

        let proj = BvhTools::point_triangle_projection(&point, &v0, &v1, &v2);
        assert!((proj[0] - 1.0).abs() < EPSILON);
        assert!((proj[1] - 1.0).abs() < EPSILON);
        assert!((proj[2] - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_box_box_distance_corner_to_corner() {
        let min1 = [0.0, 0.0, 0.0];
        let max1 = [1.0, 1.0, 1.0];
        let min2 = [2.0, 2.0, 2.0];
        let max2 = [3.0, 3.0, 3.0];

        let dist = BvhTools::box_box_square_distance(&min1, &max1, &min2, &max2);
        assert!((dist - 3.0).abs() < EPSILON); // sqrt(3) squared
    }

    #[test]
    fn test_ray_box_negative_direction() {
        let origin = [2.0, 0.5, 0.5];
        let direction = [-1.0, 0.0, 0.0];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let result = BvhTools::ray_box_intersection(&origin, &direction, &box_min, &box_max);
        assert!(result.is_some());
        let (enter, leave) = result.unwrap();
        assert!((enter - 1.0).abs() < EPSILON);
        assert!((leave - 2.0).abs() < EPSILON);
    }

    #[test]
    fn test_ray_box_behind_origin() {
        let origin = [2.0, 0.5, 0.5];
        let direction = [1.0, 0.0, 0.0];
        let box_min = [0.0, 0.0, 0.0];
        let box_max = [1.0, 1.0, 1.0];

        let result = BvhTools::ray_box_intersection(&origin, &direction, &box_min, &box_max);
        assert!(result.is_none());
    }
}

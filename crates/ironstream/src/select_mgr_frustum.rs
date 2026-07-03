// FILE: select_mgr_frustum.rs
// occt: SelectMgr_Frustum

/// Template parameter for frustum size (number of sides)
const DEFAULT_FRUSTUM_SIZE: usize = 6; // For rectangular frustum

/// Precision::Confusion
const PRECISION_CONFUSION: f64 = 1e-7;
/// Precision::Angular
const PRECISION_ANGULAR: f64 = 1e-12;

type Vec3 = (f64, f64, f64);

#[inline]
fn dot(a: Vec3, b: Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

#[inline]
fn cross(a: Vec3, b: Vec3) -> Vec3 {
    (
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

#[inline]
fn sub(a: Vec3, b: Vec3) -> Vec3 {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

#[inline]
fn modulus(a: Vec3) -> f64 {
    dot(a, a).sqrt()
}

/// Represents a rectangular selecting frustum for point and box selection.
/// Uses separating axis theorem (SAT) for overlap detection, following
/// the algorithms of OCCT `SelectMgr_Frustum.lxx`.
///
/// An "empty" frustum (no vertices or planes set yet) represents an empty
/// volume: its vertex projection intervals are empty, so every SAT test
/// reports separation and no entity overlaps it.
pub struct SelectMgrFrustum {
    // Generic frustum data with N planes
    num_planes: usize,
    // Cached projections of vertices onto frustum plane directions
    max_verts_projections: Vec<f64>,
    min_verts_projections: Vec<f64>,
    // Cached projections of vertices onto {i, j, k}
    max_ortho_verts_projections: [f64; 3],
    min_ortho_verts_projections: [f64; 3],
    // Vertex data
    vertices: Vec<Vec3>,
    // Plane normals
    planes: Vec<Vec3>,
    // Edge directions
    edge_dirs: Vec<Vec3>,
}

impl SelectMgrFrustum {
    /// Creates a new frustum with default parameters.
    /// Initializes projection caches to empty intervals.
    pub fn new() -> Self {
        let num_planes = DEFAULT_FRUSTUM_SIZE + 2; // N + 2 planes
        SelectMgrFrustum {
            num_planes,
            max_verts_projections: vec![f64::NEG_INFINITY; num_planes],
            min_verts_projections: vec![f64::INFINITY; num_planes],
            max_ortho_verts_projections: [f64::NEG_INFINITY; 3],
            min_ortho_verts_projections: [f64::INFINITY; 3],
            vertices: Vec::with_capacity(DEFAULT_FRUSTUM_SIZE * 2),
            planes: Vec::with_capacity(num_planes),
            edge_dirs: vec![(0.0, 0.0, 0.0); 6],
        }
    }

    /// Returns true if the frustum volume is not defined yet
    /// (no vertices or no planes). An undefined volume overlaps nothing.
    fn is_empty_volume(&self) -> bool {
        self.vertices.is_empty() || self.planes.is_empty()
    }

    /// Caches projections of the frustum vertices onto its plane directions
    /// and onto {i, j, k} (occt: SelectMgr_RectangularFrustum::cacheVertexProjections).
    fn cache_vertex_projections(&mut self) {
        self.max_verts_projections = vec![f64::NEG_INFINITY; self.planes.len().max(self.num_planes)];
        self.min_verts_projections = vec![f64::INFINITY; self.planes.len().max(self.num_planes)];
        for (plane_idx, plane) in self.planes.iter().enumerate() {
            let mut min = f64::INFINITY;
            let mut max = f64::NEG_INFINITY;
            for vertex in &self.vertices {
                let proj = dot(*plane, *vertex);
                min = min.min(proj);
                max = max.max(proj);
            }
            self.min_verts_projections[plane_idx] = min;
            self.max_verts_projections[plane_idx] = max;
        }

        self.max_ortho_verts_projections = [f64::NEG_INFINITY; 3];
        self.min_ortho_verts_projections = [f64::INFINITY; 3];
        for vertex in &self.vertices {
            let coords = [vertex.0, vertex.1, vertex.2];
            for dim in 0..3 {
                self.min_ortho_verts_projections[dim] =
                    self.min_ortho_verts_projections[dim].min(coords[dim]);
                self.max_ortho_verts_projections[dim] =
                    self.max_ortho_verts_projections[dim].max(coords[dim]);
            }
        }
    }

    /// Checks if AABB and frustum are separated along the given axis
    /// (occt: SelectMgr_Frustum::isSeparated for boxes).
    fn is_separated_box(&self, box_min: Vec3, box_max: Vec3, direct: Vec3) -> bool {
        let min_b = direct.0 * (if direct.0 < 0.0 { box_max.0 } else { box_min.0 })
            + direct.1 * (if direct.1 < 0.0 { box_max.1 } else { box_min.1 })
            + direct.2 * (if direct.2 < 0.0 { box_max.2 } else { box_min.2 });
        let max_b = direct.0 * (if direct.0 < 0.0 { box_min.0 } else { box_max.0 })
            + direct.1 * (if direct.1 < 0.0 { box_min.1 } else { box_max.1 })
            + direct.2 * (if direct.2 < 0.0 { box_min.2 } else { box_max.2 });

        // frustum projection
        let mut min_f = f64::INFINITY;
        let mut max_f = f64::NEG_INFINITY;
        for vertex in &self.vertices {
            let proj = dot(*vertex, direct);
            min_f = min_f.min(proj);
            max_f = max_f.max(proj);
            if min_f <= max_b && max_f >= min_b {
                return false;
            }
        }

        min_f > max_b || max_f < min_b
    }

    /// Checks if triangle and frustum are separated along the given axis
    /// (occt: SelectMgr_Frustum::isSeparated for triangles).
    fn is_separated_triangle(&self, pnt1: Vec3, pnt2: Vec3, pnt3: Vec3, axis: Vec3) -> bool {
        // triangle projection
        let proj1 = dot(axis, pnt1);
        let proj2 = dot(axis, pnt2);
        let proj3 = dot(axis, pnt3);
        let min_tr = proj1.min(proj2).min(proj3);
        let max_tr = proj1.max(proj2).max(proj3);

        // frustum projection
        let mut min_f = f64::INFINITY;
        let mut max_f = f64::NEG_INFINITY;
        for vertex in &self.vertices {
            let proj = dot(*vertex, axis);
            min_f = min_f.min(proj);
            max_f = max_f.max(proj);
            if min_f <= max_tr && max_f >= min_tr {
                return false;
            }
        }

        min_f > max_tr || max_f < min_tr
    }

    /// Returns true if selecting volume is overlapped by axis-aligned bounding box
    /// with minimum and maximum points given
    /// (occt: SelectMgr_Frustum::hasBoxOverlap).
    pub fn has_box_overlap(
        &self,
        box_min_x: f64,
        box_min_y: f64,
        box_min_z: f64,
        box_max_x: f64,
        box_max_y: f64,
        box_max_z: f64,
    ) -> bool {
        if self.is_empty_volume() {
            return false;
        }

        let box_min = (box_min_x, box_min_y, box_min_z);
        let box_max = (box_max_x, box_max_y, box_max_z);

        // Test the coordinate axes {i, j, k}
        let mins = [box_min_x, box_min_y, box_min_z];
        let maxs = [box_max_x, box_max_y, box_max_z];
        for axis in 0..3 {
            if mins[axis] > self.max_ortho_verts_projections[axis]
                || maxs[axis] < self.min_ortho_verts_projections[axis]
            {
                return false; // fully separated
            }
        }

        // Test frustum plane directions
        for (plane_idx, plane) in self.planes.iter().enumerate() {
            let box_proj_min = plane.0 * (if plane.0 < 0.0 { box_max.0 } else { box_min.0 })
                + plane.1 * (if plane.1 < 0.0 { box_max.1 } else { box_min.1 })
                + plane.2 * (if plane.2 < 0.0 { box_max.2 } else { box_min.2 });
            let box_proj_max = plane.0 * (if plane.0 < 0.0 { box_min.0 } else { box_max.0 })
                + plane.1 * (if plane.1 < 0.0 { box_min.1 } else { box_max.1 })
                + plane.2 * (if plane.2 < 0.0 { box_min.2 } else { box_max.2 });

            if box_proj_min > self.max_verts_projections[plane_idx]
                || box_proj_max < self.min_verts_projections[plane_idx]
            {
                return false; // fully separated
            }
        }

        // Test cross products of box axes {i, j, k} with frustum edge directions
        for dim in 0..3 {
            // the following code performs a speedup of cross-product
            // of vector with 1.0 at the position dim and the edge direction
            let next = (dim + 1) % 3;
            let next_next = (dim + 2) % 3;
            for edge_dir in &self.edge_dirs {
                let edge = [edge_dir.0, edge_dir.1, edge_dir.2];
                let mut direction = [0.0f64; 3];
                direction[dim] = 0.0;
                direction[next] = -edge[next_next];
                direction[next_next] = edge[next];

                if self.is_separated_box(
                    box_min,
                    box_max,
                    (direction[0], direction[1], direction[2]),
                ) {
                    return false;
                }
            }
        }

        true
    }

    /// Returns true if selecting volume is overlapped by axis-aligned bounding box.
    pub fn overlaps_box_simple(
        &self,
        box_min_x: f64,
        box_min_y: f64,
        box_min_z: f64,
        box_max_x: f64,
        box_max_y: f64,
        box_max_z: f64,
    ) -> bool {
        self.has_box_overlap(
            box_min_x, box_min_y, box_min_z, box_max_x, box_max_y, box_max_z,
        )
    }

    /// SAT intersection test between defined volume and given point
    /// (occt: SelectMgr_Frustum::hasPointOverlap).
    pub fn has_point_overlap(&self, point_x: f64, point_y: f64, point_z: f64) -> bool {
        if self.is_empty_volume() {
            return false;
        }

        let pnt = (point_x, point_y, point_z);
        for (plane_idx, plane) in self.planes.iter().enumerate() {
            let point_proj = dot(*plane, pnt);
            if point_proj > self.max_verts_projections[plane_idx]
                || point_proj < self.min_verts_projections[plane_idx]
            {
                return false;
            }
        }

        true
    }

    /// SAT intersection test between defined volume and given point.
    pub fn overlaps_point(&self, point_x: f64, point_y: f64, point_z: f64) -> bool {
        self.has_point_overlap(point_x, point_y, point_z)
    }

    /// SAT intersection test between defined volume and given segment
    /// (occt: SelectMgr_Frustum::hasSegmentOverlap).
    pub fn has_segment_overlap(
        &self,
        seg_x1: f64,
        seg_y1: f64,
        seg_z1: f64,
        seg_x2: f64,
        seg_y2: f64,
        seg_z2: f64,
    ) -> bool {
        if self.is_empty_volume() {
            return false;
        }

        let start_pnt = (seg_x1, seg_y1, seg_z1);
        let end_pnt = (seg_x2, seg_y2, seg_z2);
        let dir = sub(end_pnt, start_pnt);
        if modulus(dir) < PRECISION_CONFUSION {
            return true;
        }

        // Test frustum plane directions
        for (plane_idx, plane) in self.planes.iter().enumerate() {
            let proj1 = dot(*plane, start_pnt);
            let proj2 = dot(*plane, end_pnt);
            let min_segm = proj1.min(proj2);
            let max_segm = proj1.max(proj2);

            let max_f = self.max_verts_projections[plane_idx];
            let min_f = self.min_verts_projections[plane_idx];

            if min_segm > max_f || max_segm < min_f {
                return false;
            }
        }

        // Test the segment direction itself
        let mut min_f = f64::INFINITY;
        let mut max_f = f64::NEG_INFINITY;
        for vertex in &self.vertices {
            let proj = dot(dir, *vertex);
            min_f = min_f.min(proj);
            max_f = max_f.max(proj);
        }
        let proj1 = dot(dir, start_pnt);
        let proj2 = dot(dir, end_pnt);
        let min_segm = proj1.min(proj2);
        let max_segm = proj1.max(proj2);
        if min_segm > max_f || max_segm < min_f {
            return false;
        }

        // Test cross products of the segment direction with frustum edge directions
        for edge_dir in &self.edge_dirs {
            let test_dir = cross(dir, *edge_dir);

            let proj1 = dot(test_dir, start_pnt);
            let proj2 = dot(test_dir, end_pnt);
            let min_segm = proj1.min(proj2);
            let max_segm = proj1.max(proj2);

            let mut min_f = f64::INFINITY;
            let mut max_f = f64::NEG_INFINITY;
            for vertex in &self.vertices {
                let proj = dot(test_dir, *vertex);
                min_f = min_f.min(proj);
                max_f = max_f.max(proj);
            }

            if min_segm > max_f || max_segm < min_f {
                return false;
            }
        }

        true
    }

    /// SAT intersection test between defined volume and given triangle
    /// (occt: SelectMgr_Frustum::hasTriangleOverlap).
    pub fn has_triangle_overlap(
        &self,
        tri_x1: f64,
        tri_y1: f64,
        tri_z1: f64,
        tri_x2: f64,
        tri_y2: f64,
        tri_z2: f64,
        tri_x3: f64,
        tri_y3: f64,
        tri_z3: f64,
    ) -> bool {
        if self.is_empty_volume() {
            return false;
        }

        let pnt1 = (tri_x1, tri_y1, tri_z1);
        let pnt2 = (tri_x2, tri_y2, tri_z2);
        let pnt3 = (tri_x3, tri_y3, tri_z3);
        let tr_edges = [sub(pnt2, pnt1), sub(pnt3, pnt2), sub(pnt1, pnt3)];

        // Test frustum plane directions
        for (plane_idx, plane) in self.planes.iter().enumerate() {
            let proj1 = dot(*plane, pnt1);
            let proj2 = dot(*plane, pnt2);
            let proj3 = dot(*plane, pnt3);
            let triangle_proj_min = proj1.min(proj2).min(proj3);
            let triangle_proj_max = proj1.max(proj2).max(proj3);

            let frustum_proj_max = self.max_verts_projections[plane_idx];
            let frustum_proj_min = self.min_verts_projections[plane_idx];
            if triangle_proj_min > frustum_proj_max || triangle_proj_max < frustum_proj_min {
                return false;
            }
        }

        // Test the triangle normal
        let normal = cross(tr_edges[2], tr_edges[0]);
        if self.is_separated_triangle(pnt1, pnt2, pnt3, normal) {
            return false;
        }

        // Test cross products of triangle edges with frustum edge directions
        for tr_edge in &tr_edges {
            for edge_dir in &self.edge_dirs {
                let test_direction = cross(*edge_dir, *tr_edge);
                if self.is_separated_triangle(pnt1, pnt2, pnt3, test_direction) {
                    return false;
                }
            }
        }

        true
    }

    /// Return true if the ray (loc, ray_dir) intersects the sphere
    /// (center, radius); time_enter/time_leave receive the ray parameters
    /// (occt: SelectMgr_BaseIntersector::RaySphereIntersection).
    fn ray_sphere_intersection(
        center: Vec3,
        radius: f64,
        loc: Vec3,
        ray_dir: Vec3,
        time_enter: &mut f64,
        time_leave: &mut f64,
    ) -> bool {
        let a = dot(ray_dir, ray_dir);
        let k = dot(ray_dir, sub(loc, center));
        let dist = modulus(sub(loc, center));
        let c = dist * dist - radius * radius;
        let discr = k * k - a * c;
        if discr < 0.0 {
            return false;
        }

        let time1 = (-k - discr.sqrt()) / a;
        let time2 = (-k + discr.sqrt()) / a;
        if time1.abs() < time2.abs() {
            *time_enter = time1;
            *time_leave = time2;
        } else {
            *time_enter = time2;
            *time_leave = time1;
        }
        true
    }

    /// Returns true if the boundary polygon (boundaries, lying in the plane
    /// with normal plane_normal through the origin) intersects the sphere
    /// with the given projected center and radius
    /// (occt: SelectMgr_BaseFrustum::IsBoundaryIntersectSphere).
    fn is_boundary_intersect_sphere(
        center: Vec3,
        radius: f64,
        plane_normal: Vec3,
        boundaries: &[Vec3],
    ) -> bool {
        for idx in 0..boundaries.len() {
            let next_idx = if idx + 1 == boundaries.len() { 0 } else { idx + 1 };
            let pnt1 = boundaries[idx];
            let pnt2 = boundaries[next_idx];
            if modulus(sub(pnt2, pnt1)) < PRECISION_CONFUSION {
                continue;
            }

            // Projections of the points on the plane
            let proj1_dot = dot(pnt1, plane_normal);
            let pnt_proj1 = (
                pnt1.0 - plane_normal.0 * proj1_dot,
                pnt1.1 - plane_normal.1 * proj1_dot,
                pnt1.2 - plane_normal.2 * proj1_dot,
            );
            let proj2_dot = dot(pnt2, plane_normal);
            let pnt_proj2 = (
                pnt2.0 - plane_normal.0 * proj2_dot,
                pnt2.1 - plane_normal.1 * proj2_dot,
                pnt2.2 - plane_normal.2 * proj2_dot,
            );
            if modulus(sub(pnt_proj1, center)) < radius || modulus(sub(pnt_proj2, center)) < radius
            {
                return true; // polygon intersects the sphere
            }

            let seg = sub(pnt_proj2, pnt_proj1);
            let seg_len = modulus(seg);
            if seg_len < PRECISION_CONFUSION {
                continue;
            }
            let ray_dir = (seg.0 / seg_len, seg.1 / seg_len, seg.2 / seg_len);
            let mut time_enter = 0.0;
            let mut time_leave = 0.0;
            if Self::ray_sphere_intersection(
                center,
                radius,
                pnt_proj1,
                ray_dir,
                &mut time_enter,
                &mut time_leave,
            ) && ((time_enter > 0.0 && time_enter < seg_len)
                || (time_leave > 0.0 && time_leave < seg_len))
            {
                return true; // polygon crosses the sphere
            }
        }
        false
    }

    /// Intersection test between defined volume and given sphere
    /// (occt: SelectMgr_Frustum::hasSphereOverlap).
    pub fn has_sphere_overlap(
        &self,
        center_x: f64,
        center_y: f64,
        center_z: f64,
        radius: f64,
    ) -> bool {
        if self.is_empty_volume() {
            return false;
        }

        let pnt = (center_x, center_y, center_z);
        let mut is_overlap_full = true;
        for (plane_idx, plane) in self.planes.iter().enumerate() {
            let norm_vec_len = dot(*plane, *plane).sqrt();
            if norm_vec_len < PRECISION_CONFUSION {
                continue;
            }
            let center_proj = dot(*plane, pnt) / norm_vec_len;
            let max_dist = self.max_verts_projections[plane_idx] / norm_vec_len;
            let min_dist = self.min_verts_projections[plane_idx] / norm_vec_len;
            if center_proj > max_dist + radius || center_proj < min_dist - radius {
                return false; // fully separated
            }
            is_overlap_full &=
                center_proj >= min_dist + radius && center_proj <= max_dist - radius;
        }
        if is_overlap_full {
            return true;
        }

        // Refine: check whether the sphere intersects the near-face polygon
        // (vertices at even indices, following OCCT's vertex layout where
        // near/far vertices alternate).
        let half_count = self.vertices.len() / 2;
        if half_count < 3 || self.vertices.len() % 2 != 0 {
            // Not enough data to build the boundary polygon; the plane slab
            // tests found no separating axis, so report overlap.
            return true;
        }
        let vec_plane1 = sub(self.vertices[2], self.vertices[0]);
        let vec_plane2 = sub(self.vertices[2 * half_count - 2], self.vertices[0]);
        let a_cross = cross(vec_plane1, vec_plane2);
        let cross_len = modulus(a_cross);
        if cross_len < PRECISION_ANGULAR * modulus(vec_plane1) * modulus(vec_plane2) {
            return false; // boundary plane is degenerate
        }
        let norm = (
            a_cross.0 / cross_len,
            a_cross.1 / cross_len,
            a_cross.2 / cross_len,
        );
        let mut boundaries = Vec::with_capacity(half_count);
        for idx in (0..self.vertices.len()).step_by(2) {
            boundaries.push(self.vertices[idx]);
        }
        // distance from point(x,y,z) to plane(A,B,C,D):
        // d = | Ax + By + Cz + D | / sqrt (A^2 + B^2 + C^2) = pnt.Dot(norm) / 1
        let center_dot = dot(pnt, norm);
        let center_proj = (
            pnt.0 - norm.0 * center_dot,
            pnt.1 - norm.1 * center_dot,
            pnt.2 - norm.2 * center_dot,
        );
        Self::is_boundary_intersect_sphere(center_proj, radius, norm, &boundaries)
    }

    /// Sets a plane for the frustum.
    pub fn set_plane(&mut self, index: usize, nx: f64, ny: f64, nz: f64) {
        // Normalize the normal vector
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        if len > 1e-10 {
            while self.planes.len() <= index {
                self.planes.push((0.0, 0.0, 0.0));
            }
            self.planes[index] = (nx / len, ny / len, nz / len);
            self.cache_vertex_projections();
        }
    }

    /// Sets a vertex for the frustum.
    pub fn set_vertex(&mut self, index: usize, x: f64, y: f64, z: f64) {
        while self.vertices.len() <= index {
            self.vertices.push((0.0, 0.0, 0.0));
        }
        self.vertices[index] = (x, y, z);
        self.cache_vertex_projections();
    }

    /// Sets an edge direction for the frustum.
    pub fn set_edge_dir(&mut self, index: usize, dx: f64, dy: f64, dz: f64) {
        if index < self.edge_dirs.len() {
            self.edge_dirs[index] = (dx, dy, dz);
        }
    }

    /// Gets the number of planes in this frustum.
    pub fn num_planes(&self) -> usize {
        self.num_planes
    }
}

impl Default for SelectMgrFrustum {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frustum_creation() {
        let frustum = SelectMgrFrustum::new();
        assert_eq!(frustum.num_planes(), DEFAULT_FRUSTUM_SIZE + 2);
    }

    #[test]
    fn test_set_vertex() {
        let mut frustum = SelectMgrFrustum::new();
        frustum.set_vertex(0, 1.0, 2.0, 3.0);
        frustum.set_vertex(1, 4.0, 5.0, 6.0);

        assert_eq!(frustum.vertices.len(), 2);
    }

    #[test]
    fn test_set_plane() {
        let mut frustum = SelectMgrFrustum::new();
        frustum.set_plane(0, 1.0, 0.0, 0.0);
        frustum.set_plane(1, 0.0, 1.0, 0.0);

        assert_eq!(frustum.planes.len(), 2);
        // Check that planes are normalized
        let (nx, ny, nz) = frustum.planes[0];
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        assert!((len - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_empty_frustum_point_overlap() {
        let frustum = SelectMgrFrustum::new();
        // Empty frustum should not contain any point
        assert!(!frustum.overlaps_point(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_frustum_box_overlap_empty() {
        let frustum = SelectMgrFrustum::new();
        assert!(!frustum.overlaps_box_simple(-1.0, -1.0, -1.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn test_frustum_segment_overlap() {
        let frustum = SelectMgrFrustum::new();
        assert!(!frustum.has_segment_overlap(0.0, 0.0, 0.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn test_frustum_triangle_overlap() {
        let frustum = SelectMgrFrustum::new();
        assert!(!frustum.has_triangle_overlap(
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0
        ));
    }

    #[test]
    fn test_frustum_sphere_overlap_empty() {
        let frustum = SelectMgrFrustum::new();
        // Empty frustum should not overlap with any sphere
        assert!(!frustum.has_sphere_overlap(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn test_set_edge_dir() {
        let mut frustum = SelectMgrFrustum::new();
        frustum.set_edge_dir(0, 1.0, 0.0, 0.0);
        frustum.set_edge_dir(1, 0.0, 1.0, 0.0);

        assert_eq!(frustum.edge_dirs[0], (1.0, 0.0, 0.0));
        assert_eq!(frustum.edge_dirs[1], (0.0, 1.0, 0.0));
    }

    #[test]
    fn test_frustum_with_vertices_and_planes() {
        let mut frustum = SelectMgrFrustum::new();

        // Set up a simple frustum with vertices and planes
        frustum.set_vertex(0, 0.0, 0.0, 0.0);
        frustum.set_vertex(1, 1.0, 0.0, 0.0);
        frustum.set_vertex(2, 1.0, 1.0, 0.0);
        frustum.set_vertex(3, 0.0, 1.0, 0.0);

        // Set planes
        frustum.set_plane(0, 1.0, 0.0, 0.0); // X plane
        frustum.set_plane(1, 0.0, 1.0, 0.0); // Y plane

        // Box overlap test with non-empty frustum
        let overlaps = frustum.overlaps_box_simple(-0.5, -0.5, -0.5, 0.5, 0.5, 0.5);
        assert!(overlaps);
    }
}

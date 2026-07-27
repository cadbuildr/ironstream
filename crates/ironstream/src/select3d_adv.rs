// FILE: select3d_adv.rs
// occt: Select3D_SensitiveTriangulation
// occt-ref: Select3D_SensitiveBox
//       Select3D_SensitiveSphere, Select3D_SensitiveCylinder,
//       Select3D_BVHTree, SelectMgr_FrustumBuilder

/// Sensitivity type for picking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SensitivityType {
    Point,
    Line,
    Box,
    Triangle,
    Triangulation,
    Sphere,
    Cylinder,
    Group,
    Composite,
}

/// Frustum type for selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrustumType {
    Point,
    Box,
    Polyline,
}

/// Result of an intersection test.
#[derive(Clone, Debug)]
pub struct SensitiveIntersect {
    pub is_inside: bool,
    pub depth: f64,
    pub closest_point: [f64; 3],
    pub sensitive_index: u32,
}

impl SensitiveIntersect {
    pub fn miss() -> Self {
        Self { is_inside: false, depth: f64::INFINITY, closest_point: [0.0; 3], sensitive_index: 0 }
    }

    pub fn hit(depth: f64, pt: [f64; 3], idx: u32) -> Self {
        Self { is_inside: true, depth, closest_point: pt, sensitive_index: idx }
    }
}

// occt-ref: Select3D_SensitiveBox
#[derive(Clone, Debug)]
pub struct SensitiveBox {
    pub owner_id: u32,
    pub bbox_min: [f64; 3],
    pub bbox_max: [f64; 3],
    pub sensitivity_type: SensitivityType,
}

impl SensitiveBox {
    pub fn new(owner_id: u32, mn: [f64; 3], mx: [f64; 3]) -> Self {
        Self { owner_id, bbox_min: mn, bbox_max: mx, sensitivity_type: SensitivityType::Box }
    }

    pub fn contains_point(&self, pt: &[f64; 3]) -> bool {
        (0..3).all(|i| pt[i] >= self.bbox_min[i] && pt[i] <= self.bbox_max[i])
    }

    pub fn center(&self) -> [f64; 3] {
        [
            (self.bbox_min[0] + self.bbox_max[0]) * 0.5,
            (self.bbox_min[1] + self.bbox_max[1]) * 0.5,
            (self.bbox_min[2] + self.bbox_max[2]) * 0.5,
        ]
    }

    pub fn diagonal(&self) -> f64 {
        let d = [
            self.bbox_max[0] - self.bbox_min[0],
            self.bbox_max[1] - self.bbox_min[1],
            self.bbox_max[2] - self.bbox_min[2],
        ];
        (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt()
    }

    pub fn test_point(&self, pt: [f64; 3]) -> SensitiveIntersect {
        if self.contains_point(&pt) {
            let c = self.center();
            let depth = ((c[0]-pt[0]).powi(2) + (c[1]-pt[1]).powi(2) + (c[2]-pt[2]).powi(2)).sqrt();
            SensitiveIntersect::hit(depth, pt, self.owner_id)
        } else {
            SensitiveIntersect::miss()
        }
    }
}

// occt-ref: Select3D_SensitiveSphere
#[derive(Clone, Debug)]
pub struct SensitiveSphere {
    pub owner_id: u32,
    pub center: [f64; 3],
    pub radius: f64,
}

impl SensitiveSphere {
    pub fn new(owner_id: u32, center: [f64; 3], radius: f64) -> Self {
        Self { owner_id, center, radius }
    }

    pub fn contains_point(&self, pt: &[f64; 3]) -> bool {
        let dx = pt[0]-self.center[0]; let dy = pt[1]-self.center[1]; let dz = pt[2]-self.center[2];
        dx*dx + dy*dy + dz*dz <= self.radius * self.radius
    }

    pub fn test_point(&self, pt: [f64; 3]) -> SensitiveIntersect {
        let dx = pt[0]-self.center[0]; let dy = pt[1]-self.center[1]; let dz = pt[2]-self.center[2];
        let d = (dx*dx + dy*dy + dz*dz).sqrt();
        if d <= self.radius {
            SensitiveIntersect::hit(d, pt, self.owner_id)
        } else {
            SensitiveIntersect::miss()
        }
    }

    pub fn surface_area(&self) -> f64 { 4.0 * std::f64::consts::PI * self.radius * self.radius }
}

// occt: Select3D_SensitiveTriangulation
#[derive(Clone, Debug)]
pub struct SensitiveTriangulation {
    pub owner_id: u32,
    pub vertices: Vec<[f64; 3]>,
    pub triangles: Vec<[u32; 3]>,
    pub sensitivity_type: SensitivityType,
}

impl SensitiveTriangulation {
    pub fn new(owner_id: u32, vertices: Vec<[f64; 3]>, triangles: Vec<[u32; 3]>) -> Self {
        Self { owner_id, vertices, triangles, sensitivity_type: SensitivityType::Triangulation }
    }

    pub fn nb_triangles(&self) -> usize { self.triangles.len() }
    pub fn nb_vertices(&self) -> usize { self.vertices.len() }

    pub fn triangle_center(&self, tri_idx: usize) -> Option<[f64; 3]> {
        let tri = self.triangles.get(tri_idx)?;
        let v0 = self.vertices.get(tri[0] as usize)?;
        let v1 = self.vertices.get(tri[1] as usize)?;
        let v2 = self.vertices.get(tri[2] as usize)?;
        Some([
            (v0[0]+v1[0]+v2[0]) / 3.0,
            (v0[1]+v1[1]+v2[1]) / 3.0,
            (v0[2]+v1[2]+v2[2]) / 3.0,
        ])
    }

    pub fn bounding_box(&self) -> ([f64; 3], [f64; 3]) {
        if self.vertices.is_empty() { return ([0.0; 3], [0.0; 3]); }
        let mut mn = [f64::INFINITY; 3];
        let mut mx = [f64::NEG_INFINITY; 3];
        for v in &self.vertices {
            for i in 0..3 { mn[i] = mn[i].min(v[i]); mx[i] = mx[i].max(v[i]); }
        }
        (mn, mx)
    }
}

// occt: Select3D_SensitiveCylinder
#[derive(Clone, Debug)]
pub struct SensitiveCylinder {
    pub owner_id: u32,
    pub axis_origin: [f64; 3],
    pub axis_dir: [f64; 3],
    pub radius: f64,
    pub height: f64,
    pub is_hollow: bool,
}

impl SensitiveCylinder {
    pub fn new(owner_id: u32, origin: [f64; 3], dir: [f64; 3], r: f64, h: f64) -> Self {
        Self { owner_id, axis_origin: origin, axis_dir: dir, radius: r, height: h, is_hollow: false }
    }

    pub fn with_hollow(mut self) -> Self { self.is_hollow = true; self }

    pub fn volume(&self) -> f64 {
        if self.is_hollow { 0.0 } else { std::f64::consts::PI * self.radius * self.radius * self.height }
    }

    pub fn lateral_area(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius * self.height }
}

/// Simple BVH node for axis-aligned bounding boxes.
#[derive(Clone, Debug)]
pub struct BvhNode {
    pub id: u32,
    pub bbox_min: [f64; 3],
    pub bbox_max: [f64; 3],
    pub left_child: Option<u32>,
    pub right_child: Option<u32>,
}

impl BvhNode {
    pub fn leaf(id: u32, mn: [f64; 3], mx: [f64; 3]) -> Self {
        Self { id, bbox_min: mn, bbox_max: mx, left_child: None, right_child: None }
    }

    pub fn is_leaf(&self) -> bool { self.left_child.is_none() && self.right_child.is_none() }

    pub fn overlaps(&self, mn: &[f64; 3], mx: &[f64; 3]) -> bool {
        (0..3).all(|i| self.bbox_max[i] >= mn[i] && self.bbox_min[i] <= mx[i])
    }
}

// occt-note: Select3D_BVHTree (simplified)
#[derive(Clone, Debug, Default)]
pub struct Select3dBvhTree {
    pub nodes: Vec<BvhNode>,
}

impl Select3dBvhTree {
    pub fn new() -> Self { Self::default() }

    pub fn add_leaf(&mut self, id: u32, mn: [f64; 3], mx: [f64; 3]) {
        self.nodes.push(BvhNode::leaf(id, mn, mx));
    }

    pub fn select_overlapping(&self, mn: [f64; 3], mx: [f64; 3]) -> Vec<u32> {
        self.nodes.iter()
            .filter(|n| n.is_leaf() && n.overlaps(&mn, &mx))
            .map(|n| n.id)
            .collect()
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_leaves(&self) -> usize { self.nodes.iter().filter(|n| n.is_leaf()).count() }
}

// occt: SelectMgr_FrustumBuilder
#[derive(Clone, Debug)]
pub struct FrustumBuilder {
    pub frustum_type: FrustumType,
    pub pixel_tolerance: u32,
    pub screen_origin: [f64; 2],
    pub screen_size: [f64; 2],
    pub near_plane: f64,
    pub far_plane: f64,
}

impl FrustumBuilder {
    pub fn point(pixel_tol: u32) -> Self {
        Self {
            frustum_type: FrustumType::Point,
            pixel_tolerance: pixel_tol,
            screen_origin: [0.0, 0.0],
            screen_size: [1.0, 1.0],
            near_plane: 0.0,
            far_plane: 1.0,
        }
    }

    pub fn box_select(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        let mut fb = Self::point(0);
        fb.frustum_type = FrustumType::Box;
        fb.screen_origin = [x0.min(x1), y0.min(y1)];
        fb.screen_size = [(x1-x0).abs(), (y1-y0).abs()];
        fb
    }

    pub fn area(&self) -> f64 { self.screen_size[0] * self.screen_size[1] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_box_test() {
        let b = SensitiveBox::new(0, [0.0, 0.0, 0.0], [2.0, 2.0, 2.0]);
        assert!(b.contains_point(&[1.0, 1.0, 1.0]));
        assert!(!b.contains_point(&[3.0, 1.0, 1.0]));
        let hit = b.test_point([1.0, 1.0, 1.0]);
        assert!(hit.is_inside);
        let miss = b.test_point([5.0, 5.0, 5.0]);
        assert!(!miss.is_inside);
    }

    #[test]
    fn sensitive_sphere_test() {
        let s = SensitiveSphere::new(1, [0.0, 0.0, 0.0], 1.0);
        assert!(s.contains_point(&[0.5, 0.0, 0.0]));
        assert!(!s.contains_point(&[2.0, 0.0, 0.0]));
        assert!((s.surface_area() - 4.0 * std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn sensitive_triangulation() {
        let verts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[0.5,1.0,0.0]];
        let tris = vec![[0,1,2]];
        let st = SensitiveTriangulation::new(0, verts, tris);
        assert_eq!(st.nb_triangles(), 1);
        assert_eq!(st.nb_vertices(), 3);
        let c = st.triangle_center(0).unwrap();
        assert!((c[1] - 1.0/3.0).abs() < 1e-10);
    }

    #[test]
    fn bvh_tree_select() {
        let mut tree = Select3dBvhTree::new();
        tree.add_leaf(0, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        tree.add_leaf(1, [2.0, 0.0, 0.0], [3.0, 1.0, 1.0]);
        let hits = tree.select_overlapping([0.5, 0.0, 0.0], [1.5, 1.0, 1.0]);
        assert_eq!(hits, vec![0]);
    }

    #[test]
    fn frustum_builder() {
        let fb = FrustumBuilder::box_select(10.0, 20.0, 50.0, 80.0);
        assert_eq!(fb.frustum_type, FrustumType::Box);
        assert!((fb.area() - 40.0*60.0).abs() < 1e-10);
    }
}

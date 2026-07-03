// FILE: bvh_ext.rs
// occt: BVH_Box<T,N>, BVH_Ray<T,N>, BVH_Tree<T,N>,
//       BVH_PrimitiveSet3d

/// A 3D axis-aligned bounding box for BVH nodes.
/// occt: BVH_Box<Standard_ShortReal, 3>
#[derive(Clone, Copy, Debug)]
pub struct BvhBox3d {
    pub min: [f32; 3],
    pub max: [f32; 3],
    pub is_void: bool,
}

impl Default for BvhBox3d {
    fn default() -> Self {
        Self { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3], is_void: true }
    }
}

impl BvhBox3d {
    pub fn new(min: [f32; 3], max: [f32; 3]) -> Self {
        Self { min, max, is_void: false }
    }

    pub fn is_void(&self) -> bool { self.is_void }

    pub fn add_point(&mut self, p: [f32; 3]) {
        for k in 0..3 { if p[k] < self.min[k] { self.min[k] = p[k]; } if p[k] > self.max[k] { self.max[k] = p[k]; } }
        self.is_void = false;
    }

    pub fn combine(&mut self, other: &BvhBox3d) {
        if other.is_void { return; }
        for k in 0..3 { if other.min[k] < self.min[k] { self.min[k] = other.min[k]; } if other.max[k] > self.max[k] { self.max[k] = other.max[k]; } }
        self.is_void = false;
    }

    pub fn center(&self) -> [f32; 3] {
        [(self.min[0]+self.max[0])/2.0, (self.min[1]+self.max[1])/2.0, (self.min[2]+self.max[2])/2.0]
    }

    pub fn size(&self) -> [f32; 3] {
        [self.max[0]-self.min[0], self.max[1]-self.min[1], self.max[2]-self.min[2]]
    }

    pub fn surface_area(&self) -> f32 {
        let s = self.size();
        2.0 * (s[0]*s[1] + s[1]*s[2] + s[2]*s[0])
    }

    pub fn contains_point(&self, p: [f32; 3]) -> bool {
        !self.is_void &&
        p[0] >= self.min[0] && p[0] <= self.max[0] &&
        p[1] >= self.min[1] && p[1] <= self.max[1] &&
        p[2] >= self.min[2] && p[2] <= self.max[2]
    }

    pub fn intersects(&self, other: &BvhBox3d) -> bool {
        if self.is_void || other.is_void { return false; }
        for k in 0..3 {
            if self.max[k] < other.min[k] || other.max[k] < self.min[k] { return false; }
        }
        true
    }

    /// Longest axis (0=X, 1=Y, 2=Z).
    pub fn longest_axis(&self) -> usize {
        let s = self.size();
        if s[0] >= s[1] && s[0] >= s[2] { 0 }
        else if s[1] >= s[2] { 1 }
        else { 2 }
    }
}

/// A 3D ray for BVH intersection tests.
/// occt: BVH_Ray<Standard_ShortReal, 3>
#[derive(Clone, Copy, Debug)]
pub struct BvhRay3d {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
    pub inv_direction: [f32; 3],
}

impl BvhRay3d {
    pub fn new(origin: [f32; 3], direction: [f32; 3]) -> Self {
        let len = (direction[0]*direction[0]+direction[1]*direction[1]+direction[2]*direction[2]).sqrt();
        let d = if len > 1e-7 { [direction[0]/len, direction[1]/len, direction[2]/len] } else { [0.0, 0.0, 1.0] };
        let inv = [if d[0].abs() > 1e-14 { 1.0/d[0] } else { f32::INFINITY },
                   if d[1].abs() > 1e-14 { 1.0/d[1] } else { f32::INFINITY },
                   if d[2].abs() > 1e-14 { 1.0/d[2] } else { f32::INFINITY }];
        Self { origin, direction: d, inv_direction: inv }
    }

    /// Test intersection with AABB; returns (t_min, t_max) or None.
    pub fn intersect_box(&self, b: &BvhBox3d) -> Option<(f32, f32)> {
        if b.is_void { return None; }
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;
        for k in 0..3 {
            let t1 = (b.min[k] - self.origin[k]) * self.inv_direction[k];
            let t2 = (b.max[k] - self.origin[k]) * self.inv_direction[k];
            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }
        if tmax >= tmin && tmax >= 0.0 { Some((tmin, tmax)) } else { None }
    }

    pub fn point_at(&self, t: f32) -> [f32; 3] {
        [self.origin[0]+self.direction[0]*t, self.origin[1]+self.direction[1]*t, self.origin[2]+self.direction[2]*t]
    }
}

/// A BVH tree node.
/// occt: BVH_Tree node
#[derive(Clone, Debug)]
pub struct BvhNode {
    pub bounding_box: BvhBox3d,
    pub left_child: Option<usize>,
    pub right_child: Option<usize>,
    pub leaf_start: usize,  // index into leaf data (valid if is_leaf)
    pub leaf_end: usize,
    pub is_leaf: bool,
}

impl BvhNode {
    pub fn leaf(bounding_box: BvhBox3d, start: usize, end: usize) -> Self {
        Self { bounding_box, left_child: None, right_child: None, leaf_start: start, leaf_end: end, is_leaf: true }
    }

    pub fn internal(bounding_box: BvhBox3d, left: usize, right: usize) -> Self {
        Self { bounding_box, left_child: Some(left), right_child: Some(right), leaf_start: 0, leaf_end: 0, is_leaf: false }
    }

    pub fn nb_elements(&self) -> usize {
        if self.is_leaf { self.leaf_end - self.leaf_start } else { 0 }
    }
}

/// A flat BVH tree over triangles.
/// occt: BVH_Tree<Standard_ShortReal, 3> / BVH_PrimitiveSet3d
#[derive(Clone, Debug, Default)]
pub struct BvhTree3d {
    pub nodes: Vec<BvhNode>,
    pub nb_primitives: usize,
    pub depth: usize,
}

impl BvhTree3d {
    pub fn new() -> Self { Self::default() }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_primitives(&self) -> usize { self.nb_primitives }
    pub fn depth(&self) -> usize { self.depth }
    pub fn is_built(&self) -> bool { !self.nodes.is_empty() }

    /// Stub builder: creates a single leaf node for n primitives.
    pub fn build_for(&mut self, primitives: &[BvhBox3d]) {
        self.nb_primitives = primitives.len();
        self.depth = 1;
        let mut root_box = BvhBox3d::default();
        for b in primitives { root_box.combine(b); }
        self.nodes = vec![BvhNode::leaf(root_box, 0, self.nb_primitives)];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bvh_box_add_point() {
        let mut b = BvhBox3d::default();
        b.add_point([1.0, 2.0, 3.0]);
        b.add_point([-1.0, 0.0, 5.0]);
        assert!(!b.is_void());
        assert!((b.min[0] + 1.0).abs() < 1e-6);
        assert!((b.max[2] - 5.0).abs() < 1e-6);
    }

    #[test]
    fn bvh_box_intersects() {
        let a = BvhBox3d::new([0.0;3], [1.0;3]);
        let b = BvhBox3d::new([0.5, 0.5, 0.5], [2.0, 2.0, 2.0]);
        let c = BvhBox3d::new([2.0, 2.0, 2.0], [3.0, 3.0, 3.0]);
        assert!(a.intersects(&b));
        assert!(!a.intersects(&c));
    }

    #[test]
    fn bvh_box_longest_axis() {
        let b = BvhBox3d::new([0.0;3], [3.0, 1.0, 2.0]);
        assert_eq!(b.longest_axis(), 0);
    }

    #[test]
    fn bvh_ray_intersect_box() {
        let ray = BvhRay3d::new([0.0, 0.0, -5.0], [0.0, 0.0, 1.0]);
        let b = BvhBox3d::new([-1.0;3], [1.0;3]);
        let hit = ray.intersect_box(&b);
        assert!(hit.is_some());
        let (t0, t1) = hit.unwrap();
        assert!(t0 < t1);
        assert!(t0 > 0.0);
    }

    #[test]
    fn bvh_ray_miss() {
        let ray = BvhRay3d::new([5.0, 5.0, -5.0], [0.0, 0.0, 1.0]);
        let b = BvhBox3d::new([-1.0;3], [1.0;3]);
        assert!(ray.intersect_box(&b).is_none());
    }
}

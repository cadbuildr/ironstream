// FILE: bvh_prim.rs
// occt: BVH_PrimitiveSet3d, BVH_DistanceField, BVH_BoxSet

/// Axis-aligned bounding box for BVH primitives.
#[derive(Clone, Copy, Debug)]
pub struct BvhBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Default for BvhBox {
    fn default() -> Self {
        Self { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3] }
    }
}

impl BvhBox {
    pub fn new(min: [f32; 3], max: [f32; 3]) -> Self { Self { min, max } }

    pub fn is_valid(&self) -> bool {
        self.min[0] <= self.max[0] && self.min[1] <= self.max[1] && self.min[2] <= self.max[2]
    }

    pub fn center(&self) -> [f32; 3] {
        [(self.min[0] + self.max[0]) * 0.5,
         (self.min[1] + self.max[1]) * 0.5,
         (self.min[2] + self.max[2]) * 0.5]
    }

    pub fn size(&self) -> [f32; 3] {
        [self.max[0] - self.min[0],
         self.max[1] - self.min[1],
         self.max[2] - self.min[2]]
    }

    pub fn contains(&self, p: [f32; 3]) -> bool {
        p[0] >= self.min[0] && p[0] <= self.max[0] &&
        p[1] >= self.min[1] && p[1] <= self.max[1] &&
        p[2] >= self.min[2] && p[2] <= self.max[2]
    }

    pub fn extend(&mut self, p: [f32; 3]) {
        for i in 0..3 {
            if p[i] < self.min[i] { self.min[i] = p[i]; }
            if p[i] > self.max[i] { self.max[i] = p[i]; }
        }
    }

    pub fn combine(&self, other: &BvhBox) -> Self {
        Self {
            min: [self.min[0].min(other.min[0]),
                  self.min[1].min(other.min[1]),
                  self.min[2].min(other.min[2])],
            max: [self.max[0].max(other.max[0]),
                  self.max[1].max(other.max[1]),
                  self.max[2].max(other.max[2])],
        }
    }
}

/// A primitive (triangle/sphere/point) in a BVH primitive set.
/// occt: BVH_PrimitiveSet3d // primitive entry
#[derive(Clone, Debug)]
pub struct BvhPrimitive {
    pub id: u32,
    pub bounds: BvhBox,
    pub center: [f32; 3],
}

impl BvhPrimitive {
    pub fn from_box(id: u32, bounds: BvhBox) -> Self {
        let center = bounds.center();
        Self { id, bounds, center }
    }

    pub fn from_point(id: u32, p: [f32; 3]) -> Self {
        let bounds = BvhBox::new(p, p);
        Self { id, bounds, center: p }
    }
}

/// A set of primitives that can be queried with a BVH.
/// occt: BVH_PrimitiveSet3d
#[derive(Clone, Debug, Default)]
pub struct BvhPrimitiveSet3d {
    pub primitives: Vec<BvhPrimitive>,
}

impl BvhPrimitiveSet3d {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, p: BvhPrimitive) { self.primitives.push(p); }

    pub fn nb_primitives(&self) -> usize { self.primitives.len() }

    pub fn total_bounds(&self) -> BvhBox {
        let mut b = BvhBox::default();
        for p in &self.primitives {
            for i in 0..3 {
                if p.bounds.min[i] < b.min[i] { b.min[i] = p.bounds.min[i]; }
                if p.bounds.max[i] > b.max[i] { b.max[i] = p.bounds.max[i]; }
            }
        }
        b
    }

    /// Linear search for closest primitive to query point.
    pub fn closest(&self, p: [f32; 3]) -> Option<(u32, f32)> {
        let mut best_id = None;
        let mut best_dist = f32::INFINITY;
        for prim in &self.primitives {
            let c = prim.center;
            let d = ((c[0]-p[0]).powi(2) + (c[1]-p[1]).powi(2) + (c[2]-p[2]).powi(2)).sqrt();
            if d < best_dist { best_dist = d; best_id = Some(prim.id); }
        }
        best_id.map(|id| (id, best_dist))
    }
}

/// 3D signed distance field on a uniform grid.
/// occt: BVH_DistanceField
#[derive(Clone, Debug)]
pub struct BvhDistanceField {
    pub bounds: BvhBox,
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    data: Vec<f32>,
}

impl BvhDistanceField {
    pub fn new(bounds: BvhBox, nx: usize, ny: usize, nz: usize) -> Self {
        let n = nx.max(1) * ny.max(1) * nz.max(1);
        Self { bounds, nx: nx.max(1), ny: ny.max(1), nz: nz.max(1), data: vec![f32::INFINITY; n] }
    }

    fn idx(&self, ix: usize, iy: usize, iz: usize) -> Option<usize> {
        if ix >= self.nx || iy >= self.ny || iz >= self.nz { return None; }
        Some(ix * self.ny * self.nz + iy * self.nz + iz)
    }

    pub fn set(&mut self, ix: usize, iy: usize, iz: usize, v: f32) -> bool {
        if let Some(i) = self.idx(ix, iy, iz) { self.data[i] = v; true } else { false }
    }

    pub fn get(&self, ix: usize, iy: usize, iz: usize) -> Option<f32> {
        Some(self.data[self.idx(ix, iy, iz)?])
    }

    pub fn nb_cells(&self) -> usize { self.data.len() }

    pub fn max_dist(&self) -> f32 {
        self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bbox_valid_contains() {
        let b = BvhBox::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        assert!(b.is_valid());
        assert!(b.contains([0.5, 0.5, 0.5]));
        assert!(!b.contains([2.0, 0.0, 0.0]));
    }

    #[test]
    fn bbox_extend_combine() {
        let mut b = BvhBox::default();
        b.extend([1.0, 2.0, 3.0]);
        b.extend([-1.0, 0.0, 1.0]);
        assert!((b.min[0] - (-1.0)).abs() < 1e-6);
        assert!((b.max[2] - 3.0).abs() < 1e-6);
    }

    #[test]
    fn primitive_set_closest() {
        let mut s = BvhPrimitiveSet3d::new();
        s.add(BvhPrimitive::from_point(1, [0.0, 0.0, 0.0]));
        s.add(BvhPrimitive::from_point(2, [10.0, 0.0, 0.0]));
        let (id, dist) = s.closest([1.0, 0.0, 0.0]).unwrap();
        assert_eq!(id, 1);
        assert!((dist - 1.0).abs() < 1e-5);
    }

    #[test]
    fn distance_field_set_get() {
        let b = BvhBox::new([0.0; 3], [1.0; 3]);
        let mut df = BvhDistanceField::new(b, 4, 4, 4);
        df.set(1, 2, 3, 0.5);
        assert_eq!(df.get(1, 2, 3), Some(0.5));
        assert!(df.get(10, 0, 0).is_none());
    }

    #[test]
    fn primitive_set_bounds() {
        let mut s = BvhPrimitiveSet3d::new();
        let b1 = BvhBox::new([0.0; 3], [1.0; 3]);
        let b2 = BvhBox::new([-2.0, -2.0, -2.0], [0.5, 0.5, 0.5]);
        s.add(BvhPrimitive::from_box(1, b1));
        s.add(BvhPrimitive::from_box(2, b2));
        let total = s.total_bounds();
        assert!(total.min[0] <= -2.0);
        assert!(total.max[0] >= 1.0);
    }
}

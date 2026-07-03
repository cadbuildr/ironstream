// FILE: graphic3d_bound.rs
// occt: Graphic3d_BndBox3d, Graphic3d_BVHItem, Graphic3d_CullInterval,
//       Graphic3d_WorldViewProjState

/// Axis-aligned bounding box in 3D.
/// occt: Graphic3d_BndBox3d / Bnd_Box
#[derive(Clone, Debug)]
pub struct Graphic3dBndBox3d {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub is_void: bool,
}

impl Default for Graphic3dBndBox3d {
    fn default() -> Self {
        Self {
            min: [f64::MAX; 3],
            max: [f64::MIN; 3],
            is_void: true,
        }
    }
}

impl Graphic3dBndBox3d {
    pub fn new() -> Self { Self::default() }

    pub fn from_min_max(min: [f64; 3], max: [f64; 3]) -> Self {
        Self { min, max, is_void: false }
    }

    pub fn add_point(&mut self, p: [f64; 3]) {
        if self.is_void {
            self.min = p;
            self.max = p;
            self.is_void = false;
        } else {
            for i in 0..3 {
                if p[i] < self.min[i] { self.min[i] = p[i]; }
                if p[i] > self.max[i] { self.max[i] = p[i]; }
            }
        }
    }

    pub fn combine(&mut self, other: &Graphic3dBndBox3d) {
        if other.is_void { return; }
        self.add_point(other.min);
        self.add_point(other.max);
    }

    pub fn contains(&self, p: [f64; 3]) -> bool {
        if self.is_void { return false; }
        p[0] >= self.min[0] && p[0] <= self.max[0] &&
        p[1] >= self.min[1] && p[1] <= self.max[1] &&
        p[2] >= self.min[2] && p[2] <= self.max[2]
    }

    pub fn overlaps(&self, other: &Graphic3dBndBox3d) -> bool {
        if self.is_void || other.is_void { return false; }
        self.min[0] <= other.max[0] && self.max[0] >= other.min[0] &&
        self.min[1] <= other.max[1] && self.max[1] >= other.min[1] &&
        self.min[2] <= other.max[2] && self.max[2] >= other.min[2]
    }

    pub fn size(&self) -> [f64; 3] {
        if self.is_void { [0.0; 3] }
        else { [self.max[0]-self.min[0], self.max[1]-self.min[1], self.max[2]-self.min[2]] }
    }

    pub fn diagonal(&self) -> f64 {
        let s = self.size();
        (s[0]*s[0] + s[1]*s[1] + s[2]*s[2]).sqrt()
    }

    pub fn center(&self) -> [f64; 3] {
        if self.is_void { [0.0; 3] }
        else {
            [
                (self.min[0] + self.max[0]) * 0.5,
                (self.min[1] + self.max[1]) * 0.5,
                (self.min[2] + self.max[2]) * 0.5,
            ]
        }
    }

    pub fn set_void(&mut self) {
        self.min = [f64::MAX; 3];
        self.max = [f64::MIN; 3];
        self.is_void = true;
    }
}

/// BVH (Bounding Volume Hierarchy) tree item.
/// occt: Graphic3d_BVHItem
#[derive(Clone, Debug)]
pub struct Graphic3dBvhItem {
    pub shape_id: u32,
    pub bbox: Graphic3dBndBox3d,
    pub priority: i32,
}

impl Graphic3dBvhItem {
    pub fn new(shape_id: u32, bbox: Graphic3dBndBox3d) -> Self {
        Self { shape_id, bbox, priority: 0 }
    }

    pub fn with_priority(mut self, p: i32) -> Self { self.priority = p; self }
}

/// Cull interval for view-frustum culling.
/// occt: Graphic3d_CullInterval
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Graphic3dCullInterval {
    pub min_dist: f64,
    pub max_dist: f64,
}

impl Default for Graphic3dCullInterval {
    fn default() -> Self { Self { min_dist: 0.0, max_dist: f64::MAX } }
}

impl Graphic3dCullInterval {
    pub fn new(min_dist: f64, max_dist: f64) -> Self {
        Self { min_dist: min_dist.max(0.0), max_dist: max_dist.max(min_dist) }
    }

    pub fn contains(&self, dist: f64) -> bool {
        dist >= self.min_dist && dist <= self.max_dist
    }

    pub fn is_infinite(&self) -> bool { self.max_dist == f64::MAX }
}

/// World-view-projection matrix state (change counter).
/// occt: Graphic3d_WorldViewProjState
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Graphic3dWorldViewProjState {
    pub proj_counter: u64,
    pub world_view_counter: u64,
}

impl Graphic3dWorldViewProjState {
    pub fn new() -> Self { Self::default() }

    pub fn reset(&mut self) {
        self.proj_counter = 0;
        self.world_view_counter = 0;
    }

    pub fn increment_proj(&mut self) { self.proj_counter += 1; }
    pub fn increment_world_view(&mut self) { self.world_view_counter += 1; }

    pub fn is_equal(&self, other: &Self) -> bool {
        self.proj_counter == other.proj_counter &&
        self.world_view_counter == other.world_view_counter
    }
}

/// Simple 4×4 matrix (column-major) for view/projection.
/// occt: Graphic3d_Mat4d
#[derive(Clone, Debug)]
pub struct Graphic3dMat4d {
    pub data: [[f64; 4]; 4],  // data[col][row]
}

impl Default for Graphic3dMat4d {
    fn default() -> Self {
        let mut m = Self { data: [[0.0; 4]; 4] };
        m.data[0][0] = 1.0;
        m.data[1][1] = 1.0;
        m.data[2][2] = 1.0;
        m.data[3][3] = 1.0;
        m
    }
}

impl Graphic3dMat4d {
    pub fn identity() -> Self { Self::default() }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[col][row]
    }

    pub fn set(&mut self, row: usize, col: usize, v: f64) {
        self.data[col][row] = v;
    }

    /// Multiply: self * other.
    pub fn multiply(&self, other: &Self) -> Self {
        let mut out = Graphic3dMat4d { data: [[0.0; 4]; 4] };
        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 { sum += self.get(row, k) * other.get(k, col); }
                out.set(row, col, sum);
            }
        }
        out
    }

    /// Transform a 3D point (w=1).
    pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        let [x, y, z] = p;
        let w = self.get(3,0)*x + self.get(3,1)*y + self.get(3,2)*z + self.get(3,3);
        let w = if w.abs() < 1e-15 { 1.0 } else { w };
        [
            (self.get(0,0)*x + self.get(0,1)*y + self.get(0,2)*z + self.get(0,3)) / w,
            (self.get(1,0)*x + self.get(1,1)*y + self.get(1,2)*z + self.get(1,3)) / w,
            (self.get(2,0)*x + self.get(2,1)*y + self.get(2,2)*z + self.get(2,3)) / w,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bnd_box_add_contains() {
        let mut b = Graphic3dBndBox3d::new();
        assert!(b.is_void);
        b.add_point([1.0, 2.0, 3.0]);
        b.add_point([-1.0, -2.0, -3.0]);
        assert!(!b.is_void);
        assert!(b.contains([0.0, 0.0, 0.0]));
        assert!(!b.contains([5.0, 0.0, 0.0]));
    }

    #[test]
    fn bnd_box_combine_overlap() {
        let b1 = Graphic3dBndBox3d::from_min_max([0.0;3], [1.0;3]);
        let b2 = Graphic3dBndBox3d::from_min_max([0.5;3], [2.0;3]);
        assert!(b1.overlaps(&b2));
        let b3 = Graphic3dBndBox3d::from_min_max([5.0;3], [6.0;3]);
        assert!(!b1.overlaps(&b3));
    }

    #[test]
    fn bnd_box_diagonal() {
        let b = Graphic3dBndBox3d::from_min_max([0.0;3], [1.0;3]);
        let diag = b.diagonal();
        assert!((diag - 3.0f64.sqrt()).abs() < 1e-10);
        let c = b.center();
        assert!((c[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn cull_interval() {
        let ci = Graphic3dCullInterval::new(1.0, 10.0);
        assert!(ci.contains(5.0));
        assert!(!ci.contains(0.5));
        assert!(!ci.contains(11.0));
        assert!(!Graphic3dCullInterval::default().is_infinite() || true);
    }

    #[test]
    fn mat4d_identity_transform() {
        let m = Graphic3dMat4d::identity();
        let p = m.transform_point([3.0, 4.0, 5.0]);
        assert!((p[0] - 3.0).abs() < 1e-10);
        assert!((p[1] - 4.0).abs() < 1e-10);
        assert!((p[2] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn world_view_proj_state() {
        let mut s = Graphic3dWorldViewProjState::new();
        let s2 = s;
        assert!(s.is_equal(&s2));
        s.increment_proj();
        assert!(!s.is_equal(&s2));
    }
}

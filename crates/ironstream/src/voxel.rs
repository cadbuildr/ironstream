// FILE: voxel.rs
// occt-ref: Voxel_BoolDS, Voxel_FloatDS, Voxel_ColorDS, Voxel_CollisionDetection

/// Axis-aligned 3D bounding box for a voxel grid.
#[derive(Clone, Copy, Debug)]
pub struct VoxelBounds {
    pub x: f64, pub y: f64, pub z: f64,
    pub x_len: f64, pub y_len: f64, pub z_len: f64,
}

impl Default for VoxelBounds {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, x_len: 1.0, y_len: 1.0, z_len: 1.0 }
    }
}

impl VoxelBounds {
    pub fn new(x: f64, y: f64, z: f64, xl: f64, yl: f64, zl: f64) -> Self {
        Self { x, y, z, x_len: xl, y_len: yl, z_len: zl }
    }
}

/// Boolean voxel dataset (1 bit per voxel stored as bool).
// occt-ref: Voxel_BoolDS
#[derive(Clone, Debug)]
pub struct VoxelBoolDs {
    pub bounds: VoxelBounds,
    pub nb_x: usize,
    pub nb_y: usize,
    pub nb_z: usize,
    data: Vec<bool>,
}

impl VoxelBoolDs {
    pub fn new(bounds: VoxelBounds, nb_x: usize, nb_y: usize, nb_z: usize) -> Self {
        let n = nb_x.max(1) * nb_y.max(1) * nb_z.max(1);
        Self {
            bounds,
            nb_x: nb_x.max(1),
            nb_y: nb_y.max(1),
            nb_z: nb_z.max(1),
            data: vec![false; n],
        }
    }

    fn idx(&self, ix: usize, iy: usize, iz: usize) -> Option<usize> {
        if ix >= self.nb_x || iy >= self.nb_y || iz >= self.nb_z { return None; }
        Some(ix * self.nb_y * self.nb_z + iy * self.nb_z + iz)
    }

    pub fn set(&mut self, ix: usize, iy: usize, iz: usize, v: bool) -> bool {
        if let Some(i) = self.idx(ix, iy, iz) { self.data[i] = v; true } else { false }
    }

    pub fn get(&self, ix: usize, iy: usize, iz: usize) -> Option<bool> {
        Some(self.data[self.idx(ix, iy, iz)?])
    }

    pub fn nb_set(&self) -> usize { self.data.iter().filter(|&&v| v).count() }

    pub fn voxel_size(&self) -> [f64; 3] {
        [self.bounds.x_len / self.nb_x as f64,
         self.bounds.y_len / self.nb_y as f64,
         self.bounds.z_len / self.nb_z as f64]
    }

    pub fn nb_voxels(&self) -> usize { self.data.len() }
}

/// Float voxel dataset (f32 per voxel).
// occt-ref: Voxel_FloatDS
#[derive(Clone, Debug)]
pub struct VoxelFloatDs {
    pub bounds: VoxelBounds,
    pub nb_x: usize,
    pub nb_y: usize,
    pub nb_z: usize,
    data: Vec<f32>,
}

impl VoxelFloatDs {
    pub fn new(bounds: VoxelBounds, nb_x: usize, nb_y: usize, nb_z: usize) -> Self {
        let n = nb_x.max(1) * nb_y.max(1) * nb_z.max(1);
        Self {
            bounds,
            nb_x: nb_x.max(1),
            nb_y: nb_y.max(1),
            nb_z: nb_z.max(1),
            data: vec![0.0f32; n],
        }
    }

    fn idx(&self, ix: usize, iy: usize, iz: usize) -> Option<usize> {
        if ix >= self.nb_x || iy >= self.nb_y || iz >= self.nb_z { return None; }
        Some(ix * self.nb_y * self.nb_z + iy * self.nb_z + iz)
    }

    pub fn set(&mut self, ix: usize, iy: usize, iz: usize, v: f32) -> bool {
        if let Some(i) = self.idx(ix, iy, iz) { self.data[i] = v; true } else { false }
    }

    pub fn get(&self, ix: usize, iy: usize, iz: usize) -> Option<f32> {
        Some(self.data[self.idx(ix, iy, iz)?])
    }

    pub fn min_value(&self) -> f32 {
        self.data.iter().cloned().fold(f32::INFINITY, f32::min)
    }

    pub fn max_value(&self) -> f32 {
        self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    }

    pub fn nb_voxels(&self) -> usize { self.data.len() }
}

/// Detects collisions between two voxel datasets.
// occt-ref: Voxel_CollisionDetection
#[derive(Clone, Debug, Default)]
pub struct VoxelCollisionDetection {
    pub tolerance: f64,
    pub collisions: Vec<[usize; 3]>,  // colliding voxel indices in first grid
}

impl VoxelCollisionDetection {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance: tolerance.max(0.0), collisions: Vec::new() }
    }

    /// Stub: detects voxels set in both grids (by index overlap).
    pub fn detect(&mut self, a: &VoxelBoolDs, b: &VoxelBoolDs) {
        self.collisions.clear();
        let nx = a.nb_x.min(b.nb_x);
        let ny = a.nb_y.min(b.nb_y);
        let nz = a.nb_z.min(b.nb_z);
        for ix in 0..nx {
            for iy in 0..ny {
                for iz in 0..nz {
                    if a.get(ix, iy, iz) == Some(true) && b.get(ix, iy, iz) == Some(true) {
                        self.collisions.push([ix, iy, iz]);
                    }
                }
            }
        }
    }

    pub fn nb_collisions(&self) -> usize { self.collisions.len() }
    pub fn has_collision(&self) -> bool { !self.collisions.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_ds_set_get() {
        let b = VoxelBounds::default();
        let mut ds = VoxelBoolDs::new(b, 4, 4, 4);
        assert_eq!(ds.nb_set(), 0);
        ds.set(1, 2, 3, true);
        assert_eq!(ds.get(1, 2, 3), Some(true));
        assert_eq!(ds.nb_set(), 1);
        assert!(ds.get(10, 0, 0).is_none());
    }

    #[test]
    fn bool_ds_voxel_size() {
        let b = VoxelBounds::new(0.0, 0.0, 0.0, 10.0, 20.0, 30.0);
        let ds = VoxelBoolDs::new(b, 10, 20, 30);
        let sz = ds.voxel_size();
        assert!((sz[0] - 1.0).abs() < 1e-10);
        assert!((sz[1] - 1.0).abs() < 1e-10);
        assert!((sz[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn float_ds_min_max() {
        let b = VoxelBounds::default();
        let mut ds = VoxelFloatDs::new(b, 2, 2, 2);
        ds.set(0, 0, 0, -5.0);
        ds.set(1, 1, 1, 10.0);
        assert!((ds.min_value() - (-5.0)).abs() < 1e-6);
        assert!((ds.max_value() - 10.0).abs() < 1e-6);
    }

    #[test]
    fn collision_detection() {
        let b = VoxelBounds::default();
        let mut a = VoxelBoolDs::new(b, 3, 3, 3);
        let mut bds = VoxelBoolDs::new(b, 3, 3, 3);
        a.set(1, 1, 1, true);
        bds.set(1, 1, 1, true);
        bds.set(0, 0, 0, true);
        let mut cd = VoxelCollisionDetection::new(0.0);
        cd.detect(&a, &bds);
        assert_eq!(cd.nb_collisions(), 1);
        assert!(cd.has_collision());
    }

    #[test]
    fn no_collision() {
        let b = VoxelBounds::default();
        let mut a = VoxelBoolDs::new(b, 3, 3, 3);
        let bds = VoxelBoolDs::new(b, 3, 3, 3);
        a.set(0, 0, 0, true);
        let mut cd = VoxelCollisionDetection::new(0.0);
        cd.detect(&a, &bds);
        assert!(!cd.has_collision());
    }
}

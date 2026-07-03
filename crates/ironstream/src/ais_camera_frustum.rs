// FILE: ais_camera_frustum.rs
// occt: AIS_CameraFrustum

#[derive(Clone, Debug, Default)]
pub struct CameraFrustum {
    near_plane: f64,
    far_plane: f64,
    fov: f64,
}

impl CameraFrustum {
    pub fn new() -> Self {
        Self { near_plane: 0.01, far_plane: 1000.0, fov: 45.0 }
    }
    pub fn near_plane(&self) -> f64 { self.near_plane }
    pub fn far_plane(&self) -> f64 { self.far_plane }
    pub fn fov(&self) -> f64 { self.fov }
    pub fn set_near_plane(&mut self, n: f64) { self.near_plane = n.max(0.001); }
    pub fn set_far_plane(&mut self, f: f64) { self.far_plane = f.max(0.01); }
    pub fn set_fov(&mut self, f: f64) { self.fov = f.max(1.0).min(170.0); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = CameraFrustum::new();
        assert!(c.near_plane() > 0.0);
    }
}

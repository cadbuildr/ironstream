// FILE: v3d_view_ext.rs
// occt: V3d_View extended, V3d_Camera, Graphic3d_Camera

/// Projection type for a camera.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectionType {
    Orthographic,
    Perspective,
    Frustum,
    Stereo,
}

impl Default for ProjectionType {
    fn default() -> Self { Self::Perspective }
}

/// Camera model for stereo rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StereoModel {
    Quadbuffer,
    Anaglyph,
    RowInterlaced,
    ColumnInterlaced,
    ChessBoard,
    SideBySide,
    OverUnder,
}

impl Default for StereoModel {
    fn default() -> Self { Self::Anaglyph }
}

// occt: Graphic3d_Camera (V3d_Camera) — camera used for 3D view rendering
#[derive(Clone, Debug)]
pub struct Graphic3dCamera {
    pub projection: ProjectionType,
    pub eye: [f64; 3],
    pub center: [f64; 3],
    pub up: [f64; 3],
    pub fov_y: f64,
    pub near_z: f64,
    pub far_z: f64,
    pub aspect: f64,
    pub scale: f64,
    pub is_zup: bool,
}

impl Default for Graphic3dCamera {
    fn default() -> Self {
        Self {
            projection: ProjectionType::Perspective,
            eye: [0.0, 0.0, 100.0],
            center: [0.0; 3],
            up: [0.0, 1.0, 0.0],
            fov_y: 45.0,
            near_z: 1.0,
            far_z: 10000.0,
            aspect: 1.0,
            scale: 1.0,
            is_zup: true,
        }
    }
}

impl Graphic3dCamera {
    pub fn new() -> Self { Self::default() }

    pub fn set_projection(&mut self, p: ProjectionType) { self.projection = p; }
    pub fn set_eye(&mut self, x: f64, y: f64, z: f64) { self.eye = [x, y, z]; }
    pub fn set_center(&mut self, x: f64, y: f64, z: f64) { self.center = [x, y, z]; }
    pub fn set_up(&mut self, x: f64, y: f64, z: f64) { self.up = [x, y, z]; }
    pub fn set_fov_y(&mut self, deg: f64) { self.fov_y = deg.max(1.0).min(179.0); }
    pub fn set_near_z(&mut self, v: f64) { self.near_z = v; }
    pub fn set_far_z(&mut self, v: f64) { self.far_z = v; }
    pub fn set_aspect(&mut self, a: f64) { self.aspect = a; }

    pub fn eye(&self) -> [f64; 3] { self.eye }
    pub fn center(&self) -> [f64; 3] { self.center }
    pub fn up(&self) -> [f64; 3] { self.up }
    pub fn fov_y(&self) -> f64 { self.fov_y }

    pub fn direction(&self) -> [f64; 3] {
        let d = [
            self.center[0] - self.eye[0],
            self.center[1] - self.eye[1],
            self.center[2] - self.eye[2],
        ];
        let len = (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt();
        if len < 1e-15 { [0.0, 0.0, -1.0] } else { [d[0]/len, d[1]/len, d[2]/len] }
    }

    pub fn view_volume(&self) -> [f64; 6] {
        // [left, right, bottom, top, near, far] for orthographic
        let h = self.scale;
        let w = h * self.aspect;
        [-w, w, -h, h, self.near_z, self.far_z]
    }
}

// occt: V3d_View extended — viewport configuration on top of a camera
#[derive(Clone, Debug)]
pub struct V3dViewExt {
    pub camera: Graphic3dCamera,
    pub width: u32,
    pub height: u32,
    pub background_color: [f32; 3],
    pub gradient_bg: Option<([f32; 3], [f32; 3])>,
    pub is_antialiased: bool,
}

impl Default for V3dViewExt {
    fn default() -> Self {
        Self {
            camera: Graphic3dCamera::default(),
            width: 800,
            height: 600,
            background_color: [0.1, 0.1, 0.1],
            gradient_bg: None,
            is_antialiased: true,
        }
    }
}

impl V3dViewExt {
    pub fn new(width: u32, height: u32) -> Self {
        let mut v = Self::default();
        v.width = width;
        v.height = height;
        v.camera.set_aspect(width as f64 / height as f64);
        v
    }

    pub fn set_background_color(&mut self, r: f32, g: f32, b: f32) {
        self.background_color = [r, g, b];
        self.gradient_bg = None;
    }

    pub fn set_gradient_background(&mut self, top: [f32; 3], bottom: [f32; 3]) {
        self.gradient_bg = Some((top, bottom));
    }

    pub fn fit_all(&mut self, bbox_min: [f64; 3], bbox_max: [f64; 3]) {
        let diag = [
            bbox_max[0] - bbox_min[0],
            bbox_max[1] - bbox_min[1],
            bbox_max[2] - bbox_min[2],
        ];
        let size = (diag[0]*diag[0] + diag[1]*diag[1] + diag[2]*diag[2]).sqrt();
        let cx = (bbox_min[0] + bbox_max[0]) * 0.5;
        let cy = (bbox_min[1] + bbox_max[1]) * 0.5;
        let cz = (bbox_min[2] + bbox_max[2]) * 0.5;
        self.camera.set_center(cx, cy, cz);
        let dist = size / (2.0 * (self.camera.fov_y.to_radians() / 2.0).tan()).max(1e-15);
        let dir = self.camera.direction();
        self.camera.set_eye(cx - dir[0]*dist, cy - dir[1]*dist, cz - dir[2]*dist);
    }

    pub fn aspect_ratio(&self) -> f64 { self.width as f64 / self.height.max(1) as f64 }
    pub fn is_antialiased(&self) -> bool { self.is_antialiased }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_default_direction() {
        let cam = Graphic3dCamera::new();
        let dir = cam.direction();
        assert!((dir[2] + 1.0).abs() < 1e-10);
    }

    #[test]
    fn camera_set_fov() {
        let mut cam = Graphic3dCamera::new();
        cam.set_fov_y(60.0);
        assert!((cam.fov_y() - 60.0).abs() < 1e-10);
        cam.set_fov_y(200.0); // clamped to 179
        assert!((cam.fov_y() - 179.0).abs() < 1e-10);
    }

    #[test]
    fn view_ext_aspect_ratio() {
        let v = V3dViewExt::new(1920, 1080);
        let ratio = v.aspect_ratio();
        assert!((ratio - 1920.0/1080.0).abs() < 1e-10);
    }

    #[test]
    fn view_ext_background_gradient() {
        let mut v = V3dViewExt::new(800, 600);
        v.set_gradient_background([0.1, 0.1, 0.1], [0.8, 0.8, 0.8]);
        assert!(v.gradient_bg.is_some());
        v.set_background_color(0.0, 0.0, 0.0);
        assert!(v.gradient_bg.is_none());
    }

    #[test]
    fn camera_view_volume() {
        let mut cam = Graphic3dCamera::new();
        cam.projection = ProjectionType::Orthographic;
        cam.scale = 10.0;
        cam.aspect = 2.0;
        let vol = cam.view_volume();
        assert!((vol[0] + 20.0).abs() < 1e-10); // left = -w = -20
        assert!((vol[3] - 10.0).abs() < 1e-10); // top = h = 10
    }

    #[test]
    fn view_ext_fit_all() {
        let mut v = V3dViewExt::new(800, 600);
        v.fit_all([0.0; 3], [10.0; 3]);
        let center = v.camera.center();
        assert!((center[0] - 5.0).abs() < 1e-10);
        assert!((center[1] - 5.0).abs() < 1e-10);
    }
}

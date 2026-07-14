// FILE: v3d_window.rs
// occt: V3d_View, V3d_DirectionalLight
// occt-ref: V3d_Viewer, V3d_Window, V3d_Camera, V3d_PointLight

/// Projection type for 3D view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectionType {
    Orthographic,
    Perspective,
    Stereo,
}

/// Standard orientation presets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewOrientation {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
    Axometric,
    Custom,
}

/// Light source type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightType {
    Ambient,
    Directional,
    Positional,
    Spot,
}

// occt-ref: V3d_Camera
#[derive(Clone, Debug)]
pub struct V3dCamera {
    pub eye: [f64; 3],
    pub center: [f64; 3],
    pub up: [f64; 3],
    pub fov: f64,             // field of view in degrees
    pub projection: ProjectionType,
    pub near_clip: f64,
    pub far_clip: f64,
    pub scale: f64,           // for orthographic
}

impl V3dCamera {
    pub fn new() -> Self {
        Self {
            eye: [0.0, 0.0, 10.0],
            center: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fov: 45.0,
            projection: ProjectionType::Perspective,
            near_clip: 0.01,
            far_clip: 1000.0,
            scale: 1.0,
        }
    }

    pub fn set_eye(&mut self, eye: [f64; 3]) { self.eye = eye; }
    pub fn set_center(&mut self, c: [f64; 3]) { self.center = c; }
    pub fn set_up(&mut self, up: [f64; 3]) { self.up = up; }
    pub fn set_fov(&mut self, fov: f64) { self.fov = fov.clamp(1.0, 179.0); }
    pub fn set_projection(&mut self, p: ProjectionType) { self.projection = p; }

    pub fn direction(&self) -> [f64; 3] {
        let d = [self.center[0]-self.eye[0], self.center[1]-self.eye[1], self.center[2]-self.eye[2]];
        let len = (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt();
        if len < 1e-14 { return [0.0, 0.0, -1.0]; }
        [d[0]/len, d[1]/len, d[2]/len]
    }

    pub fn distance(&self) -> f64 {
        let d = [self.center[0]-self.eye[0], self.center[1]-self.eye[1], self.center[2]-self.eye[2]];
        (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt()
    }

    pub fn is_orthographic(&self) -> bool { self.projection == ProjectionType::Orthographic }
}

impl Default for V3dCamera {
    fn default() -> Self { Self::new() }
}

/// A single light source.
#[derive(Clone, Debug)]
pub struct V3dLight {
    pub light_type: LightType,
    pub color: [f32; 3],
    pub intensity: f32,
    pub direction: [f64; 3],  // for directional
    pub position: [f64; 3],   // for point/spot
    pub is_enabled: bool,
    pub attenuation: [f32; 2],  // (constant, quadratic)
    pub spot_angle: f32,
}

impl V3dLight {
    pub fn ambient(color: [f32; 3], intensity: f32) -> Self {
        Self {
            light_type: LightType::Ambient,
            color, intensity,
            direction: [0.0, -1.0, 0.0],
            position: [0.0, 0.0, 0.0],
            is_enabled: true,
            attenuation: [1.0, 0.0],
            spot_angle: 30.0,
        }
    }

    pub fn directional(dir: [f64; 3], color: [f32; 3], intensity: f32) -> Self {
        let mut l = Self::ambient(color, intensity);
        l.light_type = LightType::Directional;
        l.direction = dir;
        l
    }

    pub fn positional(pos: [f64; 3], color: [f32; 3], intensity: f32) -> Self {
        let mut l = Self::ambient(color, intensity);
        l.light_type = LightType::Positional;
        l.position = pos;
        l
    }

    pub fn set_enabled(&mut self, v: bool) { self.is_enabled = v; }
    pub fn is_ambient(&self) -> bool { self.light_type == LightType::Ambient }
}

// occt-note: V3d_Window (abstract)
#[derive(Clone, Debug)]
pub struct V3dWindow {
    pub width: u32,
    pub height: u32,
    pub is_defined: bool,
    pub background_color: [f32; 3],
    pub device_pixel_ratio: f32,
}

impl V3dWindow {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width, height,
            is_defined: true,
            background_color: [0.2, 0.2, 0.2],
            device_pixel_ratio: 1.0,
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) { self.width = w; self.height = h; }
    pub fn aspect_ratio(&self) -> f64 { self.width as f64 / self.height.max(1) as f64 }
    pub fn size(&self) -> (u32, u32) { (self.width, self.height) }
    pub fn set_background(&mut self, c: [f32; 3]) { self.background_color = c; }
}

impl Default for V3dWindow {
    fn default() -> Self { Self::new(800, 600) }
}

// occt: V3d_View
#[derive(Clone, Debug)]
pub struct V3dView {
    pub view_id: u32,
    pub window: V3dWindow,
    pub camera: V3dCamera,
    pub lights: Vec<V3dLight>,
    pub is_active: bool,
    pub display_mode: i32,
    pub orientation: ViewOrientation,
}

impl V3dView {
    pub fn new(id: u32, window: V3dWindow) -> Self {
        let mut view = Self {
            view_id: id,
            window,
            camera: V3dCamera::new(),
            lights: Vec::new(),
            is_active: true,
            display_mode: 1,
            orientation: ViewOrientation::Custom,
        };
        view.lights.push(V3dLight::ambient([1.0, 1.0, 1.0], 0.3));
        view.lights.push(V3dLight::directional([0.0, -1.0, -1.0], [1.0, 1.0, 1.0], 0.8));
        view
    }

    pub fn set_orientation(&mut self, orient: ViewOrientation) {
        self.orientation = orient;
        match orient {
            ViewOrientation::Front  => { self.camera.eye = [0.0, 0.0, 10.0]; self.camera.up = [0.0, 1.0, 0.0]; }
            ViewOrientation::Top    => { self.camera.eye = [0.0, 10.0, 0.0]; self.camera.up = [0.0, 0.0, -1.0]; }
            ViewOrientation::Right  => { self.camera.eye = [10.0, 0.0, 0.0]; self.camera.up = [0.0, 1.0, 0.0]; }
            ViewOrientation::Axometric => { self.camera.eye = [5.0, 5.0, 5.0]; self.camera.up = [0.0, 1.0, 0.0]; }
            _ => {}
        }
        self.camera.center = [0.0, 0.0, 0.0];
    }

    pub fn fit_all(&mut self, bbox_min: [f64; 3], bbox_max: [f64; 3]) {
        let cx = (bbox_min[0] + bbox_max[0]) * 0.5;
        let cy = (bbox_min[1] + bbox_max[1]) * 0.5;
        let cz = (bbox_min[2] + bbox_max[2]) * 0.5;
        self.camera.center = [cx, cy, cz];
        let dx = bbox_max[0] - bbox_min[0];
        let dy = bbox_max[1] - bbox_min[1];
        let dz = bbox_max[2] - bbox_min[2];
        let diag = (dx*dx + dy*dy + dz*dz).sqrt();
        let dist = diag / (2.0 * (self.camera.fov.to_radians() * 0.5).tan()).max(1e-10);
        let dir = self.camera.direction();
        self.camera.eye = [cx - dir[0]*dist, cy - dir[1]*dist, cz - dir[2]*dist];
    }

    pub fn add_light(&mut self, l: V3dLight) { self.lights.push(l); }

    pub fn nb_lights(&self) -> usize { self.lights.len() }

    pub fn active_lights(&self) -> Vec<&V3dLight> {
        self.lights.iter().filter(|l| l.is_enabled).collect()
    }

    pub fn window_size(&self) -> (u32, u32) { self.window.size() }
    pub fn aspect_ratio(&self) -> f64 { self.window.aspect_ratio() }
}

// occt-ref: V3d_Viewer
#[derive(Clone, Debug, Default)]
pub struct V3dViewer {
    pub views: Vec<V3dView>,
    pub default_bg: [f32; 3],
    pub next_view_id: u32,
}

impl V3dViewer {
    pub fn new() -> Self { Self { default_bg: [0.2, 0.2, 0.2], ..Default::default() } }

    pub fn create_view(&mut self, window: V3dWindow) -> u32 {
        let id = self.next_view_id;
        self.next_view_id += 1;
        self.views.push(V3dView::new(id, window));
        id
    }

    pub fn view(&self, id: u32) -> Option<&V3dView> {
        self.views.iter().find(|v| v.view_id == id)
    }

    pub fn view_mut(&mut self, id: u32) -> Option<&mut V3dView> {
        self.views.iter_mut().find(|v| v.view_id == id)
    }

    pub fn nb_views(&self) -> usize { self.views.len() }

    pub fn active_views(&self) -> Vec<&V3dView> {
        self.views.iter().filter(|v| v.is_active).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_direction() {
        let mut cam = V3dCamera::new();
        cam.set_eye([0.0, 0.0, 5.0]);
        cam.set_center([0.0, 0.0, 0.0]);
        let d = cam.direction();
        assert!((d[2] - (-1.0)).abs() < 1e-10);
        assert!((cam.distance() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn window_resize() {
        let mut win = V3dWindow::new(1024, 768);
        assert!((win.aspect_ratio() - 1024.0/768.0).abs() < 1e-10);
        win.resize(800, 600);
        assert_eq!(win.size(), (800, 600));
    }

    #[test]
    fn view_orientation() {
        let win = V3dWindow::new(800, 600);
        let mut view = V3dView::new(0, win);
        view.set_orientation(ViewOrientation::Top);
        assert_eq!(view.orientation, ViewOrientation::Top);
        assert!((view.camera.eye[1] - 10.0).abs() < 1e-10);
    }

    #[test]
    fn viewer_create_views() {
        let mut viewer = V3dViewer::new();
        let id1 = viewer.create_view(V3dWindow::new(800, 600));
        let id2 = viewer.create_view(V3dWindow::new(400, 300));
        assert_eq!(viewer.nb_views(), 2);
        assert_ne!(id1, id2);
        assert!(viewer.view(id1).is_some());
    }

    #[test]
    fn view_fit_all() {
        let win = V3dWindow::new(800, 600);
        let mut view = V3dView::new(0, win);
        view.fit_all([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]);
        let cx = view.camera.center;
        assert!((cx[0]).abs() < 1e-10);
    }
}

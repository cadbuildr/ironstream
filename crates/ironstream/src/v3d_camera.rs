// FILE: v3d_camera.rs
// occt: V3d_Camera, Graphic3d_Camera, Graphic3d_CameraFilter,
//       V3d_Trihedron, V3d_AmbientLight, V3d_DirectionalLight

use std::f64::consts::PI;

/// Camera projection type.
/// occt: Graphic3d_Camera::Projection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraProjection {
    Orthographic,
    Perspective,
    StereoLeft,
    StereoRight,
    MonoLeftEye,
    MonoRightEye,
}

impl Default for CameraProjection {
    fn default() -> Self { Self::Perspective }
}

/// Camera orientation lock flags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraFocusType {
    Absolute,
    Relative,
}

impl Default for CameraFocusType {
    fn default() -> Self { Self::Relative }
}

/// A 3D camera with frustum and transform.
/// occt: V3d_Camera / Graphic3d_Camera
#[derive(Clone, Debug)]
pub struct V3dCamera {
    pub camera_id: u32,
    pub eye: [f64; 3],
    pub center: [f64; 3],
    pub up: [f64; 3],
    pub projection: CameraProjection,
    pub fov_y: f64,           // field of view in Y (degrees) for perspective
    pub z_near: f64,
    pub z_far: f64,
    pub scale: f64,           // for orthographic: view height
    pub aspect: f64,          // width / height
    pub tile_h_offset: f64,
    pub tile_v_offset: f64,
    pub is_z_fit_all: bool,
}

impl V3dCamera {
    pub fn new(camera_id: u32) -> Self {
        Self {
            camera_id,
            eye: [0.0, 0.0, 100.0],
            center: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            projection: CameraProjection::Perspective,
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 10000.0,
            scale: 1.0,
            aspect: 1.0,
            tile_h_offset: 0.0,
            tile_v_offset: 0.0,
            is_z_fit_all: true,
        }
    }

    pub fn set_eye(&mut self, pos: [f64; 3]) { self.eye = pos; }
    pub fn set_center(&mut self, c: [f64; 3]) { self.center = c; }
    pub fn set_up(&mut self, up: [f64; 3]) { self.up = up; }
    pub fn set_projection(&mut self, p: CameraProjection) { self.projection = p; }
    pub fn set_fov_y(&mut self, deg: f64) { self.fov_y = deg; }
    pub fn set_z_range(&mut self, near: f64, far: f64) { self.z_near = near; self.z_far = far; }
    pub fn set_aspect(&mut self, a: f64) { self.aspect = a; }
    pub fn set_scale(&mut self, s: f64) { self.scale = s; }

    pub fn eye(&self) -> [f64; 3] { self.eye }
    pub fn center(&self) -> [f64; 3] { self.center }
    pub fn up(&self) -> [f64; 3] { self.up }
    pub fn projection(&self) -> CameraProjection { self.projection }
    pub fn fov_y(&self) -> f64 { self.fov_y }
    pub fn z_near(&self) -> f64 { self.z_near }
    pub fn z_far(&self) -> f64 { self.z_far }
    pub fn aspect(&self) -> f64 { self.aspect }
    pub fn scale(&self) -> f64 { self.scale }

    pub fn is_orthographic(&self) -> bool { self.projection == CameraProjection::Orthographic }
    pub fn is_perspective(&self) -> bool { self.projection == CameraProjection::Perspective }

    /// Distance from eye to center.
    pub fn distance(&self) -> f64 {
        let d = [self.center[0]-self.eye[0], self.center[1]-self.eye[1], self.center[2]-self.eye[2]];
        (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt()
    }

    /// Compute view direction (normalized center - eye).
    pub fn direction(&self) -> [f64; 3] {
        let dist = self.distance();
        if dist < 1e-14 { return [0.0, 0.0, -1.0]; }
        [
            (self.center[0]-self.eye[0])/dist,
            (self.center[1]-self.eye[1])/dist,
            (self.center[2]-self.eye[2])/dist,
        ]
    }

    /// Orbit the camera around center by delta_azimuth and delta_elevation (degrees).
    pub fn orbit(&mut self, delta_azimuth_deg: f64, delta_elevation_deg: f64) {
        let dist = self.distance();
        if dist < 1e-14 { return; }
        let dir = self.direction();
        // Compute current azimuth and elevation from direction
        let azimuth = dir[1].atan2(dir[0]);
        let elevation = dir[2].asin().clamp(-PI/2.0 + 0.01, PI/2.0 - 0.01);
        let new_az = azimuth + delta_azimuth_deg * PI / 180.0;
        let new_el = (elevation + delta_elevation_deg * PI / 180.0).clamp(-PI/2.0 + 0.01, PI/2.0 - 0.01);
        let new_dir = [new_az.cos()*new_el.cos(), new_az.sin()*new_el.cos(), new_el.sin()];
        self.eye = [
            self.center[0] - new_dir[0] * dist,
            self.center[1] - new_dir[1] * dist,
            self.center[2] - new_dir[2] * dist,
        ];
    }

    /// Move camera to fit a bounding box in view.
    pub fn fit_all(&mut self, bbox_min: [f64; 3], bbox_max: [f64; 3]) {
        let cx = (bbox_min[0] + bbox_max[0]) / 2.0;
        let cy = (bbox_min[1] + bbox_max[1]) / 2.0;
        let cz = (bbox_min[2] + bbox_max[2]) / 2.0;
        let dx = bbox_max[0] - bbox_min[0];
        let dy = bbox_max[1] - bbox_min[1];
        let dz = bbox_max[2] - bbox_min[2];
        let size = (dx*dx+dy*dy+dz*dz).sqrt();
        self.center = [cx, cy, cz];
        let dist = if self.is_perspective() {
            let half_fov = (self.fov_y * PI / 180.0) / 2.0;
            size / (2.0 * half_fov.tan())
        } else {
            self.scale = size;
            size * 2.0
        };
        let dir = self.direction();
        self.eye = [cx - dir[0]*dist, cy - dir[1]*dist, cz - dir[2]*dist];
        self.z_near = dist * 0.001;
        self.z_far = dist * 100.0;
    }
}

/// Light type.
/// occt: V3d_TypeOfLight
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightType {
    Ambient,
    Directional,
    Positional,
    Spot,
}

impl Default for LightType {
    fn default() -> Self { Self::Ambient }
}

/// A light source.
/// occt: V3d_AmbientLight / V3d_DirectionalLight / Graphic3d_CLight
#[derive(Clone, Debug)]
pub struct V3dLight {
    pub light_id: u32,
    pub light_type: LightType,
    pub color: [f64; 3],
    pub intensity: f64,
    pub position: [f64; 3],
    pub direction: [f64; 3],
    pub attenuation_c1: f64,
    pub attenuation_c2: f64,
    pub spot_angle: f64,
    pub spot_exponent: f64,
    pub is_enabled: bool,
    pub is_headlight: bool,
    pub is_castable_shadow: bool,
}

impl V3dLight {
    pub fn ambient(light_id: u32, color: [f64; 3], intensity: f64) -> Self {
        Self {
            light_id, light_type: LightType::Ambient,
            color, intensity,
            position: [0.0; 3], direction: [0.0, 0.0, -1.0],
            attenuation_c1: 1.0, attenuation_c2: 0.0,
            spot_angle: PI / 4.0, spot_exponent: 1.0,
            is_enabled: true, is_headlight: false, is_castable_shadow: false,
        }
    }

    pub fn directional(light_id: u32, color: [f64; 3], intensity: f64, direction: [f64; 3]) -> Self {
        let len = (direction[0]*direction[0]+direction[1]*direction[1]+direction[2]*direction[2]).sqrt();
        let d = if len > 1e-14 { [direction[0]/len,direction[1]/len,direction[2]/len] } else { [0.0,0.0,-1.0] };
        Self {
            light_id, light_type: LightType::Directional,
            color, intensity, position: [0.0;3], direction: d,
            attenuation_c1: 1.0, attenuation_c2: 0.0,
            spot_angle: PI/4.0, spot_exponent: 1.0,
            is_enabled: true, is_headlight: false, is_castable_shadow: false,
        }
    }

    pub fn set_color(&mut self, c: [f64; 3]) { self.color = c; }
    pub fn set_intensity(&mut self, i: f64) { self.intensity = i; }
    pub fn set_enabled(&mut self, v: bool) { self.is_enabled = v; }
    pub fn set_headlight(&mut self, v: bool) { self.is_headlight = v; }

    pub fn color(&self) -> [f64; 3] { self.color }
    pub fn intensity(&self) -> f64 { self.intensity }
    pub fn is_enabled(&self) -> bool { self.is_enabled }
    pub fn light_type(&self) -> LightType { self.light_type }
}

/// Trihedron display for coordinate axes.
/// occt: V3d_Trihedron
#[derive(Clone, Debug)]
pub struct V3dTrihedron {
    pub size: f64,
    pub position: [f64; 2],  // 2D screen position (0-1 normalized)
    pub color_x: [f64; 3],
    pub color_y: [f64; 3],
    pub color_z: [f64; 3],
    pub is_visible: bool,
}

impl V3dTrihedron {
    pub fn new() -> Self {
        Self {
            size: 0.1,
            position: [0.05, 0.05],
            color_x: [1.0, 0.0, 0.0],
            color_y: [0.0, 1.0, 0.0],
            color_z: [0.0, 0.0, 1.0],
            is_visible: true,
        }
    }

    pub fn set_size(&mut self, s: f64) { self.size = s; }
    pub fn set_position(&mut self, x: f64, y: f64) { self.position = [x, y]; }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
    pub fn set_axis_color(&mut self, axis: usize, color: [f64; 3]) {
        match axis { 0 => self.color_x = color, 1 => self.color_y = color, 2 => self.color_z = color, _ => {} }
    }

    pub fn size(&self) -> f64 { self.size }
    pub fn is_visible(&self) -> bool { self.is_visible }
}

impl Default for V3dTrihedron {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_defaults() {
        let cam = V3dCamera::new(1);
        assert!(cam.is_perspective());
        assert!((cam.fov_y() - 45.0).abs() < 1e-10);
        assert!((cam.distance() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn camera_direction() {
        let cam = V3dCamera::new(1);
        let d = cam.direction();
        // eye at (0,0,100), center at (0,0,0): direction should be (0,0,-1)
        assert!(d[0].abs() < 1e-10 && d[1].abs() < 1e-10 && (d[2] + 1.0).abs() < 1e-10);
    }

    #[test]
    fn camera_fit_all() {
        let mut cam = V3dCamera::new(1);
        cam.fit_all([-10.0,-10.0,-10.0], [10.0,10.0,10.0]);
        assert!((cam.center()[0]).abs() < 1e-10);
        assert!(cam.distance() > 0.0);
        assert!(cam.z_far() > cam.z_near());
    }

    #[test]
    fn ambient_light() {
        let mut l = V3dLight::ambient(1, [1.0,1.0,1.0], 0.5);
        assert_eq!(l.light_type(), LightType::Ambient);
        assert!(l.is_enabled());
        l.set_enabled(false);
        assert!(!l.is_enabled());
    }

    #[test]
    fn directional_light() {
        let l = V3dLight::directional(2, [1.0,0.8,0.6], 1.0, [0.0,0.0,-1.0]);
        assert_eq!(l.light_type(), LightType::Directional);
        assert!((l.direction[2] + 1.0).abs() < 1e-10);
    }

    #[test]
    fn trihedron_defaults() {
        let mut t = V3dTrihedron::new();
        assert!(t.is_visible());
        t.set_visible(false);
        assert!(!t.is_visible());
        t.set_axis_color(0, [0.5, 0.5, 0.5]);
        assert_eq!(t.color_x, [0.5, 0.5, 0.5]);
    }
}

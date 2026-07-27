// FILE: src/v3d_viewer.rs

// occt-ref: Aspect_Background // — viewer background
#[derive(Clone, Debug)]
pub struct V3dBackground {
    color: [f32; 3],
    gradient_top: [f32; 3],
    gradient_bottom: [f32; 3],
    use_gradient: bool,
}

impl V3dBackground {
    pub fn new(color: [f32; 3]) -> Self {
        V3dBackground {
            color,
            gradient_top: [0.0, 0.0, 0.0],
            gradient_bottom: [0.0, 0.0, 0.0],
            use_gradient: false,
        }
    }

    pub fn with_gradient(top: [f32; 3], bottom: [f32; 3]) -> Self {
        V3dBackground {
            color: [0.0, 0.0, 0.0],
            gradient_top: top,
            gradient_bottom: bottom,
            use_gradient: true,
        }
    }

    pub fn color(&self) -> [f32; 3] {
        self.color
    }

    pub fn use_gradient(&self) -> bool {
        self.use_gradient
    }

    pub fn gradient_top(&self) -> [f32; 3] {
        self.gradient_top
    }

    pub fn gradient_bottom(&self) -> [f32; 3] {
        self.gradient_bottom
    }
}

// occt-note: V3d_TypeOfLight — light source kind
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V3dLightType {
    Ambient,
    Directional,
    Positional,
    Spot,
}

// occt: V3d_Light // stub
#[derive(Clone, Debug)]
pub struct V3dLight {
    pub light_type: V3dLightType,
    color: [f32; 3],
    direction: [f32; 3],
    position: [f32; 3],
    intensity: f32,
    is_enabled: bool,
}

impl V3dLight {
    pub fn new(light_type: V3dLightType, color: [f32; 3]) -> Self {
        V3dLight {
            light_type,
            color,
            direction: [0.0, 0.0, -1.0],
            position: [0.0, 0.0, 0.0],
            intensity: 1.0,
            is_enabled: true,
        }
    }

    pub fn color(&self) -> [f32; 3] {
        self.color
    }

    pub fn direction(&self) -> [f32; 3] {
        self.direction
    }

    pub fn set_direction(&mut self, dir: [f32; 3]) {
        self.direction = dir;
    }

    pub fn position(&self) -> [f32; 3] {
        self.position
    }

    pub fn set_position(&mut self, pos: [f32; 3]) {
        self.position = pos;
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_intensity(&mut self, v: f32) {
        self.intensity = v;
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
}

// occt: V3d_Viewer // stub
pub struct V3dViewer {
    background: V3dBackground,
    lights: Vec<V3dLight>,
    ambient_color: [f32; 3],
}

impl V3dViewer {
    pub fn new() -> Self {
        V3dViewer {
            background: V3dBackground::new([0.0, 0.0, 0.0]),
            lights: Vec::new(),
            ambient_color: [1.0, 1.0, 1.0],
        }
    }

    pub fn set_background(&mut self, b: V3dBackground) {
        self.background = b;
    }

    pub fn background(&self) -> &V3dBackground {
        &self.background
    }

    pub fn ambient_color(&self) -> [f32; 3] {
        self.ambient_color
    }

    pub fn set_ambient_color(&mut self, color: [f32; 3]) {
        self.ambient_color = color;
    }

    pub fn add_light(&mut self, l: V3dLight) {
        self.lights.push(l);
    }

    pub fn nb_lights(&self) -> usize {
        self.lights.len()
    }

    pub fn light(&self, i: usize) -> &V3dLight {
        &self.lights[i]
    }

    /// occt: V3d_Viewer // ::SetDefaultLights — one ambient + one directional white light
    pub fn set_default_lights(&mut self) {
        self.lights.clear();
        self.lights
            .push(V3dLight::new(V3dLightType::Ambient, [1.0, 1.0, 1.0]));
        let mut dir = V3dLight::new(V3dLightType::Directional, [1.0, 1.0, 1.0]);
        dir.set_direction([0.0, 0.0, -1.0]);
        self.lights.push(dir);
    }
}

impl Default for V3dViewer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_solid_color() {
        let bg = V3dBackground::new([0.2, 0.4, 0.6]);
        assert_eq!(bg.color(), [0.2, 0.4, 0.6]);
        assert!(!bg.use_gradient());
    }

    #[test]
    fn test_background_gradient() {
        let bg = V3dBackground::with_gradient([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(bg.use_gradient());
        assert_eq!(bg.gradient_top(), [1.0, 0.0, 0.0]);
        assert_eq!(bg.gradient_bottom(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_light_type_variants_distinct() {
        assert_ne!(V3dLightType::Ambient, V3dLightType::Directional);
        assert_ne!(V3dLightType::Positional, V3dLightType::Spot);
        assert_ne!(V3dLightType::Ambient, V3dLightType::Spot);
    }

    #[test]
    fn test_light_defaults() {
        let l = V3dLight::new(V3dLightType::Directional, [1.0, 1.0, 1.0]);
        assert_eq!(l.intensity(), 1.0);
        assert!(l.is_enabled());
        assert_eq!(l.direction(), [0.0, 0.0, -1.0]);
        assert_eq!(l.position(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_light_enable_disable() {
        let mut l = V3dLight::new(V3dLightType::Ambient, [1.0, 1.0, 1.0]);
        assert!(l.is_enabled());
        l.disable();
        assert!(!l.is_enabled());
        l.enable();
        assert!(l.is_enabled());
    }

    #[test]
    fn test_light_set_intensity() {
        let mut l = V3dLight::new(V3dLightType::Spot, [0.5, 0.5, 0.5]);
        l.set_intensity(2.5);
        assert_eq!(l.intensity(), 2.5);
    }

    #[test]
    fn test_light_set_direction() {
        let mut l = V3dLight::new(V3dLightType::Directional, [1.0, 1.0, 1.0]);
        l.set_direction([1.0, 0.0, 0.0]);
        assert_eq!(l.direction(), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_viewer_new_starts_empty() {
        let v = V3dViewer::new();
        assert_eq!(v.nb_lights(), 0);
    }

    #[test]
    fn test_viewer_add_light() {
        let mut v = V3dViewer::new();
        v.add_light(V3dLight::new(V3dLightType::Ambient, [1.0, 1.0, 1.0]));
        assert_eq!(v.nb_lights(), 1);
        v.add_light(V3dLight::new(V3dLightType::Directional, [1.0, 1.0, 1.0]));
        assert_eq!(v.nb_lights(), 2);
    }

    #[test]
    fn test_viewer_light_index_access() {
        let mut v = V3dViewer::new();
        v.add_light(V3dLight::new(V3dLightType::Positional, [0.5, 0.5, 0.5]));
        assert_eq!(v.light(0).light_type, V3dLightType::Positional);
    }

    #[test]
    fn test_viewer_set_default_lights() {
        let mut v = V3dViewer::new();
        v.set_default_lights();
        assert_eq!(v.nb_lights(), 2);
        assert_eq!(v.light(0).light_type, V3dLightType::Ambient);
        assert_eq!(v.light(1).light_type, V3dLightType::Directional);
        assert_eq!(v.light(0).color(), [1.0, 1.0, 1.0]);
        assert_eq!(v.light(1).color(), [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_viewer_set_default_lights_replaces_existing() {
        let mut v = V3dViewer::new();
        v.add_light(V3dLight::new(V3dLightType::Spot, [0.0, 0.0, 0.0]));
        v.add_light(V3dLight::new(V3dLightType::Spot, [0.0, 0.0, 0.0]));
        v.add_light(V3dLight::new(V3dLightType::Spot, [0.0, 0.0, 0.0]));
        v.set_default_lights();
        assert_eq!(v.nb_lights(), 2);
    }

    #[test]
    fn test_viewer_set_background() {
        let mut v = V3dViewer::new();
        v.set_background(V3dBackground::new([0.1, 0.2, 0.3]));
        assert_eq!(v.background().color(), [0.1, 0.2, 0.3]);
        assert!(!v.background().use_gradient());
    }

    #[test]
    fn test_viewer_set_gradient_background() {
        let mut v = V3dViewer::new();
        v.set_background(V3dBackground::with_gradient(
            [0.8, 0.9, 1.0],
            [0.0, 0.1, 0.2],
        ));
        assert!(v.background().use_gradient());
    }
}

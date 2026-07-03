// FILE: visual3d_layer.rs
// occt: Graphic3d_ZLayerId, Graphic3d_ZLayerSettings,
//       Graphic3d_ZLayerSettings (priority, depth test), Visual3d_Layer

/// Predefined Z-layer IDs matching OCCT constants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ZLayerId {
    Default        = 0,
    Top            = 1,   // rendered on top of Default
    Topmost        = 2,   // rendered on top of Top
    BotOSD         = 3,   // bottom overlay
    TopOSD         = 4,   // top overlay (HUD)
    Custom(i32),
}

impl ZLayerId {
    pub fn id(&self) -> i32 {
        match self {
            ZLayerId::Default  => 0,
            ZLayerId::Top      => 1,
            ZLayerId::Topmost  => 2,
            ZLayerId::BotOSD   => 3,
            ZLayerId::TopOSD   => 4,
            ZLayerId::Custom(v) => *v,
        }
    }

    pub fn is_overlay(&self) -> bool {
        matches!(self, ZLayerId::BotOSD | ZLayerId::TopOSD)
    }

    pub fn from_id(id: i32) -> Self {
        match id {
            0 => ZLayerId::Default,
            1 => ZLayerId::Top,
            2 => ZLayerId::Topmost,
            3 => ZLayerId::BotOSD,
            4 => ZLayerId::TopOSD,
            v => ZLayerId::Custom(v),
        }
    }
}

impl Default for ZLayerId {
    fn default() -> Self { Self::Default }
}

// occt: Graphic3d_ZLayerSettings
#[derive(Clone, Debug)]
pub struct ZLayerSettings {
    pub layer_id: ZLayerId,
    pub name: String,
    pub is_depth_test_enabled: bool,
    pub is_depth_write_enabled: bool,
    pub is_depth_offset_enabled: bool,
    pub depth_offset_factor: f32,
    pub depth_offset_units: f32,
    pub is_lighting_enabled: bool,
    pub is_fog_enabled: bool,
    pub rendering_scale: f64,
    pub priority: i32,
    pub is_frustum_culling_enabled: bool,
}

impl ZLayerSettings {
    pub fn new(layer_id: ZLayerId) -> Self {
        Self {
            layer_id,
            name: format!("Layer_{}", layer_id.id()),
            is_depth_test_enabled: true,
            is_depth_write_enabled: true,
            is_depth_offset_enabled: false,
            depth_offset_factor: 1.0,
            depth_offset_units: 1.0,
            is_lighting_enabled: true,
            is_fog_enabled: true,
            rendering_scale: 1.0,
            priority: layer_id.id(),
            is_frustum_culling_enabled: true,
        }
    }

    pub fn default_layer() -> Self { Self::new(ZLayerId::Default) }
    pub fn top_layer() -> Self {
        let mut s = Self::new(ZLayerId::Top);
        s.is_depth_test_enabled = false;
        s
    }

    pub fn overlay() -> Self {
        let mut s = Self::new(ZLayerId::TopOSD);
        s.is_depth_test_enabled = false;
        s.is_depth_write_enabled = false;
        s.is_lighting_enabled = false;
        s.is_fog_enabled = false;
        s
    }

    pub fn with_depth_offset(mut self, factor: f32, units: f32) -> Self {
        self.is_depth_offset_enabled = true;
        self.depth_offset_factor = factor;
        self.depth_offset_units = units;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self { self.name = name.to_owned(); self }
    pub fn with_priority(mut self, p: i32) -> Self { self.priority = p; self }
    pub fn enable_frustum_culling(mut self, v: bool) -> Self { self.is_frustum_culling_enabled = v; self }
}

impl Default for ZLayerSettings {
    fn default() -> Self { Self::default_layer() }
}

/// A single managed Z-layer with its contained presentation IDs.
#[derive(Clone, Debug)]
pub struct ZLayer {
    pub settings: ZLayerSettings,
    pub presentations: Vec<u32>,
}

impl ZLayer {
    pub fn new(settings: ZLayerSettings) -> Self {
        Self { settings, presentations: Vec::new() }
    }

    pub fn add_presentation(&mut self, id: u32) { self.presentations.push(id); }
    pub fn remove_presentation(&mut self, id: u32) { self.presentations.retain(|&p| p != id); }
    pub fn nb_presentations(&self) -> usize { self.presentations.len() }
    pub fn layer_id(&self) -> ZLayerId { self.settings.layer_id }
    pub fn is_empty(&self) -> bool { self.presentations.is_empty() }
}

// occt: Visual3d_Layer (deprecated overlay layer for 2D annotation drawing)
#[derive(Clone, Debug)]
pub struct Visual3dLayer {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub begin_frame_called: bool,
    pub draw_calls: Vec<LayerDrawCall>,
}

/// Minimal annotation draw call.
#[derive(Clone, Debug)]
pub enum LayerDrawCall {
    Line { x1: f64, y1: f64, x2: f64, y2: f64, color: [f32; 3] },
    Text { x: f64, y: f64, text: String, color: [f32; 3] },
    Rectangle { x: f64, y: f64, w: f64, h: f64, color: [f32; 3] },
}

impl Visual3dLayer {
    pub fn new(id: u32, width: u32, height: u32) -> Self {
        Self { id, width, height, begin_frame_called: false, draw_calls: Vec::new() }
    }

    pub fn begin(&mut self) {
        self.draw_calls.clear();
        self.begin_frame_called = true;
    }

    pub fn end(&mut self) { self.begin_frame_called = false; }

    pub fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: [f32; 3]) {
        self.draw_calls.push(LayerDrawCall::Line { x1, y1, x2, y2, color });
    }

    pub fn draw_text(&mut self, x: f64, y: f64, text: &str, color: [f32; 3]) {
        self.draw_calls.push(LayerDrawCall::Text { x, y, text: text.to_owned(), color });
    }

    pub fn draw_rect(&mut self, x: f64, y: f64, w: f64, h: f64, color: [f32; 3]) {
        self.draw_calls.push(LayerDrawCall::Rectangle { x, y, w, h, color });
    }

    pub fn nb_draw_calls(&self) -> usize { self.draw_calls.len() }
    pub fn aspect_ratio(&self) -> f64 { self.width as f64 / self.height.max(1) as f64 }
}

/// Manager that owns an ordered sequence of Z-layers.
#[derive(Clone, Debug, Default)]
pub struct ZLayerManager {
    pub layers: Vec<ZLayer>,
}

impl ZLayerManager {
    pub fn new() -> Self {
        let mut m = Self::default();
        m.layers.push(ZLayer::new(ZLayerSettings::default_layer()));
        m.layers.push(ZLayer::new(ZLayerSettings::top_layer()));
        m.layers.push(ZLayer::new(ZLayerSettings::overlay()));
        m
    }

    pub fn add_layer(&mut self, settings: ZLayerSettings) -> ZLayerId {
        let id = settings.layer_id;
        self.layers.push(ZLayer::new(settings));
        id
    }

    pub fn layer(&self, id: ZLayerId) -> Option<&ZLayer> {
        self.layers.iter().find(|l| l.layer_id() == id)
    }

    pub fn layer_mut(&mut self, id: ZLayerId) -> Option<&mut ZLayer> {
        self.layers.iter_mut().find(|l| l.layer_id() == id)
    }

    pub fn nb_layers(&self) -> usize { self.layers.len() }

    pub fn add_presentation_to_layer(&mut self, layer_id: ZLayerId, prs_id: u32) -> bool {
        if let Some(layer) = self.layer_mut(layer_id) {
            layer.add_presentation(prs_id);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z_layer_id() {
        assert_eq!(ZLayerId::Default.id(), 0);
        assert_eq!(ZLayerId::TopOSD.id(), 4);
        assert!(ZLayerId::TopOSD.is_overlay());
        assert!(!ZLayerId::Top.is_overlay());
        assert_eq!(ZLayerId::from_id(2), ZLayerId::Topmost);
    }

    #[test]
    fn z_layer_settings() {
        let s = ZLayerSettings::overlay();
        assert!(!s.is_depth_test_enabled);
        assert!(!s.is_lighting_enabled);
        let s2 = ZLayerSettings::default_layer().with_depth_offset(1.0, 2.0);
        assert!(s2.is_depth_offset_enabled);
    }

    #[test]
    fn z_layer_manager() {
        let mut mgr = ZLayerManager::new();
        assert_eq!(mgr.nb_layers(), 3);
        mgr.add_presentation_to_layer(ZLayerId::Default, 42);
        let layer = mgr.layer(ZLayerId::Default).unwrap();
        assert_eq!(layer.nb_presentations(), 1);
    }

    #[test]
    fn visual3d_layer_draw() {
        let mut layer = Visual3dLayer::new(0, 800, 600);
        layer.begin();
        layer.draw_line(0.0, 0.0, 1.0, 1.0, [1.0, 0.0, 0.0]);
        layer.draw_text(10.0, 10.0, "hello", [1.0, 1.0, 1.0]);
        assert_eq!(layer.nb_draw_calls(), 2);
        layer.end();
    }

    #[test]
    fn z_layer_presentations() {
        let mut layer = ZLayer::new(ZLayerSettings::default_layer());
        layer.add_presentation(1);
        layer.add_presentation(2);
        assert_eq!(layer.nb_presentations(), 2);
        layer.remove_presentation(1);
        assert_eq!(layer.nb_presentations(), 1);
    }
}

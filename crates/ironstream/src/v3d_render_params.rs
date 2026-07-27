// FILE: v3d_render_params.rs
// occt-ref: Graphic3d_RenderingParams, Graphic3d_ToneMappingMethod
//       Graphic3d_StereoMode, Graphic3d_AntiAliasingMode

/// Tone-mapping method for HDR rendering.
// occt-ref: Graphic3d_ToneMappingMethod
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToneMappingMethod {
    #[default]
    Disabled,
    Filmic,
}

/// Stereo rendering mode.
/// occt: Graphic3d_StereoMode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StereoMode {
    #[default]
    QuadBuffer,
    Anaglyph,
    RowInterlaced,
    ColumnInterlaced,
    ChessBoard,
    SideBySide,
    OverUnder,
    SoftPageFlip,
}

/// Anti-aliasing method.
// occt-ref: Graphic3d_AntiAliasingMode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AntiAliasingMode {
    #[default]
    None,
    Fxaa,
    Msaa2x,
    Msaa4x,
    Msaa8x,
}

impl AntiAliasingMode {
    pub fn msaa_samples(&self) -> u32 {
        match self {
            Self::Msaa2x => 2,
            Self::Msaa4x => 4,
            Self::Msaa8x => 8,
            _ => 1,
        }
    }
}

/// Shadow quality settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ShadowQuality {
    #[default]
    Off,
    Low,
    Medium,
    High,
}

impl ShadowQuality {
    pub fn shadow_map_size(&self) -> u32 {
        match self {
            Self::Off => 0,
            Self::Low => 512,
            Self::Medium => 1024,
            Self::High => 2048,
        }
    }
}

/// Ambient occlusion parameters.
// occt-ref: Graphic3d_RenderingParams // ::SsaoParams
#[derive(Clone, Debug, PartialEq)]
pub struct SsaoParams {
    pub is_enabled: bool,
    pub radius: f32,
    pub bias: f32,
    pub quality: u32,
}

impl Default for SsaoParams {
    fn default() -> Self { Self { is_enabled: false, radius: 0.5, bias: 0.01, quality: 32 } }
}

impl SsaoParams {
    pub fn enable(&mut self) { self.is_enabled = true; }
    pub fn disable(&mut self) { self.is_enabled = false; }
    pub fn set_radius(&mut self, r: f32) { self.radius = r.max(0.0); }
}

/// Full rendering parameter set.
// occt-ref: Graphic3d_RenderingParams
#[derive(Clone, Debug)]
pub struct Graphic3dRenderingParams {
    pub method: u32,                 // 0 = rasterization, 1 = ray tracing
    pub anti_aliasing: AntiAliasingMode,
    pub stereo_mode: StereoMode,
    pub tone_mapping: ToneMappingMethod,
    pub shadow_quality: ShadowQuality,
    pub ssao: SsaoParams,
    pub ray_tracing_depth: u32,
    pub is_reflections_enabled: bool,
    pub is_global_illumination_enabled: bool,
    pub nb_ray_tracing_samples: u32,
    pub oit_depth_factor: f64,
    pub resolution: u32,
    pub render_resolution_scale: f32,
    pub cubemap_lod_bias: f32,
}

impl Default for Graphic3dRenderingParams {
    fn default() -> Self {
        Self {
            method: 0,
            anti_aliasing: AntiAliasingMode::None,
            stereo_mode: StereoMode::QuadBuffer,
            tone_mapping: ToneMappingMethod::Disabled,
            shadow_quality: ShadowQuality::Off,
            ssao: SsaoParams::default(),
            ray_tracing_depth: 3,
            is_reflections_enabled: false,
            is_global_illumination_enabled: false,
            nb_ray_tracing_samples: 1,
            oit_depth_factor: 0.0,
            resolution: 72,
            render_resolution_scale: 1.0,
            cubemap_lod_bias: 0.0,
        }
    }
}

impl Graphic3dRenderingParams {
    pub fn new() -> Self { Self::default() }

    pub fn enable_ray_tracing(&mut self, depth: u32) {
        self.method = 1;
        self.ray_tracing_depth = depth.clamp(1, 10);
    }

    pub fn enable_rasterization(&mut self) { self.method = 0; }
    pub fn is_ray_tracing(&self) -> bool { self.method == 1 }

    pub fn set_anti_aliasing(&mut self, mode: AntiAliasingMode) { self.anti_aliasing = mode; }
    pub fn set_stereo_mode(&mut self, mode: StereoMode) { self.stereo_mode = mode; }
    pub fn set_tone_mapping(&mut self, m: ToneMappingMethod) { self.tone_mapping = m; }
    pub fn set_shadow_quality(&mut self, q: ShadowQuality) { self.shadow_quality = q; }

    pub fn set_nb_samples(&mut self, n: u32) { self.nb_ray_tracing_samples = n.max(1); }
    pub fn set_resolution_scale(&mut self, s: f32) { self.render_resolution_scale = s.max(0.1); }

    pub fn shadow_map_size(&self) -> u32 { self.shadow_quality.shadow_map_size() }
    pub fn msaa_samples(&self) -> u32 { self.anti_aliasing.msaa_samples() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_params() {
        let p = Graphic3dRenderingParams::new();
        assert!(!p.is_ray_tracing());
        assert_eq!(p.shadow_map_size(), 0);
        assert_eq!(p.msaa_samples(), 1);
    }

    #[test]
    fn enable_ray_tracing() {
        let mut p = Graphic3dRenderingParams::new();
        p.enable_ray_tracing(5);
        assert!(p.is_ray_tracing());
        assert_eq!(p.ray_tracing_depth, 5);
        p.enable_rasterization();
        assert!(!p.is_ray_tracing());
    }

    #[test]
    fn anti_aliasing_msaa() {
        let mut p = Graphic3dRenderingParams::new();
        p.set_anti_aliasing(AntiAliasingMode::Msaa4x);
        assert_eq!(p.msaa_samples(), 4);
    }

    #[test]
    fn shadow_quality_map_size() {
        let mut p = Graphic3dRenderingParams::new();
        p.set_shadow_quality(ShadowQuality::High);
        assert_eq!(p.shadow_map_size(), 2048);
    }

    #[test]
    fn ssao_params() {
        let mut p = Graphic3dRenderingParams::new();
        p.ssao.enable();
        p.ssao.set_radius(0.8);
        assert!(p.ssao.is_enabled);
        assert!((p.ssao.radius - 0.8).abs() < 1e-6);
    }
}

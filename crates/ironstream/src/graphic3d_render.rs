// FILE: graphic3d_render.rs
// occt: Graphic3d_RenderingParams, Graphic3d_ToneMappingMethod,
//       Graphic3d_StereoMode, Graphic3d_RenderTransparentMethod,
//       Graphic3d_BSDF, Graphic3d_PBRMaterial

/// Tone mapping method for HDR rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToneMappingMethod {
    Disabled,
    Filmic,
}

/// Stereo rendering mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StereoMode {
    QuadBuffer,
    Anaglyph,
    RowInterlaced,
    ColumnInterlaced,
    ChessBoard,
    SideBySide,
    OverUnder,
    SoftPageFlip,
}

/// OIT (order-independent transparency) method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransparencyMethod {
    AlphaBlending,
    WeightedBlended,
    DepthPeeling,
}

/// Anti-aliasing method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AntiAliasingMethod {
    None,
    Fxaa,
    Msaa2,
    Msaa4,
    Msaa8,
    Ssaa,
}

impl AntiAliasingMethod {
    pub fn sample_count(&self) -> u32 {
        match self {
            AntiAliasingMethod::None => 1,
            AntiAliasingMethod::Fxaa => 1,
            AntiAliasingMethod::Msaa2 => 2,
            AntiAliasingMethod::Msaa4 => 4,
            AntiAliasingMethod::Msaa8 => 8,
            AntiAliasingMethod::Ssaa => 4,
        }
    }

    pub fn is_hardware(&self) -> bool {
        matches!(self, AntiAliasingMethod::Msaa2 | AntiAliasingMethod::Msaa4 | AntiAliasingMethod::Msaa8)
    }
}

/// Shadow rendering quality.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowQuality {
    Off,
    Low,
    Medium,
    High,
}

// occt: Graphic3d_RenderingParams
#[derive(Clone, Debug)]
pub struct RenderingParams {
    pub method: RenderMethod,
    pub transparency: TransparencyMethod,
    pub aa: AntiAliasingMethod,
    pub tone_mapping: ToneMappingMethod,
    pub stereo_mode: StereoMode,
    pub shadow_quality: ShadowQuality,
    pub ray_tracing_depth: u32,
    pub is_shadows_enabled: bool,
    pub is_reflections_enabled: bool,
    pub is_antialias_enabled: bool,
    pub is_global_illumination: bool,
    pub nb_ray_tracing_tiles: u32,
    pub image_width: u32,
    pub image_height: u32,
    pub exposure: f32,
    pub white_point: f32,
    pub resolution: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMethod {
    Rasterization,
    RayTracing,
}

impl RenderingParams {
    pub fn default_rasterization() -> Self {
        Self {
            method: RenderMethod::Rasterization,
            transparency: TransparencyMethod::AlphaBlending,
            aa: AntiAliasingMethod::Fxaa,
            tone_mapping: ToneMappingMethod::Disabled,
            stereo_mode: StereoMode::QuadBuffer,
            shadow_quality: ShadowQuality::Off,
            ray_tracing_depth: 3,
            is_shadows_enabled: false,
            is_reflections_enabled: false,
            is_antialias_enabled: true,
            is_global_illumination: false,
            nb_ray_tracing_tiles: 16,
            image_width: 800,
            image_height: 600,
            exposure: 0.0,
            white_point: 1.0,
            resolution: 72,
        }
    }

    pub fn default_ray_tracing() -> Self {
        let mut p = Self::default_rasterization();
        p.method = RenderMethod::RayTracing;
        p.is_shadows_enabled = true;
        p.is_reflections_enabled = true;
        p.is_global_illumination = true;
        p.ray_tracing_depth = 5;
        p
    }

    pub fn is_ray_tracing(&self) -> bool { self.method == RenderMethod::RayTracing }
    pub fn aspect_ratio(&self) -> f32 { self.image_width as f32 / self.image_height.max(1) as f32 }
    pub fn nb_pixels(&self) -> u64 { self.image_width as u64 * self.image_height as u64 }
}

impl Default for RenderingParams {
    fn default() -> Self { Self::default_rasterization() }
}

// occt: Graphic3d_BSDF (Bidirectional Scattering Distribution Function)
#[derive(Clone, Debug)]
pub struct Bsdf {
    pub kd: [f32; 4],     // diffuse reflection (with fresnel)
    pub kr: [f32; 4],     // specular reflection
    pub kt: [f32; 4],     // specular transmission
    pub le: [f32; 4],     // emittance
    pub fresnelCoat: [f32; 4],
    pub fresnelBase: [f32; 4],
    pub absorption_color: [f32; 3],
    pub absorption_coeff: f32,
}

impl Bsdf {
    pub fn diffuse(color: [f32; 3]) -> Self {
        Self {
            kd: [color[0], color[1], color[2], 1.0],
            kr: [0.0; 4], kt: [0.0; 4], le: [0.0; 4],
            fresnelCoat: [0.0; 4], fresnelBase: [0.0; 4],
            absorption_color: [1.0; 3], absorption_coeff: 0.0,
        }
    }

    pub fn mirror() -> Self {
        let mut b = Self::diffuse([0.0, 0.0, 0.0]);
        b.kr = [1.0, 1.0, 1.0, 1.0];
        b
    }

    pub fn glass(ior: f32) -> Self {
        let mut b = Self::diffuse([1.0, 1.0, 1.0]);
        b.kt = [1.0, 1.0, 1.0, 1.0];
        b.fresnelBase = [ior, 0.0, 0.0, 0.0];
        b
    }

    pub fn is_emissive(&self) -> bool {
        self.le[0] > 0.0 || self.le[1] > 0.0 || self.le[2] > 0.0
    }

    pub fn is_transparent(&self) -> bool {
        self.kt[0] > 0.0 || self.kt[1] > 0.0 || self.kt[2] > 0.0
    }
}

// occt: Graphic3d_PBRMaterial (physically-based rendering)
#[derive(Clone, Debug)]
pub struct PbrMaterial {
    pub base_color: [f32; 4],   // RGBA, alpha = opacity
    pub metallic: f32,
    pub roughness: f32,
    pub emission: [f32; 3],
    pub ior: f32,               // index of refraction
    pub coat_strength: f32,     // clear coat
    pub coat_roughness: f32,
}

impl PbrMaterial {
    pub fn new() -> Self {
        Self {
            base_color: [0.8, 0.8, 0.8, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            emission: [0.0, 0.0, 0.0],
            ior: 1.5,
            coat_strength: 0.0,
            coat_roughness: 0.0,
        }
    }

    pub fn metallic(color: [f32; 3]) -> Self {
        let mut m = Self::new();
        m.base_color = [color[0], color[1], color[2], 1.0];
        m.metallic = 1.0;
        m.roughness = 0.1;
        m
    }

    pub fn plastic(color: [f32; 3]) -> Self {
        let mut m = Self::new();
        m.base_color = [color[0], color[1], color[2], 1.0];
        m.metallic = 0.0;
        m.roughness = 0.7;
        m
    }

    pub fn glass() -> Self {
        let mut m = Self::new();
        m.base_color = [1.0, 1.0, 1.0, 0.05];
        m.roughness = 0.0;
        m.ior = 1.52;
        m
    }

    pub fn is_opaque(&self) -> bool { self.base_color[3] >= 1.0 - 1e-6 }
    pub fn is_emissive(&self) -> bool { self.emission.iter().any(|&e| e > 0.0) }
    pub fn has_clear_coat(&self) -> bool { self.coat_strength > 1e-6 }

    pub fn with_metallic(mut self, v: f32) -> Self { self.metallic = v.clamp(0.0, 1.0); self }
    pub fn with_roughness(mut self, v: f32) -> Self { self.roughness = v.clamp(0.0, 1.0); self }
    pub fn with_emission(mut self, e: [f32; 3]) -> Self { self.emission = e; self }
    pub fn with_ior(mut self, ior: f32) -> Self { self.ior = ior; self }
}

impl Default for PbrMaterial {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rendering_params_default() {
        let p = RenderingParams::default();
        assert!(!p.is_ray_tracing());
        assert!(p.is_antialias_enabled);
        assert!((p.aspect_ratio() - 800.0/600.0).abs() < 1e-5);
    }

    #[test]
    fn rendering_params_ray_tracing() {
        let p = RenderingParams::default_ray_tracing();
        assert!(p.is_ray_tracing());
        assert!(p.is_shadows_enabled);
        assert_eq!(p.ray_tracing_depth, 5);
    }

    #[test]
    fn bsdf_types() {
        let diff = Bsdf::diffuse([1.0, 0.0, 0.0]);
        assert!(!diff.is_emissive());
        assert!(!diff.is_transparent());
        let glass = Bsdf::glass(1.5);
        assert!(glass.is_transparent());
    }

    #[test]
    fn pbr_material() {
        let m = PbrMaterial::metallic([0.7, 0.7, 0.7]);
        assert!((m.metallic - 1.0).abs() < 1e-6);
        assert!(m.is_opaque());
        assert!(!m.is_emissive());
        let g = PbrMaterial::glass();
        assert!(!g.is_opaque());
    }

    #[test]
    fn aa_method() {
        assert_eq!(AntiAliasingMethod::Msaa4.sample_count(), 4);
        assert!(AntiAliasingMethod::Msaa4.is_hardware());
        assert!(!AntiAliasingMethod::Fxaa.is_hardware());
    }
}

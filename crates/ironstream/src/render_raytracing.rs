// FILE: render_raytracing.rs
// occt: Graphic3d_RaytracingAspect, Graphic3d_RenderingParams (raytracing section),
//       Graphic3d_PathTracingParameters

/// Ray-tracing algorithm type.
/// occt: Graphic3d_TypeOfShadingModel (RAYTRACING variant)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RaytracingAlgorithm {
    WhittedStyle,   // classical recursive raytracing
    PathTracing,    // Monte Carlo path tracing
    HybridModes,   // rasterize + raytrace selective
}

impl Default for RaytracingAlgorithm {
    fn default() -> Self { Self::WhittedStyle }
}

/// Tone mapping method for HDR output.
/// occt: Graphic3d_ToneMappingMethod
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToneMappingMethod {
    Disabled,
    Filmic,
    Reinhard,
    Uncharted2,
    Aces,
}

impl Default for ToneMappingMethod {
    fn default() -> Self { Self::Disabled }
}

/// Raytracing rendering parameters.
/// occt: Graphic3d_RaytracingAspect / path-tracing portion of Graphic3d_RenderingParams
#[derive(Clone, Debug)]
pub struct RaytracingParams {
    pub algorithm: RaytracingAlgorithm,
    pub max_ray_depth: u32,
    pub nb_aa_samples: u32,
    pub is_global_illumination: bool,
    pub is_ambient_occlusion: bool,
    pub ao_radius: f64,
    pub is_adaptive_sampling: bool,
    pub adaptive_variance_threshold: f64,
    pub is_shadow_enabled: bool,
    pub is_reflection_enabled: bool,
    pub is_refraction_enabled: bool,
    pub nb_tiles_x: u32,
    pub nb_tiles_y: u32,
    pub radiance_clamp_value: f64,
}

impl Default for RaytracingParams {
    fn default() -> Self {
        Self {
            algorithm: RaytracingAlgorithm::WhittedStyle,
            max_ray_depth: 5,
            nb_aa_samples: 1,
            is_global_illumination: false,
            is_ambient_occlusion: false,
            ao_radius: 0.5,
            is_adaptive_sampling: false,
            adaptive_variance_threshold: 0.001,
            is_shadow_enabled: true,
            is_reflection_enabled: true,
            is_refraction_enabled: false,
            nb_tiles_x: 16,
            nb_tiles_y: 16,
            radiance_clamp_value: 30.0,
        }
    }
}

impl RaytracingParams {
    pub fn new() -> Self { Self::default() }

    pub fn set_max_depth(&mut self, d: u32) { self.max_ray_depth = d.clamp(1, 50); }
    pub fn set_nb_samples(&mut self, n: u32) { self.nb_aa_samples = n.max(1); }
    pub fn set_global_illumination(&mut self, v: bool) {
        self.is_global_illumination = v;
        if v { self.algorithm = RaytracingAlgorithm::PathTracing; }
    }
    pub fn set_shadow_enabled(&mut self, v: bool) { self.is_shadow_enabled = v; }
    pub fn set_ao_enabled(&mut self, v: bool) { self.is_ambient_occlusion = v; }
    pub fn set_ao_radius(&mut self, r: f64) { self.ao_radius = r.max(0.0); }
    pub fn set_adaptive_sampling(&mut self, v: bool) { self.is_adaptive_sampling = v; }
    pub fn set_radiance_clamp(&mut self, v: f64) { self.radiance_clamp_value = v.max(0.0); }

    pub fn algorithm(&self) -> RaytracingAlgorithm { self.algorithm }
    pub fn max_ray_depth(&self) -> u32 { self.max_ray_depth }
    pub fn nb_aa_samples(&self) -> u32 { self.nb_aa_samples }
    pub fn is_path_tracing(&self) -> bool { self.algorithm == RaytracingAlgorithm::PathTracing }
}

/// HDR tone-mapping parameters.
/// occt: Graphic3d_ToneMappingMethod + parameters
#[derive(Clone, Debug)]
pub struct ToneMappingParams {
    pub method: ToneMappingMethod,
    pub white_point: f64,       // Reinhard white point
    pub exposure: f64,          // scene exposure value (EV)
    pub gamma: f64,             // output gamma (2.2 = sRGB)
    pub saturation: f64,        // color saturation multiplier
}

impl Default for ToneMappingParams {
    fn default() -> Self {
        Self { method: ToneMappingMethod::Disabled, white_point: 11.2, exposure: 1.0, gamma: 2.2, saturation: 1.0 }
    }
}

impl ToneMappingParams {
    pub fn new(method: ToneMappingMethod) -> Self { Self { method, ..Self::default() } }
    pub fn set_exposure(&mut self, e: f64) { self.exposure = e; }
    pub fn set_gamma(&mut self, g: f64) { self.gamma = g.max(0.1); }
    pub fn method(&self) -> ToneMappingMethod { self.method }
    pub fn is_enabled(&self) -> bool { self.method != ToneMappingMethod::Disabled }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raytracing_defaults() {
        let p = RaytracingParams::new();
        assert_eq!(p.algorithm(), RaytracingAlgorithm::WhittedStyle);
        assert_eq!(p.max_ray_depth(), 5);
        assert!(!p.is_path_tracing());
    }

    #[test]
    fn raytracing_enable_gi() {
        let mut p = RaytracingParams::new();
        p.set_global_illumination(true);
        assert!(p.is_path_tracing());
        assert!(p.is_global_illumination);
    }

    #[test]
    fn raytracing_depth_clamped() {
        let mut p = RaytracingParams::new();
        p.set_max_depth(100);
        assert_eq!(p.max_ray_depth(), 50);
        p.set_max_depth(0);
        assert_eq!(p.max_ray_depth(), 1);
    }

    #[test]
    fn tone_mapping_enabled() {
        let tm = ToneMappingParams::new(ToneMappingMethod::Filmic);
        assert!(tm.is_enabled());
        assert_eq!(tm.method(), ToneMappingMethod::Filmic);
    }

    #[test]
    fn tone_mapping_disabled() {
        let tm = ToneMappingParams::default();
        assert!(!tm.is_enabled());
    }
}

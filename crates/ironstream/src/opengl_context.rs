// FILE: opengl_context.rs
// occt: OpenGl_Context, OpenGl_Caps
// occt-ref: OpenGl_Info, OpenGl_FrameBuffer

/// OpenGL version information.
// occt-ref: OpenGl_Info
#[derive(Clone, Debug, Default)]
pub struct OpenGlInfo {
    pub vendor: String,
    pub renderer: String,
    pub version: String,
    pub glsl_version: String,
    pub max_texture_size: u32,
    pub max_combined_texture_units: u32,
    pub max_msaa_samples: u32,
}

impl OpenGlInfo {
    pub fn new(vendor: &str, renderer: &str, version: &str) -> Self {
        Self {
            vendor: vendor.to_string(),
            renderer: renderer.to_string(),
            version: version.to_string(),
            glsl_version: String::new(),
            max_texture_size: 4096,
            max_combined_texture_units: 16,
            max_msaa_samples: 8,
        }
    }

    pub fn vendor(&self) -> &str { &self.vendor }
    pub fn renderer(&self) -> &str { &self.renderer }
    pub fn version(&self) -> &str { &self.version }
}

/// OpenGL capability flags.
/// occt: OpenGl_Caps
#[derive(Clone, Debug)]
pub struct OpenGlCaps {
    pub core_profile: bool,
    pub srgb_framebuffer: bool,
    pub vbo_enabled: bool,
    pub fbo_enabled: bool,
    pub msaa_enabled: bool,
    pub geometry_shader: bool,
    pub compute_shader: bool,
    pub anisotropic_filter: bool,
    pub max_anisotropy: f32,
    pub software_rendering: bool,
    pub raytracing_supported: bool,
}

impl Default for OpenGlCaps {
    fn default() -> Self {
        Self {
            core_profile: true,
            srgb_framebuffer: false,
            vbo_enabled: true,
            fbo_enabled: true,
            msaa_enabled: true,
            geometry_shader: true,
            compute_shader: false,
            anisotropic_filter: true,
            max_anisotropy: 16.0,
            software_rendering: false,
            raytracing_supported: false,
        }
    }
}

impl OpenGlCaps {
    pub fn new() -> Self { Self::default() }

    pub fn set_core_profile(&mut self, v: bool) { self.core_profile = v; }
    pub fn set_srgb(&mut self, v: bool) { self.srgb_framebuffer = v; }
    pub fn set_raytracing(&mut self, v: bool) { self.raytracing_supported = v; }
}

/// An OpenGL framebuffer object (FBO).
// occt-ref: OpenGl_FrameBuffer
#[derive(Clone, Debug)]
pub struct OpenGlFrameBuffer {
    pub fbo_id: u32,
    pub width: u32,
    pub height: u32,
    pub color_format: u32,  // stub: 0=RGBA8, 1=RGBA16F, 2=RGBA32F
    pub depth_format: u32,  // stub: 0=D24, 1=D32F
    pub nb_samples: u32,
    pub is_valid: bool,
}

impl OpenGlFrameBuffer {
    pub fn new(fbo_id: u32, width: u32, height: u32) -> Self {
        Self {
            fbo_id,
            width, height,
            color_format: 0,
            depth_format: 0,
            nb_samples: 1,
            is_valid: width > 0 && height > 0,
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.width = w; self.height = h;
        self.is_valid = w > 0 && h > 0;
    }

    pub fn set_nb_samples(&mut self, n: u32) { self.nb_samples = n.max(1); }
    pub fn is_valid(&self) -> bool { self.is_valid }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn aspect_ratio(&self) -> f64 {
        if self.height == 0 { 1.0 } else { self.width as f64 / self.height as f64 }
    }
}

/// An OpenGL rendering context.
/// occt: OpenGl_Context
#[derive(Clone, Debug)]
pub struct OpenGlContext {
    pub context_id: u32,
    pub info: OpenGlInfo,
    pub caps: OpenGlCaps,
    pub is_initialized: bool,
    pub ref_count: u32,
    pub active_fbo: Option<u32>,
    pub default_fbo_width: u32,
    pub default_fbo_height: u32,
}

impl OpenGlContext {
    pub fn new(context_id: u32) -> Self {
        Self {
            context_id,
            info: OpenGlInfo::default(),
            caps: OpenGlCaps::default(),
            is_initialized: false,
            ref_count: 0,
            active_fbo: None,
            default_fbo_width: 0,
            default_fbo_height: 0,
        }
    }

    pub fn init(&mut self, vendor: &str, renderer: &str, version: &str) {
        self.info = OpenGlInfo::new(vendor, renderer, version);
        self.is_initialized = true;
    }

    pub fn inc_ref(&mut self) { self.ref_count += 1; }
    pub fn dec_ref(&mut self) { if self.ref_count > 0 { self.ref_count -= 1; } }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.default_fbo_width = width;
        self.default_fbo_height = height;
    }

    pub fn bind_fbo(&mut self, fbo_id: u32) { self.active_fbo = Some(fbo_id); }
    pub fn unbind_fbo(&mut self) { self.active_fbo = None; }

    pub fn is_initialized(&self) -> bool { self.is_initialized }
    pub fn ref_count(&self) -> u32 { self.ref_count }
    pub fn active_fbo(&self) -> Option<u32> { self.active_fbo }

    pub fn supports_raytracing(&self) -> bool { self.caps.raytracing_supported }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_init() {
        let mut ctx = OpenGlContext::new(1);
        assert!(!ctx.is_initialized());
        ctx.init("NVIDIA", "GeForce RTX 3080", "4.6");
        assert!(ctx.is_initialized());
        assert_eq!(ctx.info.vendor(), "NVIDIA");
    }

    #[test]
    fn context_ref_count() {
        let mut ctx = OpenGlContext::new(1);
        ctx.inc_ref();
        ctx.inc_ref();
        assert_eq!(ctx.ref_count(), 2);
        ctx.dec_ref();
        assert_eq!(ctx.ref_count(), 1);
    }

    #[test]
    fn fbo_creation() {
        let fbo = OpenGlFrameBuffer::new(1, 1920, 1080);
        assert!(fbo.is_valid());
        assert!((fbo.aspect_ratio() - 1920.0/1080.0).abs() < 1e-6);
    }

    #[test]
    fn fbo_resize() {
        let mut fbo = OpenGlFrameBuffer::new(1, 800, 600);
        fbo.resize(0, 0);
        assert!(!fbo.is_valid());
        fbo.resize(1280, 720);
        assert!(fbo.is_valid());
    }

    #[test]
    fn caps_defaults() {
        let caps = OpenGlCaps::default();
        assert!(caps.vbo_enabled);
        assert!(!caps.compute_shader);
        assert!(!caps.software_rendering);
    }
}

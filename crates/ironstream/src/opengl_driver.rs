// FILE: opengl_driver.rs
// occt: OpenGl_GraphicDriver, OpenGl_Context, OpenGl_FrameBuffer,
//       OpenGl_Texture, OpenGl_VertexBuffer

/// OpenGL version supported by the driver.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct OpenGlVersion {
    pub major: u8,
    pub minor: u8,
}

impl OpenGlVersion {
    pub fn new(major: u8, minor: u8) -> Self { Self { major, minor } }
    pub fn gl_33() -> Self { Self::new(3, 3) }
    pub fn gl_45() -> Self { Self::new(4, 5) }
    pub fn supports(&self, required: OpenGlVersion) -> bool { *self >= required }
}

impl std::fmt::Display for OpenGlVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Capabilities of the current OpenGL context.
#[derive(Clone, Debug)]
pub struct OpenGlCaps {
    pub version: OpenGlVersion,
    pub vendor: String,
    pub renderer: String,
    pub supports_vbo: bool,
    pub supports_fbo: bool,
    pub supports_msaa: bool,
    pub supports_compute: bool,
    pub max_texture_size: u32,
    pub max_anisotropy: f32,
    pub nb_color_attachments: u32,
}

impl Default for OpenGlCaps {
    fn default() -> Self {
        Self {
            version: OpenGlVersion::gl_33(),
            vendor: "Unknown".into(),
            renderer: "Unknown".into(),
            supports_vbo: true,
            supports_fbo: true,
            supports_msaa: true,
            supports_compute: false,
            max_texture_size: 4096,
            max_anisotropy: 1.0,
            nb_color_attachments: 8,
        }
    }
}

/// Pixel format for framebuffers and textures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum OpenGlPixelFormat {
    #[default]
    RGBA8,
    RGB8,
    Depth24Stencil8,
    Depth32F,
    RGBA16F,
    RGBA32F,
    R8,
    R32F,
}

impl OpenGlPixelFormat {
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            Self::RGBA8 | Self::Depth24Stencil8 | Self::R32F => 4,
            Self::RGB8  => 3,
            Self::Depth32F | Self::RGBA16F => 8,
            Self::RGBA32F  => 16,
            Self::R8       => 1,
        }
    }

    pub fn has_alpha(&self) -> bool { matches!(self, Self::RGBA8 | Self::RGBA16F | Self::RGBA32F) }
    pub fn is_depth(&self) -> bool { matches!(self, Self::Depth24Stencil8 | Self::Depth32F) }
}

/// Framebuffer object stub.
/// occt: OpenGl_FrameBuffer
#[derive(Clone, Debug)]
pub struct OpenGlFrameBuffer {
    pub fbo_id: u32,
    pub width: u32,
    pub height: u32,
    pub color_format: OpenGlPixelFormat,
    pub depth_format: OpenGlPixelFormat,
    pub nb_samples: u32,
    pub is_valid: bool,
}

impl OpenGlFrameBuffer {
    pub fn new(width: u32, height: u32, color_format: OpenGlPixelFormat) -> Self {
        Self {
            fbo_id: width.wrapping_mul(height) % 1000 + 1,
            width,
            height,
            color_format,
            depth_format: OpenGlPixelFormat::Depth24Stencil8,
            nb_samples: 1,
            is_valid: false,
        }
    }

    pub fn init(&mut self) { self.is_valid = true; }
    pub fn release(&mut self) { self.is_valid = false; }
    pub fn is_valid(&self) -> bool { self.is_valid }
    pub fn size_x(&self) -> u32 { self.width }
    pub fn size_y(&self) -> u32 { self.height }

    pub fn pixel_count(&self) -> u64 { self.width as u64 * self.height as u64 }

    pub fn memory_usage_bytes(&self) -> u64 {
        let color_bpp = self.color_format.bytes_per_pixel() as u64;
        let depth_bpp = self.depth_format.bytes_per_pixel() as u64;
        self.pixel_count() * (color_bpp + depth_bpp) * self.nb_samples as u64
    }
}

/// Texture object stub.
/// occt: OpenGl_Texture
#[derive(Clone, Debug)]
pub struct OpenGlTexture {
    pub tex_id: u32,
    pub width: u32,
    pub height: u32,
    pub format: OpenGlPixelFormat,
    pub has_mipmaps: bool,
    pub is_valid: bool,
}

impl OpenGlTexture {
    pub fn new(tex_id: u32, width: u32, height: u32, format: OpenGlPixelFormat) -> Self {
        Self { tex_id, width, height, format, has_mipmaps: false, is_valid: false }
    }

    pub fn init(&mut self, with_mipmaps: bool) {
        self.has_mipmaps = with_mipmaps;
        self.is_valid = true;
    }

    pub fn release(&mut self) { self.is_valid = false; }
    pub fn is_valid(&self) -> bool { self.is_valid }
}

/// Stub graphic driver.
/// occt: OpenGl_GraphicDriver
#[derive(Clone, Debug, Default)]
pub struct OpenGlGraphicDriver {
    pub caps: OpenGlCaps,
    pub is_initialized: bool,
    pub frame_buffers: Vec<OpenGlFrameBuffer>,
    pub textures: Vec<OpenGlTexture>,
    pub next_tex_id: u32,
}

impl OpenGlGraphicDriver {
    pub fn new() -> Self { Self { next_tex_id: 1, ..Self::default() } }

    pub fn initialize(&mut self, vendor: impl Into<String>, renderer: impl Into<String>) {
        self.caps.vendor = vendor.into();
        self.caps.renderer = renderer.into();
        self.is_initialized = true;
    }

    pub fn create_fbo(&mut self, w: u32, h: u32, fmt: OpenGlPixelFormat) -> usize {
        let mut fbo = OpenGlFrameBuffer::new(w, h, fmt);
        fbo.init();
        let idx = self.frame_buffers.len();
        self.frame_buffers.push(fbo);
        idx
    }

    pub fn create_texture(&mut self, w: u32, h: u32, fmt: OpenGlPixelFormat) -> u32 {
        let id = self.next_tex_id;
        self.next_tex_id += 1;
        let mut tex = OpenGlTexture::new(id, w, h, fmt);
        tex.init(false);
        self.textures.push(tex);
        id
    }

    pub fn nb_fbos(&self) -> usize { self.frame_buffers.len() }
    pub fn nb_textures(&self) -> usize { self.textures.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_ordering() {
        assert!(OpenGlVersion::gl_45() > OpenGlVersion::gl_33());
        assert!(OpenGlVersion::gl_33().supports(OpenGlVersion::gl_33()));
        assert!(!OpenGlVersion::gl_33().supports(OpenGlVersion::gl_45()));
    }

    #[test]
    fn pixel_format_bytes() {
        assert_eq!(OpenGlPixelFormat::RGBA8.bytes_per_pixel(), 4);
        assert_eq!(OpenGlPixelFormat::R8.bytes_per_pixel(), 1);
        assert!(OpenGlPixelFormat::RGBA8.has_alpha());
        assert!(!OpenGlPixelFormat::RGB8.has_alpha());
        assert!(OpenGlPixelFormat::Depth32F.is_depth());
    }

    #[test]
    fn fbo_init_release() {
        let mut fbo = OpenGlFrameBuffer::new(1920, 1080, OpenGlPixelFormat::RGBA8);
        assert!(!fbo.is_valid());
        fbo.init();
        assert!(fbo.is_valid());
        assert_eq!(fbo.pixel_count(), 1920 * 1080);
        fbo.release();
        assert!(!fbo.is_valid());
    }

    #[test]
    fn driver_create_fbo() {
        let mut d = OpenGlGraphicDriver::new();
        d.initialize("NVIDIA", "GeForce RTX 3090");
        assert!(d.is_initialized);
        let idx = d.create_fbo(800, 600, OpenGlPixelFormat::RGBA8);
        assert_eq!(d.nb_fbos(), 1);
        assert!(d.frame_buffers[idx].is_valid());
    }

    #[test]
    fn driver_create_texture() {
        let mut d = OpenGlGraphicDriver::new();
        let id1 = d.create_texture(512, 512, OpenGlPixelFormat::RGBA8);
        let id2 = d.create_texture(256, 256, OpenGlPixelFormat::R8);
        assert_ne!(id1, id2);
        assert_eq!(d.nb_textures(), 2);
    }

    #[test]
    fn fbo_memory_estimate() {
        let mut fbo = OpenGlFrameBuffer::new(100, 100, OpenGlPixelFormat::RGBA8);
        fbo.init();
        // color=4 bytes + depth=4 bytes per pixel, 1 sample → 100*100*8 = 80000
        assert_eq!(fbo.memory_usage_bytes(), 80_000);
    }
}

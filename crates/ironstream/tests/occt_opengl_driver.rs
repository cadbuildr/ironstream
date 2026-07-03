use ironstream::opengl_driver::*;

#[test]
fn opengl_version_ordering_and_supports() {
    let v33 = OpenGlVersion::gl_33();
    let v45 = OpenGlVersion::gl_45();
    assert!(v45 > v33);
    assert!(v33.supports(v33));
    assert!(!v33.supports(v45));
    assert!(v45.supports(v33));
    assert!(v45.supports(v45));
}

#[test]
fn opengl_pixel_format_bytes_per_pixel() {
    assert_eq!(OpenGlPixelFormat::RGBA8.bytes_per_pixel(), 4);
    assert_eq!(OpenGlPixelFormat::RGB8.bytes_per_pixel(), 3);
    assert_eq!(OpenGlPixelFormat::Depth24Stencil8.bytes_per_pixel(), 4);
    assert_eq!(OpenGlPixelFormat::Depth32F.bytes_per_pixel(), 8);
    assert_eq!(OpenGlPixelFormat::RGBA16F.bytes_per_pixel(), 8);
    assert_eq!(OpenGlPixelFormat::RGBA32F.bytes_per_pixel(), 16);
    assert_eq!(OpenGlPixelFormat::R8.bytes_per_pixel(), 1);
    assert_eq!(OpenGlPixelFormat::R32F.bytes_per_pixel(), 4);
}

#[test]
fn opengl_pixel_format_has_alpha_is_depth() {
    assert!(OpenGlPixelFormat::RGBA8.has_alpha());
    assert!(OpenGlPixelFormat::RGBA16F.has_alpha());
    assert!(OpenGlPixelFormat::RGBA32F.has_alpha());
    assert!(!OpenGlPixelFormat::RGB8.has_alpha());
    assert!(!OpenGlPixelFormat::R8.has_alpha());

    assert!(OpenGlPixelFormat::Depth24Stencil8.is_depth());
    assert!(OpenGlPixelFormat::Depth32F.is_depth());
    assert!(!OpenGlPixelFormat::RGBA8.is_depth());
}

#[test]
fn opengl_fbo_init_release_memory() {
    let mut fbo = OpenGlFrameBuffer::new(1920, 1080, OpenGlPixelFormat::RGBA8);
    assert!(!fbo.is_valid());
    fbo.init();
    assert!(fbo.is_valid());
    assert_eq!(fbo.pixel_count(), 1920u64 * 1080);
    // color=4 + depth=4 (Depth24Stencil8) per pixel, 1 sample
    let expected = 1920u64 * 1080 * (4 + 4) * 1;
    assert_eq!(fbo.memory_usage_bytes(), expected);
    fbo.release();
    assert!(!fbo.is_valid());
}

#[test]
fn opengl_driver_create_fbo_and_texture() {
    let mut d = OpenGlGraphicDriver::new();
    d.initialize("NVIDIA", "RTX 3090");
    assert!(d.is_initialized);

    let idx = d.create_fbo(800, 600, OpenGlPixelFormat::RGBA8);
    assert_eq!(d.nb_fbos(), 1);
    assert!(d.frame_buffers[idx].is_valid());

    let id1 = d.create_texture(512, 512, OpenGlPixelFormat::RGBA8);
    let id2 = d.create_texture(256, 256, OpenGlPixelFormat::R8);
    assert_ne!(id1, id2);
    assert_eq!(d.nb_textures(), 2);
}

#[test]
fn opengl_texture_init_release() {
    let mut tex = OpenGlTexture::new(1, 512, 512, OpenGlPixelFormat::RGBA8);
    assert!(!tex.is_valid());
    tex.init(true);
    assert!(tex.is_valid());
    assert!(tex.has_mipmaps);
    tex.release();
    assert!(!tex.is_valid());
}

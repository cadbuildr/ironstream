use ironstream::opengl_context::*;

#[test]
fn context_not_initialized_before_init() {
    let ctx = OpenGlContext::new(1);
    assert!(!ctx.is_initialized());
}

#[test]
fn context_is_initialized_after_init() {
    let mut ctx = OpenGlContext::new(1);
    ctx.init("NVIDIA", "GeForce RTX 3080", "4.6");
    assert!(ctx.is_initialized());
    assert_eq!(ctx.info.vendor(), "NVIDIA");
}

#[test]
fn context_ref_count_inc_dec() {
    let mut ctx = OpenGlContext::new(1);
    ctx.inc_ref();
    ctx.inc_ref();
    assert_eq!(ctx.ref_count(), 2);
    ctx.dec_ref();
    assert_eq!(ctx.ref_count(), 1);
}

#[test]
fn fbo_creation_valid_and_aspect_ratio() {
    let fbo = OpenGlFrameBuffer::new(1, 1920, 1080);
    assert!(fbo.is_valid());
    let expected = 1920.0 / 1080.0;
    assert!((fbo.aspect_ratio() - expected).abs() < 1e-6);
}

#[test]
fn fbo_resize_to_zero_not_valid() {
    let mut fbo = OpenGlFrameBuffer::new(1, 800, 600);
    fbo.resize(0, 0);
    assert!(!fbo.is_valid());
}

#[test]
fn caps_defaults() {
    let caps = OpenGlCaps::default();
    assert!(caps.vbo_enabled);
    assert!(caps.fbo_enabled);
    assert!(!caps.compute_shader);
    assert!(!caps.software_rendering);
    assert!(!caps.raytracing_supported);
}

use ironstream::image_texture::*;

#[test]
fn pixel_format_float_types() {
    assert_eq!(PixelFormat::Rgb32f.nb_channels(), 3);
    assert_eq!(PixelFormat::Rgb32f.bytes_per_channel(), 4);
    assert_eq!(PixelFormat::Rgb32f.bytes_per_pixel(), 12);
    assert!(!PixelFormat::Rgb32f.is_alpha());
}

#[test]
fn pixmap_data_row_ptr() {
    let mut pm = PixMapData::new(3, 3, PixelFormat::Rgba8);
    pm.set_pixel_rgba(0, 1, 10, 20, 30, 40);
    let row = pm.row_ptr(1);
    assert_eq!(row[0], 10);
    assert_eq!(row[1], 20);
}

#[test]
fn image_texture_from_pixmap_has_dimensions() {
    let pm = PixMapData::new(16, 8, PixelFormat::Rgb8);
    let tex = ImageTexture::from_pixmap(2, pm);
    assert_eq!(tex.width(), Some(16));
    assert_eq!(tex.height(), Some(8));
    assert_eq!(tex.format(), Some(PixelFormat::Rgb8));
    assert!(tex.has_data());
}

#[test]
fn image_texture_set_scale_offset() {
    let pm = PixMapData::new(4, 4, PixelFormat::Rgba8);
    let mut tex = ImageTexture::from_pixmap(3, pm);
    tex.set_scale(2.0, 3.0);
    tex.set_offset(0.1, 0.2);
    assert!((tex.u_scale - 2.0).abs() < 1e-10);
    assert!((tex.v_offset - 0.2).abs() < 1e-10);
}

#[test]
fn image_texture_sample_mirror_wrap() {
    let mut pm = PixMapData::new(2, 2, PixelFormat::Rgba8);
    pm.set_pixel_rgba(0, 0, 50, 100, 150, 200);
    let mut tex = ImageTexture::from_pixmap(4, pm);
    tex.set_wrap(TextureWrap::MirrorRepeat, TextureWrap::MirrorRepeat);
    // u=1.5 in mirror mode => 2.0 - 1.5 = 0.5 => maps to left half
    let _ = tex.sample_uv(1.5, 0.0);
}

#[test]
fn texture_filter_default_is_linear() {
    assert_eq!(TextureFilter::default(), TextureFilter::Linear);
    let pm = PixMapData::new(4, 4, PixelFormat::Rgba8);
    let mut tex = ImageTexture::from_pixmap(5, pm);
    tex.set_filter(TextureFilter::MipMap);
    assert_eq!(tex.filter, TextureFilter::MipMap);
}

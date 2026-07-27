use ironstream::image_proc::*;

#[test]
fn rgb_hex_roundtrip() {
    let c = ImageColorRgb::from_hex(0xFF8000);
    assert_eq!(c.r, 0xFF);
    assert_eq!(c.g, 0x80);
    assert_eq!(c.b, 0x00);
    assert_eq!(c.to_hex(), 0xFF8000);
}

#[test]
fn rgba_hex_roundtrip() {
    let c = ImageColorRgba::from_hex(0x12345678);
    assert_eq!(c.r, 0x12);
    assert_eq!(c.g, 0x34);
    assert_eq!(c.b, 0x56);
    assert_eq!(c.a, 0x78);
    assert!(!c.is_opaque());
    assert_eq!(c.to_hex(), 0x12345678);
}

#[test]
fn bgr_from_rgb() {
    let rgb = ImageColorRgb::new(10, 20, 30);
    let bgr = ImageColorBgr::from_rgb(rgb);
    assert_eq!(bgr.r, 10);
    assert_eq!(bgr.g, 20);
    assert_eq!(bgr.b, 30);
    assert_eq!(bgr.to_rgb(), rgb);
}

#[test]
fn supported_formats() {
    let mut fmt = ImageSupportedFormats::new();
    fmt.add("PNG");
    fmt.add("JPEG");
    assert!(fmt.supports("png"));
    assert!(fmt.supports("JPEG"));
    assert!(!fmt.supports("BMP"));
    assert_eq!(fmt.nb_formats(), 2);
}

#[test]
fn pixmap_data_set_get_rgba() {
    let mut pm = ImagePixMapData::new(4, 4, 4);
    let c = ImageColorRgba::opaque(255, 128, 0);
    assert!(pm.set_rgba(1, 2, c));
    let g = pm.get_rgba(1, 2).unwrap();
    assert_eq!(g, c);
}

#[test]
fn pixmap_data_out_of_bounds() {
    let pm = ImagePixMapData::new(2, 2, 4);
    assert!(pm.get_rgba(10, 10).is_none());
    assert_eq!(pm.size_bytes(), 2 * 2 * 4);
}

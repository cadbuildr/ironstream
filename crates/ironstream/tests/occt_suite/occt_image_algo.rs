use ironstream::image_algo::*;

#[test]
fn new_pixmap_size() {
    let p = ImagePixMap::new(100, 50, ImageFormat::RGBA8);
    assert!(!p.is_empty());
    assert_eq!(p.width, 100);
    assert_eq!(p.height, 50);
    assert_eq!(p.size_bytes(), 100 * 50 * 4);
}

#[test]
fn pixel_set_get_roundtrip() {
    let mut p = ImagePixMap::new(10, 10, ImageFormat::RGBA8);
    p.set_pixel_rgba(3, 4, [255, 128, 0, 200]);
    let px = p.get_pixel_rgba(3, 4).unwrap();
    assert_eq!(px[0], 255);
    assert_eq!(px[1], 128);
    assert_eq!(px[2], 0);
    assert_eq!(px[3], 200);
}

#[test]
fn pixel_out_of_bounds_returns_none() {
    let p = ImagePixMap::new(10, 10, ImageFormat::RGBA8);
    assert!(p.get_pixel_rgba(10, 0).is_none(), "x == width should be out of bounds");
    assert!(p.get_pixel_rgba(0, 10).is_none(), "y == height should be out of bounds");
    assert!(p.get_pixel_rgba(100, 100).is_none());
}

#[test]
fn image_format_bytes_per_pixel() {
    assert_eq!(ImageFormat::Gray8.bytes_per_pixel(), 1);
    assert_eq!(ImageFormat::Gray16.bytes_per_pixel(), 2);
    assert_eq!(ImageFormat::RGB8.bytes_per_pixel(), 3);
    assert_eq!(ImageFormat::RGBA8.bytes_per_pixel(), 4);
    assert_eq!(ImageFormat::RGB32F.bytes_per_pixel(), 12);
    assert_eq!(ImageFormat::RGBA32F.bytes_per_pixel(), 16);
}

#[test]
fn alien_pixmap_load_png_ok() {
    let mut p = ImageAlienPixMap::new();
    assert!(p.load("photo.png"), ".png should succeed");
    assert!(p.is_loaded());
    assert_eq!(p.width(), 1);
    assert_eq!(p.height(), 1);
}

#[test]
fn alien_pixmap_load_stl_fails() {
    let mut p = ImageAlienPixMap::new();
    assert!(!p.load("model.stl"), ".stl is not an image format");
    assert!(!p.is_loaded());
    assert_eq!(p.width(), 0);
}

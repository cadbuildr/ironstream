// FILE: tests/occt_image_pixel.rs
// Integration tests for image_pixel module
extern crate ironstream;

use ironstream::image_pixel::{ImageAlienPixMap, ImageDiff, ImagePixMap, PixelFormat};

#[test]
fn pixel_format_bytes_per_pixel_and_components() {
    assert_eq!(PixelFormat::RGBA.bytes_per_pixel(), 4);
    assert_eq!(PixelFormat::RGBA.nb_components(), 4);
    assert_eq!(PixelFormat::RGB.bytes_per_pixel(), 3);
    assert_eq!(PixelFormat::RGB.nb_components(), 3);
    assert_eq!(PixelFormat::Gray.bytes_per_pixel(), 1);
    assert_eq!(PixelFormat::Gray.nb_components(), 1);
    assert_eq!(PixelFormat::RGBf.bytes_per_pixel(), 12);
    assert_eq!(PixelFormat::RGBf.nb_components(), 3);
    assert_eq!(PixelFormat::RGBAf.bytes_per_pixel(), 16);
    assert_eq!(PixelFormat::RGBAf.nb_components(), 4);
    assert_eq!(PixelFormat::GrayF.bytes_per_pixel(), 4);
    assert_eq!(PixelFormat::GrayF.nb_components(), 1);
}

#[test]
fn pixel_format_is_float() {
    assert!(!PixelFormat::RGBA.is_float());
    assert!(!PixelFormat::RGB.is_float());
    assert!(!PixelFormat::Gray.is_float());
    assert!(PixelFormat::GrayF.is_float());
    assert!(PixelFormat::RGBf.is_float());
    assert!(PixelFormat::RGBAf.is_float());
}

#[test]
fn pixmap_new_size_and_row_bytes() {
    let p = ImagePixMap::new(8, 4, PixelFormat::RGBA);
    assert_eq!(p.width, 8);
    assert_eq!(p.height, 4);
    assert_eq!(p.size_bytes(), 8 * 4 * 4);
    assert_eq!(p.row_bytes(), 8 * 4);
}

#[test]
fn pixmap_set_get_pixel_rgba() {
    let mut p = ImagePixMap::new(4, 4, PixelFormat::RGBA);
    assert!(p.set_pixel_rgba(2, 3, 10, 20, 30, 200));
    let px = p.get_pixel_rgba(2, 3).unwrap();
    assert_eq!(px, [10, 20, 30, 200]);
    // Other pixels remain zero
    let other = p.get_pixel_rgba(0, 0).unwrap();
    assert_eq!(other, [0, 0, 0, 0]);
}

#[test]
fn pixmap_fill_clear_and_flip_vertical() {
    let mut p = ImagePixMap::new(2, 3, PixelFormat::RGBA);
    p.fill(100, 150, 200, 255);
    assert_eq!(p.get_pixel_rgba(0, 0).unwrap(), [100, 150, 200, 255]);
    assert_eq!(p.get_pixel_rgba(1, 2).unwrap(), [100, 150, 200, 255]);

    // Set distinct rows to verify flip
    p.fill(0, 0, 0, 0);
    p.set_pixel_rgba(0, 0, 1, 0, 0, 255);
    p.set_pixel_rgba(0, 2, 3, 0, 0, 255);
    p.flip_vertical();
    assert_eq!(p.get_pixel_rgba(0, 0).unwrap()[0], 3);
    assert_eq!(p.get_pixel_rgba(0, 2).unwrap()[0], 1);

    p.clear();
    assert_eq!(p.get_pixel_rgba(0, 0).unwrap(), [0, 0, 0, 0]);
}

#[test]
fn image_diff_identical_and_different() {
    let mut p1 = ImagePixMap::new(4, 4, PixelFormat::RGBA);
    let p2 = ImagePixMap::new(4, 4, PixelFormat::RGBA);

    // Identical (both zeroed)
    let d = ImageDiff::new(0).compare(&p1, &p2).unwrap();
    assert!(d.is_equal());
    assert_eq!(d.nb_different_pixels, 0);
    assert!(d.rms_error < 1e-10);

    // Different: set a pixel in p1
    p1.set_pixel_rgba(0, 0, 255, 0, 0, 255);
    let d2 = ImageDiff::new(0).compare(&p1, &p2).unwrap();
    assert!(!d2.is_equal());
    assert!(d2.nb_different_pixels > 0);
    assert!(d2.max_channel_diff > 0);

    // Incompatible sizes → None
    let p3 = ImagePixMap::new(2, 2, PixelFormat::RGBA);
    assert!(ImageDiff::new(0).compare(&p1, &p3).is_none());
}

#[test]
fn image_alien_pixmap_load_stub() {
    let mut alien = ImageAlienPixMap::new();
    assert!(!alien.is_loaded());
    assert_eq!(alien.width(), 0);
    assert_eq!(alien.height(), 0);

    alien.load_stub("test/image.png", 640, 480, PixelFormat::RGB);
    assert!(alien.is_loaded());
    assert_eq!(alien.width(), 640);
    assert_eq!(alien.height(), 480);
    assert!(alien.pixmap().is_some());
}

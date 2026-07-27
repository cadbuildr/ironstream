// FILE: tests/occt_image_diff.rs
extern crate ironstream;

use ironstream::image_diff::{ImageDiff, ImageFormat, ImagePixMap};

#[test]
fn image_pixmap_fill_and_pixel_rgba8() {
    let mut img = ImagePixMap::new(4, 4, ImageFormat::Rgba8);
    img.fill([255, 128, 0, 255]);
    let px = img.pixel_rgba8(0, 0);
    assert_eq!(px, [255, 128, 0, 255]);
    let px2 = img.pixel_rgba8(3, 3);
    assert_eq!(px2, [255, 128, 0, 255]);
}

#[test]
fn image_diff_compare_identical_images() {
    let mut a = ImagePixMap::new(8, 8, ImageFormat::Rgba8);
    a.fill([100, 150, 200, 255]);
    let b = a.clone();
    let result = ImageDiff::new().compare(&a, &b);
    assert!(result.is_same);
    assert_eq!(result.nb_different_pixels, 0);
    assert!(result.rms_error.abs() < 1e-10);
}

#[test]
fn image_diff_compare_different_images() {
    let mut a = ImagePixMap::new(2, 2, ImageFormat::Rgba8);
    a.fill([0, 0, 0, 255]);
    let mut b = ImagePixMap::new(2, 2, ImageFormat::Rgba8);
    b.fill([255, 255, 255, 255]);
    let result = ImageDiff::new().compare(&a, &b);
    assert!(!result.is_same);
    assert_eq!(result.nb_different_pixels, 4);
    assert!(result.max_channel_diff > 0);
}

#[test]
fn image_format_bytes_per_pixel() {
    assert_eq!(ImageFormat::Gray8.bytes_per_pixel(), 1);
    assert_eq!(ImageFormat::Rgb8.bytes_per_pixel(), 3);
    assert_eq!(ImageFormat::Rgba8.bytes_per_pixel(), 4);
    assert_eq!(ImageFormat::Rgb32f.bytes_per_pixel(), 12);
    assert_eq!(ImageFormat::Rgba32f.bytes_per_pixel(), 16);
    assert!(!ImageFormat::Rgb8.has_alpha());
    assert!(ImageFormat::Rgba8.has_alpha());
}
